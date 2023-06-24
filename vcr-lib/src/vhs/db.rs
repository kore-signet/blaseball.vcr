use super::{split_tape, DataHeader, TapeComponents};
use crate::chron_types::*;
use crate::{EntityDatabase, OptionalEntity, VCRError, VCRResult};
use crossbeam::channel;
use memmap2::{Mmap, MmapOptions};
use moka::sync::Cache;
use serde::de::DeserializeOwned;
use std::collections::HashMap;
use std::fs::File;

use std::marker::PhantomData;
use std::ops::Range;
use std::path::Path;
use std::sync::Arc;
use std::time::Duration;
use vhs_diff::{patch_seq::*, Diff, Patch};
use xxhash_rust::xxh3;
use zstd::bulk::Decompressor;
use zstd::dict::DecoderDictionary;

type RangeTuple = (usize, usize);

pub struct Database<T: Clone + Patch + Send + Sync> {
    pub index: HashMap<[u8; 16], DataHeader>,
    pub id_list: Vec<[u8; 16]>,
    inner: Mmap,
    decoder: Option<DecoderDictionary<'static>>,
    cache: Cache<RangeTuple, Arc<Vec<u8>>, xxh3::Xxh3Builder>,
    _record_type: PhantomData<T>,
}

impl<T: Clone + Patch + DeserializeOwned + Send + Sync + serde::Serialize> Database<T> {
    pub fn from_single(path: impl AsRef<Path>) -> VCRResult<Database<T>> {
        // let mut file = File::open(path)?;
        // let mut len_bytes: [u8; 8] = [0; 8];
        // file.read_exact(&mut len_bytes)?;

        // let dict_len = u64::from_le_bytes(len_bytes) as usize;

        // let dict = if dict_len > 0 {
        //     let mut dict = vec![0u8; dict_len];
        //     file.read_exact(&mut dict)?;
        //     Some(DecoderDictionary::copy(&dict[..]))
        // } else {
        //     None
        // };

        // file.read_exact(&mut len_bytes)?;
        // let header_len = u64::from_le_bytes(len_bytes) as usize;

        // let mut header_bytes = vec![0u8; header_len];
        // file.read_exact(&mut header_bytes)?;

        // let headers: Vec<DataHeader> =
        //     rmp_serde::from_read(zstd::Decoder::new(&header_bytes[..])?)?;

        // let total_len = file.metadata()?.len() as usize;

        // let inner = unsafe {
        //     MmapOptions::new()
        //         .offset((dict_len + header_len + 16) as u64)
        //         .len(total_len - (dict_len + header_len + 16))
        //         .map(&file)?
        // };

        let TapeComponents {
            dict,
            header,
            store,
        } = split_tape::<Vec<DataHeader>>(path)?;

        let index: HashMap<[u8; 16], DataHeader> = header.into_iter().map(|v| (v.id, v)).collect();
        let id_list = index.keys().copied().collect();

        Ok(Database {
            index,
            id_list,
            decoder: dict,
            inner: store,
            cache: Cache::builder()
                .max_capacity(100)
                .time_to_live(Duration::from_secs(20 * 60))
                .time_to_idle(Duration::from_secs(10 * 60))
                .build_with_hasher(xxh3::Xxh3Builder::new()),
            _record_type: PhantomData,
        })
    }

    pub fn from_files(
        header: impl AsRef<Path>,
        database: impl AsRef<Path>,
        dict: impl AsRef<Path>,
        pre_populate: bool,
    ) -> VCRResult<Database<T>> {
        let header_f = File::open(header)?;
        let database_f = File::open(database)?;
        let dict = std::fs::read(dict)?;

        let headers: Vec<DataHeader> = rmp_serde::from_read(zstd::Decoder::new(header_f)?)?;
        let inner = unsafe {
            let mut mmap = &mut MmapOptions::new();
            if pre_populate {
                mmap = mmap.populate();
            };

            mmap.map(&database_f)?
        };

        let index: HashMap<[u8; 16], DataHeader> = headers.into_iter().map(|v| (v.id, v)).collect();
        let id_list = index.keys().copied().collect();

        Ok(Database {
            index,
            id_list,
            decoder: Some(DecoderDictionary::copy(&dict)),
            inner,
            cache: Cache::builder()
                .max_capacity(100)
                .time_to_live(Duration::from_secs(20 * 60))
                .time_to_idle(Duration::from_secs(10 * 60))
                .build_with_hasher(xxh3::Xxh3Builder::new()),
            _record_type: PhantomData,
        })
    }

