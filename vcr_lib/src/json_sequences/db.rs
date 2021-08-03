use crate::*;

use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs::{read_dir, File};
use std::io::{self, prelude::*, BufReader, Cursor, SeekFrom};
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::time::Instant;

use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use serde_json::{json, Value as JSONValue};

use json_patch::{
    patch as patch_json, AddOperation, CopyOperation, MoveOperation, Patch as JSONPatch,
    PatchOperation, PatchOperation::*, RemoveOperation, ReplaceOperation, TestOperation,
};

macro_rules! start_measure {
    ($t:tt) => {
        let $t = Instant::now();
    };
}

macro_rules! end_measure {
    ($t:tt) => {
        println!("\x1b[1;31m{}:\x1b[0m {}ms", stringify!($t), $t.elapsed().as_millis());
    };
}

pub struct Database {
    reader: BufReader<File>,
    entities: HashMap<String, EntityData>,
    dictionary: Option<Vec<u8>>,
}

impl Database {
    pub fn from_files<P: AsRef<Path> + std::fmt::Debug>(
        entities_lookup_path: P,
        db_path: P,
        dict_path: Option<P>,
    ) -> VCRResult<Database> {
        let entities_lookup_f = File::open(entities_lookup_path).map_err(VCRError::IOError)?;
        let decompressor =
            zstd::stream::Decoder::new(entities_lookup_f).map_err(VCRError::IOError)?;
        let db_f = File::open(db_path).map_err(VCRError::IOError)?;

        let compression_dict = if let Some(dict_f_path) = dict_path {
            let mut dict_f = File::open(dict_f_path).map_err(VCRError::IOError)?;
            let mut dict = Vec::new();
            dict_f.read_to_end(&mut dict).map_err(VCRError::IOError)?;
            Some(dict)
        } else {
            None
        };

        Ok(Database {
            reader: BufReader::new(db_f),
            entities: rmp_serde::from_read(decompressor).map_err(VCRError::MsgPackError)?,
            dictionary: compression_dict,
        })
    }

