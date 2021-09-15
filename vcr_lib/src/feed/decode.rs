use super::*;
use crate::{VCRError, VCRResult};
use chrono::{DateTime, Datelike, TimeZone, Timelike, Utc};
use lru::LruCache;
use patricia_tree::PatriciaMap;
use serde_json::Value as JSONValue;
use std::collections::HashMap;
use std::convert::TryInto;
use std::fs::File;
use std::io::{BufReader, Cursor, Read, Seek, SeekFrom};
use std::path::Path;
use uuid::Uuid;
use zstd::dict::DecoderDictionary;

fn make_tries<R: Read>(mut reader: R) -> (PatriciaMap<(u64, u64)>, PatriciaMap<Vec<u8>>) {
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

        if !index.is_empty() {
            let idx = index.len() - 1;
            let mut a = index[idx].clone();
            a.1 .1 = start_pos;
            index[idx] = a;
        }

        index.push((snowflake, (start_pos, 0u64)));

        last_position = start_pos;
    }

    let mut snowflake_trie: PatriciaMap<(u64, u64)> = PatriciaMap::new();
    let mut time_trie: PatriciaMap<Vec<u8>> = PatriciaMap::new();

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
    pub snowflakes: PatriciaMap<(u64, u64)>,
    times: PatriciaMap<Vec<u8>>,
    player_index: HashMap<u16, Vec<Vec<u8>>>,
    team_index: HashMap<u8, Vec<Vec<u8>>>,
    game_index: HashMap<u16, Vec<Vec<u8>>>,
    reader: BufReader<File>,
    player_tags: HashMap<u16, Uuid>,
    game_tags: HashMap<u16, Uuid>,
    team_tags: HashMap<u8, Uuid>,
    reverse_player_tags: HashMap<Uuid, u16>,
    reverse_game_tags: HashMap<Uuid, u16>,
    reverse_team_tags: HashMap<Uuid, u8>,
    millis_epoch_table: HashMap<(i8, u8), u32>,
    dictionary: DecoderDictionary<'static>,
    cache: LruCache<Vec<u8>, FeedEvent>,
}

impl FeedDatabase {
    pub fn from_files<P: AsRef<Path> + std::fmt::Debug>(
        position_index_path: P,
        db_file_path: P,
        dict_file_path: P,
        id_table_path: P,
        idx_file_path: P,
        cache_size: usize,
    ) -> VCRResult<FeedDatabase> {
        let id_file = File::open(id_table_path)?;
        let (team_tags, player_tags, game_tags, millis_epoch_table) = rmp_serde::from_read::<
            File,
            (
                HashMap<Uuid, u8>,
                HashMap<Uuid, u16>,
                HashMap<Uuid, u16>,
                HashMap<(i8, u8), u32>,
            ),
        >(id_file)?; // todo: result

        let idx_file = File::open(idx_file_path)?;
        let (team_idx, player_idx, game_idx) = rmp_serde::from_read_ref::<
            Vec<u8>,
            (
                HashMap<u8, Vec<Vec<u8>>>,
                HashMap<u16, Vec<Vec<u8>>>,
                HashMap<u16, Vec<Vec<u8>>>,
            ),
        >(&zstd::decode_all(idx_file).unwrap())?;

        let position_index_file = File::open(position_index_path)?;
        let position_index_decompressor = zstd::stream::Decoder::new(position_index_file)?;
        let (snowflakes, times) = make_tries(position_index_decompressor);

        let mut dictionary_file = File::open(dict_file_path)?;
        let mut dictionary: Vec<u8> = Vec::new();
        dictionary_file.read_to_end(&mut dictionary)?;

        let main_file = File::open(db_file_path)?;
        let main_file_reader = BufReader::new(main_file);

        let mut player_index = HashMap::new();
        let mut team_index = HashMap::new();
        let mut game_index = HashMap::new();

        for (tag, snowflakes) in player_idx {
            player_index.entry(tag).or_insert_with(Vec::new);

            if let Some(tr) = player_index.get_mut(&tag) {
                for s in snowflakes {
                    tr.push(s.clone());
                }
            }
        }

        for (tag, snowflakes) in team_idx {
            team_index.entry(tag).or_insert_with(Vec::new);

            if let Some(tr) = team_index.get_mut(&tag) {
                for s in snowflakes {
                    tr.push(s.clone());
                }
            }
        }

        for (tag, snowflakes) in game_idx {
            game_index.entry(tag).or_insert_with(Vec::new);

            if let Some(tr) = game_index.get_mut(&tag) {
                for s in snowflakes {
                    tr.push(s.clone());
                }
            }
        }

        Ok(FeedDatabase {
            snowflakes,
            times,
            reader: main_file_reader,
            player_tags: player_tags
                .clone()
                .into_iter()
                .map(|(k, v)| (v, k))
                .collect::<HashMap<u16, Uuid>>(),
            team_tags: team_tags
                .clone()
                .into_iter()
                .map(|(k, v)| (v, k))
                .collect::<HashMap<u8, Uuid>>(),
            game_tags: game_tags
                .clone()
                .into_iter()
                .map(|(k, v)| (v, k))
                .collect::<HashMap<u16, Uuid>>(),
            reverse_player_tags: player_tags,
            reverse_team_tags: team_tags,
            reverse_game_tags: game_tags,
            player_index,
            game_index,
            team_index,
            millis_epoch_table,
            dictionary: DecoderDictionary::copy(&dictionary),
            cache: LruCache::new(cache_size),
        })
    }

