use super::*;
use crate::{VCRError, VCRResult};
use chrono::{DateTime, Datelike, TimeZone, Timelike, Utc};
use qp_trie::Trie;
use serde_json::Value as JSONValue;
use std::collections::HashMap;
use std::convert::TryInto;
use std::fs::File;
use std::io::{BufReader, Cursor, Read, Seek, SeekFrom};
use std::path::Path;
use uuid::Uuid;

fn make_tries<R: Read>(mut reader: R) -> (Trie<Vec<u8>, (u64, u64)>, Trie<Vec<u8>, Vec<u8>>) {
    let mut last_position: u64 = 0;
    let mut index: Vec<(Vec<u8>, (u64, u64))> = Vec::new();

    loop {
        let mut bytes: Vec<u8> = vec![0; 10];
        if reader.read_exact(&mut bytes).is_err() {
            break;
        }

        let snowflake = bytes[0..8].to_vec();
        let position_delta = u16::from_be_bytes(bytes[8..10].try_into().unwrap());
        let start_pos = last_position + position_delta as u64;

        if index.len() > 0 {
            let idx = index.len() - 1;
            let mut a = index[idx].clone();
            a.1 .1 = start_pos;
            index[idx] = a;
        }

        index.push((snowflake, (start_pos, 0u64)));

        last_position = start_pos;
    }

    let mut snowflake_trie: Trie<Vec<u8>, (u64, u64)> = Trie::new();
    let mut time_trie: Trie<Vec<u8>, Vec<u8>> = Trie::new();

    for (snowflake, (start_pos, end_pos)) in index {
        snowflake_trie.insert(snowflake.clone(), (start_pos, end_pos));
        let timestamp = u32::from_be_bytes(snowflake[2..6].try_into().unwrap());
        let t = Utc.timestamp(timestamp as i64, 0);
        time_trie.insert(
            [
                (t.year() as u16).to_be_bytes().to_vec(),
                vec![
                    t.month() as u8,
                    t.day() as u8,
                    t.hour() as u8,
                    t.minute() as u8,
                    t.second() as u8,
                ],
            ]
            .concat(),
            snowflake,
        );
    }

    (snowflake_trie, time_trie)
}

pub struct FeedDatabase {
    pub snowflakes: Trie<Vec<u8>, (u64, u64)>,
    times: Trie<Vec<u8>, Vec<u8>>,
    reader: BufReader<File>,
    player_tags: HashMap<u16, Uuid>,
    game_tags: HashMap<u16, Uuid>,
    team_tags: HashMap<u8, Uuid>,
    dictionary: Vec<u8>,
}

impl FeedDatabase {
    pub fn from_files<P: AsRef<Path> + std::fmt::Debug>(
        position_index_path: P,
        db_file_path: P,
        dict_file_path: P,
        id_table_path: P,
    ) -> VCRResult<FeedDatabase> {
        let id_file = File::open(id_table_path).map_err(VCRError::IOError)?;
        let (team_tags, player_tags, game_tags) = rmp_serde::from_read::<
            File,
            (HashMap<Uuid, u8>, HashMap<Uuid, u16>, HashMap<Uuid, u16>),
        >(id_file)
        .unwrap(); // todo: result

        let position_index_file = File::open(position_index_path).map_err(VCRError::IOError)?;
        let position_index_decompressor =
            zstd::stream::Decoder::new(position_index_file).map_err(VCRError::IOError)?;
        let (snowflakes, times) = make_tries(position_index_decompressor);

        let mut dictionary_file = File::open(dict_file_path).map_err(VCRError::IOError)?;
        let mut dictionary: Vec<u8> = Vec::new();
        dictionary_file
            .read_to_end(&mut dictionary)
            .map_err(VCRError::IOError)?;

        let main_file = File::open(db_file_path).map_err(VCRError::IOError)?;
        let main_file_reader = BufReader::new(main_file);

        Ok(FeedDatabase {
            snowflakes: snowflakes,
            times: times,
            reader: main_file_reader,
            player_tags: player_tags
                .into_iter()
                .map(|(k, v)| (v, k))
                .collect::<HashMap<u16, Uuid>>(),
            team_tags: team_tags
                .into_iter()
                .map(|(k, v)| (v, k))
                .collect::<HashMap<u8, Uuid>>(),
            game_tags: game_tags
                .into_iter()
                .map(|(k, v)| (v, k))
                .collect::<HashMap<u16, Uuid>>(),
            dictionary: dictionary,
        })
    }

