// this is a bit of a mess

use blaseball_vcr::encoder::*;
use blaseball_vcr::*;
use chrono::{DateTime, Utc};
use clap::clap_app;
use crossbeam::channel::bounded;
use indicatif::{
    MultiProgress, MultiProgressAlignment, ProgressBar, ProgressDrawTarget, ProgressStyle,
};
use integer_encoding::VarIntWriter;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::Value as JSONValue;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufWriter, Read, Seek, Write};
use std::path::Path;
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct GameUpdate {
    game_id: String,
    timestamp: DateTime<Utc>,
    hash: String,
    data: JSONValue,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Game {
    game_id: String,
    start_time: Option<DateTime<Utc>>,
    end_time: Option<DateTime<Utc>>,
    data: GameDate,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct ChroniclerGameParameters {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "page")]
    next_page: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    game: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    order: Option<String>,
    count: Option<u32>,
}

fn paged_get<T: DeserializeOwned>(
    client: &reqwest::blocking::Client,
    url: &str,
    mut parameters: ChroniclerGameParameters,
) -> anyhow::Result<Vec<T>> {
    let mut results: Vec<T> = Vec::new();

    loop {
        let mut chron_response: ChroniclerV1Response<T> =
            client.get(url).query(&parameters).send()?.json()?;
        let res_len = chron_response.data.len() as u32;
        results.append(&mut chron_response.data);

        if res_len < parameters.count.unwrap_or(0) {
            break;
        } else if let Some(next_page) = chron_response.next_page {
            parameters.next_page = Some(next_page);
        } else {
            break;
        }
    }

    Ok(results)
}

