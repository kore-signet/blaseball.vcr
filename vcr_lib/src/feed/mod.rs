pub mod event;
#[rustfmt::skip]
pub mod lookup_tables;
pub mod block;
pub mod db;
pub mod recorder;
/*
the feed is split into blocks of 100 events each.

each block has a header storing:
- it's length in bytes when compressed and decompressed
- the timestamp of the first event in the block
- the positions of every event inside the block

so, to get an event at time <x>, we find the block which contains that time via binary search
we then decompress it, and find the event inside it with time <x>
*/

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct EncodedBlockHeader {
    pub compressed_len: u32,
    pub decompressed_len: u32,
    pub start_time: u64,
    pub event_positions: Vec<(u64, u32)>,
}

// same, but includes an offset field for ease of use
pub struct BlockHeader {
    pub compressed_len: u32,
    pub decompressed_len: u32,
    pub start_time: u64,
    pub event_positions: Vec<(u64, u32)>,
    pub offset: u32,
}