    pub fn read_event(&mut self, id: &Vec<u8>) -> VCRResult<FeedEvent> {
        if let Some(idx) = self.snowflakes.get(id) {
            let compressed_bytes: Vec<u8> = if idx.1 == 0 {
                let mut bytes: Vec<u8> = Vec::new();
                self.reader
                    .seek(SeekFrom::Start(idx.0))
                    .map_err(VCRError::IOError)?;
                self.reader
                    .read_to_end(&mut bytes)
                    .map_err(VCRError::IOError)?;
                bytes
            } else {
                let mut bytes: Vec<u8> = vec![0; (idx.1 - idx.0) as usize];

                self.reader
                    .seek(SeekFrom::Start(idx.0))
                    .map_err(VCRError::IOError)?;

                self.reader
                    .read_exact(&mut bytes)
                    .map_err(VCRError::IOError)?;
                bytes
            };

            let season = i8::from_be_bytes([id[0]]);
            let phase = u8::from_be_bytes([id[1]]);
            let timestamp = u32::from_be_bytes(id[2..6].try_into().unwrap());

            let mut decoder = zstd::stream::Decoder::with_dictionary(
                Cursor::new(compressed_bytes),
                &self.dictionary,
            )
            .map_err(VCRError::IOError)?;

            let mut category: [u8; 1] = [0; 1];
            let mut etype: [u8; 2] = [0; 2];
            let mut day: [u8; 2] = [0; 2];

            decoder
                .read_exact(&mut category)
                .map_err(VCRError::IOError)?;
            decoder.read_exact(&mut etype).map_err(VCRError::IOError)?;
            decoder.read_exact(&mut day).map_err(VCRError::IOError)?;
            
            let mut description_len_bytes: [u8; 2] = [0; 2];
            decoder
                .read_exact(&mut description_len_bytes)
                .map_err(VCRError::IOError)?;
            let description_len = u16::from_be_bytes(description_len_bytes);
            let mut description_bytes: Vec<u8> = vec![0; description_len as usize];
            decoder
                .read_exact(&mut description_bytes)
                .map_err(VCRError::IOError)?;

            let mut player_tag_len_bytes: [u8; 1] = [0; 1];
            decoder
                .read_exact(&mut player_tag_len_bytes)
                .map_err(VCRError::IOError)?;
            let player_tag_len = u8::from_be_bytes(player_tag_len_bytes);
            let mut player_tag_bytes: Vec<u8> = vec![0; (player_tag_len * 2) as usize];
            decoder
                .read_exact(&mut player_tag_bytes)
                .map_err(VCRError::IOError)?;

            let mut team_tag_len_bytes: [u8; 1] = [0; 1];
            decoder
                .read_exact(&mut team_tag_len_bytes)
                .map_err(VCRError::IOError)?;
            let team_tag_len = u8::from_be_bytes(team_tag_len_bytes);
            let mut team_tag_bytes: Vec<u8> = vec![0; team_tag_len as usize];
            decoder
                .read_exact(&mut team_tag_bytes)
                .map_err(VCRError::IOError)?;

            let mut game_tag_len_bytes: [u8; 1] = [0; 1];
            decoder
                .read_exact(&mut game_tag_len_bytes)
                .map_err(VCRError::IOError)?;
            let game_tag_len = u8::from_be_bytes(game_tag_len_bytes);
            let mut game_tag_bytes: Vec<u8> = vec![0; (game_tag_len * 2) as usize];
            decoder
                .read_exact(&mut game_tag_bytes)
                .map_err(VCRError::IOError)?;

            let mut metadata_bytes: Vec<u8> = Vec::new();
            decoder
                .read_to_end(&mut metadata_bytes)
                .map_err(VCRError::IOError)?;

            let player_tags: Vec<Uuid> = {
                let mut player_tag_ids: Vec<u16> = Vec::new();
                while player_tag_bytes.len() > 0 {
                    player_tag_ids.push(u16::from_be_bytes([
                        player_tag_bytes.remove(0),
                        player_tag_bytes.remove(0),
                    ]));
                }

                player_tag_ids
                    .into_iter()
                    .map(|id| self.player_tags[&id].clone())
                    .collect()
            };

            let team_tags: Vec<Uuid> = {
                let mut team_tag_ids: Vec<u8> = Vec::new();
                while team_tag_bytes.len() > 0 {
                    team_tag_ids.push(u8::from_be_bytes([team_tag_bytes.remove(0)]));
                }

                team_tag_ids
                    .into_iter()
                    .map(|id| self.team_tags[&id].clone())
                    .collect()
            };

            let game_tags: Vec<Uuid> = {
                let mut game_tag_ids: Vec<u16> = Vec::new();
                while game_tag_bytes.len() > 0 {
                    game_tag_ids.push(u16::from_be_bytes([
                        game_tag_bytes.remove(0),
                        game_tag_bytes.remove(0),
                    ]));
                }

                game_tag_ids
                    .into_iter()
                    .map(|id| self.game_tags[&id].clone())
                    .collect()
            };

            let metadata: JSONValue = rmp_serde::from_read_ref(&metadata_bytes).unwrap();

            Ok(FeedEvent {
                id: Uuid::nil(),
                category: i8::from_be_bytes(category),
                created: Utc.timestamp(timestamp as i64, 0),
                day: i16::from_be_bytes(day),
                season: season,
                nuts: 0,
                phase: phase,
                player_tags: Some(player_tags),
                team_tags: Some(team_tags),
                game_tags: Some(game_tags),
                etype: i16::from_be_bytes(etype),
                tournament: -1,
                description: String::from_utf8(description_bytes).unwrap(),
                metadata: metadata,
            })
        } else {
            Err(VCRError::EntityNotFound)
        }
    }