    #[inline(always)]
    fn decompressor(&self) -> VCRResult<Decompressor> {
        let mut decompressor = if let Some(ref dict) = self.decoder {
            Decompressor::with_prepared_dictionary(dict)?
        } else {
            Decompressor::new()?
        };

        decompressor.include_magicbytes(false)?;

        Ok(decompressor)
    }

    #[inline(always)]
    pub fn get_all_entities(&self, at: i64) -> VCRResult<Vec<OptionalEntity<T>>> {
        self.get_entities_parallel(&self.id_list, at)
    }

    pub fn get_entities_parallel(
        &self,
        ids: &[[u8; 16]],
        at: i64,
    ) -> VCRResult<Vec<OptionalEntity<T>>> {
        crossbeam::scope(|s| {
            let chunks = ids.chunks(ids.len() / num_cpus::get());
            let n_chunks = chunks.len();
            let (tx, rx) = channel::unbounded();

            for chunk in chunks {
                // unwraps inside scope will be caught, according to https://docs.rs/crossbeam/latest/crossbeam/fn.scope.html

                let mut decompressor = self.decompressor().unwrap();

                let tx = tx.clone();

                s.spawn(move |_| {
                    let data = chunk
                        .iter()
                        .map(|id| self.get_entity_inner(id, at, &mut decompressor))
                        .collect::<VCRResult<Vec<OptionalEntity<T>>>>();

                    tx.send(data)
                });
            }

            let mut res = Vec::with_capacity(self.id_list.len());
            for _ in 0..n_chunks {
                let mut val = rx.recv().unwrap()?;
                res.append(&mut val);
            }

            Ok(res)
        })
        .map_err(|_| VCRError::ParallelError)?
    }

    #[inline(always)]
    fn get_data_range(
        &self,
        range: Range<usize>,
        decompressed_len: usize,
        decompressor: &mut Decompressor,
    ) -> VCRResult<Arc<Vec<u8>>> {
        let range = (range.start, range.end);
        if let Some(data) = self.cache.get(&range) {
            return Ok(data);
        }

        let data = &self.inner[Range {
            start: range.0,
            end: range.1,
        }];
        let decompressed = Arc::new(decompressor.decompress(data, decompressed_len)?);
        self.cache.insert(range, Arc::clone(&decompressed));
        Ok(decompressed)
    }

    fn get_entity_inner(
        &self,
        id: &[u8; 16],
        at: i64,
        decompressor: &mut Decompressor,
    ) -> VCRResult<OptionalEntity<T>> {
        if let Some(header) = self.index.get(id) {
            let index = match header.times.binary_search(&at) {
                Ok(i) => i,
                Err(i) => {
                    if i > 0 {
                        i - 1
                    } else {
                        i
                    }
                }
            };

            let entity_time = header.times[index];

            if entity_time > at {
                return Ok(None);
            }

            let decompressed = self.get_data_range(
                header.offset as usize..(header.offset + header.compressed_len) as usize,
                header.decompressed_len as usize,
                decompressor,
            )?;

            let checkpoint_index =
                (index - (index % header.checkpoint_every)) / header.checkpoint_every;

            let slice = if let Some(start_pos) = header.checkpoint_positions.get(checkpoint_index) {
                if let Some(next) = header.checkpoint_positions.get(start_pos + 1) {
                    &decompressed[*start_pos..*next]
                } else {
                    &decompressed[*start_pos..]
                }
            } else {
                &decompressed[..]
            };

            let mut deserializer = rmp_serde::Deserializer::from_read_ref(slice);
            let mut cur = T::deserialize(&mut deserializer)?;

            // if there's patches remaining, apply 'em
            if index % header.checkpoint_every > 0 {
                ApplyPatches::apply(
                    &mut cur,
                    (index % header.checkpoint_every) - 1,
                    &mut deserializer,
                )?;
            }

            return Ok(Some(ChroniclerEntity {
                entity_id: *id,
                valid_from: entity_time,
                data: cur,
            }));
        }

        Ok(None)
    }

