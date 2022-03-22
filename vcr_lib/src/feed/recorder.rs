use super::{event::*, EncodedBlockHeader};
use crate::VCRResult;
use rkyv::ser::{serializers::AllocSerializer, Serializer};
use std::io::Write;
use zstd::bulk::Compressor;

pub struct FeedRecorder<H: Write, M: Write> {
    header_out: H,
    feed_out: M,
    compressor: Compressor<'static>,
    headers: Vec<EncodedBlockHeader>,
}

impl<H: Write, M: Write> FeedRecorder<H, M> {
    pub fn new(
        header_out: H,
        feed_out: M,
        dict: Option<Vec<u8>>,
        compression_level: i32,
    ) -> VCRResult<FeedRecorder<H, M>> {
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
            compressor,
            headers: Vec::new(),
        })
    }

    pub fn add_chunk(&mut self, chunk: Vec<FeedEvent>) -> VCRResult<()> {
        let mut ser = AllocSerializer::<1024>::default();
        let mut event_positions: Vec<(u64, u32)> = Vec::new();

        if chunk.is_empty() {
            return Ok(());
        }

        let start_time = chunk[0].created.timestamp_millis() as u64;

        for event in chunk {
            let created = event.created.timestamp_millis() as u64;
            let compact: CompactedFeedEvent = CompactedFeedEvent::convert(event);
            event_positions.push((
                created as u64,
                ser.serialize_value(&compact).unwrap() as u32,
            )); // infallible
        }

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

    pub fn finish(mut self) -> VCRResult<(H, M)> {
        self.headers.sort_by_key(|v| v.start_time);
        let mut header_out = zstd::Encoder::new(self.header_out, 11)?;
        let header_bytes = rmp_serde::to_vec(&self.headers)?;

        header_out.set_pledged_src_size(Some(header_bytes.len() as u64))?;
        header_out.long_distance_matching(true)?;

        header_out.write_all(&header_bytes)?;
        header_out.flush()?;

        let header_handle = header_out.finish()?;
        self.feed_out.flush()?;

        Ok((header_handle, self.feed_out))
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
