use crate::*;

use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs::{read_dir, File};
use std::io::{self, prelude::*};
use std::path::{Path, PathBuf};

use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use memmap2::{Mmap, MmapOptions};
use serde_json::{json, value::RawValue, Value as JSONValue};
use zstd::dict::DecoderDictionary;

use moka::sync::Cache;

use sha2::Digest;

use rayon::prelude::*;

use json_patch::{
    patch_unsafe as patch_json, AddOperation, CopyOperation, MoveOperation, Patch as JSONPatch,
    PatchOperation, PatchOperation::*, RemoveOperation, ReplaceOperation, TestOperation,
};

fn clamp(input: u32, min: u32, max: u32) -> u32 {
    if input < min {
        min
    } else if input > max {
        max
    } else {
        input
    }
}

/// Serializes the data inside a list of ChroniclerEntity's, adding a SHA224 hash of the JSON string to the object.
pub fn hash_entities(
    e: Vec<ChroniclerEntity<JSONValue>>,
) -> VCRResult<Vec<ChroniclerEntity<Box<RawValue>>>> {
    let mut hasher = sha2::Sha224::new();
    e.into_iter()
        .filter(|v| v.data != json!({}))
        .map(|v| {
            let mut buf = Vec::new();
            let mut ser = serde_json::Serializer::new(&mut buf);
            v.data.serialize(&mut ser)?;
            hasher.update(&buf);
            Ok(ChroniclerEntity {
                entity_id: v.entity_id,
                valid_from: v.valid_from,
                valid_to: v.valid_to,
                hash: format!("{:x}", hasher.finalize_reset()),
                data: RawValue::from_string(String::from_utf8(buf).unwrap())?,
            })
        })
        .collect()
}

/// A handle over a memory map of a VCR .riv file, a mapping of entity ids to positions in the file, a possible ZSTD dictionary, and a cache.
pub struct Database {
    reader: Mmap,
    entities: HashMap<String, EntityData>,
    dictionary: Option<DecoderDictionary<'static>>,
    entity_cache: Cache<(String, usize), ChroniclerEntity<JSONValue>>,
}

impl Database {
    pub fn from_files<P: AsRef<Path> + std::fmt::Debug>(
        entities_lookup_path: P,
        db_path: P,
        dict_path: Option<P>,
        cache_size: usize,
    ) -> VCRResult<Database> {
        let entities_lookup_f = File::open(entities_lookup_path)?;
        let decompressor = zstd::stream::Decoder::new(entities_lookup_f)?;
        let db_f = File::open(db_path)?;

        let compression_dict = if let Some(dict_f_path) = dict_path {
            let mut dict_f = File::open(dict_f_path)?;
            let mut dict = Vec::new();
            dict_f.read_to_end(&mut dict)?;
            Some(DecoderDictionary::copy(&dict))
        } else {
            None
        };

        Ok(Database {
            reader: unsafe { MmapOptions::new().map(&db_f)? },
            entities: decode_header(decompressor)?,
            dictionary: compression_dict,
            entity_cache: Cache::new(cache_size),
        })
    }

    /// Gets the last version of an entity, which is serialized as a standalone MSGPack object to avoid the patch system.
    pub fn get_last_version(&self, entity: &str) -> VCRResult<(u32, JSONValue)> {
        let metadata = &self.entities.get(entity).ok_or(VCRError::EntityNotFound)?;
        let (time, patch_start, patch_len) =
            *metadata.patches.last().ok_or(VCRError::InvalidPatchData)?;
        let e_bytes: Vec<u8> = if let Some(compress_dict) = &self.dictionary {
            let mut decoder = zstd::stream::Decoder::with_prepared_dictionary(
                &self.reader[(patch_start as usize)..(patch_start + patch_len) as usize],
                compress_dict,
            )?;
            let mut res = Vec::with_capacity((patch_len) as usize * 10);
            decoder.read_to_end(&mut res)?;
            res
        } else {
            let mut decoder = zstd::stream::Decoder::new(
                &self.reader[(patch_start as usize)..(patch_start + patch_len) as usize],
            )?;
            let mut res = Vec::with_capacity((patch_len) as usize * 10);
            decoder.read_to_end(&mut res)?;
            res
        };

        Ok((time, rmp_serde::from_read_ref(&e_bytes)?))
    }

