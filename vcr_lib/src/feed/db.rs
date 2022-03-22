use super::block::*;
use super::{BlockHeader, EncodedBlockHeader};
use crate::VCRResult;
use memmap2::{Mmap, MmapOptions};
use std::fs::File;
use std::io::Read;
use std::path::Path;
use zstd::bulk::Decompressor;
use zstd::dict::DecoderDictionary;

pub struct FeedDatabase {
    inner: Mmap,
    dict: Option<DecoderDictionary<'static>>,
    headers: Vec<BlockHeader>,
}

impl FeedDatabase {
    pub fn from_single(path: impl AsRef<Path>) -> VCRResult<FeedDatabase> {
        let mut file = File::open(path)?;
        let mut len_bytes: [u8; 8] = [0; 8];
        file.read_exact(&mut len_bytes)?;

        let dict_len = u64::from_le_bytes(len_bytes) as usize;

        let dict = if dict_len > 0 {
            let mut dict = vec![0u8; dict_len];
            file.read_exact(&mut dict)?;
            Some(DecoderDictionary::copy(&dict[..]))
        } else {
            None
        };

        file.read_exact(&mut len_bytes)?;
        let header_len = u64::from_le_bytes(len_bytes) as usize;

        let mut header_bytes = vec![0u8; header_len];
        file.read_exact(&mut header_bytes)?;

        let raw_headers: Vec<EncodedBlockHeader> =
            rmp_serde::from_read(zstd::Decoder::new(&header_bytes[..])?)?;

        let headers: Vec<BlockHeader> = {
            let mut offset = 0;
            let mut headers: Vec<BlockHeader> = Vec::with_capacity(raw_headers.len());

            for header in raw_headers {
                let compressed_len = header.compressed_len;

                headers.push(BlockHeader {
                    compressed_len,
                    decompressed_len: header.decompressed_len,
                    start_time: header.start_time,
                    event_positions: header.event_positions,
                    offset,
                });

                offset += compressed_len;
            }

            headers
        };

        let total_len = file.metadata()?.len() as usize;

        let inner = unsafe {
            MmapOptions::new()
                .offset((dict_len + header_len + 16) as u64)
                .len(total_len - (dict_len + header_len + 16))
                .map(&file)?
        };

        Ok(FeedDatabase {
            headers,
            dict,
            inner,
        })
    }

    #[inline(always)]
    fn decompressor(&self) -> VCRResult<Decompressor> {
        let mut decompressor = if let Some(ref dict) = self.dict {
            Decompressor::with_prepared_dictionary(dict)?
        } else {
            Decompressor::new()?
        };

        decompressor.include_magicbytes(false)?;

        Ok(decompressor)
    }

    // returns (bytes, event_positions)
    pub fn get_block_by_time(&self, at: u64) -> VCRResult<EventBlock> {
        let index = match self.headers.binary_search_by_key(&at, |v| v.start_time) {
            Ok(i) => i,
            Err(i) => {
                if i > 0 {
                    i - 1
                } else {
                    i
                }
            }
        };

        let header = &self.headers[index];

        let data =
            &self.inner[header.offset as usize..(header.offset + header.compressed_len) as usize];
        let decompressed = self
            .decompressor()?
            .decompress(data, header.decompressed_len as usize)?;

        Ok(EventBlock {
            bytes: decompressed,
            event_positions: header.event_positions.clone(),
        })
    }
}
