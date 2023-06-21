pub mod db;
pub mod recorder;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct DataHeader {
    pub id: [u8; 16],
    pub times: Vec<i64>,
    pub compressed_len: u32,
    pub decompressed_len: u32,
    pub offset: u32,
    pub checkpoint_every: usize,
    pub checkpoint_positions: Vec<usize>, // positions in the decompressed byte slice at which checkpoints happen
}