    /// Gets the JSONPatch'es associated with a specific entity until a certain time.
    pub fn get_entity_data(
        &self,
        entity: &str,
        until: u32,
        skip_to_checkpoint: bool,
        from_index: usize,
    ) -> VCRResult<Vec<(u32, Patch)>> {
        let metadata = &self.entities.get(entity).ok_or(VCRError::EntityNotFound)?;
        let mut patches: Vec<(u32, Patch)> = Vec::new();

        let patch_list: Vec<(u32, u32, u32)> = if skip_to_checkpoint {
            let patches_until: Vec<(u32, u32, u32)> = metadata
                .patches
                .split_last()
                .unwrap()
                .1
                .iter()
                .copied()
                .take_while(|x| x.0 <= until)
                .collect();
            let latest_check_idx = patches_until.len().saturating_sub(1);

            let closest_checkpoint =
                latest_check_idx - (latest_check_idx % metadata.checkpoint_every as usize);
            patches_until[closest_checkpoint..].to_vec()
        } else {
            metadata
                .patches
                .split_last()
                .unwrap()
                .1
                .iter()
                .skip(from_index)
                .copied()
                .take_while(|x| x.0 <= until)
                .collect()
        };

        for (time, patch_start, patch_len) in patch_list {
            let mut e_bytes: Vec<u8> = if let Some(compress_dict) = &self.dictionary {
                let mut decoder = zstd::stream::Decoder::with_prepared_dictionary(
                    &self.reader[(patch_start as usize)..(patch_start + patch_len) as usize],
                    compress_dict,
                )?;
                let mut res = Vec::with_capacity((patch_len) as usize * 10);
                decoder.read_to_end(&mut res)?;
                res
            } else {
                let mut decoder = zstd::stream::Decoder::new(
                    &self.reader[(patch_start as usize)..(patch_start + patch_len) as usize],
                )?;
                let mut res = Vec::with_capacity((patch_len) as usize * 10);
                decoder.read_to_end(&mut res)?;
                res
            };

            let mut result = Patch::Normal(JSONPatch(vec![]));
            let mut operations: Vec<PatchOperation> = Vec::new();

            while e_bytes.len() > 1 {
                let op_code = e_bytes.remove(0);

                if op_code == 6 {
                    let value_length = u16::from_be_bytes([e_bytes.remove(0), e_bytes.remove(0)]);
                    let val_bytes: Vec<u8> = e_bytes.drain(..value_length as usize).collect();
                    result = Patch::ReplaceRoot(rmp_serde::from_read_ref(&val_bytes)?);
                    break;
                } else {
                    let paths = if op_code == 3 || op_code == 4 {
                        vec![
                            metadata
                                .path_map
                                .get(&u16::from_be_bytes([e_bytes.remove(0), e_bytes.remove(0)]))
                                .ok_or(VCRError::PathResolutionError)?,
                            metadata
                                .path_map
                                .get(&u16::from_be_bytes([e_bytes.remove(0), e_bytes.remove(0)]))
                                .ok_or(VCRError::PathResolutionError)?,
                        ]
                    } else {
                        vec![metadata
                            .path_map
                            .get(&u16::from_be_bytes([e_bytes.remove(0), e_bytes.remove(0)]))
                            .ok_or(VCRError::PathResolutionError)?]
                    };

                    let value_length = u16::from_be_bytes([e_bytes.remove(0), e_bytes.remove(0)]);

                    let value: Option<JSONValue> = if value_length > 0 {
                        let val_bytes: Vec<u8> = e_bytes.drain(..value_length as usize).collect();
                        Some(rmp_serde::from_read_ref(&val_bytes)?)
                    } else {
                        None
                    };

                    operations.push(match op_code {
                        0 => Add(AddOperation {
                            path: paths[0].to_string(),
                            value: value.ok_or(VCRError::InvalidPatchData)?,
                        }),
                        1 => Remove(RemoveOperation {
                            path: paths[0].to_string(),
                        }),
                        2 => Replace(ReplaceOperation {
                            path: paths[0].to_string(),
                            value: value.ok_or(VCRError::InvalidPatchData)?,
                        }),
                        3 => Move(MoveOperation {
                            path: paths[0].to_string(),
                            from: paths[1].to_string(),
                        }),
                        4 => Copy(CopyOperation {
                            path: paths[0].to_string(),
                            from: paths[1].to_string(),
                        }),
                        5 => Test(TestOperation {
                            path: paths[0].to_string(),
                            value: value.ok_or(VCRError::InvalidPatchData)?,
                        }),
                        _ => return Err(VCRError::InvalidOpCode),
                    });
                }
            }

            patches.push((
                time,
                match result {
                    Patch::Normal(_) => Patch::Normal(JSONPatch(operations)),
                    Patch::ReplaceRoot(v) => Patch::ReplaceRoot(v),
                },
            ));
        }

        patches.sort_by_key(|x| x.0);
        Ok(patches)
    }

