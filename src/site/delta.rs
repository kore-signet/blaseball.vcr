use super::chron::*;
use chrono::{DateTime, Utc};
use reqwest::blocking as reqs;
use serde::{Deserialize, Serialize};
use std::io::{Seek, Write};

pub fn urls_to_deltas(mut urls: Vec<&str>) -> (Vec<u8>, Vec<Vec<u8>>) {
    // (basis, deltas)
    let mut last: Vec<u8> = Vec::new();
    let mut first_response = reqs::get(urls.remove(0)).unwrap(); // TODO: result here instead
    first_response.copy_to(&mut last).unwrap();

    let mut deltas: Vec<Vec<u8>> = Vec::new();
    let basis: Vec<u8> = last.iter().copied().collect();

    for update in urls {
        let mut next: Vec<u8> = Vec::new();
        let mut next_r = reqs::get(update).unwrap();
        next_r.copy_to(&mut next).unwrap();

        deltas.push(xdelta3::encode(&next, &last).unwrap());
        last = next;
    }

    (basis, deltas)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncodedResource {
    pub paths: Vec<(DateTime<Utc>, String, u16)>, // date:path:deltaidx
    pub basis: Vec<u8>,
    pub deltas: Vec<(u64, u64, String)>, // delta offset, length in resource file, hash
}

pub fn encode_resource<W: Write + Seek>(steps: Vec<FileStep>, out: &mut W) -> EncodedResource {
    let mut basis: Vec<u8> = Vec::new();
    let mut basis_response = reqs::get(&format!(
        "https://api.sibr.dev/chronicler/v1{}",
        &steps[0].download_url
    ))
    .unwrap();
    basis_response.copy_to(&mut basis).unwrap();

    let mut last: Vec<u8> = basis.iter().copied().collect();

    let mut deltas: Vec<(u64, u64, String)> = Vec::new();
    let mut paths: Vec<(DateTime<Utc>, String, u16)> = Vec::new();

    let total_len = steps.len();

    for (i, step) in steps.into_iter().enumerate() {
        println!("Encoding step #{}/{}", i, total_len);
        println!("downloading...");

        let mut next: Vec<u8> = Vec::new();
        let mut next_response = reqs::get(&format!(
            "https://api.sibr.dev/chronicler/v1{}",
            &step.download_url
        ))
        .unwrap();
        next_response.copy_to(&mut next).unwrap();

        println!("creating delta...");

        let delta = xdelta3::encode(&next, &last).unwrap();

        last = next;

        println!("writing....");

        let offset_start = out.stream_position().unwrap();
        out.write_all(&delta).unwrap();
        let offset_end = out.stream_position().unwrap();

        deltas.push((offset_start, (offset_end - offset_start), step.hash));
        let delta_idx = (deltas.len() - 1) as u16;

        for path in step.paths {
            paths.push((path.0, path.1, delta_idx));
        }
    }

    EncodedResource {
        paths: paths,
        deltas: deltas,
        basis: basis,
    }
}
