use super::chron::*;
use crate::{VCRError, VCRResult};
use chrono::{DateTime, Utc};
use progress_bar::color::{Color, Style};
use progress_bar::progress_bar::ProgressBar;
use reqwest;
use serde::{Deserialize, Serialize};
use std::io::{Seek, Write};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncodedResource {
    pub paths: Vec<(DateTime<Utc>, String, u16)>, // date:path:deltaidx
    pub basis: Vec<u8>,
    pub deltas: Vec<(u64, u64, String)>, // delta offset, length in resource file, hash
}

pub fn encode_resource<W: Write + Seek>(
    steps: Vec<FileStep>,
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

        let mut next: Vec<u8> = Vec::new();
        let mut next_response = client
            .get(&format!(
                "https://api.sibr.dev/chronicler/v1{}",
                &step.download_url
            ))
            .send()
            .map_err(VCRError::ReqwestError)?;
        next_response
            .copy_to(&mut next)
            .map_err(VCRError::ReqwestError)?;

        progress_bar.set_action("creating delta", Color::Blue, Style::Bold);

        let delta = xdelta3::encode(&next, &last).unwrap();

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