    pub fn read_event(&mut self, snowflake: &Vec<u8>) -> VCRResult<FeedEvent> {
        if let Some(ev) = self.cache.get(snowflake) {
            return Ok(ev.clone());
        }

        if let Some(idx) = self.snowflakes.get(snowflake) {
            let compressed_bytes: Vec<u8> = if idx.1 == 0 {
                let mut bytes: Vec<u8> = Vec::new();
                self.reader.seek(SeekFrom::Start(idx.0))?;
                self.reader.read_to_end(&mut bytes)?;
                bytes
            } else {
                let mut bytes: Vec<u8> = vec![0; (idx.1 - idx.0) as usize];

                self.reader.seek(SeekFrom::Start(idx.0))?;

                self.reader.read_exact(&mut bytes)?;
                bytes
            };

            let season = i8::from_be_bytes([snowflake[0]]);
            let phase = u8::from_be_bytes([snowflake[1]]);
            let timestamp_raw = u32::from_be_bytes(snowflake[2..6].try_into().unwrap());
            let timestamp = match self.millis_epoch_table.get(&(season, phase)) {
                Some(epoch) => (*epoch as i64) * 1000 + (timestamp_raw as i64),
                None => (timestamp_raw as i64) * 1000,
            };

            let mut decoder = zstd::stream::Decoder::with_prepared_dictionary(
                Cursor::new(compressed_bytes),
                &self.dictionary,
            )?;

            let mut category: [u8; 1] = [0; 1];
            let mut etype: [u8; 2] = [0; 2];
            let mut day: [u8; 2] = [0; 2];

            let id = if phase == 13 {
                let mut uuid: [u8; 16] = [0; 16];
                decoder.read_exact(&mut uuid)?;
                Uuid::from_bytes(uuid)
            } else {
                Uuid::nil()
            };

            decoder.read_exact(&mut category)?;
            decoder.read_exact(&mut etype)?;
            decoder.read_exact(&mut day)?;

            let mut description_len_bytes: [u8; 2] = [0; 2];
            decoder.read_exact(&mut description_len_bytes)?;
            let description_len = u16::from_be_bytes(description_len_bytes);
            let mut description_bytes: Vec<u8> = vec![0; description_len as usize];
            decoder.read_exact(&mut description_bytes)?;

            let mut player_tag_len_bytes: [u8; 1] = [0; 1];
            decoder.read_exact(&mut player_tag_len_bytes)?;
            let player_tag_len = u8::from_be_bytes(player_tag_len_bytes);
            let mut player_tag_bytes: Vec<u8> = vec![0; (player_tag_len * 2) as usize];
            decoder.read_exact(&mut player_tag_bytes)?;

            let mut team_tag_len_bytes: [u8; 1] = [0; 1];
            decoder.read_exact(&mut team_tag_len_bytes)?;
            let team_tag_len = u8::from_be_bytes(team_tag_len_bytes);
            let mut team_tag_bytes: Vec<u8> = vec![0; team_tag_len as usize];
            decoder.read_exact(&mut team_tag_bytes)?;

            let mut game_tag_len_bytes: [u8; 1] = [0; 1];
            decoder.read_exact(&mut game_tag_len_bytes)?;
            let game_tag_len = u8::from_be_bytes(game_tag_len_bytes);
            let mut game_tag_bytes: Vec<u8> = vec![0; (game_tag_len * 2) as usize];
            decoder.read_exact(&mut game_tag_bytes)?;

            let mut metadata_bytes: Vec<u8> = Vec::new();
            decoder.read_to_end(&mut metadata_bytes)?;

            let player_tags: Vec<Uuid> = {
                let mut player_tag_ids: Vec<u16> = Vec::new();
                while !player_tag_bytes.is_empty() {
                    player_tag_ids.push(u16::from_be_bytes([
                        player_tag_bytes.remove(0),
                        player_tag_bytes.remove(0),
                    ]));
                }

                player_tag_ids
                    .into_iter()
                    .map(|id| self.player_tags[&id])
                    .collect()
            };

            let team_tags: Vec<Uuid> = {
                let mut team_tag_ids: Vec<u8> = Vec::new();
                while !team_tag_bytes.is_empty() {
                    team_tag_ids.push(u8::from_be_bytes([team_tag_bytes.remove(0)]));
                }

                team_tag_ids
                    .into_iter()
                    .map(|id| self.team_tags[&id])
                    .collect()
            };

            let game_tags: Vec<Uuid> = {
                let mut game_tag_ids: Vec<u16> = Vec::new();
                while !game_tag_bytes.is_empty() {
                    game_tag_ids.push(u16::from_be_bytes([
                        game_tag_bytes.remove(0),
                        game_tag_bytes.remove(0),
                    ]));
                }

                game_tag_ids
                    .into_iter()
                    .map(|id| self.game_tags[&id])
                    .collect()
            };

            let metadata: JSONValue = rmp_serde::from_read_ref(&metadata_bytes)?;

            let ev = FeedEvent {
                id,
                category: i8::from_be_bytes(category),
                created: Utc.timestamp_millis(timestamp),
                day: i16::from_be_bytes(day),
                season,
                nuts: 0,
                phase,
                player_tags: Some(player_tags),
                team_tags: Some(team_tags),
                game_tags: Some(game_tags),
                etype: i16::from_be_bytes(etype),
                tournament: -1,
                description: String::from_utf8(description_bytes).unwrap(),
                metadata,
            };

            self.cache.put(snowflake.clone(), ev.clone());
            Ok(ev)
        } else {
            Err(VCRError::EntityNotFound)
        }
    }

