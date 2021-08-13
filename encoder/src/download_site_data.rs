use blaseball_vcr::{
    site::{chron, chron::*, *},
    ChroniclerV1Response, VCRError, VCRResult,
};

use reqwest::blocking;
use std::env;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

use chrono::{DateTime, Utc};
use progress_bar::color::{Color, Style};
use progress_bar::progress_bar::ProgressBar;
use reqwest;
use std::io::{Read, Seek, Write};

use std::collections::HashMap;

use serde::Deserialize;

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
    replaces: &Vec<Replace>,
    out: &mut W,
) -> VCRResult<EncodedResource> {
    let client = reqwest::blocking::Client::new();

    let mut basis: Vec<u8> = Vec::new();
    let mut basis_response = client
        .get(&format!(
            "https://api.sibr.dev/chronicler/v1{}",
            &steps[0].download_url
        ))
        .send()
        .map_err(VCRError::ReqwestError)?;
    basis_response
        .copy_to(&mut basis)
        .map_err(VCRError::ReqwestError)?;

    let mut last: Vec<u8> = basis.iter().copied().collect();

    let mut deltas: Vec<(u64, u64, String)> = Vec::new();
    let mut paths: Vec<(DateTime<Utc>, String, u16)> = Vec::new();

    let total_len = steps.len();

    let mut progress_bar = ProgressBar::new(total_len);

    for step in steps {
        progress_bar.set_action("downloading", Color::Green, Style::Bold);

        let next: Vec<u8> = {
            let mut basis = client
                .get(&format!(
                    "https://api.sibr.dev/chronicler/v1{}",
                    &step.download_url
                ))
                .send()
                .map_err(VCRError::ReqwestError)?
                .text()
                .map_err(VCRError::ReqwestError)?;
            for r in replaces {
                basis = basis.replace(&r.replace, &r.with);
            }

            basis.as_bytes().to_vec()
        };

        progress_bar.set_action("creating delta", Color::Blue, Style::Bold);

        let delta = xdelta3::encode(&next, &last, 9i32 << 20i32).unwrap();

        last = next;

        progress_bar.set_action("writing", Color::Red, Style::Bold);

        let offset_start = out.stream_position().map_err(VCRError::IOError)?;
        out.write_all(&delta).map_err(VCRError::IOError)?;
        let offset_end = out.stream_position().map_err(VCRError::IOError)?;

        deltas.push((offset_start, (offset_end - offset_start), step.hash));
        let delta_idx = (deltas.len() - 1) as u16;

        for path in step.paths {
            paths.push((path.0, path.1, delta_idx));
        }

        progress_bar.inc();
    }

    progress_bar.finalize();

    Ok(EncodedResource {
        paths: paths,
        deltas: deltas,
        basis: basis,
    })
}

// usage: download_site_data <out folder> <optional toml file with replaces>
fn main() -> VCRResult<()> {
    let chron_res: ChroniclerV1Response<chron::SiteUpdate> =
        blocking::get("https://api.sibr.dev/chronicler/v1/site/updates")
            .map_err(VCRError::ReqwestError)?
            .json()
            .map_err(VCRError::ReqwestError)?;
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
