use super::block::*;
use super::{BlockHeader, EncodedBlockHeader};
use crate::VCRResult;
use memmap2::{Mmap, MmapOptions};
use moka::sync::Cache;
use serde::ser::{Error, Serialize, SerializeSeq, Serializer};
use std::cell::Cell;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::sync::Arc;
use std::time::Duration;
use xxhash_rust::xxh3;
use zstd::bulk::Decompressor;
use zstd::dict::DecoderDictionary;

pub struct FeedDatabase {
    inner: Mmap,
    dict: Option<DecoderDictionary<'static>>,
    headers: Vec<BlockHeader>,
    cache: Cache<u16, Arc<EventBlock>, xxh3::Xxh3Builder>,
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
            cache: Cache::builder()
                .max_capacity(100)
                .time_to_live(Duration::from_secs(20 * 60))
                .time_to_idle(Duration::from_secs(10 * 60))
                .build_with_hasher(xxh3::Xxh3Builder::new()),
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

    pub fn get_block_by_index(&self, index: u16) -> VCRResult<Arc<EventBlock>> {
        if let Some(cached) = self.cache.get(&index) {
            return Ok(cached);
        }

        let header = &self.headers[index as usize];

        let data =
            &self.inner[header.offset as usize..(header.offset + header.compressed_len) as usize];
        let decompressed = self
            .decompressor()?
            .decompress(data, header.decompressed_len as usize)?;

        let block = Arc::new(EventBlock {
            bytes: decompressed,
            event_positions: header.event_positions.clone(),
        });

        self.cache.insert(index, Arc::clone(&block));

        Ok(block)
    }

    pub fn get_block_by_time(&self, at: i64) -> VCRResult<Arc<EventBlock>> {
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

        self.get_block_by_index(index as u16)
    }

    pub fn get_events_after(&self, after: i64, count: usize) -> EventRangeSerializer<'_> {
        let block_index = match self.headers.binary_search_by_key(&after, |v| v.start_time) {
            Ok(i) => i,
            Err(i) => {
                if i > 0 {
                    i - 1
                } else {
                    i
                }
            }
        } as u16;

        EventRangeSerializer {
            inner: Cell::new(Some(EventRangeIter {
                db: self,
                block_index,
                go_up: true,
                count_left: count,
            })),
            before: i64::MAX,
            after,
        }
    }

    pub fn get_events_before(&self, before: i64, count: usize) -> EventRangeSerializer<'_> {
        let block_index = match self.headers.binary_search_by_key(&before, |v| v.start_time) {
            Ok(i) => i,
            Err(i) => {
                if i > 0 {
                    i - 1
                } else {
                    i
                }
            }
        } as u16;

        EventRangeSerializer {
            inner: Cell::new(Some(EventRangeIter {
                db: self,
                block_index,
                go_up: false,
                count_left: count,
            })),
            before,
            after: 0,
        }
    }
}

pub struct EventRangeSerializer<'a> {
    inner: Cell<Option<EventRangeIter<'a>>>,
    before: i64,
    after: i64,
}

impl<'a> Serialize for EventRangeSerializer<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut iter = self.inner.take().ok_or(S::Error::custom(
            "NEventsAfterSerializer: inner iterator already consumed",
        ))?;
        let mut seq = serializer.serialize_seq(Some(iter.count_left))?;

        while let Some(block) = iter.next() {
            let block = block.map_err(S::Error::custom)?;
            let mut events = block.all_events();

            if !iter.go_up {
                events.retain(|v| v.created < self.before);
            } else {
                events.retain(|v| v.created > self.after);
            }

            if !iter.go_up {
                events.reverse();
            }

            events.truncate(iter.count_left);

            iter.count_left -= events.len();

            for event in events {
                seq.serialize_element(&event)?;
            }
        }

        seq.end()
    }
}

pub struct EventRangeIter<'a> {
    db: &'a FeedDatabase,
    block_index: u16,
    go_up: bool, // if true, increment block_index every step; else, subtract
    count_left: usize,
}

impl<'a> Iterator for EventRangeIter<'a> {
    type Item = VCRResult<Arc<EventBlock>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.go_up && self.block_index as usize >= self.db.headers.len() {
            return None;
        }

        let block = match self.db.get_block_by_index(self.block_index) {
            Ok(v) => v,
            Err(e) => return Some(Err(e)),
        };

        if self.go_up {
            self.block_index = self.block_index.checked_add(1)?;
        } else {
            self.block_index = self.block_index.checked_sub(1)?;
        }

        Some(Ok(block))
    }
}
