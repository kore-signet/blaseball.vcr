// this is a bit of a mess

use blaseball_vcr::encoder::*;
use blaseball_vcr::*;
use chrono::{DateTime, Utc};
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

async fn paged_get<T: DeserializeOwned>(
    client: &reqwest::Client,
    progress: &mut ProgressBar,
    show_progress: bool,
    url: &str,
    mut parameters: ChroniclerGameParameters,
) -> VCRResult<Vec<T>> {
    let mut results: Vec<T> = Vec::new();

    let mut page = 1;

    loop {
        if show_progress {
            progress.print_info(
                "fetching",
                &format!(
                    "page #{} - {} total entities",
                    page,
                    page * parameters.count.unwrap_or(1000)
                ),
                Color::Red,
                Style::Italic,
            );
        }

        let mut chron_response: ChroniclerV1Response<T> = client
            .get(url)
            .query(&parameters)
            .send()
            .await?
            .json()
            .await?;
        results.append(&mut chron_response.data);

        if let Some(next_page) = chron_response.next_page {
            parameters.next_page = Some(next_page);
            page += 1;
        } else {
            break;
        }
    }

    Ok(results)
}

#[tokio::main]
pub async fn main() -> VCRResult<()> {
    let client = reqwest::Client::new(); // let entity_types = vec!["team"];

    let mut progress_bar = ProgressBar::new(0);

    let games: Vec<Game> = paged_get(
        &client,
        &mut progress_bar,
        false,
        "https://api.sibr.dev/chronicler/v1/games",
        ChroniclerGameParameters {
            next_page: None,
            game: None,
            order: None,
            count: None,
        },
    )
    .await?
    .into_iter()
    .collect();

    let mut entity_lookup_table: HashMap<String, EntityData> = HashMap::new();
    let mut game_date_lookup_table: HashMap<
        GameDate,
        Vec<(String, Option<DateTime<Utc>>, Option<DateTime<Utc>>)>,
    > = HashMap::new();

    println!("| found {} entities", games.len());
    let mut progress_bar = ProgressBar::new(games.len());
    progress_bar.set_action(
        "Loading & encoding entity versions",
        Color::Blue,
        Style::Bold,
    );

    let out_file = File::create(&format!("./tapes/game_updates.riv")).map_err(VCRError::IOError)?;
    let mut out = BufWriter::new(out_file);

    let mut patch_compressor = zstd::block::Compressor::new();

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

        progress_bar.set_action(&id, Color::Green, Style::Bold);

        let entity_start_pos = out.stream_position().map_err(VCRError::IOError)?;

        let mut entity_versions: Vec<(u32, JSONValue)> = paged_get::<GameUpdate>(
            &client,
            &mut progress_bar,
            true,
            "https://api.sibr.dev/chronicler/v1/games/updates",
            ChroniclerGameParameters {
                next_page: None,
                game: Some(id.to_owned()),
                order: Some("asc".to_owned()),
                count: Some(1000),
            },
        )
        .await?
        .into_iter()
        .map(|e| (e.timestamp.timestamp() as u32, e.data))
        .collect();

        entity_versions.sort_by_key(|v| v.0);

        let (patches, path_map) = encode(entity_versions, u32::MAX);

        let mut offsets: Vec<(u32, u64, u64)> = Vec::new(); // timestamp:start_position:end_position

        for (time, patch) in patches {
            let start_pos = out.stream_position().map_err(VCRError::IOError)?;

            let patch_bytes = patch.concat();
            out.write_all(&patch_compressor.compress(&patch_bytes, 22).unwrap())
                .unwrap();

            let end_pos = out.stream_position().map_err(VCRError::IOError)?;
            offsets.push((time, start_pos, end_pos));
        }

        entity_lookup_table.insert(
            id.to_owned(),
            EntityData {
                data_offset: entity_start_pos,
                patches: offsets,
                path_map: path_map,
                checkpoint_every: u32::MAX,
            },
        );

        progress_bar.inc();

        out.flush().map_err(VCRError::IOError)?;
    }

    progress_bar.finalize();

    let mut entity_table_f = File::create(&format!("./tapes/game_updates.header.riv.zstd"))
        .map_err(VCRError::IOError)?;
    entity_table_f
        .write_all(
            &patch_compressor
                .compress(
                    &rmp_serde::to_vec(&entity_lookup_table).map_err(VCRError::MsgPackEncError)?,
                    22,
                )
                .map_err(VCRError::IOError)?,
        )
        .map_err(VCRError::IOError)?;

    let mut date_table_f =
        File::create(&format!("./tapes/game_updates.dates.riv.zstd")).map_err(VCRError::IOError)?;
    date_table_f
        .write_all(
            &patch_compressor
                .compress(
                    &rmp_serde::to_vec(&game_date_lookup_table)
                        .map_err(VCRError::MsgPackEncError)?,
                    22,
                )
                .map_err(VCRError::IOError)?,
        )
        .map_err(VCRError::IOError)?;

    Ok(())
}