    pub fn events_after(
        &mut self,
        timestamp: DateTime<Utc>,
        count: usize,
    ) -> VCRResult<Vec<FeedEvent>> {
        let mut prefix = [
            (timestamp.year() as u16).to_be_bytes().to_vec(),
            vec![
                timestamp.month() as u8,
                timestamp.day() as u8,
                timestamp.hour() as u8,
            ],
        ]
        .concat();

        let mut ids = Vec::new();

        while ids.len() < 1 {
            ids = self
                .times
                .iter_prefix(&prefix)
                .filter_map(|(k, v)| {
                    let date = Utc
                        .ymd(
                            u16::from_be_bytes([k[0], k[1]]) as i32,
                            u8::from_be_bytes([k[2]]) as u32,
                            u8::from_be_bytes([k[3]]) as u32,
                        )
                        .and_hms(
                            u8::from_be_bytes([k[4]]) as u32,
                            u8::from_be_bytes([k[5]]) as u32,
                            u8::from_be_bytes([k[6]]) as u32,
                        );
                    if date >= timestamp {
                        Some((date, v.into_iter().copied().collect()))
                    } else {
                        None
                    }
                })
                .collect::<Vec<(DateTime<Utc>, Vec<u8>)>>();

            prefix.pop();
        }

        ids.sort_by_key(|(t, _)| t.clone());

        ids.iter()
            .map(|(_, snowflake)| self.read_event(snowflake))
            .collect::<VCRResult<Vec<FeedEvent>>>()
    }

    pub fn events_before(
        &mut self,
        timestamp: DateTime<Utc>,
        count: usize,
    ) -> VCRResult<Vec<FeedEvent>> {
        let mut prefix = [
            (timestamp.year() as u16).to_be_bytes().to_vec(),
            vec![
                timestamp.month() as u8,
                timestamp.day() as u8,
                timestamp.hour() as u8,
            ],
        ]
        .concat();

        let mut ids = Vec::new();
        while ids.len() < 1 {
            ids = self
                .times
                .iter_prefix(&prefix)
                .filter_map(|(k, v)| {
                    let date = Utc
                        .ymd(
                            u16::from_be_bytes([k[0], k[1]]) as i32,
                            u8::from_be_bytes([k[2]]) as u32,
                            u8::from_be_bytes([k[3]]) as u32,
                        )
                        .and_hms(
                            u8::from_be_bytes([k[4]]) as u32,
                            u8::from_be_bytes([k[5]]) as u32,
                            u8::from_be_bytes([k[6]]) as u32,
                        );
                    if date <= timestamp {
                        Some((date, v.into_iter().copied().collect()))
                    } else {
                        None
                    }
                })
                .collect::<Vec<(DateTime<Utc>, Vec<u8>)>>();

            prefix.pop();
        }

        ids.sort_by_key(|(t, _)| t.clone());
        ids.reverse();

        ids.iter()
            .take(count)
            .map(|(_, snowflake)| self.read_event(snowflake))
            .collect::<VCRResult<Vec<FeedEvent>>>()
    }

    pub fn events_by_phase(
        &mut self,
        season: i8,
        phase: u8,
        count: usize,
    ) -> VCRResult<Vec<FeedEvent>> {
        let ids = self
            .snowflakes
            .iter_prefix(&[season.to_be_bytes(), phase.to_be_bytes()].concat())
            .map(|(k, _)| k.into_iter().copied().collect())
            .take(count)
            .collect::<Vec<Vec<u8>>>();

        ids.iter()
            .map(|snowflake| self.read_event(snowflake))
            .collect::<VCRResult<Vec<FeedEvent>>>()
    }
}
