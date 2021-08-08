use blaseball_vcr::encoder::*;
use blaseball_vcr::*;
use progress_bar::color::{Color, Style};
use progress_bar::progress_bar::ProgressBar;
use serde::Serialize;
use serde_json::Value as JSONValue;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufWriter, Read, Seek, Write};

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
) -> VCRResult<Vec<ChroniclerEntity>> {
    let mut results: Vec<ChroniclerEntity> = Vec::new();

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

        let mut chron_response: ChroniclerResponse<ChroniclerEntity> = client
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
    let mut entity_types: Vec<String> = env::args().skip(1).collect();
    let checkpoint_every = entity_types.remove(0).parse::<u32>().unwrap_or(u32::MAX);

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

        let mut out_file = File::create(&format!("./zstd-dictionaries/{}.dict", &etype))
            .map_err(VCRError::IOError)?;

        let mut samples: Vec<u8> = Vec::new();
        let mut sample_sizes: Vec<usize> = Vec::new();

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

            for (time, patch) in patches {
                let mut patch_bytes = patch.concat();
                sample_sizes.push(patch_bytes.len());
                samples.append(&mut patch_bytes);
            }

            progress_bar.inc();
        }

        progress_bar.set_action("Training dictionary", Color::Blue, Style::Bold);

        out_file
            .write_all(&zstd::dict::from_continuous(&samples, &sample_sizes, 112640).unwrap())
            .unwrap();

        progress_bar.finalize();
    }

    Ok(())
}
