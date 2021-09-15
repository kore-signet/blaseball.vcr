use crate::*;

use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs::{read_dir, File};
use std::io::{self, prelude::*, BufReader, Cursor, SeekFrom};
use std::path::{Path, PathBuf};
use std::sync::Mutex;

use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use serde_json::{json, Value as JSONValue};

use zstd::dict::DecoderDictionary;

use lru::LruCache;

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

pub struct Database {
    reader: BufReader<File>,
    entities: HashMap<String, EntityData>,
    dictionary: Option<DecoderDictionary<'static>>,
    entity_cache: LruCache<(String, usize), ChroniclerEntity>,
}

impl Database {
    pub fn from_files<P: AsRef<Path> + std::fmt::Debug>(
        entities_lookup_path: P,
        db_path: P,
        dict_path: Option<P>,
        cache_size: usize,
    ) -> VCRResult<Database> {
        let entities_lookup_f = File::open(entities_lookup_path).map_err(VCRError::IOError)?;
        let decompressor =
            zstd::stream::Decoder::new(entities_lookup_f).map_err(VCRError::IOError)?;
        let db_f = File::open(db_path).map_err(VCRError::IOError)?;

        let compression_dict = if let Some(dict_f_path) = dict_path {
            let mut dict_f = File::open(dict_f_path).map_err(VCRError::IOError)?;
            let mut dict = Vec::new();
            dict_f.read_to_end(&mut dict).map_err(VCRError::IOError)?;
            Some(DecoderDictionary::copy(&dict))
        } else {
            None
        };

        Ok(Database {
            reader: BufReader::new(db_f),
            entities: rmp_serde::from_read(decompressor).map_err(VCRError::MsgPackError)?,
            dictionary: compression_dict,
            entity_cache: LruCache::new(cache_size),
        })
    }