    /// Gets all versions of an entity between two UNIX timestamps.
    pub fn get_entity_versions(
        &self,
        entity: &str,
        before: u32,
        after: u32,
    ) -> VCRResult<Vec<ChroniclerEntity<JSONValue>>> {
        let mut entity_value = self
            .entities
            .get(entity)
            .ok_or(VCRError::EntityNotFound)?
            .base
            .clone();
        let patches = self.get_entity_data(entity, before, false, 0)?;
        let patches_len = patches.len();
        let mut results: Vec<ChroniclerEntity<JSONValue>> = Vec::with_capacity(patches_len);

        for slice in patches.windows(2) {
            let time = slice[0].0;
            let next_time = slice[1].0;
            let patch: &json_sequences::Patch = &slice[0].1;

            match patch {
                Patch::ReplaceRoot(v) => {
                    entity_value = v.clone();
                }
                Patch::Normal(p) => {
                    patch_json(&mut entity_value, &p)?;
                }
            }

            if time > after {
                results.push(ChroniclerEntity {
                    data: entity_value.clone(),
                    entity_id: entity.to_owned(),
                    valid_from: DateTime::<Utc>::from_utc(
                        NaiveDateTime::from_timestamp(time as i64, 0),
                        Utc,
                    ),
                    valid_to: Some(DateTime::<Utc>::from_utc(
                        NaiveDateTime::from_timestamp(next_time as i64, 0),
                        Utc,
                    ).to_rfc3339()),
                    hash: String::from("ahh"),
                });
            }
        }

        let (time, patch) = &patches[patches_len - 1];

        match patch {
            Patch::ReplaceRoot(v) => {
                entity_value = v.clone();
            }
            Patch::Normal(p) => {
                patch_json(&mut entity_value, &p)?;
            }
        }

        if time > &after {
            results.push(ChroniclerEntity {
                data: entity_value.clone(),
                entity_id: entity.to_owned(),
                valid_from: DateTime::<Utc>::from_utc(
                    NaiveDateTime::from_timestamp(*time as i64, 0),
                    Utc,
                ),
                valid_to: None,
                hash: String::from("ahh2"),
            });
        }

        Ok(results)
    }

    pub fn get_next_time(&self, entity: &str, at: u32) -> u32 {
        self.entities[entity].patches[match self.entities[entity]
            .patches
            .binary_search_by_key(&at, |(t, _, _)| *t)
        {
            Ok(idx) => idx,
            Err(idx) => idx,
        }]
        .0
    }

    /// Gets an entity at a certain point in time.
    pub fn get_entity(&self, entity: &str, at: u32) -> VCRResult<ChroniclerEntity<JSONValue>> {
        let patch_idx = match self.entities[entity]
            .patches
            .binary_search_by_key(&at, |(t, _, _)| *t)
        {
            Ok(idx) => idx,
            Err(idx) => idx.saturating_sub(1),
        };

        if patch_idx > 0 {
            if let Some(val) = self.entity_cache.get(&(entity.to_owned(), patch_idx)) {
                return Ok(val);
            } else if patch_idx == self.entities[entity].patches.len() - 1 {
                let (time, data) = self.get_last_version(entity)?;
                return Ok(ChroniclerEntity {
                    data: data,
                    entity_id: entity.to_owned(),
                    valid_from: DateTime::<Utc>::from_utc(
                        NaiveDateTime::from_timestamp(time as i64, 0),
                        Utc,
                    ),
                    valid_to: None,
                    hash: String::new(),
                });
            }
        }

        let mut entity_value = self
            .entities
            .get(entity)
            .ok_or(VCRError::EntityNotFound)?
            .base
            .clone();

        let mut patch_data_idx = 0;

        if self.entities[entity].checkpoint_every != u16::MAX && patch_idx > 0 {
            if let Some(val) = self.entity_cache.get(&(entity.to_owned(), patch_idx - 1)) {
                entity_value = val.data;
                patch_data_idx = patch_idx - 1;
            }
        }

        let mut last_time = 0;

        for (time, patch) in self.get_entity_data(entity, at, true, patch_data_idx)? {
            match patch {
                Patch::ReplaceRoot(v) => {
                    entity_value = v.clone();
                }
                Patch::Normal(p) => {
                    patch_json(&mut entity_value, &p)?;
                }
            }
            last_time = time;
        }

        let e = ChroniclerEntity {
            data: entity_value,
            entity_id: entity.to_owned(),
            valid_from: DateTime::<Utc>::from_utc(
                NaiveDateTime::from_timestamp(last_time as i64, 0),
                Utc,
            ),
            valid_to: None,
            hash: String::new(),
        };

        if patch_idx != 0 {
            self.entity_cache
                .insert((entity.to_owned(), patch_idx), e.clone());
        }

        Ok(e)
    }

