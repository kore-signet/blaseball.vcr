use super::*;
use crate::{VCRError, VCRResult};
use chrono::{DateTime, TimeZone, Utc};
use lru::LruCache;

use serde_json::Value as JSONValue;
use std::collections::HashMap;
use std::convert::TryInto;
use std::fs::File;
use std::io::{BufReader, Cursor, Read, Seek, SeekFrom};
use std::path::Path;
use uuid::Uuid;
use zstd::dict::DecoderDictionary;

fn make_offset_table<R: Read>(mut reader: R) -> Vec<(DateTime<Utc>, (u32, u16))> {
    let mut last_position: u64 = 0;
    let mut index: Vec<(DateTime<Utc>, (u32, u16))> = Vec::with_capacity(5110062);

    loop {
        let mut snowflake: Vec<u8> = vec![0; 6];
        if reader.read_exact(&mut snowflake).is_err() {
            break;
        }

        let position_delta = u16::from_be_bytes(snowflake[0..2].try_into().unwrap());
        let start_pos = last_position + position_delta as u64;

        if !index.is_empty() {
            let idx = index.len() - 1;
            let mut a = index[idx];
            a.1 .1 = (start_pos as u32 - a.1 .0) as u16;
            index[idx] = a;
        }

        index.push((
            Utc.timestamp(
                u32::from_be_bytes(snowflake[2..6].try_into().unwrap()) as i64,
                0,
            ),
            (start_pos as u32, 0u16),
        ));

        last_position = start_pos;
    }

    index.sort_unstable_by_key(|(t, _)| t.timestamp());

    index
}

pub struct FeedDatabase {
    offset_table: Vec<(DateTime<Utc>, (u32, u16))>,
    meta_index: MetaIndex,
    event_index: EventIndex,
    reader: BufReader<File>,
    dictionary: DecoderDictionary<'static>,
    cache: LruCache<u32, FeedEvent>,
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
        let meta_idx: MetaIndex = rmp_serde::from_read(id_file)?;

        let idx_file = File::open(idx_file_path)?;
        let mut idx_decoder = zstd::Decoder::new(idx_file)?;

        let game_index = {
            let mut idx: HashMap<u16, Vec<(u32, (u32, u16))>> = HashMap::new();
            let idx_len = read_u32!(idx_decoder);
            let mut bytes: Vec<u8> = vec![0; idx_len as usize];
            idx_decoder.read_exact(&mut bytes)?;
            let mut cursor = Cursor::new(bytes);

            while cursor.position() < idx_len as u64 {
                let key = read_u16!(cursor);
                let klen: u64 = read_u32!(cursor) as u64;
                let start_pos = cursor.position();

                let entry = idx
                    .entry(key)
                    .or_insert_with(|| Vec::with_capacity(klen as usize));

                while (cursor.position() - start_pos) < klen {
                    entry.push((
                        read_u32!(cursor),
                        (read_u32!(cursor), decode_varint!(cursor)),
                    ));
                }
            }

            idx
        };

        let player_index = {
            let mut idx: HashMap<u16, Vec<(u32, (u32, u16))>> = HashMap::new();
            let idx_len = read_u32!(idx_decoder);
            let mut bytes: Vec<u8> = vec![0; idx_len as usize];
            idx_decoder.read_exact(&mut bytes)?;
            let mut cursor = Cursor::new(bytes);

            while cursor.position() < idx_len as u64 {
                let key = read_u16!(cursor);
                let klen: u64 = read_u32!(cursor) as u64;
                let start_pos = cursor.position();

                let entry = idx
                    .entry(key)
                    .or_insert_with(|| Vec::with_capacity(klen as usize));

                while (cursor.position() - start_pos) < klen {
                    entry.push((
                        read_u32!(cursor),
                        (read_u32!(cursor), decode_varint!(cursor)),
                    ));
                }
            }

            idx
        };

        let team_index = {
            let mut idx: HashMap<u8, Vec<(u32, (u32, u16))>> = HashMap::new();
            let idx_len = read_u32!(idx_decoder);
            let mut bytes: Vec<u8> = vec![0; idx_len as usize];
            idx_decoder.read_exact(&mut bytes)?;
            let mut cursor = Cursor::new(bytes);

            while cursor.position() < idx_len as u64 {
                let key = read_u8!(cursor);
                let klen: u64 = read_u32!(cursor) as u64;
                let start_pos = cursor.position();

                let entry = idx
                    .entry(key)
                    .or_insert_with(|| Vec::with_capacity(klen as usize));

                while (cursor.position() - start_pos) < klen {
                    entry.push((
                        read_u32!(cursor),
                        (read_u32!(cursor), decode_varint!(cursor)),
                    ));
                }
            }

            idx
        };

