use crate::{hash_entities, read_u32, ChronV2EndpointKind, InternalPaging};
use crate::{ChroniclerEntity, VCRResult};
use chrono::{DateTime, NaiveDateTime, Utc};
use integer_encoding::VarIntReader;
use serde_json::{json, value::RawValue, Value as JSONValue};
use std::collections::HashMap;
use std::convert::TryInto;
use std::fs::File;
use std::io::{BufReader, Cursor, Read, Seek, SeekFrom};
use std::path::Path;
use uuid::Uuid;

static TEAMS_EPOCH: u32 = 1623642600;

pub struct TributesDatabase {
    times: Vec<(u32, u32, u16)>,     // (time, start, length)
    ids: HashMap<u16, (Uuid, bool)>, // (id_number, (id, is_team))
    reader: BufReader<File>,
}

impl TributesDatabase {
    pub fn from_files<P: AsRef<Path> + std::fmt::Debug>(
        header_path: P,
        db_path: P,
    ) -> VCRResult<TributesDatabase> {
        let header_f = File::open(header_path)?;
        let mut header_reader = BufReader::new(header_f);

        let ids: HashMap<u16, (Uuid, bool)> = {
            let mut ids = HashMap::new();
            let ids_len = read_u32!(header_reader);
            let mut bytes: Vec<u8> = vec![0; ids_len as usize];
            header_reader.read_exact(&mut bytes)?;

            while !bytes.is_empty() {
                let uuid = Uuid::from_slice(&bytes.drain(..16).collect::<Vec<u8>>()).unwrap();
                let id_bytes =
                    u16::from_be_bytes(bytes.drain(..2).collect::<Vec<u8>>().try_into().unwrap());
                let is_team = ((id_bytes >> 15) & 0xFFF) != 0;
                let idx = id_bytes & 0xFFF;

                ids.insert(idx, (uuid, is_team));
            }

            ids
        };

        let mut times: Vec<(u32, u32, u16)> = Vec::new();

        loop {
            let mut bytes: [u8; 10] = [0; 10];
            if header_reader.read_exact(&mut bytes).is_err() {
                break;
            }

            times.push((
                u32::from_be_bytes(bytes[0..4].try_into().unwrap()),
                u32::from_be_bytes(bytes[4..8].try_into().unwrap()),
                u16::from_be_bytes(bytes[8..10].try_into().unwrap()),
            ));
        }

        let main_file = File::open(db_path)?;

        Ok(TributesDatabase {
            times,
            ids,
            reader: BufReader::new(main_file),
        })
    }

    pub fn get_versions(
        &mut self,
        before: u32,
        after: u32,
    ) -> VCRResult<Vec<ChroniclerEntity<JSONValue>>> {
        let mut vals: HashMap<(Uuid, bool), u64> = HashMap::new();
        let mut versions: Vec<ChroniclerEntity<JSONValue>> = Vec::new();

        for (time, start, length) in &self.times {
            if time > &before {
                break;
            }

            self.reader.seek(SeekFrom::Start(*start as u64))?;

            let mut bytes: Vec<u8> = vec![0; *length as usize];
            self.reader.read_exact(&mut bytes)?;
            let mut bytes = Cursor::new(bytes);

            loop {
                let idx = bytes.read_varint::<u16>();
                if idx.is_err() {
                    // todo: check specifically if it's an io::ErrorKind::UnexpectedEof
                    break;
                }

                let idx = idx.unwrap();
                if idx == 0 {
                    let len = bytes.read_varint::<u8>()?;
                    for _ in 0..len {
                        let ridx = bytes.read_varint::<u16>()?;
                        vals.remove(&self.ids[&ridx]);
                    }
                } else {
                    let val = bytes.read_varint::<u64>()?;
                    vals.insert(self.ids[&idx], val);
                }
            }

            if time > &after {
                versions.push(ChroniclerEntity {
                    data: if time < &TEAMS_EPOCH {
                        let mut players = vals
                            .iter()
                            .map(|(k, v)| {
                                json!({
                                    "playerId": k.0.to_string(),
                                    "peanuts": v
                                })
                            })
                            .collect::<Vec<JSONValue>>();
                        players.sort_by_key(|v| v["peanuts"].as_u64().unwrap());
                        players.reverse();
                        json!(players)
                    } else {
                        let (teams, players): (
                            Vec<(&(Uuid, bool), &u64)>,
                            Vec<(&(Uuid, bool), &u64)>,
                        ) = vals.iter().partition(|&(k, _)| k.1);
                        let mut players: Vec<JSONValue> = players
                            .iter()
                            .map(|(k, v)| {
                                json!({
                                    "playerId": k.0.to_string(),
                                    "peanuts": v
                                })
                            })
                            .collect::<Vec<JSONValue>>();
                        players.sort_by_key(|v| v["peanuts"].as_u64().unwrap());
                        players.reverse();

                        let mut teams: Vec<JSONValue> = teams
                            .iter()
                            .map(|(k, v)| {
                                json!({
                                    "teamId": k.0.to_string(),
                                    "peanuts": v
                                })
                            })
                            .collect::<Vec<JSONValue>>();
                        teams.sort_by_key(|v| v["peanuts"].as_u64().unwrap());
                        teams.reverse();

                        json!({
                            "teams": teams,
                            "players": players,
                        })
                    },
                    entity_id: Uuid::nil().to_string(),
                    valid_from: DateTime::<Utc>::from_utc(
                        NaiveDateTime::from_timestamp(*time as i64, 0),
                        Utc,
                    ),
                    valid_to: None,
                    hash: String::new(),
                })
            }
        }

        Ok(versions)
    }

