mod db;

pub mod encoder;

pub use db::*;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct EntityData {
    pub data_offset: u64,
    pub patches: Vec<(u32, u64, u64)>, // timestamp, offset, end of patch
    pub path_map: HashMap<u16, String>, // path_id:path
}
