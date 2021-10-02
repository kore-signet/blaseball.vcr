use blaseball_vcr::encoder::*;
use blaseball_vcr::*;
use indicatif::{ProgressBar, ProgressStyle};
use serde::Serialize;
use serde_json::Value as JSONValue;

use std::env;
use std::fs::File;
use std::io::Write;

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
    url: &str,
    mut parameters: ChroniclerParameters,
) -> anyhow::Result<Vec<ChroniclerEntity<JSONValue>>> {
    let mut results: Vec<ChroniclerEntity<JSONValue>> = Vec::new();

    let mut page = 1;
    let spinny = ProgressBar::new_spinner();
    spinny.enable_steady_tick(120);
    spinny.set_style(ProgressStyle::default_spinner().template("{spinner:.blue} {msg}"));
    loop {
        spinny.set_message(format!("downloading entities - page {}", page));
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
    spinny.finish_and_clear();

    Ok(results)
}

#[tokio::main]
pub async fn main() -> anyhow::Result<()> {
    let client = reqwest::Client::new();
    let mut entity_types: Vec<String> = env::args().skip(1).collect();
    let checkpoint_every = entity_types.remove(0).parse::<u16>().unwrap_or(u16::MAX);

    for etype in entity_types {
        println!("-> Fetching list of entities of type {}", etype);
        let entity_ids: Vec<String> = paged_get(
            &client,
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

        let bar_style = ProgressStyle::default_bar()
            .template("{msg:.bold} - {pos}/{len} {wide_bar:40.green/white}");

        let entity_id_bar = ProgressBar::new(entity_ids.len() as u64);
        entity_id_bar.set_style(bar_style.clone());
        entity_id_bar.set_message("encoding entities");

        let mut out_file = File::create(&format!("./zstd-dictionaries/{}.dict", &etype))
            .map_err(VCRError::IOError)?;

        let mut samples: Vec<u8> = Vec::new();
        let mut sample_sizes: Vec<usize> = Vec::new();

        for id in entity_id_bar.wrap_iter(entity_ids.into_iter()) {
            entity_id_bar.tick();
            entity_id_bar.set_message(format!("encoding {}", id));

            let mut entity_versions: Vec<(u32, JSONValue)> = paged_get(
                &client,
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

            let (patches, _path_map, _baseval) = encode(entity_versions, checkpoint_every);

            for (_time, patch) in patches {
                let mut patch_bytes = patch.concat();
                sample_sizes.push(patch_bytes.len());
                samples.append(&mut patch_bytes);
            }
        }

        entity_id_bar.set_message("training dictionary");

        out_file
            .write_all(&zstd::dict::from_continuous(&samples, &sample_sizes, 112640).unwrap())
            .unwrap();

        entity_id_bar.finish_with_message("done!");
    }

    Ok(())
}