    fn get_first_entity_inner(
        &self,
        id: &[u8; 16],
        decompressor: &mut Decompressor,
    ) -> VCRResult<OptionalEntity<T>> {
        if let Some(first_time) = self.index.get(id).and_then(|header| header.times.first()) {
            self.get_entity_inner(id, *first_time, decompressor)
        } else {
            Ok(None)
        }
    }

    // TODO: we need to add times here
    fn get_versions_inner(
        &self,
        id: &[u8; 16],
        before: i64,
        after: i64,
        decompressor: &mut Decompressor,
    ) -> VCRResult<Option<Vec<ChroniclerEntity<T>>>> {
        if let Some(header) = self.index.get(id) {
            let end_index = match header.times.binary_search(&before) {
                Ok(i) => i,
                Err(i) => {
                    if i > 0 {
                        i - 1
                    } else {
                        i
                    }
                }
            };

            let start_index = match header.times.binary_search(&after) {
                Ok(i) => i,
                Err(i) => {
                    if i > 0 {
                        i - 1
                    } else {
                        i
                    }
                }
            };

            let times = &header.times[start_index..=end_index];

            let start_checkpoint =
                (start_index - (start_index % header.checkpoint_every)) / header.checkpoint_every;
            let end_checkpoint =
                (end_index - (end_index % header.checkpoint_every)) / header.checkpoint_every;

            let decompressed = self.get_data_range(
                header.offset as usize..(header.offset + header.compressed_len) as usize,
                header.decompressed_len as usize,
                decompressor,
            )?;

            let mut out = Vec::with_capacity(end_index - start_index);

            // if the versions are in a single checkpoint range, we can just return that.
            if start_checkpoint == end_checkpoint {
                let start_index = start_index % header.checkpoint_every;
                let end_index = end_index % header.checkpoint_every;

                let range = start_index..end_index - 1;

                self.get_version_range(
                    header,
                    &mut out,
                    start_checkpoint,
                    range,
                    &decompressed[..],
                    true,
                )?;
            // else, if the versions are spread across two consecutive ranges,
            } else if start_checkpoint + 1 == end_checkpoint {
                // we deserialize the first range, sliced from the starting index to it's end
                let start_index = start_index % header.checkpoint_every;
                let range = start_index..usize::MAX;
                self.get_version_range(
                    header,
                    &mut out,
                    start_checkpoint,
                    range,
                    &decompressed[..],
                    true,
                )?;

                // then, we get the ending range, sliced from it's start to the end index
                let end_index = end_index % header.checkpoint_every;
                let range = 0..end_index;
                self.get_version_range(
                    header,
                    &mut out,
                    end_checkpoint,
                    range,
                    &decompressed[..],
                    false,
                )?
            // else, if the versions are spread across multiple checkpoint ranges
            } else if end_checkpoint > start_checkpoint {
                // we make an iterator of all the indices
                let middle_checkpoint_indices = start_checkpoint + 1..=end_checkpoint - 1;

                // we get the first range
                let start_index = start_index % header.checkpoint_every;
                let range = start_index..usize::MAX;
                self.get_version_range(
                    header,
                    &mut out,
                    start_checkpoint,
                    range,
                    &decompressed[..],
                    true,
                )?;

                // we apply all the middle ranges fully
                for check_idx in middle_checkpoint_indices {
                    self.get_version_range(
                        header,
                        &mut out,
                        check_idx,
                        0..usize::MAX,
                        &decompressed[..],
                        false,
                    )?;
                }

                // we apply the last range
                let end_index = end_index % header.checkpoint_every;
                let range = 0..end_index + 1;
                self.get_version_range(
                    header,
                    &mut out,
                    end_checkpoint,
                    range,
                    &decompressed[..],
                    false,
                )?
            }

            return Ok(Some(
                out.into_iter()
                    .enumerate()
                    .map(|(i, entity_data)| ChroniclerEntity {
                        entity_id: *id,
                        valid_from: times[i],
                        data: entity_data,
                    })
                    .collect(),
            ));
        }

        Ok(None)
    }

