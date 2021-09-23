// this is a bit of a mess

use blaseball_vcr::encoder::*;
use blaseball_vcr::*;
use chrono::{DateTime, Utc};
use crossbeam::channel::bounded;
use progress_bar::color::{Color, Style};
use progress_bar::progress_bar::ProgressBar;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::Value as JSONValue;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufWriter, Read, Seek, Write};

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
) -> VCRResult<Vec<T>> {
    let mut results: Vec<T> = Vec::new();

    loop {
        let mut chron_response: ChroniclerV1Response<T> =
            client.get(url).query(&parameters).send()?.json()?;
        results.append(&mut chron_response.data);

        if let Some(next_page) = chron_response.next_page {
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
        let client = reqwest::blocking::Client::new(); // let entity_types = vec!["team"];
        let mut args: Vec<String> = env::args().skip(1).collect();
        let dict_path = if !args.is_empty() {
            args.remove(0)
        } else {
            "nodict".to_string()
        };
        let compress_level = (if !args.is_empty() {
            args.remove(0)
        } else {
            "22".to_string()
        })
        .parse::<i32>()
        .unwrap();

        println!(
            "Set zstd dictionary to {} and compression level to {}",
            dict_path, compress_level
        );

        let mut dict_f = File::open(dict_path).map_err(VCRError::IOError).unwrap();
        let mut dict: Vec<u8> = Vec::new();
        dict_f
            .read_to_end(&mut dict)
            .map_err(VCRError::IOError)
            .unwrap();

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

        println!("| found {} entities", games.len());
        let mut progress_bar = ProgressBar::new(games.len());
        progress_bar.set_action(
            "Loading & encoding entity versions",
            Color::Blue,
            Style::Bold,
        );

        let n_workers = 8;

        let out_file = File::create(&"./tapes/game_updates.riv".to_string())
            .map_err(VCRError::IOError)
            .unwrap();
        let mut out = BufWriter::new(out_file);

        s.spawn(|_| {
            let mut table_compressor = zstd::block::Compressor::new();
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

            let mut date_table_f = File::create(&"./tapes/game_updates.dates.riv.zstd".to_string())
                .map_err(VCRError::IOError)
                .unwrap();
            date_table_f
                .write_all(
                    &table_compressor
                        .compress(
                            &rmp_serde::to_vec(&game_date_lookup_table)
                                .map_err(VCRError::MsgPackEncError)
                                .unwrap(),
                            22,
                        )
                        .map_err(VCRError::IOError)
                        .unwrap(),
                )
                .map_err(VCRError::IOError)
                .unwrap();

            drop(snd1);
        });

        for _ in 0..n_workers {
            let (sendr, recvr) = (snd2.clone(), rcv1.clone());
            let zstd_dict = dict.clone();
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
                    let (patches, path_map, base) = encode(entity_versions, u32::MAX);
                    sendr
                        .send((
                            id,
                            patches
                                .into_iter()
                                .map(|(t, v)| {
                                    (t, compressor.compress(&v.concat(), compress_level).unwrap())
                                })
                                .collect::<Vec<(u32, Vec<u8>)>>(),
                            path_map,
                            base,
                        ))
                        .unwrap();
                }
            });
        }

        drop(snd2);

        let mut entity_lookup_table: HashMap<String, EntityData> = HashMap::new();

        for (id, patches, path_map, base) in rcv2.iter() {
            let entity_start_pos = out.stream_position().map_err(VCRError::IOError).unwrap();
            let mut offsets: Vec<(u32, u64, u64)> = Vec::new(); // timestamp:start_position:end_position
            progress_bar.set_action(&id, Color::Green, Style::Bold);

            for (time, patch) in patches {
                let start_pos = out.stream_position().map_err(VCRError::IOError).unwrap();

                out.write_all(&patch).unwrap();

                let end_pos = out.stream_position().map_err(VCRError::IOError).unwrap();
                offsets.push((time, start_pos, end_pos));
            }

            entity_lookup_table.insert(
                id.to_owned(),
                EntityData {
                    data_offset: entity_start_pos,
                    patches: offsets,
                    path_map,
                    checkpoint_every: u32::MAX,
                    base,
                },
            );

            progress_bar.inc();

            out.flush().map_err(VCRError::IOError).unwrap();
        }

        progress_bar.finalize();

        let entity_table_f = File::create(&"./tapes/game_updates.header.riv.zstd".to_string())
            .map_err(VCRError::IOError)
            .unwrap();
        let mut entity_table_compressor = zstd::Encoder::new(entity_table_f, 21).unwrap();
        entity_table_compressor
            .long_distance_matching(true)
            .unwrap();
        entity_table_compressor
            .write_all(
                &rmp_serde::to_vec(&entity_lookup_table)
                    .map_err(VCRError::MsgPackEncError)
                    .unwrap(),
            )
            .map_err(VCRError::IOError)
            .unwrap();
        entity_table_compressor.finish().unwrap();
    })
    .unwrap();

    Ok(())
}
