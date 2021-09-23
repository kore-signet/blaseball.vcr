use blaseball_vcr::{
    feed::{CompactedFeedEvent, FeedEvent, MetaIndex},
    utils::encode_varint,
};

use crossbeam::channel::bounded;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Read, Seek, Write};


fn main() {
    let (snd1, rcv1) = bounded(1);
    let (snd2, rcv2) = bounded(1);
    let n_workers = 6;

    let mut feed_dict: Vec<u8> = Vec::new();
    let mut dict_f = File::open("zstd-dictionaries/feed.dict").unwrap();
    dict_f.read_to_end(&mut feed_dict).unwrap();

    crossbeam::scope(|s| {
        // Producer thread
        s.spawn(|_| {
            let mut indexes: MetaIndex = Default::default();

            let f = File::open("feed.json").unwrap();
            let reader = BufReader::new(f);

            for l in reader.lines() {
                let event: FeedEvent = serde_json::from_str(&l.unwrap()).unwrap();

                let compact_player_tags: Vec<u16> = event
                    .player_tags
                    .clone()
                    .unwrap_or_default()
                    .iter()
                    .map(|id| {
                        if let Some(n) = indexes.reverse_player_tags.get(id) {
                            *n
                        } else {
                            let n = indexes.reverse_player_tags.len() as u16;
                            indexes.reverse_player_tags.insert(*id, n);
                            indexes.player_tags.insert(n, *id);
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
                        if let Some(n) = indexes.reverse_game_tags.get(id) {
                            *n
                        } else {
                            let n = indexes.reverse_game_tags.len() as u16;
                            indexes.reverse_game_tags.insert(*id, n);
                            indexes.game_tags.insert(n, *id);
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
                        if let Some(n) = indexes.reverse_team_tags.get(id) {
                            *n
                        } else {
                            let n = indexes.reverse_team_tags.len() as u8;
                            indexes.reverse_team_tags.insert(*id, n);
                            indexes.team_tags.insert(n, *id);
                            n
                        }
                    })
                    .collect();

                snd1.send(
                    CompactedFeedEvent {
                        id: event.id,
                        category: event.category,
                        day: event.day,
                        created: event.created,
                        description: event.description,
                        player_tags: compact_player_tags,
                        game_tags: compact_game_tags,
                        team_tags: compact_team_tags,
                        etype: event.etype,
                        tournament: event.tournament,
                        metadata: event.metadata,
                        phase: event.phase,
                        season: event.season,
                    },
                )
                .unwrap();
            }

            let mut f = File::create("./tapes/feed/id_lookup.bin").unwrap();
            f.write_all(&rmp_serde::to_vec(&indexes).unwrap()).unwrap();

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
                for event in recvr.iter() {
                    let compressed_bytes = feed_compressor.compress(&event.encode(), 1).unwrap();
                    sendr.send((event, compressed_bytes)).unwrap();
                }
            });
        }
        // Close the channel, otherwise sink will never
        // exit the for-loop
        drop(snd2);

        let mut game_tag_idx: HashMap<u16, Vec<(u32, (u32, u16))>> = HashMap::new();
        let mut player_tag_idx: HashMap<u16, Vec<(u32, (u32, u16))>> = HashMap::new();
        let mut team_tag_idx: HashMap<u8, Vec<(u32, (u32, u16))>> = HashMap::new();
        let mut phase_idx: HashMap<(i8, u8), Vec<(i64, (u32, u16))>> = HashMap::new();

        // Sink
        let out_f = File::create("./tapes/feed/feed.riv").unwrap();
        let mut out = BufWriter::new(out_f);

        let id_out_f = File::create("./tapes/feed/feed.fp").unwrap();
        let mut id_out = zstd::Encoder::new(id_out_f, 21).unwrap();
        id_out.long_distance_matching(true).unwrap();

        let mut last_position = out.stream_position().unwrap() as u32;

        for (i, (event, bytes)) in rcv2.iter().enumerate() {
            println!("#{}", i);
            let start_pos = out.stream_position().unwrap() as u32;
            out.write_all(&bytes).unwrap();
            let end_pos = out.stream_position().unwrap() as u32;

            let length = (end_pos - start_pos) as u16;

            if event.season >= 11 && [3, 5, 13].contains(&event.phase) {
                phase_idx
                    .entry((event.season, event.phase))
                    .or_insert_with(Vec::new)
                    .push((event.created.timestamp_millis() as i64, (start_pos, length)));
            }

            for game_tag in event.game_tags {
                game_tag_idx
                    .entry(game_tag)
                    .or_insert_with(Vec::new)
                    .push((event.created.timestamp() as u32, (start_pos, length)));
            }

            for player_tag in event.player_tags {
                player_tag_idx
                    .entry(player_tag)
                    .or_insert_with(Vec::new)
                    .push((event.created.timestamp() as u32, (start_pos, length)));
            }

            for team_tag in event.team_tags {
                team_tag_idx
                    .entry(team_tag)
                    .or_insert_with(Vec::new)
                    .push((event.created.timestamp() as u32, (start_pos, length)));
            }

            id_out
                .write_all(&((start_pos - last_position) as u16).to_be_bytes())
                .unwrap();
            id_out
                .write_all(&(event.created.timestamp() as u32).to_be_bytes())
                .unwrap();

            last_position = start_pos;
        }

        out.flush().unwrap();
        id_out.finish().unwrap();

        let idx_f = File::create("./tapes/feed/tag_indexes.fp").unwrap();
        let mut idx_out = zstd::Encoder::new(idx_f, 21).unwrap();
        idx_out.long_distance_matching(true).unwrap();

        let game_tag_idx_bytes = game_tag_idx
            .into_iter()
            .map(|(k, v)| {
                let v_bytes = v
                    .into_iter()
                    .map(|(time, (offset, length))| {
                        vec![
                            time.to_be_bytes().to_vec(),
                            offset.to_be_bytes().to_vec(),
                            encode_varint(length),
                        ]
                        .concat()
                    })
                    .flatten()
                    .collect::<Vec<u8>>();
                vec![
                    k.to_be_bytes().to_vec(),
                    (v_bytes.len() as u32).to_be_bytes().to_vec(),
                    v_bytes,
                ]
                .concat()
            })
            .flatten()
            .collect::<Vec<u8>>();

        let player_tag_idx_bytes = player_tag_idx
            .into_iter()
            .map(|(k, v)| {
                let v_bytes = v
                    .into_iter()
                    .map(|(time, (offset, length))| {
                        vec![
                            time.to_be_bytes().to_vec(),
                            offset.to_be_bytes().to_vec(),
                            encode_varint(length),
                        ]
                        .concat()
                    })
                    .flatten()
                    .collect::<Vec<u8>>();
                vec![
                    k.to_be_bytes().to_vec(),
                    (v_bytes.len() as u32).to_be_bytes().to_vec(),
                    v_bytes,
                ]
                .concat()
            })
            .flatten()
            .collect::<Vec<u8>>();

        let team_tag_idx_bytes = team_tag_idx
            .into_iter()
            .map(|(k, v)| {
                let v_bytes = v
                    .into_iter()
                    .map(|(time, (offset, length))| {
                        vec![
                            time.to_be_bytes().to_vec(),
                            offset.to_be_bytes().to_vec(),
                            encode_varint(length),
                        ]
                        .concat()
                    })
                    .flatten()
                    .collect::<Vec<u8>>();
                vec![
                    k.to_be_bytes().to_vec(),
                    (v_bytes.len() as u32).to_be_bytes().to_vec(),
                    v_bytes,
                ]
                .concat()
            })
            .flatten()
            .collect::<Vec<u8>>();

        let phase_idx_bytes = phase_idx
            .into_iter()
            .map(|(k, v)| {
                let v_bytes = v
                    .into_iter()
                    .map(|(time, (offset, length))| {
                        vec![
                            time.to_be_bytes().to_vec(),
                            offset.to_be_bytes().to_vec(),
                            encode_varint(length),
                        ]
                        .concat()
                    })
                    .flatten()
                    .collect::<Vec<u8>>();
                vec![
                    k.0.to_be_bytes().to_vec(),
                    k.1.to_be_bytes().to_vec(),
                    (v_bytes.len() as u32).to_be_bytes().to_vec(),
                    v_bytes,
                ]
                .concat()
            })
            .flatten()
            .collect::<Vec<u8>>();

        idx_out
            .write_all(&(game_tag_idx_bytes.len() as u32).to_be_bytes())
            .unwrap();
        idx_out.write_all(&game_tag_idx_bytes).unwrap();

        idx_out
            .write_all(&(player_tag_idx_bytes.len() as u32).to_be_bytes())
            .unwrap();
        idx_out.write_all(&player_tag_idx_bytes).unwrap();

        idx_out
            .write_all(&(team_tag_idx_bytes.len() as u32).to_be_bytes())
            .unwrap();
        idx_out.write_all(&team_tag_idx_bytes).unwrap();

        idx_out
            .write_all(&(phase_idx_bytes.len() as u32).to_be_bytes())
            .unwrap();
        idx_out.write_all(&phase_idx_bytes).unwrap();

        idx_out.finish().unwrap();
    })
    .unwrap();
}