    /// Gets the very first version of an entity.
    pub fn get_first_entity(&self, entity: &str) -> VCRResult<ChroniclerEntity<JSONValue>> {
        let mut entity_value = self
            .entities
            .get(entity)
            .ok_or(VCRError::EntityNotFound)?
            .base
            .clone();

        let patches = self.get_entity_data(entity, u32::MAX, true, 0)?;
        let (time, patch) = &patches[0];

        match patch {
            Patch::ReplaceRoot(v) => {
                entity_value = v.clone();
            }
            Patch::Normal(p) => {
                patch_json(&mut entity_value, p)?;
            }
        }

        Ok(ChroniclerEntity {
            data: entity_value,
            entity_id: entity.to_owned(),
            valid_from: DateTime::<Utc>::from_utc(
                NaiveDateTime::from_timestamp(*time as i64, 0),
                Utc,
            ),
            valid_to: None,
            hash: String::new(),
        })
    }

    /// Fetches (in parallel) a list of entities at a certain time.
    pub fn get_entities(
        &self,
        entities: Vec<String>,
        at: u32,
    ) -> VCRResult<Vec<ChroniclerEntity<JSONValue>>> {
        entities
            .par_iter()
            .map(|e| self.get_entity(e, at))
            .collect()
    }

    /// Fetches (in parallel) all the versions of a list of entities between two UNIX timestamps.
    pub fn get_entities_versions(
        &self,
        entities: Vec<String>,
        before: u32,
        after: u32,
    ) -> VCRResult<Vec<ChroniclerEntity<JSONValue>>> {
        Ok(entities
            .par_iter()
            .map(|e| self.get_entity_versions(e, before, after))
            .collect::<VCRResult<Vec<Vec<ChroniclerEntity<JSONValue>>>>>()?
            .concat())
    }

    /// Gets all entities registered at a certain point in time.
    pub fn all_entities(&self, at: u32) -> VCRResult<Vec<ChroniclerEntity<JSONValue>>> {
        self.entities
            .par_iter()
            .map(|(e, _)| self.get_entity(e, at))
            .collect()
    }

    /// Gets the versions of all registered entities between two timestamps.
    pub fn all_entities_versions(
        &self,
        before: u32,
        after: u32,
    ) -> VCRResult<Vec<ChroniclerEntity<JSONValue>>> {
        Ok(self
            .entities
            .par_iter()
            .map(|(e, _)| self.get_entity_versions(e, before, after))
            .collect::<VCRResult<Vec<Vec<ChroniclerEntity<JSONValue>>>>>()?
            .concat())
    }

    /// Fetches a 'page' of data, loading data into a buffer until it reaches the requested object count. If the buffer length is higher than the requested count, the buffer will be used to (at least partially) fulfill the next request.
    pub fn fetch_page(
        &self,
        page: &mut InternalPaging<Box<RawValue>>,
        count: usize,
        order: Order,
    ) -> VCRResult<Vec<ChroniclerEntity<Box<RawValue>>>> {
        while page.remaining_data.len() < count {
            if !page.remaining_ids.is_empty() {
                page.remaining_data.append(&mut match page.kind {
                    ChronV2EndpointKind::Versions(before, after) => self
                        .get_entity_versions(&page.remaining_ids.pop().unwrap(), before, after)
                        .and_then(hash_entities)?,
                    ChronV2EndpointKind::Entities(at) => self
                        .get_entity(&page.remaining_ids.pop().unwrap(), at)
                        .and_then(|v| hash_entities(vec![v]))?,
                });
            } else {
                break;
            }
        }

        page.remaining_data.sort_by_key(|x| x.valid_from);
        if order == Order::Desc {
            page.remaining_data.reverse();
        }

        Ok(page
            .remaining_data
            .drain(..std::cmp::min(count, page.remaining_data.len()))
            .collect())
    }
}

/// A handle over a group of databases, including a special database for Tributes and an index over game times.
pub struct MultiDatabase {
    pub dbs: HashMap<String, Database>, // entity_type:db
    pub game_index: HashMap<GameDate, Vec<(String, Option<DateTime<Utc>>, Option<DateTime<Utc>>)>>,
    pub tributes: TributesDatabase,
}

