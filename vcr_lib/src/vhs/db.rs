use super::DataHeader;
use crate::{EntityDatabase, VCRError, VCRResult};
use memmap2::{Mmap, MmapOptions};
use serde::de::DeserializeOwned;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::marker::PhantomData;
use std::ops::Range;
use std::path::Path;
use vhs_diff::{patch_seq::*, Diff, Patch};
use zstd::bulk::Decompressor;
use zstd::dict::DecoderDictionary;

use crossbeam::channel;

pub struct Database<T: Clone + Patch + Send + Sync> {
    index: HashMap<[u8; 16], DataHeader>,
    id_list: Vec<[u8; 16]>,
    inner: Mmap,
    decoder: Option<DecoderDictionary<'static>>,
    _record_type: PhantomData<T>,
}

impl<T: Clone + Patch + DeserializeOwned + Send + Sync + serde::Serialize> Database<T> {
    pub fn from_single(path: impl AsRef<Path>) -> VCRResult<Database<T>> {
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

        let headers: Vec<DataHeader> =
            rmp_serde::from_read(zstd::Decoder::new(&header_bytes[..])?)?;

        let total_len = file.metadata()?.len() as usize;

        let inner = unsafe {
            MmapOptions::new()
                .offset((dict_len + header_len + 16) as u64)
                .len(total_len - (dict_len + header_len + 16))
                .map(&file)?
        };

        let index: HashMap<[u8; 16], DataHeader> = headers.into_iter().map(|v| (v.id, v)).collect();
        let id_list = index.keys().copied().collect();

        Ok(Database {
            index,
            id_list,
            decoder: dict,
            inner,
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
    pub fn get_all_entities(&self, at: u32) -> VCRResult<Vec<Option<(u32, T)>>> {
        self.get_entities_parallel(&self.id_list, at)
    }

    pub fn get_entities_parallel(
        &self,
        ids: &[[u8; 16]],
        at: u32,
    ) -> VCRResult<Vec<Option<(u32, T)>>> {
        crossbeam::scope(|s| {
            let chunks = ids.chunks(ids.len() / num_cpus::get());
            let n_chunks = chunks.len();
            let (tx, rx) = channel::bounded(n_chunks);

            for chunk in chunks {
                // unwraps inside scope will be caught, according to https://docs.rs/crossbeam/latest/crossbeam/fn.scope.html

                let mut decompressor = self.decompressor().unwrap();

                let tx = tx.clone();

                s.spawn(move |_| {
                    tx.send(
                        chunk
                            .iter()
                            .map(|id| self.get_entity_inner(id, at, &mut decompressor))
                            .collect::<VCRResult<Vec<Option<(u32, T)>>>>(),
                    )
                    .unwrap();
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
    fn get_entity_inner(
        &self,
        id: &[u8; 16],
        at: u32,
        decompressor: &mut Decompressor,
    ) -> VCRResult<Option<(u32, T)>> {
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

            let data = &self.inner
                [header.offset as usize..(header.offset + header.compressed_len) as usize];
            let decompressed = decompressor.decompress(data, header.decompressed_len as usize)?;

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

            return Ok(Some((entity_time, cur)));
        }

        Ok(None)
    }

    // pub fn get_versions_inner(
    //     &self,
    //     id: &[u8; 16],
    //     before: u32,
    //     after: u32,
    // ) -> VCRResult<Option<Vec<T>>> {
    //     if let Some(header) = self.index.get(id) {
    //         let end_index = match header.times.binary_search(&before) {
    //             Ok(i) => i,
    //             Err(i) => {
    //                 if i > 0 {
    //                     i - 1
    //                 } else {
    //                     i
    //                 }
    //             }
    //         };

    //         let start_index = match header.times.binary_search(&after) {
    //             Ok(i) => i,
    //             Err(i) => {
    //                 if i > 0 {
    //                     i - 1
    //                 } else {
    //                     i
    //                 }
    //             }
    //         };

    //         let start_checkpoint =
    //             (start_index - (start_index % header.checkpoint_every)) / header.checkpoint_every;
    //         let end_checkpoint =
    //             (end_index - (end_index % header.checkpoint_every)) / header.checkpoint_every;

    //         let data = &self.inner
    //             [header.offset as usize..(header.offset + header.compressed_len) as usize];
    //         let decompressed = self
    //             .decompressor()?
    //             .decompress(&data, header.decompressed_len as usize)?;

    //         let range = start_index..(end_index.checked_sub(1).unwrap_or(0));

    //         println!("{}, {}", start_checkpoint, end_checkpoint);

    //         let mut vec = Vec::new();

    //         self.get_version_range(&header, &mut vec, 0, 0..9, &decompressed[..])?;
    //         println!("{}", vec.len());
    //         println!("{}", serde_json::to_string_pretty(&vec).unwrap());
    //     }

    //     Ok(None)
    // }

    // fn get_version_range(
    //     &self,
    //     header: &DataHeader,
    //     out: &mut Vec<T>,
    //     checkpoint_index: usize,
    //     range: Range<usize>,
    //     decompressed: &[u8],
    // ) -> VCRResult<()> {
    //     let slice = if let Some(start_pos) = header.checkpoint_positions.get(checkpoint_index) {
    //         if let Some(next) = header.checkpoint_positions.get(start_pos + 1) {
    //             &decompressed[*start_pos..*next]
    //         } else {
    //             &decompressed[*start_pos..]
    //         }
    //     } else {
    //         &decompressed[..]
    //     };

    //     let mut deserializer = rmp_serde::Deserializer::from_read_ref(slice);
    //     let mut cur = T::deserialize(&mut deserializer)?;

    //     if range.contains(&0) {
    //         out.push(cur.clone());
    //     }

    //     PatchesToVec::apply_range(cur, out, range, &mut deserializer)?;

    //     Ok(())
    // }

    // fn get_versions_inner(
    //     &self,
    //     id: &[u8; 16],
    //     before: u32,
    //     after: u32,
    //     decompressor: &mut Decompressor,
    // ) -> VCRResult<Option<Vec<T>>> {
    // if let Some(header) = self.index.get(id) {
    //     let end_index = match header.times.binary_search(&before) {
    //         Ok(i) => i,
    //         Err(i) => {
    //             if i > 0 {
    //                 i - 1
    //             } else {
    //                 i
    //             }
    //         }
    //     };

    //     let start_index = match header.times.binary_search(&after) {
    //         Ok(i) => i,
    //         Err(i) => {
    //             if i > 0 {
    //                 i - 1
    //             } else {
    //                 i
    //             }
    //         }
    //     };

    // let range = start_index..(end_index.checked_sub(1).unwrap_or(0));

    // let mut versions = Vec::with_capacity(range.len());

    //         if start_index == 0 {
    //             versions.push(header.starter.clone());
    //         }

    // let data = &self.inner
    //     [header.offset as usize..(header.offset + header.compressed_len) as usize];
    // let decompressed = decompressor.decompress(&data, header.decompressed_len as usize)?;

    // let mut deserializer = rmp_serde::Deserializer::from_read_ref(&decompressed[..]);

    //         PatchesToVec::apply_range(
    //             header.starter.clone(),
    //             &mut versions,
    //             range,
    //             &mut deserializer,
    //         )?;

    //         Ok(Some(versions))
    //     } else {
    //         Ok(None)
    //     }
    // }
}

impl<T: Clone + Patch + Diff + DeserializeOwned + Send + Sync + serde::Serialize> EntityDatabase
    for Database<T>
{
    type Record = T;

    fn get_entity(&self, id: &[u8; 16], at: u32) -> VCRResult<Option<(u32, T)>> {
        let mut decompressor = self.decompressor()?;
        self.get_entity_inner(id, at, &mut decompressor)
    }

    fn get_entities(&self, ids: &[[u8; 16]], at: u32) -> VCRResult<Vec<Option<(u32, T)>>> {
        if ids.len() < num_cpus::get() {
            let mut decompressor = self.decompressor()?;

            return ids
                .iter()
                .map(|id| self.get_entity_inner(id, at, &mut decompressor))
                .collect::<VCRResult<Vec<Option<(u32, Self::Record)>>>>();
        }

        self.get_entities_parallel(ids, at)
    }

    // fn get_versions(&self, id: &[u8; 16], before: u32, after: u32) -> VCRResult<Option<Vec<T>>> {
    //     let mut decompressor = self.decompressor()?;

    //     self.get_versions_inner(id, before, after, &mut decompressor)
    // }

    fn all_ids(&self) -> &[[u8; 16]] {
        &self.id_list
    }
}
