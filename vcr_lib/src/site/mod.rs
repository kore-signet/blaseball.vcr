pub mod chron;
pub mod manager;

use chrono::{DateTime, Utc};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EncodedResource {
    pub paths: Vec<(DateTime<Utc>, String, u16)>, // date:path:deltaidx
    pub basis: Vec<u8>,
    pub deltas: Vec<(u64, u64, String)>, // delta offset, length in resource file, hash
}
