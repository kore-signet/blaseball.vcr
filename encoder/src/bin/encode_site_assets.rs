use blaseball_vcr::site::*;
use blaseball_vcr::*;
use bsdiff::diff;
use clap::clap_app;
use serde::Deserialize;
use std::collections::BTreeMap;
use std::fs::{self, File};
use std::io::{self, BufWriter, Seek, SeekFrom, Write};
use std::path::PathBuf;
use zstd::bulk::Compressor;

#[derive(Deserialize)]
struct JsonSiteUpdate {
    timestamp: iso8601_timestamp::Timestamp,
    path: String,
    #[serde(rename = "where")]
    file_path: String,
}

fn main() -> VCRResult<()> {
    let matches = clap_app!(train_vhs_dict =>
        (version: "1.0")
        (author: "emily signet <emily@sibr.dev>")
        (about: "blaseball.vcr gen 2 site assets encoder")
        (@arg INPUT: +required -i --input [FOLDER] "input folder")
        (@arg OUTPUT: +required -o --output [FILE] "set output file for asset tape")
    )
    .get_matches();

    let input_folder = PathBuf::from(matches.value_of("INPUT").unwrap());
    let metadata_path = input_folder.join("files.json");

    let asset_metadata: BTreeMap<AssetType, Vec<JsonSiteUpdate>> =
        serde_json::from_reader(File::open(&metadata_path)?)?;

    let mut asset_tape = BufWriter::new(tempfile::tempfile()?);
    let mut patch_metadata: BTreeMap<AssetType, PatchSet> = BTreeMap::new();

    let mut compressor = Compressor::new(3)?;

    for (asset_type, files) in asset_metadata {
        println!("serializing ~ {}", serde_json::to_string(&asset_type)?);
        let file_data: Vec<Vec<u8>> = files
            .iter()
            .map(|metadata| fs::read(input_folder.join(&metadata.file_path)))
            .collect::<io::Result<Vec<Vec<u8>>>>()?;

        let mut assets: Vec<(JsonSiteUpdate, Vec<u8>)> = files.into_iter().zip(file_data).collect();
        assets.sort_by_key(|(k, _)| k.timestamp);

        let mut hasher = blake2s_simd::blake2sp::Params::new();
        hasher.hash_length(16);

        let mut source = assets[0].1.clone();
        let mut patch_set = PatchSet {
            initial: source.clone(),
            patches: Vec::new(),
        };

        let total_len = assets.len();

        for (i, (asset_metadata, bytes)) in assets.into_iter().enumerate() {
            print!("\x1b[2K\r#{i}/{total_len}");
            io::stdout().flush()?;

            let hash = hasher.hash(&bytes);

            asset_tape.flush()?;

            let offset = asset_tape.stream_position()? as u32;
            let mut patch_out = Vec::new();
            diff::diff(&source, &bytes, &mut patch_out)?;

            let uncompressed_patch_length = patch_out.len() as u32;

            let compressed_patch = compressor.compress(&patch_out)?;
            asset_tape.write_all(&compressed_patch)?;
            asset_tape.flush()?;

            let length = asset_tape.stream_position()? as u32 - offset;
            let uncompressed_length = bytes.len() as u32;

            source = bytes;

            patch_set.patches.push(PatchHeader {
                path: asset_metadata.path,
                timestamp: timestamp_to_nanos(asset_metadata.timestamp),
                hash: hash.as_bytes().try_into().unwrap(),
                offset,
                length,
                uncompressed_length,
                uncompressed_patch_length,
            });
        }

        println!();

        patch_metadata.insert(asset_type, patch_set);
    }

    let header = rmp_serde::to_vec(&patch_metadata)?;
    let mut main_out = BufWriter::new(File::create(matches.value_of("OUTPUT").unwrap())?);
    main_out.write_all(&(header.len() as u32).to_le_bytes())?;
    main_out.write_all(&header)?;
    asset_tape.seek(SeekFrom::Start(0))?;
    io::copy(asset_tape.get_mut(), &mut main_out)?;
    main_out.flush()?;

    Ok(())
}
