use super::{event::*, EncodedBlockHeader, EventId};
use crate::VCRResult;
use rkyv::ser::{serializers::AllocSerializer, Serializer};
use std::collections::HashMap;
use std::io::Write;
use uuid::Uuid;
use zstd::bulk::Compressor;

pub struct FeedRecorder<H: Write, M: Write, A: Write> {
    header_out: H,
    feed_out: M,
    aux_out: A,                               // output for uuid_to_internal
    uuid_to_internal: HashMap<Uuid, [u8; 4]>, // map of event id -> (block index, event index relative to block)
    block_index: usize,
    compressor: Compressor<'static>,
    headers: Vec<EncodedBlockHeader>,
}

impl<H: Write, M: Write, A: Write> FeedRecorder<H, M, A> {
    pub fn new(
        header_out: H,
        feed_out: M,
        aux_out: A,
        dict: Option<Vec<u8>>,
        compression_level: i32,
    ) -> VCRResult<FeedRecorder<H, M, A>> {
        let mut compressor = if let Some(ref d) = dict {
            Compressor::with_dictionary(compression_level, d)?
        } else {
            Compressor::new(compression_level)?
        };

        compressor.include_dictid(false)?;
        compressor.include_magicbytes(false)?;

        Ok(FeedRecorder {
            header_out,
            feed_out,
            aux_out,
            compressor,
            uuid_to_internal: HashMap::with_capacity(5110062),
            block_index: 0,
            headers: Vec::new(),
        })
    }

    pub fn add_chunk(&mut self, chunk: Vec<FeedEvent>) -> VCRResult<()> {
        let mut ser = AllocSerializer::<1024>::default();
        let mut event_positions: Vec<(i64, u32)> = Vec::new();

        if chunk.is_empty() {
            return Ok(());
        }

        let start_time = chunk[0].created;

        for (idx, event) in chunk.into_iter().enumerate() {
            self.uuid_to_internal.insert(
                event.id,
                EventId::new()
                    .with_chunk(self.block_index.try_into().unwrap())
                    .with_idx(idx.try_into().unwrap())
                    .into_bytes(), // (
                                   //     self.block_index.try_into().unwrap(),
                                   //     idx.try_into().unwrap(),
                                   // ),
            );

            let created = event.created;
            let compact: CompactedFeedEvent = CompactedFeedEvent::convert(event);
            event_positions.push((created, ser.serialize_value(&compact).unwrap() as u32));
            // infallible
        }

        self.block_index += 1;

        event_positions.sort_by_key(|&(k, _)| k);

        let bytes = ser.into_serializer().into_inner();
        let compressed_bytes = self.compressor.compress(&bytes[..])?;

        self.headers.push(EncodedBlockHeader {
            compressed_len: compressed_bytes.len() as u32,
            decompressed_len: bytes.len() as u32,
            start_time,
            event_positions,
        });

        self.feed_out.write_all(&compressed_bytes)?;

        Ok(())
    }

    pub fn finish(mut self) -> VCRResult<(H, M, A)> {
        self.headers.sort_by_key(|v| v.start_time);
        let mut header_out = zstd::Encoder::new(self.header_out, 11)?;
        let header_bytes = rmp_serde::to_vec(&self.headers)?;

        header_out.set_pledged_src_size(Some(header_bytes.len() as u64))?;
        header_out.long_distance_matching(true)?;

        header_out.write_all(&header_bytes)?;
        header_out.flush()?;

        let header_handle = header_out.finish()?;
        self.feed_out.flush()?;

        rmp_serde::encode::write(&mut self.aux_out, &self.uuid_to_internal)?;

        Ok((header_handle, self.feed_out, self.aux_out))
    }
}

pub struct FeedDictTrainer {
    sample_sizes: Vec<usize>,
    samples: Vec<u8>,
}

impl FeedDictTrainer {
    pub fn new() -> FeedDictTrainer {
        FeedDictTrainer {
            sample_sizes: Vec::new(),
            samples: Vec::new(),
        }
    }

    pub fn add_chunk(&mut self, chunk: Vec<FeedEvent>) {
        let mut ser = AllocSerializer::<1024>::default();

        if chunk.is_empty() {
            return;
        }

        for event in chunk {
            let compact: CompactedFeedEvent = CompactedFeedEvent::convert(event);
            ser.serialize_value(&compact).unwrap(); // infallible
        }

        let bytes = ser.into_serializer().into_inner();
        self.sample_sizes.push(bytes.len());
        self.samples.extend_from_slice(&bytes[..]);
    }

    pub fn train(self, dict_size: usize) -> VCRResult<Vec<u8>> {
        Ok(zstd::dict::from_continuous(
            &self.samples,
            &self.sample_sizes,
            dict_size,
        )?)
    }
}