    pub fn get_entity(&mut self, at: u32) -> VCRResult<ChroniclerEntity<JSONValue>> {
        let mut vals: HashMap<(Uuid, bool), u64> = HashMap::new();
        let mut last_time = 0;

        for (time, start, length) in &self.times {
            last_time = *time;
            self.reader.seek(SeekFrom::Start(*start as u64))?;

            let mut bytes: Vec<u8> = vec![0; *length as usize];
            self.reader.read_exact(&mut bytes)?;
            let mut bytes = Cursor::new(bytes);

            loop {
                let idx = bytes.read_varint::<u16>();
                if idx.is_err() {
                    // todo: check specifically if it's an io::ErrorKind::UnexpectedEof
                    break;
                }
                let idx = idx.unwrap();
                if idx == 0 {
                    let len = bytes.read_varint::<u8>()?;
                    for _ in 0..len {
                        let ridx = bytes.read_varint::<u16>()?;
                        vals.remove(&self.ids[&ridx]);
                    }
                } else {
                    let val = bytes.read_varint::<u64>()?;
                    vals.insert(self.ids[&idx], val);
                }
            }

            if time >= &at {
                break;
            }
        }

        Ok(ChroniclerEntity {
            data: if last_time < TEAMS_EPOCH {
                let mut players = vals
                    .iter()
                    .map(|(k, v)| {
                        json!({
                            "playerId": k.0.to_string(),
                            "peanuts": v
                        })
                    })
                    .collect::<Vec<JSONValue>>();
                players.sort_by_key(|v| v["peanuts"].as_u64().unwrap());
                players.reverse();
                json!(players)
            } else {
                let (teams, players): (Vec<(&(Uuid, bool), &u64)>, Vec<(&(Uuid, bool), &u64)>) =
                    vals.iter().partition(|&(k, _)| k.1);
                let mut players: Vec<JSONValue> = players
                    .iter()
                    .map(|(k, v)| {
                        json!({
                            "playerId": k.0.to_string(),
                            "peanuts": v
                        })
                    })
                    .collect::<Vec<JSONValue>>();
                players.sort_by_key(|v| v["peanuts"].as_u64().unwrap());
                players.reverse();

                let mut teams: Vec<JSONValue> = teams
                    .iter()
                    .map(|(k, v)| {
                        json!({
                            "teamId": k.0.to_string(),
                            "peanuts": v
                        })
                    })
                    .collect::<Vec<JSONValue>>();
                teams.sort_by_key(|v| v["peanuts"].as_u64().unwrap());
                teams.reverse();

                json!({
                    "teams": teams,
                    "players": players,
                })
            },
            entity_id: Uuid::nil().to_string(),
            valid_from: DateTime::<Utc>::from_utc(
                NaiveDateTime::from_timestamp(last_time as i64, 0),
                Utc,
            ),
            valid_to: None,
            hash: String::new(),
        })
    }

    pub fn fetch_page(
        &mut self,
        page: &mut InternalPaging<Box<RawValue>>,
        count: usize,
    ) -> VCRResult<Vec<ChroniclerEntity<Box<RawValue>>>> {
        page.remaining_ids = vec![];

        if page.remaining_data.len() < count {
            page.remaining_data = match page.kind {
                ChronV2EndpointKind::Versions(before, after) => {
                    self.get_versions(before, after).and_then(hash_entities)?
                }
                ChronV2EndpointKind::Entities(at) => {
                    self.get_entity(at).and_then(|v| hash_entities(vec![v]))?
                }
            };
        }

        Ok(page
            .remaining_data
            .drain(..std::cmp::min(count, page.remaining_data.len()))
            .collect())
    }
}
