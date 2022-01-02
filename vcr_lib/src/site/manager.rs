use super::chron::*;
use super::*;
use crate::*;
use bsdiff::patch::patch;
use memmap2::{Mmap, MmapOptions};
use std::collections::HashMap;
use std::fs::{read_dir, File};
use std::io::{self, BufReader};
use std::path::{Path, PathBuf};

pub struct ResourceManager {
    headers: HashMap<String, EncodedResource>,
    resources: HashMap<String, Mmap>,
}

impl ResourceManager {
    // type, header, main file
    pub fn from_folder<P: AsRef<Path>>(folder: P) -> VCRResult<ResourceManager> {
        let (mut header_paths, mut db_paths): (Vec<PathBuf>, Vec<PathBuf>) = read_dir(folder)?
            .map(|res| res.map(|e| e.path()))
            .collect::<Result<Vec<PathBuf>, io::Error>>()?
            .into_iter()
            .filter(|path| path.is_file())
            .partition(|path| {
                if let Some(name) = path.file_name() {
                    name.to_str().unwrap().contains(".header.riv")
                } else {
                    false
                }
            });
        header_paths.sort();
        db_paths.sort();
        let entries: Vec<(String, PathBuf, PathBuf)> = header_paths
            .into_iter()
            .zip(db_paths.into_iter())
            .map(|(header, main)| {
                let e_type = main
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .split_once('.')
                    .unwrap()
                    .0
                    .to_owned();
                (e_type, header, main)
            })
            .collect();

        let mut headers: HashMap<String, EncodedResource> = HashMap::new();
        let mut resources: HashMap<String, Mmap> = HashMap::new();

        for (r_type, r_header, r_file) in entries {
            let header_f = File::open(r_header)?;
            let header_r = BufReader::new(header_f);
            let header: EncodedResource = rmp_serde::from_read(header_r)?;

            let main_f = File::open(r_file)?;
            let reader = unsafe { MmapOptions::new().populate().map(&main_f)? };

            resources.insert(r_type.to_owned(), reader);
            headers.insert(r_type.to_owned(), header);
        }

        Ok(ResourceManager { headers, resources })
    }

    pub fn from_files(files: Vec<(&str, &str, &str)>) -> VCRResult<ResourceManager> {
        let mut headers: HashMap<String, EncodedResource> = HashMap::new();
        let mut resources: HashMap<String, Mmap> = HashMap::new();

        for (r_type, r_header, r_file) in files {
            let header_f = File::open(r_header)?;
            let header: EncodedResource = rmp_serde::from_read(header_f)?;

            let main_f = File::open(r_file)?;
            let reader = unsafe { MmapOptions::new().populate().map(&main_f)? };

            resources.insert(r_type.to_owned(), reader);
            headers.insert(r_type.to_owned(), header);
        }

        Ok(ResourceManager { headers, resources })
    }

    pub fn get_resource(&self, name: &str, delta_idx: u16) -> VCRResult<Vec<u8>> {
        let delta_file = &self.resources[name];
        let header = &self.headers[name];

        let mut res: Vec<u8> = header.basis.clone();
        let mut decompressor = zstd::block::Decompressor::new();

        for idx in 0..delta_idx + 1 {
            let metadata = &header.deltas[idx as usize];
            let mut patch_data = io::Cursor::new(decompressor.decompress(
                &delta_file[metadata.offset as usize
                    ..(metadata.offset + metadata.compressed_patch_length) as usize],
                metadata.uncompressed_patch_length as usize,
            )?);

            let mut patched = vec![0; metadata.original_length as usize];
            patch(&res, &mut patch_data, &mut patched)?;
            res = patched;
        }

        Ok(res)
    }

    pub fn expand_site_updates(&self, base_url: &str) -> Vec<SiteUpdate> {
        self.headers
            .iter()
            .flat_map(|(key, resources)| {
                resources
                    .paths
                    .iter()
                    .map(|(time, path, idx)| SiteUpdate {
                        timestamp: *time,
                        path: path.to_owned(),
                        hash: resources.deltas[*idx as usize].hash.clone(),
                        download_url: format!(
                            "{base_url}/{kind}/{idx}",
                            base_url = base_url,
                            kind = key,
                            idx = idx
                        ),
                    })
                    .collect::<Vec<SiteUpdate>>()
            })
            .collect::<Vec<SiteUpdate>>()
    }
}
