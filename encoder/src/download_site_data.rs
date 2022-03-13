use blaseball_vcr::{
    site::{chron, chron::*, *},
    ChroniclerV1Response, VCRError,
};

use reqwest::blocking;
use std::env;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

use chrono::{DateTime, Utc};
use indicatif::{ProgressBar, ProgressStyle};

use std::io::{Read, Seek, Write};

use std::collections::HashMap;

use serde::Deserialize;

use sha2::{Digest, Sha224};

use bsdiff::diff::diff;

#[derive(Deserialize)]
pub struct Replace {
    replace: String,
    with: String,
}

#[derive(Deserialize)]
pub struct AssetConfig {
    patches: Vec<Replace>,
}

pub fn encode_resource<W: Write + Seek>(
    steps: Vec<FileStep>,
    replaces: &[Replace],
    out: &mut W,
) -> anyhow::Result<EncodedResource> {
    let client = reqwest::blocking::Client::new();

    let mut basis: Vec<u8> = Vec::new();
    let mut basis_response = client
        .get(&format!(
            "https://api.sibr.dev/chronicler/v1{}",
            &steps[0].download_url
        ))
        .send()?;
    basis_response.copy_to(&mut basis)?;

    let mut last: Vec<u8> = basis.clone();

    let mut deltas: Vec<PatchData> = Vec::new();
    let mut paths: Vec<(DateTime<Utc>, String, u16)> = Vec::new();

    let total_len = steps.len();

    let progress_bar = ProgressBar::new(total_len as u64);
    progress_bar.set_style(
        ProgressStyle::default_bar()
            .template("{pos}/{len:4} {bar:70.green/white} {percent:.bold}%")
            .unwrap(),
    );

    let mut compressor = zstd::block::Compressor::new();
    let mut hasher = Sha224::new();

    for step in progress_bar.wrap_iter(steps.into_iter()) {
        let next: Vec<u8> = {
            let mut basis = client
                .get(&format!(
                    "https://api.sibr.dev/chronicler/v1{}",
                    &step.download_url
                ))
                .send()?
                .text()?;
            for r in replaces {
                basis = basis.replace(&r.replace, &r.with);
            }

            basis.into_bytes()
        };

        hasher.update(&next);
        let hash = hasher.finalize_reset();

        // let delta = xdelta3::encode(&next, &last, 9i32 << 20i32).unwrap();
        let mut delta: Vec<u8> = Vec::new();
        diff(&last, &next, &mut delta).unwrap();
        let uncompressed_patch_length = delta.len() as u32;
        delta = compressor.compress(&delta, 11).unwrap();

        last = next;

        let offset_start = out.stream_position()?;
        out.write_all(&delta)?;
        let offset_end = out.stream_position()?;

        deltas.push(PatchData {
            offset: offset_start as u32,
            compressed_patch_length: (offset_end - offset_start) as u32,
            uncompressed_patch_length,
            original_length: last.len() as u32,
            hash: format!("{:x}", hash),
        });

        let delta_idx = (deltas.len() - 1) as u16;

        for path in step.paths {
            paths.push((path.0, path.1, delta_idx));
        }
    }

    progress_bar.finish_with_message("done!");

    Ok(EncodedResource {
        paths,
        deltas,
        basis,
    })
}

// usage: download_site_data <out folder> <optional toml file with replaces>
fn main() -> anyhow::Result<()> {
    let chron_res: ChroniclerV1Response<chron::SiteUpdate> =
        blocking::get("https://api.sibr.dev/chronicler/v1/site/updates")?.json()?;
    let all_steps = chron::updates_to_steps(chron_res.data);
    let args: Vec<String> = env::args().collect();

    let replacer: HashMap<String, AssetConfig> = {
        if let Some(path) = args.get(2) {
            let mut cfile = File::open(path).map_err(VCRError::IOError)?;
            let mut cfg = String::new();
            cfile.read_to_string(&mut cfg).map_err(VCRError::IOError)?;
            toml::from_str(&cfg).unwrap()
        } else {
            HashMap::new()
        }
    };

    for (name, steps) in all_steps {
        println!("Recording asset {}", name);
        let main_path = Path::new(&args[1]).join(&format!("{}.riv", name));
        let header_path = Path::new(&args[1]).join(&format!("{}.header.riv", name));

        let main_f = File::create(main_path).map_err(VCRError::IOError)?;
        let mut main_out = BufWriter::new(main_f);

        let header = encode_resource(
            steps,
            replacer
                .get(&name)
                .map(|c| &c.patches)
                .unwrap_or(&Vec::new()),
            &mut main_out,
        )?;

        let mut header_f = File::create(header_path).map_err(VCRError::IOError)?;
        rmp_serde::encode::write(&mut header_f, &header).map_err(VCRError::MsgPackEncError)?;
    }

    Ok(())
}
