use super::chron::*;
use super::delta::*;
use crate::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read, Seek, SeekFrom};
use std::sync::Mutex;

pub struct ResourceManager {
    headers: HashMap<String, EncodedResource>,
    resources: HashMap<String, Mutex<BufReader<File>>>,
}

impl ResourceManager {
    // type, header, main file
    pub fn from_files(files: Vec<(&str, &str, &str)>) -> VCRResult<ResourceManager> {
        let mut headers: HashMap<String, EncodedResource> = HashMap::new();
        let mut resources: HashMap<String, Mutex<BufReader<File>>> = HashMap::new();

        for (r_type, r_header, r_file) in files {
            let header_f = File::open(r_header).map_err(VCRError::IOError)?;
            let header: EncodedResource =
                rmp_serde::from_read(header_f).map_err(VCRError::MsgPackError)?;

            let main_f = File::open(r_file).map_err(VCRError::IOError)?;
            let mut reader = BufReader::new(main_f);

            resources.insert(r_type.to_owned(), Mutex::new(reader));
            headers.insert(r_type.to_owned(), header);
        }

        Ok(ResourceManager {
            headers: headers,
            resources: resources,
        })
    }

    pub fn get_resource(&self, name: &str, delta_idx: u16) -> VCRResult<Vec<u8>> {
        let mut delta_file = self.resources[name].lock().unwrap();
        let header = &self.headers[name];

        let mut res: Vec<u8> = header.basis.iter().copied().collect();

        for idx in 0..delta_idx {
            let (offset, length, _) = header.deltas[idx as usize];
            delta_file
                .seek(SeekFrom::Start(offset))
                .map_err(VCRError::IOError)?;

            let mut delta_buffer: Vec<u8> = vec![0; length as usize];
            delta_file
                .read_exact(&mut delta_buffer)
                .map_err(VCRError::IOError)?;

            res = xdelta3::decode(&delta_buffer, &res).unwrap();
        }

        Ok(res)
    }

    pub fn expand_site_updates(&self, base_url: &str) -> Vec<SiteUpdate> {
        self.headers
            .iter()
            .map(|(key, resources)| {
                resources
                    .paths
                    .iter()
                    .map(|(time, path, idx)| SiteUpdate {
                        timestamp: time.clone(),
                        path: path.to_owned(),
                        hash: resources.deltas[*idx as usize].2.clone(),
                        download_url: format!(
                            "{base_url}/{kind}/{idx}",
                            base_url = base_url,
                            kind = key,
                            idx = idx
                        ),
                    })
                    .collect::<Vec<SiteUpdate>>()
            })
            .flatten()
            .collect::<Vec<SiteUpdate>>()
    }
}
