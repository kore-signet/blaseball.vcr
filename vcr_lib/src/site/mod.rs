pub mod chron;
pub mod manager;

use chrono::{DateTime, Utc};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EncodedResource {
    pub paths: Vec<(DateTime<Utc>, String, u16)>, // date:path:deltaidx
    pub basis: Vec<u8>,
    pub deltas: Vec<PatchData>, // delta offset,  length in resource file, original length, hash
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PatchData {
    pub offset: u32,
    pub compressed_patch_length: u32,
    pub uncompressed_patch_length: u32,
    pub original_length: u32,
    pub hash: String,
}