    fn get_version_range(
        &self,
        header: &DataHeader,
        out: &mut Vec<T>,
        checkpoint_index: usize,
        range: Range<usize>,
        decompressed: &[u8],
        add_cur: bool,
    ) -> VCRResult<()> {
        let slice = if let Some(start_pos) = header.checkpoint_positions.get(checkpoint_index) {
            if let Some(next) = header.checkpoint_positions.get(start_pos + 1) {
                &decompressed[*start_pos..*next]
            } else {
                &decompressed[*start_pos..]
            }
        } else {
            decompressed
        };

        let mut deserializer = rmp_serde::Deserializer::from_read_ref(slice);
        let cur = T::deserialize(&mut deserializer)?;

        if add_cur {
            out.push(cur.clone());
        }

        PatchesToVec::apply_range(cur, out, range, &mut deserializer)?;

        Ok(())
    }
}

impl<T: Clone + Patch + Diff + DeserializeOwned + Send + Sync + serde::Serialize + 'static>
    EntityDatabase for Database<T>
{
    type Record = T;

    fn get_entity(&self, id: &[u8; 16], at: i64) -> VCRResult<OptionalEntity<T>> {
        let mut decompressor = self.decompressor()?;
        self.get_entity_inner(id, at, &mut decompressor)
    }

    fn get_first_entity(&self, id: &[u8; 16]) -> VCRResult<OptionalEntity<Self::Record>> {
        let mut decompressor = self.decompressor()?;
        self.get_first_entity_inner(id, &mut decompressor)
    }

    fn get_first_entities(&self, ids: &[[u8; 16]]) -> VCRResult<Vec<OptionalEntity<Self::Record>>> {
        let mut decompressor = self.decompressor()?;
        let mut result = Vec::with_capacity(ids.len());
        for id in ids {
            result.push(self.get_first_entity_inner(id, &mut decompressor)?);
        }

        Ok(result)
    }

    fn get_next_time(&self, id: &[u8; 16], at: i64) -> Option<i64> {
        self.index.get(id).and_then(|header| {
            header
                .times
                .get(match header.times.binary_search(&at) {
                    Ok(idx) => idx,
                    Err(idx) => idx,
                })
                .copied()
        })
    }

    fn get_entities(&self, ids: &[[u8; 16]], at: i64) -> VCRResult<Vec<OptionalEntity<T>>> {
        if ids.len() < num_cpus::get() {
            let mut decompressor = self.decompressor()?;

            return ids
                .iter()
                .map(|id| self.get_entity_inner(id, at, &mut decompressor))
                .collect::<VCRResult<Vec<OptionalEntity<T>>>>();
        }

        self.get_entities_parallel(ids, at)
    }

    fn get_versions(
        &self,
        id: &[u8; 16],
        before: i64,
        after: i64,
    ) -> VCRResult<Option<Vec<ChroniclerEntity<T>>>> {
        let mut decompressor = self.decompressor()?;

        self.get_versions_inner(id, before, after, &mut decompressor)
    }

    fn all_ids(&self) -> &[[u8; 16]] {
        &self.id_list
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
