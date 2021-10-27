use blaseball_vcr::{
    feed::{CompactedFeedEvent, FeedEvent, MetaIndex},
    utils::encode_varint,
};

use clap::clap_app;
use crossbeam::channel::bounded;
use indicatif::{BinaryBytes, MultiProgress, MultiProgressAlignment, ProgressBar, ProgressStyle};
use std::collections::HashMap;
use std::convert::TryInto;
use std::fs::{self, File};
use std::io::{BufRead, BufReader, BufWriter, Read, Seek, Write};
use std::path::Path;

macro_rules! encode_index {
    ($idx:expr) => {
        $idx.into_iter()
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
            .collect::<Vec<u8>>()
    };
}

fn main() {
    let (snd1, rcv1) = bounded(1);
    let (snd2, rcv2) = bounded(1);

    let matches = clap_app!(encode_feed =>
        (version: "1.0")
        (author: "allie signet <allie@sibr.dev>")
        (about: "blaseball.vcr feed encoder")
        (@arg ZSTD_DICT: -d --dict [FILE] "set zstd dictionary to use")
        (@arg COMPRESSION_LEVEL: -l --level [LEVEL] "set compression level")
        (@arg THREADS: -t --threads [THREADS] "set amount of threads to use")
        (@arg EVENT_TYPES: --types [EVENT_TYPES] "set event types to create index for. comma separated")
        (@arg INPUT: <FILE> "input feed dump in NDJSON format")
        (@arg OUT: <FOLDER> "output folder")
    )
    .get_matches();

    let compression_level = matches
        .value_of("COMPRESSION_LEVEL")
        .unwrap_or("19")
        .parse::<i32>()
        .unwrap();
    let n_workers = matches
        .value_of("THREADS")
        .unwrap_or("2")
        .parse::<i32>()
        .unwrap();

    let event_types_to_index: Vec<i16> = matches
        .value_of("EVENT_TYPES")
        .unwrap_or("")
        .split(',')
        .map(|s| s.parse::<i16>().unwrap())
        .collect();

    let base_path = Path::new(matches.value_of("OUT").unwrap());
    let input_path = matches.value_of("INPUT").unwrap();
    let main_path = base_path.join("feed.riv");
    let id_path = base_path.join("feed.fp");
    let lookup_path = base_path.join("id_lookup.bin");
    let tag_indexes_path = base_path.join("tag_indexes.fp");

    let feed_dict: Option<Vec<u8>> = if let Some(dict_path) = matches.value_of("ZSTD_DICT") {
        let mut feed_dict: Vec<u8> = Vec::new();
        let mut dict_f = File::open(dict_path).unwrap();
        dict_f.read_to_end(&mut feed_dict).unwrap();
        Some(feed_dict)
    } else {
        None
    };

    crossbeam::scope(|s| {
        // Producer thread
        s.spawn(|_| {
            let mut indexes: MetaIndex = Default::default();

            let f = File::open(input_path).unwrap();
            let reader = BufReader::new(f);

            for l in reader.lines() {
                let event: FeedEvent = serde_json::from_str(&l.unwrap()).unwrap();
                if event.season == 0 {
                    continue;
                }

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

                // println!("{:#?}",event.metadata);

                snd1.send(CompactedFeedEvent {
                    id: event.id,
                    category: event.category,
                    day: event.day.try_into().unwrap_or(255),
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
                })
                .unwrap();
            }

            let mut f = File::create(lookup_path).unwrap();
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
                let mut feed_compressor = if let Some(dict) = zstd_dict {
                    zstd::block::Compressor::with_dict(dict)
                } else {
                    zstd::block::Compressor::new()
                };
                // Receive until channel closes
                for event in recvr.iter() {
                    let compressed_bytes = feed_compressor
                        .compress(&event.encode(), compression_level)
                        .unwrap();
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
        let mut etype_idx: HashMap<i16, Vec<(u32, (u32, u16))>> = HashMap::new();
        let mut phase_idx: HashMap<(u8, u8), Vec<(i64, (u32, u16))>> = HashMap::new();

        // Sink
        let out_f = File::create(&main_path).unwrap();
        let mut out = BufWriter::new(out_f);

        let id_out_f = File::create(id_path).unwrap();
        let mut id_out = zstd::Encoder::new(id_out_f, 21).unwrap();
        id_out.long_distance_matching(true).unwrap();

        let mut last_position = out.stream_position().unwrap() as u32;

        let bars = MultiProgress::new();
        bars.set_alignment(MultiProgressAlignment::Top);

        let spinner_style = ProgressStyle::default_spinner()
            .template("{spinner:.blue} {msg}")
            .tick_strings(&["-", "-"]);

        let feed_size_spinny = bars.add(ProgressBar::new_spinner());
        feed_size_spinny.set_style(spinner_style);

        feed_size_spinny.set_message("starting feed writer");

        let feed_progress_bar = bars.add(ProgressBar::new(5_110_061)); // static dataset len go brr
        feed_progress_bar.set_style(
            ProgressStyle::default_bar()
                .template("{percent}% [{bar:60.blue/cyan}] {pos:>7}/{len:7}")
                .progress_chars("##-"),
        );
        // bars.set_draw_target(indicatif::ProgressDrawTarget::hidden());

        for (i, (event, bytes)) in rcv2.iter().enumerate() {
            feed_progress_bar.tick();

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

            if event_types_to_index.contains(&event.etype) {
                etype_idx
                    .entry(event.etype)
                    .or_insert_with(Vec::new)
                    .push((event.created.timestamp() as u32, (start_pos, length)));
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
            feed_progress_bar.set_position(i as u64 + 1);

            if i > 0 && i % 1000 == 0 {
                let feed_riv_len = fs::metadata(&main_path).unwrap().len();
                let per_event = feed_riv_len / i as u64;
                feed_size_spinny.set_message(format!(
                    "feed.riv @ {} - est. total @ {} (avg {} bytes per event)",
                    BinaryBytes(feed_riv_len),
                    BinaryBytes(per_event * 5_110_061),
                    per_event
                ));
            }
        }

        out.flush().unwrap();
        id_out.finish().unwrap();

        let idx_f = File::create(tag_indexes_path).unwrap();
        let mut idx_out = zstd::Encoder::new(idx_f, 21).unwrap();
        idx_out.long_distance_matching(true).unwrap();

        let game_tag_idx_bytes = encode_index!(game_tag_idx);
        let player_tag_idx_bytes = encode_index!(player_tag_idx);
        let team_tag_idx_bytes = encode_index!(team_tag_idx);
        let etype_idx_bytes = encode_index!(etype_idx);

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
                    ((k.0 - 10) | (k.1 << 4)).to_be_bytes().to_vec(),
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
            .write_all(&(etype_idx_bytes.len() as u32).to_be_bytes())
            .unwrap();
        idx_out.write_all(&etype_idx_bytes).unwrap();

        idx_out
            .write_all(&(phase_idx_bytes.len() as u32).to_be_bytes())
            .unwrap();
        idx_out.write_all(&phase_idx_bytes).unwrap();

        idx_out.finish().unwrap();

        feed_progress_bar.finish();
        feed_size_spinny.finish();
    })
    .unwrap();
}