pub fn main() -> VCRResult<()> {
    let (snd1, rcv1) = bounded(1);
    let (snd2, rcv2) = bounded(1);

    crossbeam::scope(|s| {
        let matches = clap_app!(build_games =>
            (version: "1.0")
            (author: "allie signet <allie@sibr.dev>")
            (about: "blaseball.vcr game update encoder")
            (@arg ZSTD_DICT: -d --dict [FILE] "set zstd dictionary to use")
            (@arg COMPRESSION_LEVEL: -l --level [LEVEL] "set compression level")
            (@arg THREADS: -t --threads [THREADS] "set amount of threads to use")
            (@arg WHEE: --whee "show extra progress bars for patch compression")
            (@arg OUT: <FOLDER> "set output folder")
        )
        .get_matches();

        let dict_path = matches.value_of("ZSTD_DICT").unwrap_or("nodict");
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
        let base_path = Path::new(matches.value_of("OUT").unwrap());
        let main_path = base_path.join("game_updates.riv");
        let date_table_path = base_path.join("game_updates.dates.riv.zstd");
        let header_path = base_path.join("game_updates.header.riv.zstd");

        println!(
            "Set zstd dictionary to {} and compression level to {}",
            dict_path, compression_level
        );

        let client = reqwest::blocking::Client::new();

        let mut dict_f = File::open(dict_path).unwrap();
        let mut dict: Vec<u8> = Vec::new();
        dict_f.read_to_end(&mut dict).unwrap();

        let bars = MultiProgress::new();
        bars.set_alignment(MultiProgressAlignment::Top);
        bars.set_draw_target(ProgressDrawTarget::stderr_with_hz(10));

        let spinny = bars.add(ProgressBar::new_spinner());
        spinny.enable_steady_tick(120);
        spinny.set_style(ProgressStyle::default_spinner().template("{spinner:.blue} {msg}"));
        spinny.set_message("fetching game list..");

        let games: Vec<Game> = paged_get::<Game>(
            &client,
            "https://api.sibr.dev/chronicler/v1/games",
            ChroniclerGameParameters {
                next_page: None,
                game: None,
                order: None,
                count: None,
            },
        )
        .unwrap()
        .into_iter()
        .collect();

        spinny.finish_and_clear();
        bars.remove(&spinny);

        println!("| found {} entities", games.len());
        let progress_bar = bars.add(ProgressBar::new(games.len() as u64));
        progress_bar.set_style(
            ProgressStyle::default_bar()
                .template("{msg:.bold} {pos:>7}/{len:7} \n{percent:.bold}% {bar:70.green/white}"),
        );
        progress_bar.tick();

        let out_file = File::create(main_path).unwrap();
        let mut out = BufWriter::new(out_file);

        s.spawn(|_| {
            let mut game_date_lookup_table: HashMap<
                GameDate,
                Vec<(String, Option<DateTime<Utc>>, Option<DateTime<Utc>>)>,
            > = HashMap::new();

            for game in games {
                let game_date = game.data;
                let id = game.game_id;

                if let Some(date_idx) = game_date_lookup_table.get_mut(&game_date) {
                    (*date_idx).push((id.to_owned(), game.start_time, game.end_time));
                } else {
                    game_date_lookup_table.insert(
                        game_date,
                        vec![(id.to_owned(), game.start_time, game.end_time)],
                    );
                }

                snd1.send(id).unwrap();
            }

            let date_table_f = File::create(date_table_path).unwrap();
            let mut date_table_writer = zstd::Encoder::new(date_table_f, 21).unwrap();
            date_table_writer
                .write_all(&rmp_serde::to_vec(&game_date_lookup_table).unwrap())
                .unwrap();
            date_table_writer.finish().unwrap();

            drop(snd1);
        });

        for threadn in 0..n_workers {
            let (sendr, recvr) = (snd2.clone(), rcv1.clone());
            let zstd_dict = dict.clone();
            let pb = bars.add(ProgressBar::new(0));

            pb.set_style(
                ProgressStyle::default_bar()
                    .template("{msg:.bold} [{bar:40.blue/cyan}] {pos:>7}/{len:7} ")
                    .progress_chars("##-"),
            );

            pb.set_message(format!("[THREAD {} - compressing]", threadn + 1));

            if !matches.is_present("WHEE") {
                pb.set_draw_target(ProgressDrawTarget::hidden());
            }

            s.spawn(move |_| {
                let mut compressor = zstd::block::Compressor::with_dict(zstd_dict);
                let loc_client = reqwest::blocking::Client::new();
                for id in recvr.iter() {
                    let mut entity_versions: Vec<(u32, JSONValue)> = paged_get::<GameUpdate>(
                        &loc_client,
                        "https://api.sibr.dev/chronicler/v1/games/updates",
                        ChroniclerGameParameters {
                            next_page: None,
                            game: Some(id.to_owned()),
                            order: Some("asc".to_owned()),
                            count: Some(1000),
                        },
                    )
                    .unwrap()
                    .into_iter()
                    .map(|e| (e.timestamp.timestamp() as u32, e.data))
                    .collect();

                    entity_versions.sort_by_key(|v| v.0);
                    let (patches, path_map, base) = encode(entity_versions, u16::MAX);
                    pb.set_length(patches.len() as u64);
                    sendr
                        .send((
                            id,
                            patches
                                .into_iter()
                                .map(|(t, v)| {
                                    pb.inc(1);
                                    (
                                        t,
                                        compressor
                                            .compress(&v.concat(), compression_level)
                                            .unwrap(),
                                    )
                                })
                                .collect::<Vec<(u32, Vec<u8>)>>(),
                            path_map,
                            base,
                        ))
                        .unwrap();
                    pb.set_position(0);
                }
            });
        }

        drop(snd2);

        let entity_table_f = File::create(header_path).unwrap();
        let mut entity_table_writer = zstd::Encoder::new(entity_table_f, 21).unwrap();
        entity_table_writer.long_distance_matching(true).unwrap();

        for (id, patches, path_map, base) in progress_bar.wrap_iter(rcv2.iter()) {
            progress_bar.set_message(format!("writing game {}", id));

            let mut last_position = out.stream_position().unwrap() as u32;
            let mut header_encoder =
                HeaderEncoder::new(base, u16::MAX, path_map, last_position, Vec::new()).unwrap();

            for (time, patch) in patches {
                let start_pos = out.stream_position().map_err(VCRError::IOError).unwrap() as u32;
                header_encoder
                    .write_patch(time, start_pos - last_position)
                    .unwrap();

                out.write_all(&patch).unwrap();
                last_position = start_pos;
            }

            let header = header_encoder.release();
            entity_table_writer
                .write_varint(header.len() as u32)
                .unwrap();
            entity_table_writer
                .write_varint(out.stream_position().unwrap() as u32)
                .unwrap();
            entity_table_writer
                .write_all(Uuid::parse_str(&id).unwrap().as_bytes())
                .unwrap();
            entity_table_writer.write_all(&header).unwrap();

            out.flush().map_err(VCRError::IOError).unwrap();
        }

        progress_bar.finish_with_message("done!");

        entity_table_writer.finish().unwrap();
    })
    .unwrap();

    Ok(())
}
