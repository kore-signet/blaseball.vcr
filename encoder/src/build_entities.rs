use blaseball_vcr::encoder::*;
use blaseball_vcr::*;
use clap::clap_app;
use integer_encoding::VarIntWriter;
use progress_bar::color::{Color, Style};
use progress_bar::progress_bar::ProgressBar;
use serde::Serialize;
use serde_json::Value as JSONValue;
use std::fs::File;
use std::io::{BufWriter, Read, Seek, Write};
use std::path::Path;
use uuid::Uuid;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct ChroniclerParameters {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "page")]
    next_page: Option<String>,
    #[serde(rename = "type")]
    entity_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    order: Option<String>,
    count: u32,
}

async fn paged_get(
    client: &reqwest::Client,
    progress: &mut ProgressBar,
    show_progress: bool,
    url: &str,
    mut parameters: ChroniclerParameters,
) -> VCRResult<Vec<ChroniclerEntity<JSONValue>>> {
    let mut results: Vec<ChroniclerEntity<JSONValue>> = Vec::new();

    let mut page = 1;

    loop {
        if show_progress {
            progress.print_info(
                "fetching",
                &format!(
                    "page #{} - {} total entities",
                    page,
                    page * parameters.count
                ),
                Color::Red,
                Style::Italic,
            );
        }

        let mut chron_response: ChroniclerResponse<ChroniclerEntity<JSONValue>> = client
            .get(url)
            .query(&parameters)
            .send()
            .await?
            .json()
            .await?;
        results.append(&mut chron_response.items);

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
    let client = reqwest::Client::new();

    let matches = clap_app!(build_entities =>
        (version: "1.0")
        (author: "allie signet <allie@sibr.dev>")
        (about: "blaseball.vcr general purpose encoder")
        (@arg ZSTD_DICT: -d --dict [FILE] "set zstd dictionary to use")
        (@arg COMPRESSION_LEVEL: -l --level [LEVEL] "set compression level")
        (@arg CHECKPOINTS: -c --checkpoints [CHECKPOINTS] "make a checkpoint every n entities")
        (@arg OUTPUT_FOLDER: -o --output [FOLDER] "set output folder for resulting tapes")
        (@arg ENTITIES: <TYPE> ... "entity types to encode")
    )
    .get_matches();

    let compression_level = matches
        .value_of("COMPRESSION_LEVEL")
        .map(|v| v.parse::<i32>().unwrap())
        .unwrap_or(19);

    let checkpoint_every = matches
        .value_of("CHECKPOINTS")
        .map(|v| v.parse::<u16>().unwrap())
        .unwrap_or(u16::MAX);

    let base_path = Path::new(matches.value_of("OUTPUT_FOLDER").unwrap_or("./tapes"));
    let entity_types: Vec<&str> = matches.values_of("ENTITIES").unwrap().collect();

    let mut patch_compressor = if let Some(dict_path) = matches.value_of("ZSTD_DICT") {
        let mut dict_f = File::open(&dict_path).map_err(VCRError::IOError)?;
        let mut dict: Vec<u8> = Vec::new();
        dict_f.read_to_end(&mut dict).map_err(VCRError::IOError)?;
        zstd::block::Compressor::with_dict(dict)
    } else {
        zstd::block::Compressor::new()
    };

    for etype in entity_types {
        let mut progress_bar = ProgressBar::new(0);
        println!("-> Fetching list of entities of type {}", etype);
        let entity_ids: Vec<String> = paged_get(
            &client,
            &mut progress_bar,
            false,
            "https://api.sibr.dev/chronicler/v2/entities",
            ChroniclerParameters {
                next_page: None,
                entity_type: etype.to_owned(),
                id: None,
                order: None,
                count: 1000,
            },
        )
        .await?
        .into_iter()
        .map(|e| e.entity_id)
        .collect();

        println!("| found {} entities", entity_ids.len());
        let mut progress_bar = ProgressBar::new(entity_ids.len());
        progress_bar.set_action(
            "Loading & encoding entity versions",
            Color::Blue,
            Style::Bold,
        );

        let out_file =
            File::create(base_path.join(&format!("{}.riv", etype))).map_err(VCRError::IOError)?;
        let mut out = BufWriter::new(out_file);

        let entity_table_f = File::create(base_path.join(&format!("{}.header.riv.zstd", etype)))
            .map_err(VCRError::IOError)?;
        let mut entity_table_writer = zstd::Encoder::new(entity_table_f, 21).unwrap();

        for id in entity_ids {
            progress_bar.set_action(&id, Color::Green, Style::Bold);

            progress_bar.print_info(
                "downloading",
                &format!("entity {} of type {}", id, etype),
                Color::Green,
                Style::Italic,
            );

            let mut entity_versions: Vec<(u32, JSONValue)> = paged_get(
                &client,
                &mut progress_bar,
                true,
                "https://api.sibr.dev/chronicler/v2/versions",
                ChroniclerParameters {
                    next_page: None,
                    entity_type: etype.to_owned(),
                    id: Some(id.clone()),
                    order: Some("asc".to_owned()),
                    count: 1000,
                },
            )
            .await?
            .into_iter()
            .map(|e| (e.valid_from.timestamp() as u32, e.data))
            .collect();

            entity_versions.sort_by_key(|v| v.0);

            let (patches, path_map, baseval) = encode(entity_versions, checkpoint_every);

            let mut last_position = out.stream_position().unwrap() as u32;
            let mut header_encoder = HeaderEncoder::new(
                baseval,
                checkpoint_every,
                path_map,
                last_position,
                Vec::new(),
            )
            .unwrap();

            for (time, patch) in patches {
                let start_pos = out.stream_position().map_err(VCRError::IOError)? as u32;
                header_encoder
                    .write_patch(time, start_pos - last_position)
                    .unwrap();

                let patch_bytes = patch.concat();
                out.write_all(
                    &patch_compressor
                        .compress(&patch_bytes, compression_level)
                        .unwrap(),
                )
                .unwrap();

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

            progress_bar.inc();

            out.flush().map_err(VCRError::IOError)?;
        }

        progress_bar.finalize();
        entity_table_writer.finish().unwrap();

        out.get_mut().sync_all().map_err(VCRError::IOError)?;
    }

    Ok(())
}