    pub fn events_after(
        &mut self,
        timestamp: DateTime<Utc>,
        count: usize,
        category: i8,
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

        let mut events: Vec<(DateTime<Utc>, FeedEvent, Vec<u8>)> = Vec::with_capacity(count * 2);
        while events.len() < count {
            events.extend(
                self.times
                    .iter_prefix(&prefix)
                    .filter_map(|(_, v)| {
                        let date = Utc
                            .timestamp(u32::from_be_bytes(v[2..6].try_into().unwrap()) as i64, 0);
                        if date >= timestamp {
                            Some((date, v.clone()))
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<(DateTime<Utc>, Vec<u8>)>>()
                    .into_iter()
                    .map(|(t, v)| (t, self.read_event(&v).unwrap(), v))
                    .filter_map(|(t, e, v)| {
                        if category == -3 || e.category == category {
                            Some((t, e, v))
                        } else {
                            None
                        }
                    }),
            );

            prefix.pop();
        }

        events.sort_by_key(|&(t, _, _)| t);
        events.dedup_by_key(|(_, _, s)| s.clone()); // clone :ballclark: (i don't know how i could avoid it here so, here we are.)
        Ok(events
            .into_iter()
            .map(|(_, e, _)| e)
            .take(count)
            .collect::<Vec<FeedEvent>>())
    }

    pub fn events_before(
        &mut self,
        timestamp: DateTime<Utc>,
        count: usize,
        category: i8,
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

        let mut events: Vec<(DateTime<Utc>, FeedEvent, Vec<u8>)> = Vec::with_capacity(count * 2);
        while events.len() < count {
            events.extend(
                self.times
                    .iter_prefix(&prefix)
                    .filter_map(|(_, v)| {
                        let date = Utc
                            .timestamp(u32::from_be_bytes(v[2..6].try_into().unwrap()) as i64, 0);
                        if date <= timestamp {
                            Some((date, v.clone()))
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<(DateTime<Utc>, Vec<u8>)>>()
                    .into_iter()
                    .map(|(t, v)| (t, self.read_event(&v).unwrap(), v)) // so, not unwrapping here means collecting twice. personally i'm ok trading that off for now, though it might not be worth it.
                    .filter_map(|(t, e, v)| {
                        if category == -3 || e.category == category {
                            Some((t, e, v))
                        } else {
                            None
                        }
                    }),
            );

            prefix.pop();
        }

        events.sort_by_key(|&(t, _, _)| t);
        events.dedup_by_key(|(_, _, s)| s.clone()); // clone :ballclark: (i don't know how i could avoid it here so, here we are.)
        events.reverse();
        Ok(events
            .into_iter()
            .map(|(_, e, _)| e)
            .take(count)
            .collect::<Vec<FeedEvent>>())
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
            .map(|(k, _)| k)
            .take(count)
            .collect::<Vec<Vec<u8>>>();

        ids.iter()
            .map(|snowflake| self.read_event(snowflake))
            .collect::<VCRResult<Vec<FeedEvent>>>()
    }

    pub fn events_by_tag_and_time(
        &mut self,
        timestamp: DateTime<Utc>,
        tag: &Uuid,
        tag_type: TagType,
        count: usize,
        category: i8,
    ) -> VCRResult<Vec<FeedEvent>> {
        let tag: u16 = match tag_type {
            TagType::Game => *self
                .reverse_game_tags
                .get(tag)
                .ok_or(VCRError::EntityNotFound)? as u16,
            TagType::Team => *self
                .reverse_team_tags
                .get(tag)
                .ok_or(VCRError::EntityNotFound)? as u16,
            TagType::Player => *self
                .reverse_player_tags
                .get(tag)
                .ok_or(VCRError::EntityNotFound)? as u16,
        };

        let mut ids = Vec::new();

        while ids.is_empty() {
            ids.extend(
                (match tag_type {
                    TagType::Game => self.game_index[&tag].iter(),
                    TagType::Team => self.team_index[&(tag as u8)].iter(),
                    TagType::Player => self.player_index[&tag].iter(),
                })
                .filter_map(|snowflake| {
                    let time = u32::from_be_bytes(snowflake[2..6].try_into().unwrap());
                    let date = Utc.timestamp(time as i64, 0);
                    if date <= timestamp {
                        Some((date, snowflake.clone()))
                    } else {
                        None
                    }
                })
                .take(count - ids.len()),
            );
        }

        ids.sort_by_key(|(t, _)| *t);
        ids.reverse();

        Ok(ids
            .iter()
            .map(|(_, snowflake)| {
                self.read_event(snowflake).map(|e| {
                    if category == -3 || e.category == category {
                        Some(e)
                    } else {
                        None
                    }
                })
            })
            .collect::<VCRResult<Vec<Option<FeedEvent>>>>()?
            .into_iter()
            .flatten()
            .take(count)
            .collect::<Vec<FeedEvent>>())
    }
}