    pub fn get_entity_data(
        &mut self,
        entity: &str,
        until: u32,
        skip_to_checkpoint: bool,
    ) -> VCRResult<Vec<(u32, JSONPatch)>> {
        let metadata = &self.entities[entity];

        let mut patches: Vec<(u32, JSONPatch)> = Vec::new();

        let patch_list: Vec<(u32, u64, u64)> = if skip_to_checkpoint {
            let patches_until: Vec<(u32, u64, u64)> = metadata
                .patches
                .iter()
                .copied()
                .take_while(|x| x.0 < until)
                .collect();
            let latest_check_idx = match patches_until.binary_search_by_key(&until, |x| x.0) {
                Ok(i) => i,
                Err(i) => i,
            };

            let closest_checkpoint =
                latest_check_idx - (latest_check_idx % metadata.checkpoint_every as usize);
            patches_until[closest_checkpoint..].to_vec()
        } else {
            metadata
                .patches
                .iter()
                .copied()
                .take_while(|x| x.0 < until)
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
                let mut decoder = zstd::stream::Decoder::with_dictionary(
                    Cursor::new(compressed_bytes),
                    compress_dict,
                )
                .map_err(VCRError::IOError)?;
                let mut res = Vec::new();
                decoder.read_to_end(&mut res).map_err(VCRError::IOError)?;
                res
            } else {
                let mut decoder = zstd::stream::Decoder::new(Cursor::new(compressed_bytes))
                    .map_err(VCRError::IOError)?;
                let mut res = Vec::new();
                decoder.read_to_end(&mut res).map_err(VCRError::IOError)?;
                res
            };

            let mut operations: Vec<PatchOperation> = Vec::new();

            while e_bytes.len() > 1 {
                let op_code = e_bytes.remove(0);

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
                        value: value.unwrap(),
                    }),
                    1 => Remove(RemoveOperation {
                        path: paths[0].to_string(),
                    }),
                    2 => Replace(ReplaceOperation {
                        path: paths[0].to_string(),
                        value: value.unwrap(),
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
                        value: value.unwrap(),
                    }),
                    _ => return Err(VCRError::InvalidOpCode),
                });
            }
            patches.push((time, JSONPatch(operations)));
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
        let mut entity_value = json!({});
        let patches = self.get_entity_data(entity, before, false)?;
        let mut results: Vec<ChroniclerEntity> = Vec::with_capacity(patches.len());

        for (time, patch) in patches {
            patch_json(&mut entity_value, &patch).map_err(VCRError::JSONPatchError)?;

            if time > after {
                results.push(ChroniclerEntity {
                    data: entity_value.clone(),
                    entity_id: entity.to_owned(),
                    valid_from: DateTime::<Utc>::from_utc(
                        NaiveDateTime::from_timestamp(time as i64, 0),
                        Utc,
                    ),
                    valid_to: None,
                    hash: String::new()
                });
            }
        }

        Ok(results)
    }

    pub fn get_entity(&mut self, entity: &str, at: u32) -> VCRResult<ChroniclerEntity> {
        let mut entity_value = json!({});
        let mut last_time = 0;

        for (time, patch) in self.get_entity_data(entity, at, true)? {
            patch_json(&mut entity_value, &patch).map_err(VCRError::JSONPatchError)?;
            last_time = time;
        }

        Ok(ChroniclerEntity {
            data: entity_value,
            entity_id: entity.to_owned(),
            valid_from: DateTime::<Utc>::from_utc(
                NaiveDateTime::from_timestamp(last_time as i64, 0),
                Utc,
            ),
            valid_to: None,
            hash: String::new(),
        })
    }

    pub fn get_first_entity(&mut self, entity: &str) -> VCRResult<ChroniclerEntity> {
        let mut entity_value = json!({});

        let patches = self.get_entity_data(entity, u32::MAX, true)?;
        let (time, patch) = &patches[0];

        patch_json(&mut entity_value, &patch).map_err(VCRError::JSONPatchError)?;

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
    )  -> VCRResult<Vec<ChroniclerEntity>> {
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

    pub fn all_entities_versions(&mut self, before: u32, after: u32) -> VCRResult<Vec<ChroniclerEntity>> {
        let mut results = Vec::with_capacity(self.entities.len());
        let keys: Vec<String> = self.entities.keys().cloned().collect();
        for e in keys {
            results.append(&mut self.get_entity_versions(&e, before, after)?);
        }

        Ok(results)
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
                )?),
            );
        }

        Ok(MultiDatabase {
            dbs: dbs,
            game_index: game_index,
        })
    }

    pub fn get_entity(&self, e_type: &str, entity: &str, at: u32) -> VCRResult<ChroniclerEntity> {
        let mut db = self.dbs[e_type].lock().unwrap();
        Ok(db.get_entity(entity, at)?)
    }

    pub fn get_entity_versions(
        &self,
        e_type: &str,
        entity: &str,
        before: u32,
        after: u32,
    )  -> VCRResult<Vec<ChroniclerEntity>> {
        let mut db = self.dbs[e_type].lock().unwrap();
        Ok(db.get_entity_versions(entity, before, after)?)
    }

    pub fn get_entities(
        &self,
        e_type: &str,
        entities: Vec<String>,
        at: u32,
    ) -> VCRResult<Vec<ChroniclerEntity>> {
        let mut db = self.dbs[e_type].lock().unwrap();
        Ok(db.get_entities(entities, at)?)
    }

    pub fn get_entities_versions(
        &self,
        e_type: &str,
        entities: Vec<String>,
        before: u32,
        after: u32,
    )  -> VCRResult<Vec<ChroniclerEntity>> {
        let mut db = self.dbs[e_type].lock().unwrap();
        Ok(db.get_entities_versions(entities, before, after)?)
    }

    pub fn all_entities(&self, e_type: &str, at: u32) -> VCRResult<Vec<ChroniclerEntity>> {
        let mut db = self.dbs[e_type].lock().unwrap();
        Ok(db.all_entities(at)?)
    }

    pub fn all_entities_versions(&self, e_type: &str, before: u32, after: u32) -> VCRResult<Vec<ChroniclerEntity>> {
        let mut db = self.dbs[e_type].lock().unwrap();
        Ok(db.all_entities_versions(before,after)?)
    }

    pub fn games_by_date(&self, date: &GameDate) -> VCRResult<Vec<ChronV1Game>> {
        let mut db = self.dbs["game_updates"].lock().unwrap();
        let mut results = Vec::new();
        for (game, start_time, end_time) in self.game_index.get(date).unwrap_or(&Vec::new()) {
            results.push(ChronV1Game {
                game_id: game.to_owned(),
                start_time: *start_time,
                end_time: *end_time,
                data: db.get_first_entity(&game)?.data,
            });
        }

        Ok(results)
    }

    pub fn games_by_date_and_time(&self, date: &GameDate, at: u32) -> VCRResult<Vec<ChronV1Game>> {
        let mut db = self.dbs["game_updates"].lock().unwrap();
        let mut results = Vec::new();
        for (game, start_time, end_time) in self.game_index.get(date).unwrap_or(&Vec::new()) {
            results.push(ChronV1Game {
                game_id: game.to_owned(),
                start_time: *start_time,
                end_time: *end_time,
                data: db.get_entity(&game, at)?.data,
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
            .into_iter()
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
            .unwrap();
        let round: JSONValue = all_rounds
            .iter()
            .find(|r| r["roundNumber"].as_i64().unwrap() == round_number)
            .cloned()
            .unwrap();

        let main_matchup_ids: Vec<String> = round["matchups"]
            .as_array()
            .unwrap_or(&Vec::new())
            .into_iter()
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
            .into_iter()
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
                    .into_iter()
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
        //start_measure!(sim_time);
        //start_measure!(total_time);
        let sim = self.get_entity("sim", "00000000-0000-0000-0000-000000000000", at)?;
        //end_measure!(sim_time);

        let mut date = GameDate {
            season: sim.data.get("season").unwrap().as_i64().unwrap() as i32,
            day: sim.data.get("day").unwrap().as_i64().unwrap() as i32,
            tournament: sim
                .data
                .get("tournament")
                .map(|x| x.as_i64().unwrap() as i32),
        };

        //start_measure!(schedule_time);
        let schedule: JSONValue = self
            .games_by_date_and_time(&date, at)?
            .into_iter()
            .map(|g| g.data)
            .collect();
        //end_measure!(schedule_time);
        date.day += 1;

        //start_measure!(tomorrow_schedule_time);
        let tomorrow_schedule: Vec<JSONValue> = self
            .games_by_date(&date)?
            .into_iter()
            .map(|g| g.data)
            .collect();
        //end_measure!(tomorrow_schedule_time);

        //start_measure!(season_time);
        let season = self
            .all_entities("season", at)?
            .into_iter()
            .find(|s| s.data["seasonNumber"] == sim.data["season"])
            .unwrap();
        //end_measure!(season_time);

        //start_measure!(standings_time);
        let standings =
            self.get_entity("standings", season.data["standings"].as_str().unwrap(), at)?;
        //end_measure!(standings_time);

        //start_measure!(leagues_time);
        let leagues: Vec<JSONValue> = self
            .all_entities("league", at)?
            .into_iter()
            .map(|t| t.data)
            .filter(|t| t != &json!({}))
            .collect();
        //end_measure!(leagues_time);

        //start_measure!(subleagues_time);
        let subleague_ids: Vec<String> = leagues
            .iter()
            .map(|x| {
                x["subleagues"]
                    .as_array()
                    .unwrap_or(&Vec::new())
                    .into_iter()
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

        let subleagues: Vec<JSONValue> = self
            .get_entities("subleague", subleague_ids, at)?
            .into_iter()
            .map(|s| s.data)
            .filter(|s| s != &json!({}))
            .collect();
        //end_measure!(subleagues_time);

        let division_ids: Vec<String> = subleagues
            .iter()
            .map(|x| {
                x["divisions"]
                    .as_array()
                    .unwrap_or(&Vec::new())
                    .into_iter()
                    .map(|x| x.as_str().unwrap().to_owned())
                    .collect::<Vec<String>>()
            })
            .flatten()
            .collect();

        let divisions: Vec<JSONValue> = self
            .get_entities("division", division_ids, at)?
            .into_iter()
            .map(|d| d.data)
            .filter(|d| d != &json!({}))
            .collect();

        //start_measure!(teams_time);
        let teams: Vec<JSONValue> = self
            .all_entities("team", at)?
            .into_iter()
            .map(|t| t.data)
            .filter(|t| t != &json!({}))
            .collect();
        //end_measure!(teams_time);

        //start_measure!(fights_time);

        let fights: Vec<JSONValue> = self
            .all_entities("bossfight", at)?
            .into_iter()
            .map(|b| b.data)
            .filter(|b| b != &json!({}) && b["homeHp"] != json!("0") && b["awayHp"] != json!("0"))
            .collect();
        //end_measure!(fights_time);

        //start_measure!(stadiums_time);
        let stadiums: Vec<JSONValue> = self
            .all_entities("stadium", at)?
            .into_iter()
            .map(|s| s.data)
            .filter(|s| s != &json!({}))
            .collect();
        //end_measure!(stadiums_time);

        let tiebreakers: Vec<JSONValue> = self
            .get_entities("tiebreakers", tiebreaker_ids, at)?
            .into_iter()
            .map(|t| t.data)
            .filter(|t| t != &json!({}))
            .collect();

        let temporal = self.get_entity("temporal", "00000000-0000-0000-0000-000000000000", at)?;

        let sunsun = self.get_entity("sunsun", "00000000-0000-0000-0000-000000000000", at)?;

        let communitychest = self.get_entity(
            "communitychestprogress",
            "00000000-0000-0000-0000-000000000000",
            at,
        )?;

        //start_measure!(tournaments_and_playoffs);

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
        } else {
            if let Some(playoff_ids) = sim.data["playoffs"].as_array() {
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
            }
        };
        //end_measure!(tournaments_and_playoffs);

        //end_measure!(total_time);

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
                "temporal": temporal
            }
        }))
    }
}