        let phase_index = {
            let mut idx: HashMap<(i8, u8), Vec<(i64, (u32, u16))>> = HashMap::new();
            let idx_len = read_u32!(idx_decoder);
            let mut bytes: Vec<u8> = vec![0; idx_len as usize];
            idx_decoder.read_exact(&mut bytes)?;
            let mut cursor = Cursor::new(bytes);

            while cursor.position() < idx_len as u64 {
                let key = (read_i8!(cursor), read_u8!(cursor));
                let klen: u64 = read_u32!(cursor) as u64;
                let start_pos = cursor.position();

                let entry = idx
                    .entry(key)
                    .or_insert_with(|| Vec::with_capacity(klen as usize));

                while (cursor.position() - start_pos) < klen {
                    entry.push((
                        read_i64!(cursor),
                        (read_u32!(cursor), decode_varint!(cursor)),
                    ));
                }
            }

            idx
        };

        let event_idx: EventIndex = EventIndex {
            player_index,
            team_index,
            phase_index,
            game_index,
        };

        let position_index_file = File::open(position_index_path)?;
        let position_index_decompressor = zstd::stream::Decoder::new(position_index_file)?;
        let offset_table = make_offset_table(position_index_decompressor);

        let mut dictionary_file = File::open(dict_file_path)?;
        let mut dictionary: Vec<u8> = Vec::new();
        dictionary_file.read_to_end(&mut dictionary)?;

        let main_file = File::open(db_file_path)?;
        let main_file_reader = BufReader::new(main_file);