impl MultiDatabase {
    // dicts is the path to a zstd dictionary file. for no dictionaries, just send an empty hashmap.
    pub fn from_folder<P: AsRef<Path>>(
        folder: P,
        dicts: HashMap<String, P>,
        cache_size: usize,
    ) -> VCRResult<MultiDatabase> {
        let (mut header_paths, mut db_paths): (Vec<PathBuf>, Vec<PathBuf>) = read_dir(folder)?
            .map(|res| res.map(|e| e.path()))
            .collect::<Result<Vec<PathBuf>, io::Error>>()?
            .into_iter()
            .filter(|path| path.is_file())
            .partition(|path| {
                if let Some(name) = path.file_name() {
                    name.to_str().unwrap().contains(".header.riv")
                } else {
                    false
                }
            });

        let game_index: HashMap<
            GameDate,
            Vec<(String, Option<DateTime<Utc>>, Option<DateTime<Utc>>)>,
        > = if let Some(dates_pos) = db_paths.iter().position(|x| {
            x.file_name()
                .unwrap_or(OsStr::new(""))
                .to_str()
                .unwrap()
                .contains(".dates.riv.")
        }) {
            let game_index_path = db_paths.remove(dates_pos);
            let game_index_f = File::open(game_index_path)?;
            let decompressor = zstd::stream::Decoder::new(game_index_f)?;

            rmp_serde::from_read(decompressor)?
        } else {
            HashMap::new()
        };

        header_paths.sort();
        db_paths.sort();
        let entries: Vec<(String, PathBuf, PathBuf)> = header_paths
            .into_iter()
            .zip(db_paths.into_iter())
            .map(|(header, main)| {
                let e_type = main
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .split_once(".")
                    .unwrap()
                    .0
                    .to_owned();
                (e_type, header, main)
            })
            .collect();

        let mut dbs: HashMap<String, Database> = HashMap::new();
        let mut tributes: Option<TributesDatabase> = None;

        for (e_type, lookup_file, main_file) in entries {
            if e_type == "tributes" {
                tributes = Some(TributesDatabase::from_files(lookup_file, main_file)?);
            } else {
                dbs.insert(
                    e_type.clone(),
                    Database::from_files(
                        lookup_file,
                        main_file,
                        dicts
                            .get(&e_type)
                            .map(|p| PathBuf::from(p.as_ref().as_os_str())),
                        cache_size,
                    )?,
                );
            }
        }

        Ok(MultiDatabase {
            dbs,
            game_index,
            tributes: tributes.unwrap(),
        })
    }

    pub fn get_entity(
        &self,
        e_type: &str,
        entity: &str,
        at: u32,
    ) -> VCRResult<ChroniclerEntity<JSONValue>> {
        if e_type == "tributes" {
            self.tributes.get_entity(at)
        } else {
            self.dbs
                .get(e_type)
                .ok_or(VCRError::EntityTypeNotFound)?
                .get_entity(entity, at)
        }
    }

    pub fn get_entity_versions(
        &self,
        e_type: &str,
        entity: &str,
        before: u32,
        after: u32,
    ) -> VCRResult<Vec<ChroniclerEntity<JSONValue>>> {
        if e_type == "tributes" {
            self.tributes.get_versions(before, after)
        } else {
            self.dbs
                .get(e_type)
                .ok_or(VCRError::EntityTypeNotFound)?
                .get_entity_versions(entity, before, after)
        }
    }

    pub fn get_entities(
        &self,
        e_type: &str,
        entities: Vec<String>,
        at: u32,
    ) -> VCRResult<Vec<ChroniclerEntity<JSONValue>>> {
        if e_type == "tributes" {
            self.tributes.get_entity(at).map(|v| vec![v])
        } else {
            self.dbs
                .get(e_type)
                .ok_or(VCRError::EntityTypeNotFound)?
                .get_entities(entities, at)
        }
    }

    pub fn get_entities_versions(
        &self,
        e_type: &str,
        entities: Vec<String>,
        before: u32,
        after: u32,
    ) -> VCRResult<Vec<ChroniclerEntity<JSONValue>>> {
        if e_type == "tributes" {
            self.tributes.get_versions(before, after)
        } else {
            self.dbs
                .get(e_type)
                .ok_or(VCRError::EntityTypeNotFound)?
                .get_entities_versions(entities, before, after)
        }
    }

    pub fn all_entities(
        &self,
        e_type: &str,
        at: u32,
    ) -> VCRResult<Vec<ChroniclerEntity<JSONValue>>> {
        self.dbs
            .get(e_type)
            .ok_or(VCRError::EntityTypeNotFound)?
            .all_entities(at)
    }

    pub fn all_entities_versions(
        &self,
        e_type: &str,
        before: u32,
        after: u32,
    ) -> VCRResult<Vec<ChroniclerEntity<JSONValue>>> {
        self.dbs
            .get(e_type)
            .ok_or(VCRError::EntityTypeNotFound)?
            .all_entities_versions(before, after)
    }