    pub fn get_entity_data(
        &mut self,
        entity: &str,
        until: u32,
        skip_to_checkpoint: bool,
        from_index: usize,
    ) -> VCRResult<Vec<(u32, Patch)>> {
        let metadata = &self.entities.get(entity).ok_or(VCRError::EntityNotFound)?;

        let mut patches: Vec<(u32, Patch)> = Vec::new();

        let patch_list: Vec<(u32, u64, u64)> = if skip_to_checkpoint {
            let patches_until: Vec<(u32, u64, u64)> = metadata
                .patches
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
                .iter()
                .skip(from_index)
                .copied()
                .take_while(|x| x.0 <= until)
                .collect()
        };

        for (time, patch_start, patch_end) in patch_list {
            self.reader
                .seek(SeekFrom::Start(patch_start))
                .map_err(VCRError::IOError)?;

            let mut compressed_bytes: Vec<u8> = vec![0; (patch_end - patch_start) as usize];
            self.reader
                .read_exact(&mut compressed_bytes)
                .map_err(VCRError::IOError)?;

            let mut e_bytes: Vec<u8> = if let Some(compress_dict) = &self.dictionary {
                let mut decoder = zstd::stream::Decoder::with_prepared_dictionary(
                    Cursor::new(compressed_bytes),
                    compress_dict,
                )
                .map_err(VCRError::IOError)?;
                let mut res = Vec::with_capacity((patch_end - patch_start) as usize * 10);
                decoder.read_to_end(&mut res).map_err(VCRError::IOError)?;
                res
            } else {
                let mut decoder = zstd::stream::Decoder::new(Cursor::new(compressed_bytes))
                    .map_err(VCRError::IOError)?;
                let mut res = Vec::with_capacity((patch_end - patch_start) as usize * 10);
                decoder.read_to_end(&mut res).map_err(VCRError::IOError)?;
                res
            };

            let mut result = Patch::Normal(JSONPatch(vec![]));
            let mut operations: Vec<PatchOperation> = Vec::new();

            while e_bytes.len() > 1 {
                let op_code = e_bytes.remove(0);

                if op_code == 6 {
                    let value_length = u16::from_be_bytes([e_bytes.remove(0), e_bytes.remove(0)]);
                    let val_bytes: Vec<u8> = e_bytes.drain(..value_length as usize).collect();
                    result = Patch::ReplaceRoot(
                        rmp_serde::from_read_ref(&val_bytes).map_err(VCRError::MsgPackError)?,
                    );
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
                        Some(rmp_serde::from_read_ref(&val_bytes).map_err(VCRError::MsgPackError)?)
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

    pub fn get_entity_versions(
        &mut self,
        entity: &str,
        before: u32,
        after: u32,
    ) -> VCRResult<Vec<ChroniclerEntity>> {
        let mut entity_value = self
            .entities
            .get(entity)
            .ok_or(VCRError::EntityNotFound)?
            .base
            .clone();
        let patches = self.get_entity_data(entity, before, false, 0)?;
        let mut results: Vec<ChroniclerEntity> = Vec::with_capacity(patches.len());

        for (time, patch) in patches {
            match patch {
                Patch::ReplaceRoot(v) => {
                    entity_value = v.clone();
                }
                Patch::Normal(p) => {
                    patch_json(&mut entity_value, &p).map_err(VCRError::JSONPatchError)?;
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
                    valid_to: None,
                    hash: String::new(),
                });
            }
        }

        Ok(results)
    }

    pub fn get_entity(&mut self, entity: &str, at: u32) -> VCRResult<ChroniclerEntity> {
        let mut entity_value = self
            .entities
            .get(entity)
            .ok_or(VCRError::EntityNotFound)?
            .base
            .clone();

        let patch_idx = match self.entities[entity]
            .patches
            .binary_search_by_key(&at, |(t, _, _)| *t)
        {
            Ok(idx) => idx,
            Err(idx) => idx.saturating_sub(1),
        };

        if patch_idx > 0 {
            if let Some(val) = self.entity_cache.get(&(entity.to_owned(), patch_idx)) {
                return Ok(val.clone());
            }
        }

        let mut patch_data_idx = 0;

        if self.entities[entity].checkpoint_every != u32::MAX && patch_idx > 0 {
            if let Some(val) = self.entity_cache.get(&(entity.to_owned(), patch_idx - 1)) {
                entity_value = val.data.clone();
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
                    patch_json(&mut entity_value, &p).map_err(VCRError::JSONPatchError)?;
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
                .put((entity.to_owned(), patch_idx), e.clone());
        }

        Ok(e)
    }

    pub fn get_first_entity(&mut self, entity: &str) -> VCRResult<ChroniclerEntity> {
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
                patch_json(&mut entity_value, p).map_err(VCRError::JSONPatchError)?;
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

    pub fn get_entities(
        &mut self,
        entities: Vec<String>,
        at: u32,
    ) -> VCRResult<Vec<ChroniclerEntity>> {
        let mut results = Vec::with_capacity(entities.len());
        for e in entities {
            results.push(self.get_entity(&e, at)?);
        }

        Ok(results)
    }

    pub fn get_entities_versions(
        &mut self,
        entities: Vec<String>,
        before: u32,
        after: u32,
    ) -> VCRResult<Vec<ChroniclerEntity>> {
        let mut results = Vec::with_capacity(entities.len());
        for e in entities {
            results.append(&mut self.get_entity_versions(&e, before, after)?);
        }

        Ok(results)
    }

    pub fn all_entities(&mut self, at: u32) -> VCRResult<Vec<ChroniclerEntity>> {
        let mut results = Vec::with_capacity(self.entities.len());
        let keys: Vec<String> = self.entities.keys().cloned().collect();
        for entity in keys {
            results.push(self.get_entity(&entity, at)?);
        }

        Ok(results)
    }

    pub fn all_entities_versions(
        &mut self,
        before: u32,
        after: u32,
    ) -> VCRResult<Vec<ChroniclerEntity>> {
        let mut results = Vec::with_capacity(self.entities.len());
        let keys: Vec<String> = self.entities.keys().cloned().collect();
        for e in keys {
            results.append(&mut self.get_entity_versions(&e, before, after)?);
        }

        Ok(results)
    }

    pub fn fetch_page(
        &mut self,
        page: &mut InternalPaging,
        count: usize,
    ) -> VCRResult<Vec<ChroniclerEntity>> {
        while page.remaining_data.len() < count {
            if !page.remaining_ids.is_empty() {
                page.remaining_data.append(&mut match page.kind {
                    ChronV2EndpointKind::Versions(before, after) => {
                        self.get_entity_versions(&page.remaining_ids.pop().unwrap(), before, after)?
                    }
                    ChronV2EndpointKind::Entities(at) => {
                        vec![self.get_entity(&page.remaining_ids.pop().unwrap(), at)?]
                    }
                });
            } else {
                break;
            }
        }

        Ok(page
            .remaining_data
            .drain(..std::cmp::min(count, page.remaining_data.len()))
            .collect())
    }
}

pub struct MultiDatabase {
    pub dbs: HashMap<String, Mutex<Database>>, // entity_type:db
    pub game_index: HashMap<GameDate, Vec<(String, Option<DateTime<Utc>>, Option<DateTime<Utc>>)>>,
}

impl MultiDatabase {
    // dicts is the path to a zstd dictionary file. for no dictionaries, just send an empty hashmap.
    pub fn from_folder<P: AsRef<Path>>(
        folder: P,
        dicts: HashMap<String, P>,
        cache_size: usize,
    ) -> VCRResult<MultiDatabase> {
        let (mut header_paths, mut db_paths): (Vec<PathBuf>, Vec<PathBuf>) = read_dir(folder)
            .map_err(VCRError::IOError)?
            .map(|res| res.map(|e| e.path()))
            .collect::<Result<Vec<PathBuf>, io::Error>>()
            .map_err(VCRError::IOError)?
            .into_iter()
            .filter(|path| path.is_file())
            .partition(|path| {
                if let Some(name) = path.file_name() {
                    name.to_str().unwrap().contains(".header.riv.")
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
            let game_index_f = File::open(game_index_path).map_err(VCRError::IOError)?;
            let decompressor =
                zstd::stream::Decoder::new(game_index_f).map_err(VCRError::IOError)?;

            rmp_serde::from_read(decompressor).map_err(VCRError::MsgPackError)?
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

        let mut dbs: HashMap<String, Mutex<Database>> = HashMap::new();

        for (e_type, lookup_file, main_file) in entries {
            dbs.insert(
                e_type.clone(),
                Mutex::new(Database::from_files(
                    lookup_file,
                    main_file,
                    dicts
                        .get(&e_type)
                        .map(|p| PathBuf::from(p.as_ref().as_os_str())),
                    cache_size,
                )?),
            );
        }

        Ok(MultiDatabase {
            dbs,
            game_index,
        })
    }

    pub fn get_entity(&self, e_type: &str, entity: &str, at: u32) -> VCRResult<ChroniclerEntity> {
        let mut db = self
            .dbs
            .get(e_type)
            .ok_or(VCRError::EntityTypeNotFound)?
            .lock()
            .unwrap();
        db.get_entity(entity, at)
    }

    pub fn get_entity_versions(
        &self,
        e_type: &str,
        entity: &str,
        before: u32,
        after: u32,
    ) -> VCRResult<Vec<ChroniclerEntity>> {
        let mut db = self
            .dbs
            .get(e_type)
            .ok_or(VCRError::EntityTypeNotFound)?
            .lock()
            .unwrap();
        db.get_entity_versions(entity, before, after)
    }

    pub fn get_entities(
        &self,
        e_type: &str,
        entities: Vec<String>,
        at: u32,
    ) -> VCRResult<Vec<ChroniclerEntity>> {
        let mut db = self
            .dbs
            .get(e_type)
            .ok_or(VCRError::EntityTypeNotFound)?
            .lock()
            .unwrap();
        db.get_entities(entities, at)
    }

    pub fn get_entities_versions(
        &self,
        e_type: &str,
        entities: Vec<String>,
        before: u32,
        after: u32,
    ) -> VCRResult<Vec<ChroniclerEntity>> {
        let mut db = self
            .dbs
            .get(e_type)
            .ok_or(VCRError::EntityTypeNotFound)?
            .lock()
            .unwrap();
        db.get_entities_versions(entities, before, after)
    }

    pub fn all_entities(&self, e_type: &str, at: u32) -> VCRResult<Vec<ChroniclerEntity>> {
        let mut db = self
            .dbs
            .get(e_type)
            .ok_or(VCRError::EntityTypeNotFound)?
            .lock()
            .unwrap();
        db.all_entities(at)
    }

    pub fn all_entities_versions(
        &self,
        e_type: &str,
        before: u32,
        after: u32,
    ) -> VCRResult<Vec<ChroniclerEntity>> {
        let mut db = self
            .dbs
            .get(e_type)
            .ok_or(VCRError::EntityTypeNotFound)?
            .lock()
            .unwrap();
        db.all_entities_versions(before, after)
    }

    pub fn all_ids(&self, e_type: &str) -> VCRResult<Vec<String>> {
        let db = self
            .dbs
            .get(e_type)
            .ok_or(VCRError::EntityTypeNotFound)?
            .lock()
            .unwrap();
        Ok(db.entities.keys().map(|x| x.to_owned()).collect())
    }

    pub fn fetch_page(
        &self,
        e_type: &str,
        page: &mut InternalPaging,
        count: usize,
    ) -> VCRResult<Vec<ChroniclerEntity>> {
        let mut db = self
            .dbs
            .get(e_type)
            .ok_or(VCRError::EntityTypeNotFound)?
            .lock()
            .unwrap();
        db.fetch_page(page, count)
    }

    pub fn games_by_date(&self, date: &GameDate) -> VCRResult<Vec<ChronV1Game>> {
        let mut db = self
            .dbs
            .get("game_updates")
            .ok_or(VCRError::EntityTypeNotFound)?
            .lock()
            .unwrap();
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
        let mut db = self
            .dbs
            .get("game_updates")
            .ok_or(VCRError::EntityTypeNotFound)?
            .lock()
            .unwrap();
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
            .games_by_date_and_time(&date, at)?
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
