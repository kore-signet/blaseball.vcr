pub mod block;
pub mod db;
// pub mod desc;
pub mod event;
pub mod header;
pub mod recorder;
/*
the feed is split into blocks of 255 events each.

each block has a header storing:
- it's length in bytes when compressed and decompressed
- the timestamp of the first event in the block
- the positions of every event inside the block

so, to get an event at time <x>, we find the block which contains that time via binary search
we then decompress it, and find the event inside it with time <x>
*/

use modular_bitfield::specifiers::{B24, B8};
use serde::{Deserialize, Serialize};

use self::event::FeedEvent;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct EncodedBlockHeader {
    pub compressed_len: u32,
    pub decompressed_len: u32,
    pub start_time: i64,
    pub metadata: BlockMetadata,
    pub event_positions: Vec<(i64, u32)>,
}

#[derive(Serialize, Deserialize, Copy, Clone, PartialEq, Eq, Debug)]
pub struct BlockMetadata {
    pub tournament: i8,
    pub season: i8,
    pub phase: u8,
    // pub day: u8, //            day: ev.day.try_into().unwrap_or(255),
}

impl BlockMetadata {
    pub fn from_event(event: &FeedEvent) -> BlockMetadata {
        BlockMetadata {
            tournament: event.tournament,
            season: event.season,
            phase: event.phase,
            // day: event.day.try_into().unwrap_or(255),
        }
    }
}

// same, but includes an offset field for ease of use
pub struct BlockHeader {
    pub compressed_len: u32,
    pub decompressed_len: u32,
    pub start_time: i64,
    pub event_positions: Vec<(i64, u32)>,
    pub metadata: BlockMetadata,
    pub offset: u32,
}

#[modular_bitfield::bitfield]
pub struct EventId {
    chunk: B24,
    idx: B8,
}