    pub fn all_ids(&self, e_type: &str) -> VCRResult<Vec<String>> {
        if e_type == "tributes" {
            Ok(vec!["00000000-0000-0000-0000-000000000000".to_owned()])
        } else {
            let db = self.dbs.get(e_type).ok_or(VCRError::EntityTypeNotFound)?;
            Ok(db.entities.keys().map(|x| x.to_owned()).collect())
        }
    }

    pub fn fetch_page(
        &self,
        e_type: &str,
        page: &mut InternalPaging<Box<RawValue>>,
        count: usize,
        order: Order,
    ) -> VCRResult<Vec<ChroniclerEntity<Box<RawValue>>>> {
        if e_type == "tributes" {
            self.tributes.fetch_page(page, count)
        } else {
            self.dbs
                .get(e_type)
                .ok_or(VCRError::EntityTypeNotFound)?
                .fetch_page(page, count, order)
        }
    }

    pub fn games_by_date(&self, date: &GameDate) -> VCRResult<Vec<ChronV1Game>> {
        let db = self
            .dbs
            .get("game_updates")
            .ok_or(VCRError::EntityTypeNotFound)?;
        let mut results = Vec::new();
        for (game, start_time, end_time) in self.game_index.get(date).unwrap_or(&Vec::new()) {
            results.push(ChronV1Game {
                game_id: game.to_owned(),
                start_time: *start_time,
                end_time: *end_time,
                data: db.get_first_entity(game)?.data,
            });
        }

        Ok(results)
    }

    pub fn games_by_date_and_time(&self, date: &GameDate, at: u32) -> VCRResult<Vec<ChronV1Game>> {
        let db = self
            .dbs
            .get("game_updates")
            .ok_or(VCRError::EntityTypeNotFound)?;
        let mut results = Vec::new();
        for (game, start_time, end_time) in self.game_index.get(date).unwrap_or(&Vec::new()) {
            results.push(ChronV1Game {
                game_id: game.to_owned(),
                start_time: *start_time,
                end_time: *end_time,
                data: db.get_entity(game, at)?.data,
            });
        }

        Ok(results)
    }

    pub fn games_for_bets(&self, date: &GameDate, at: u32) -> VCRResult<Vec<ChronV1Game>> {
        let db = self
            .dbs
            .get("game_updates")
            .ok_or(VCRError::EntityTypeNotFound)?;
        let mut results = Vec::new();
        let json_zero = json!(0); // lol. lmao
        for (game, start_time, end_time) in self.game_index.get(date).unwrap_or(&Vec::new()) {
            let mut data = db.get_entity(game, at)?.data;
            let mut time = at;

            while data
                .get("awayOdds")
                .and_then(|v| if v == &json_zero { None } else { Some(()) })
                .is_none()
                && data
                    .get("homeOdds")
                    .and_then(|v| if v == &json_zero { None } else { Some(()) })
                    .is_none()
            {
                time = db.get_next_time(game, time);
                data = db.get_entity(game, time)?.data;
            }

            results.push(ChronV1Game {
                game_id: game.to_owned(),
                start_time: *start_time,
                end_time: *end_time,
                data: data,
            });
        }

        Ok(results)
    }

    pub fn games_with_date(&self, after: DateTime<Utc>) -> VCRResult<Vec<ChronV1Game>> {
        let mut results = Vec::with_capacity(self.game_index.len());
        for (date, games) in self.game_index.iter() {
            for (game, start_time, end_time) in games {
                if start_time.unwrap_or(Utc.timestamp(0, 0)) > after {
                    results.push(ChronV1Game {
                        game_id: game.to_owned(),
                        start_time: *start_time,
                        end_time: *end_time,
                        data: json!(date),
                    });
                }
            }
        }

        Ok(results)
    }