        Ok(FeedDatabase {
            offset_table,
            reader: main_file_reader,
            event_index: event_idx,
            meta_index: meta_idx,
            dictionary: DecoderDictionary::copy(&dictionary),
            cache: LruCache::new(cache_size),
        })
    }

    pub fn read_event(
        &mut self,
        offset: u32,
        len: u16,
        timestamp: DateTime<Utc>,
    ) -> VCRResult<FeedEvent> {
        if let Some(ev) = self.cache.get(&offset) {
            return Ok(ev.clone());
        }

        let compressed_bytes: Vec<u8> = if len == 0 {
            let mut bytes: Vec<u8> = Vec::new();
            self.reader.seek(SeekFrom::Start(offset as u64))?;
            self.reader.read_to_end(&mut bytes)?;
            bytes
        } else {
            let mut bytes: Vec<u8> = vec![0; len as usize];

            self.reader.seek(SeekFrom::Start(offset as u64))?;

            self.reader.read_exact(&mut bytes)?;
            bytes
        };

        // let timestamp_raw = u32::from_be_bytes(snowflake[2..6].try_into().unwrap());
        // let timestamp = match self.millis_epoch_table.get(&(season, phase)) {
        //     Some(epoch) => (*epoch as i64) * 1000 + (timestamp_raw as i64),
        //     None => (timestamp_raw as i64) * 1000,
        // };

        let mut decoder = zstd::stream::Decoder::with_prepared_dictionary(
            Cursor::new(compressed_bytes),
            &self.dictionary,
        )?;

        let mut category: [u8; 1] = [0; 1];
        let mut etype: [u8; 2] = [0; 2];
        let mut day: [u8; 2] = [0; 2];
        let mut season: [u8; 1] = [0; 1];
        let mut phase: [u8; 1] = [0; 1];

        decoder.read_exact(&mut category)?;
        decoder.read_exact(&mut etype)?;
        decoder.read_exact(&mut day)?;
        decoder.read_exact(&mut season)?;
        decoder.read_exact(&mut phase)?;

        let phase = u8::from_be_bytes(phase);

        let id = if phase == 13 {
            let mut uuid: [u8; 16] = [0; 16];
            decoder.read_exact(&mut uuid)?;
            Uuid::from_bytes(uuid)
        } else {
            Uuid::nil()
        };

        use EventDescription::*;
        let description = match EventDescription::from_type(i16::from_be_bytes(etype)) {
            Constant(s) => s.to_owned(),
            ConstantVariant(possibilities) => {
                let mut variant_byte: [u8; 1] = [0; 1];
                decoder.read_exact(&mut variant_byte)?;
                possibilities[u8::from_be(variant_byte[0]) as usize].to_owned()
            }
            Suffix(sfx) => {
                let description_len = decode_varint!(decoder);
                let mut description_bytes: Vec<u8> = vec![0; description_len as usize];
                decoder.read_exact(&mut description_bytes)?;

                String::from_utf8(description_bytes).unwrap() + sfx
            }
            Prefix(pfx) => {
                let description_len = decode_varint!(decoder);
                let mut description_bytes: Vec<u8> = vec![0; description_len as usize];
                decoder.read_exact(&mut description_bytes)?;

                pfx.to_owned() + &String::from_utf8(description_bytes).unwrap()
            }
            Variable => {
                let description_len = decode_varint!(decoder);
                let mut description_bytes: Vec<u8> = vec![0; description_len as usize];
                decoder.read_exact(&mut description_bytes)?;
                String::from_utf8(description_bytes).unwrap()
            }
        };

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
                .map(|id| self.meta_index.player_tags[&id])
                .collect()
        };

        let team_tags: Vec<Uuid> = {
            let mut team_tag_ids: Vec<u8> = Vec::new();
            while !team_tag_bytes.is_empty() {
                team_tag_ids.push(u8::from_be_bytes([team_tag_bytes.remove(0)]));
            }

            team_tag_ids
                .into_iter()
                .map(|id| self.meta_index.team_tags[&id])
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
                .map(|id| self.meta_index.game_tags[&id])
                .collect()
        };

        let metadata: JSONValue = rmp_serde::from_read_ref(&metadata_bytes)?;

        let ev = FeedEvent {
            id,
            category: i8::from_be_bytes(category),
            created: timestamp,
            day: i16::from_be_bytes(day),
            season: i8::from_be_bytes(season),
            nuts: 0,
            phase,
            player_tags: Some(player_tags),
            team_tags: Some(team_tags),
            game_tags: Some(game_tags),
            etype: i16::from_be_bytes(etype),
            tournament: -1,
            description,
            metadata,
        };

        self.cache.put(offset, ev.clone());
        Ok(ev)
    }

    pub fn events_after(
        &mut self,
        timestamp: DateTime<Utc>,
        count: usize,
        category: i8,
    ) -> VCRResult<Vec<FeedEvent>> {
        let mut idx = match self
            .offset_table
            .binary_search_by_key(&timestamp.timestamp(), |(s, _)| s.timestamp())
        {
            Ok(i) => i,
            Err(i) => i,
        };

        let mut events: Vec<FeedEvent> = Vec::with_capacity(count);
        while idx < self.offset_table.len() && events.len() < count {
            let (time, (offset, length)) = self.offset_table[idx];
            let e = self.read_event(offset, length, time)?;
            if category == -3 || e.category == category {
                events.push(e);
            }

            idx += 1;
        }

        Ok(events)
    }

    pub fn events_before(
        &mut self,
        timestamp: DateTime<Utc>,
        count: usize,
        category: i8,
    ) -> VCRResult<Vec<FeedEvent>> {
        let mut idx = match self
            .offset_table
            .binary_search_by_key(&timestamp.timestamp(), |(s, _)| s.timestamp())
        {
            Ok(i) => i,
            Err(i) => i,
        };

        let mut events: Vec<FeedEvent> = Vec::with_capacity(count);
        while idx > 0 && events.len() < count {
            let (time, (offset, length)) = self.offset_table[idx];
            let e = self.read_event(offset, length, time)?;
            if category == -3 || e.category == category {
                events.push(e);
            }

            idx -= 1;
        }
        
        Ok(events)
    }

    pub fn events_by_phase(
        &mut self,
        season: i8,
        phase: u8,
        count: usize,
    ) -> VCRResult<Vec<FeedEvent>> {
        let ids = self.event_index.phase_index[&(season, phase)]
            .iter()
            .take(count)
            .copied()
            .collect::<Vec<(i64, (u32, u16))>>();

        ids.iter()
            .map(|(time, (offset, len))| {
                self.read_event(*offset, *len, Utc.timestamp_millis(*time))
            })
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
                .meta_index
                .reverse_game_tags
                .get(tag)
                .ok_or(VCRError::EntityNotFound)? as u16,
            TagType::Team => *self
                .meta_index
                .reverse_team_tags
                .get(tag)
                .ok_or(VCRError::EntityNotFound)? as u16,
            TagType::Player => *self
                .meta_index
                .reverse_player_tags
                .get(tag)
                .ok_or(VCRError::EntityNotFound)? as u16,
        };

        let len = match tag_type {
            TagType::Game => self.event_index.game_index[&tag].len(),
            TagType::Team => self.event_index.team_index[&(tag as u8)].len(),
            TagType::Player => self.event_index.player_index[&tag].len(),
        };

        let mut idx = 0;

        let mut events = Vec::with_capacity(count);
        while idx < len && events.len() < count {
            // AaaaaaaaaaaaaAAAAAAAAAAAAAAaaaAAAAAAAAAAAAaaAAAAAA
            let (time, (offset, length)) = match tag_type {
                TagType::Game => self.event_index.game_index[&tag][idx],
                TagType::Team => self.event_index.team_index[&(tag as u8)][idx],
                TagType::Player => self.event_index.player_index[&tag][idx],
            };

            let time = Utc.timestamp(time as i64, 0);

            if time <= timestamp {
                let e = self.read_event(offset, length, time)?;
                if category == -3 || e.category == category {
                    events.push(e);
                }
            }

            idx += 1;
        }

        events.sort_by_key(|e| e.created.timestamp());
        events.dedup();
        events.reverse();

        Ok(events)
    }
}
