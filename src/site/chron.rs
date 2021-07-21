use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SiteUpdate {
    pub timestamp: DateTime<Utc>,
    pub path: String,
    pub hash: String,
    pub download_url: String,
}

// each hash/step can serve for multiple different paths
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileStep {
    pub paths: Vec<(DateTime<Utc>, String)>,
    pub hash: String,
    pub download_url: String,
}

pub fn updates_to_steps(updates: Vec<SiteUpdate>) -> HashMap<String, Vec<FileStep>> {
    updates
        .into_iter()
        .fold(
            HashMap::new(),
            |mut acc: HashMap<String, Vec<SiteUpdate>>, u| {
                let path = Path::new(&u.path);

                let key = if let Some(file_name) = path.file_name() {
                    let chunks = file_name
                        .to_str()
                        .unwrap()
                        .split(".")
                        .collect::<Vec<&str>>();
                    (*(chunks.first().unwrap())).to_owned() + *(chunks.last().unwrap())
                } else {
                    "index".to_owned()
                };

                if let Some(vals) = acc.get_mut(&key) {
                    (*vals).push(u);
                } else {
                    acc.insert(key, vec![u]);
                }

                acc
            },
        )
        .into_iter()
        .map(|(file_name, resources)| {
            let mut unique_assets: HashMap<String, FileStep> = HashMap::new();
            for r in resources {
                if let Some(step) = unique_assets.get_mut(&r.hash) {
                    (*step).paths.push((r.timestamp, r.path));
                    (*step).paths.sort_by_key(|(t, _)| *t);
                } else {
                    unique_assets.insert(
                        r.hash.clone(),
                        FileStep {
                            paths: vec![(r.timestamp, r.path)],
                            hash: r.hash,
                            download_url: r.download_url,
                        },
                    );
                }
            }

            let mut unique_assets: Vec<FileStep> =
                unique_assets.into_iter().map(|(k, v)| v).collect();
            unique_assets.sort_by_key(|v| v.paths[0].0);
            (file_name, unique_assets)
        })
        .collect()
}