    fn playoffs(&self, id: &str, round: Option<i64>, at: u32) -> VCRResult<JSONValue> {
        let playoffs = self.get_entity("playoffs", id, at)?.data;
        let round_number = round.unwrap_or_else(|| playoffs["round"].as_i64().unwrap());

        let round_ids: Vec<String> = playoffs["rounds"]
            .as_array()
            .unwrap_or(&Vec::new())
            .iter()
            .map(|x| x.as_str().unwrap().to_owned())
            .collect();
        let all_rounds: Vec<JSONValue> = self
            .get_entities("playoffround", round_ids, at)?
            .into_iter()
            .map(|t| t.data)
            .filter(|t| t != &json!({}))
            .collect();
        let tomorrow_round: JSONValue = all_rounds
            .iter()
            .find(|r| r["roundNumber"] == playoffs["tomorrowRound"])
            .cloned()
            .unwrap_or(json!({}));
        let round: JSONValue = all_rounds
            .iter()
            .find(|r| r["roundNumber"].as_i64().unwrap() == round_number)
            .cloned()
            .unwrap_or(json!({}));

        let main_matchup_ids: Vec<String> = round["matchups"]
            .as_array()
            .unwrap_or(&Vec::new())
            .iter()
            .map(|x| x.as_str().unwrap().to_owned())
            .collect();
        let main_matchups: Vec<JSONValue> = self
            .get_entities("playoffmatchup", main_matchup_ids, at)?
            .into_iter()
            .map(|t| t.data)
            .filter(|t| t != &json!({}))
            .collect();

        let tomorrow_matchup_ids: Vec<String> = tomorrow_round["matchups"]
            .as_array()
            .unwrap_or(&Vec::new())
            .iter()
            .map(|x| x.as_str().unwrap().to_owned())
            .collect();
        let tomorrow_matchups: Vec<JSONValue> = self
            .get_entities("playoffmatchup", tomorrow_matchup_ids, at)?
            .into_iter()
            .map(|t| t.data)
            .filter(|t| t != &json!({}))
            .collect();

        let all_matchups_ids: Vec<String> = all_rounds
            .iter()
            .map(|x| {
                x["matchups"]
                    .as_array()
                    .unwrap_or(&Vec::new())
                    .iter()
                    .map(|x| x.as_str().unwrap().to_owned())
                    .collect::<Vec<String>>()
            })
            .flatten()
            .collect();

        let all_matchups: Vec<JSONValue> = self
            .get_entities("playoffmatchup", all_matchups_ids, at)?
            .into_iter()
            .map(|t| t.data)
            .filter(|t| t != &json!({}))
            .collect();

        Ok(json!({
            "round": round,
            "matchups": main_matchups,
            "playoffs": playoffs,
            "allRounds": all_rounds,
            "allMatchups": all_matchups,
            "tomorrowRound": tomorrow_round,
            "tomorrowMatchups": tomorrow_matchups
        }))
    }

