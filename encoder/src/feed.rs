use blaseball_vcr::feed::{CompactedFeedEvent, FeedEvent};
use chrono::{Duration, DurationRound};
use crossbeam::channel::bounded;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Cursor, Read, Seek, Write};
use uuid::Uuid;

fn main() {
    let (snd1, rcv1) = bounded(1);
    let (snd2, rcv2) = bounded(1);
    let n_workers = 4;

    let mut feed_dict: Vec<u8> = Vec::new();
    let mut dict_f = File::open("zstd-dictionaries/feed.dict").unwrap();
    dict_f.read_to_end(&mut feed_dict).unwrap();

    crossbeam::scope(|s| {
        // Producer thread
        s.spawn(|_| {
            let mut player_tag_table: HashMap<Uuid, u16> = HashMap::new();
            let mut game_tag_table: HashMap<Uuid, u16> = HashMap::new();
            let mut team_tag_table: HashMap<Uuid, u8> = HashMap::new();
            let mut millis_epoch_table: HashMap<(i8, u8), u32> = HashMap::new();
            let mut player_tag_idx: HashMap<u16, Vec<Vec<u8>>> = HashMap::new();
            let mut game_tag_idx: HashMap<u16, Vec<Vec<u8>>> = HashMap::new();
            let mut team_tag_idx: HashMap<u8, Vec<Vec<u8>>> = HashMap::new();

            let f = File::open("feed.json").unwrap();
            let reader = BufReader::new(f);

            for l in reader.lines() {
                let event: FeedEvent = serde_json::from_str(&l.unwrap()).unwrap();

                let millis_epoch = if event.season >= 11 && [3, 5, 13].contains(&event.phase) {
                    Some(
                        millis_epoch_table
                            .entry((event.season, event.phase))
                            .or_insert(
                                event
                                    .created
                                    .duration_trunc(Duration::hours(1))
                                    .unwrap()
                                    .timestamp() as u32,
                            ),
                    )
                } else {
                    None
                };

                let snowflake_id = event.generate_id(millis_epoch.copied());
                let compact_player_tags: Vec<u16> = event
                    .player_tags
                    .clone()
                    .unwrap_or_default()
                    .iter()
                    .map(|id| {
                        if let Some(n) = player_tag_table.get(id) {
                            *n
                        } else {
                            let n = player_tag_table.len() as u16;
                            player_tag_table.insert(*id, n);
                            player_tag_idx.insert(n, Vec::new());
                            n
                        }
                    })
                    .collect();

                let compact_game_tags: Vec<u16> = event
                    .game_tags
                    .clone()
                    .unwrap_or_default()
                    .iter()
                    .map(|id| {
                        if let Some(n) = game_tag_table.get(id) {
                            *n
                        } else {
                            let n = game_tag_table.len() as u16;
                            game_tag_table.insert(*id, n);
                            game_tag_idx.insert(n, Vec::new());
                            n
                        }
                    })
                    .collect();

                let compact_team_tags: Vec<u8> = event
                    .team_tags
                    .clone()
                    .unwrap_or_default()
                    .iter()
                    .map(|id| {
                        if let Some(n) = team_tag_table.get(id) {
                            *n
                        } else {
                            let n = team_tag_table.len() as u8;
                            team_tag_table.insert(*id, n);
                            team_tag_idx.insert(n, Vec::new());
                            n
                        }
                    })
                    .collect();

                for t in &compact_team_tags {
                    if let Some(ids) = team_tag_idx.get_mut(t) {
                        ids.push(snowflake_id.clone());
                    }
                }

                for t in &compact_player_tags {
                    if let Some(ids) = player_tag_idx.get_mut(t) {
                        ids.push(snowflake_id.clone());
                    }
                }

                for t in &compact_game_tags {
                    if let Some(ids) = game_tag_idx.get_mut(t) {
                        ids.push(snowflake_id.clone());
                    }
                }

                snd1.send((
                    snowflake_id,
                    (CompactedFeedEvent {
                        id: event.id,
                        category: event.category,
                        day: event.day,
                        description: event.description,
                        player_tags: compact_player_tags,
                        game_tags: compact_game_tags,
                        team_tags: compact_team_tags,
                        etype: event.etype,
                        tournament: event.tournament,
                        metadata: event.metadata,
                        phase: event.phase,
                    })
                    .encode(),
                ))
                .unwrap();
            }

            let mut f = File::create("./tapes/feed/id_lookup.bin").unwrap();
            f.write_all(
                &rmp_serde::to_vec(&(
                    team_tag_table,
                    player_tag_table,
                    game_tag_table,
                    millis_epoch_table,
                ))
                .unwrap(),
            )
            .unwrap();

            let mut tagf = File::create("./tapes/feed/tag_lookup.bin.zstd").unwrap();
            tagf.write_all(
                &zstd::encode_all(
                    Cursor::new(
                        rmp_serde::to_vec(&(team_tag_idx, player_tag_idx, game_tag_idx)).unwrap(),
                    ),
                    22,
                )
                .unwrap(),
            )
            .unwrap();
            // Close the channel - this is necessary to exit
            // the for-loop in the worker
            drop(snd1);
        });

        for _ in 0..n_workers {
            // Send to sink, receive from source
            let (sendr, recvr) = (snd2.clone(), rcv1.clone());
            let zstd_dict = feed_dict.clone();
            // Spawn workers in separate threads
            s.spawn(move |_| {
                let mut feed_compressor = zstd::block::Compressor::with_dict(zstd_dict);
                // Receive until channel closes
                for (snowflake_id, bytes) in recvr.iter() {
                    let compressed_bytes = feed_compressor.compress(&bytes, 19).unwrap();
                    sendr.send((snowflake_id, compressed_bytes)).unwrap();
                }
            });
        }
        // Close the channel, otherwise sink will never
        // exit the for-loop
        drop(snd2);

        // Sink
        let mut position_index: Vec<(Vec<u8>, u16)> = Vec::new();
        let out_f = File::create("./tapes/feed/feed.riv").unwrap();
        let mut out = BufWriter::new(out_f);
        let mut last_position = out.stream_position().unwrap();

        for (i, (id, bytes)) in rcv2.iter().enumerate() {
            println!("#{}", i);
            let start_pos = out.stream_position().unwrap();
            position_index.push((id, (start_pos - last_position) as u16));
            out.write_all(&bytes).unwrap();
            last_position = start_pos;
        }

        out.flush().unwrap();

        let mut trie_f = File::create("./tapes/feed/feed.fp").unwrap();
        trie_f
            .write_all(
                &zstd::encode_all(
                    Cursor::new(
                        position_index
                            .into_iter()
                            .map(|(b, i)| [b, i.to_be_bytes().to_vec()].concat())
                            .flatten()
                            .collect::<Vec<u8>>(),
                    ),
                    22,
                )
                .unwrap(),
            )
            .unwrap();
    })
    .unwrap();
}