    pub fn stream_data(&self, at: u32) -> VCRResult<JSONValue> {
        let sim = self.get_entity("sim", "00000000-0000-0000-0000-000000000000", at)?;

        let mut date = GameDate {
            season: sim.data.get("season").unwrap().as_i64().unwrap() as i32,
            day: sim.data.get("day").unwrap().as_i64().unwrap() as i32,
            tournament: if sim.data.get("season") == Some(&json!(10))
                && sim.data["day"].as_i64().unwrap() < 100
                && sim.data.get("tournament").is_none()
            {
                Some(-1)
            } else {
                sim.data
                    .get("tournament")
                    .map(|x| x.as_i64().unwrap() as i32)
            },
        };

        if let Some(i) = date.tournament {
            if i != -1 {
                date.season = -1;
            }
        }

        let schedule: JSONValue = if sim
            .data
            .get("phase")
            .unwrap_or(&json!(-1))
            .as_i64()
            .unwrap()
            == 14
            && date.season == 22
        {
            json!([self
                .get_entity("game_updates", "d162b23a-9832-4e78-8d78-5d131393fd61", at)?
                .data])
        } else {
            self.games_by_date_and_time(&date, at)?
                .into_iter()
                .map(|g| g.data)
                .collect()
        };

        date.day += 1;

        let tomorrow_schedule: Vec<JSONValue> = self
            .games_for_bets(&date, at)?
            .into_iter()
            .map(|g| g.data)
            .filter(|g| g != &json!({}))
            .collect();

        let season = self
            .all_entities("season", at)?
            .into_iter()
            .find(|s| s.data["seasonNumber"] == sim.data["season"])
            .unwrap();

        let standings =
            self.get_entity("standings", season.data["standings"].as_str().unwrap(), at)?;

        let mut leagues: Vec<JSONValue> = self
            .all_entities("league", clamp(at, 1599169238, u32::MAX))?
            .into_iter()
            .map(|t| t.data)
            .filter(|t| t != &json!({}))
            .collect();

        let subleague_ids: Vec<String> = leagues
            .iter()
            .map(|x| {
                x["subleagues"]
                    .as_array()
                    .unwrap_or(&Vec::new())
                    .iter()
                    .map(|x| x.as_str().unwrap().to_owned())
                    .collect::<Vec<String>>()
            })
            .flatten()
            .collect();

        let tiebreaker_ids: Vec<String> = leagues
            .iter()
            .map(|x| {
                x.get("tiebreakers")
                    .unwrap_or(&json!(""))
                    .as_str()
                    .unwrap()
                    .to_owned()
            })
            .collect();

        let mut subleagues: Vec<JSONValue> = self
            .get_entities("subleague", subleague_ids, clamp(at, 1599169238, u32::MAX))?
            .into_iter()
            .map(|s| s.data)
            .filter(|s| s != &json!({}))
            .collect();

        let division_ids: Vec<String> = subleagues
            .iter()
            .map(|x| {
                x["divisions"]
                    .as_array()
                    .unwrap_or(&Vec::new())
                    .iter()
                    .map(|x| x.as_str().unwrap().to_owned())
                    .collect::<Vec<String>>()
            })
            .flatten()
            .collect();

        let mut divisions: Vec<JSONValue> = self
            .get_entities("division", division_ids, clamp(at, 1599169238, u32::MAX))?
            .into_iter()
            .map(|d| d.data)
            .filter(|d| d != &json!({}))
            .collect();

        let mut tiebreakers: Vec<JSONValue> = self
            .get_entities(
                "tiebreakers",
                tiebreaker_ids,
                clamp(at, 1599760290, u32::MAX),
            )?
            .into_iter()
            .map(|t| t.data)
            .filter(|t| t != &json!({}))
            .collect();

        if at < 1598224980 {
            for d in &mut divisions {
                if let Some(id) = d.get("id") {
                    d["_id"] = id.clone();
                }
            }

            for d in &mut subleagues {
                if let Some(id) = d.get("id") {
                    d["_id"] = id.clone();
                }
            }

            for d in &mut leagues {
                if let Some(id) = d.get("id") {
                    d["_id"] = id.clone();
                }
            }

            for d in &mut tiebreakers {
                if let Some(id) = d.get("id") {
                    d["_id"] = id.clone();
                }
            }
        }

        let teams: Vec<JSONValue> = self
            .all_entities("team", at)?
            .into_iter()
            .map(|t| t.data)
            .filter(|t| t != &json!({}))
            .collect();

        let fights: Vec<JSONValue> = self
            .all_entities("bossfight", at)?
            .into_iter()
            .map(|b| b.data)
            .filter(|b| b != &json!({}) && b["homeHp"] != json!("0") && b["awayHp"] != json!("0"))
            .collect();

        let stadiums: Vec<JSONValue> = self
            .all_entities("stadium", at)?
            .into_iter()
            .map(|s| s.data)
            .filter(|s| s != &json!({}))
            .collect();

        let temporal = self.get_entity("temporal", "00000000-0000-0000-0000-000000000000", at)?;

        let sunsun = self.get_entity("sunsun", "00000000-0000-0000-0000-000000000000", at)?;

        let communitychest = self.get_entity(
            "communitychestprogress",
            "00000000-0000-0000-0000-000000000000",
            at,
        )?;

        let tournament = if let Some(tourn_idx) = date.tournament {
            if tourn_idx > -1 {
                self.all_entities("tournament", at)?
                    .into_iter()
                    .last()
                    .map_or(json!({}), |x| x.data)
            } else {
                json!({})
            }
        } else {
            json!({})
        };

        let (playoff_key, playoffs): (&str, JSONValue) = if tournament != json!({}) {
            (
                "postseason",
                self.playoffs(
                    tournament["playoffs"].as_str().unwrap(),
                    sim.data.get("tournamentRound").map(|i| i.as_i64().unwrap()),
                    at,
                )?,
            )
        } else if let Some(playoff_ids) = sim.data["playoffs"].as_array() {
            let mut playoffs: Vec<JSONValue> = Vec::new();
            for id in playoff_ids {
                playoffs.push(self.playoffs(id.as_str().unwrap(), None, at)?);
            }
            ("postseasons", json!(playoffs))
        } else if let Some(playoff_id) = sim.data["playoffs"].as_str() {
            (
                "postseason",
                self.playoffs(
                    playoff_id,
                    sim.data.get("playOffRound").map(|i| i.as_i64().unwrap()),
                    at,
                )?,
            )
        } else {
            ("postseason", json!({}))
        };

        // println!("---------------\n");

        Ok(json!({
            "value": {
                "games": {
                    "sim": sim.data,
                    "season": season.data,
                    "standings": standings.data,
                    "schedule": schedule,
                    "tomorrowSchedule": tomorrow_schedule,
                    "tournament": tournament,
                    playoff_key: playoffs
                },
                "leagues": {
                    "stats": {
                        "sunsun": sunsun.data,
                        "communityChest": communitychest.data
                    },
                    "teams": teams,
                    "subleagues": subleagues,
                    "divisions": divisions,
                    "leagues": leagues,
                    "tiebreakers": tiebreakers,
                    "stadiums": stadiums
                },
                "fights": {
                    "bossFights": fights
                },
                "temporal": temporal.data
            }
        }))
    }
}
