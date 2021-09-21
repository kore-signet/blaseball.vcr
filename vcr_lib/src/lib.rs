mod err;
pub mod feed;
mod json_sequences;
pub mod site;
pub use err::*;
pub use json_sequences::*;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value as JSONValue;

pub type VCRResult<T> = Result<T, VCRError>;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChroniclerResponse<T> {
    pub next_page: Option<String>,
    pub items: Vec<T>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChroniclerV1Response<T> {
    pub next_page: Option<String>,
    pub data: Vec<T>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChroniclerEntity {
    pub entity_id: String,
    pub hash: String,
    pub valid_from: DateTime<Utc>,
    pub valid_to: Option<String>,
    pub data: JSONValue,
}

pub struct InternalPaging {
    pub remaining_ids: Vec<String>,
    pub remaining_data: Vec<ChroniclerEntity>,
    pub kind: ChronV2EndpointKind,
}

pub enum ChronV2EndpointKind {
    Versions(u32, u32),
    Entities(u32),
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct GameDate {
    pub day: i32,
    pub season: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tournament: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ChronV1Game {
    pub game_id: String,
    pub start_time: Option<DateTime<Utc>>,
    pub end_time: Option<DateTime<Utc>>,
    pub data: JSONValue,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ChronV1GameUpdate {
    pub game_id: String,
    pub timestamp: DateTime<Utc>,
    pub hash: String,
    pub data: JSONValue,
}

pub fn encode_varint(i: u16) -> Vec<u8> {
    if i >= 255 {
        vec![255_u8.to_be_bytes().to_vec(), i.to_be_bytes().to_vec()].concat()
    } else {
        (i as u8).to_be_bytes().to_vec()
    }
}

#[macro_export]
macro_rules! decode_varint {
    ($read:expr) => {{
        let mut first_byte: [u8; 1] = [0; 1];
        $read.read_exact(&mut first_byte)?;
        let length_byte = u8::from_be_bytes(first_byte);
        if length_byte == 255 {
            let mut longer_bytes: [u8; 2] = [0; 2];
            $read.read_exact(&mut longer_bytes)?;
            u16::from_be_bytes(longer_bytes)
        } else {
            length_byte as u16
        }
    }};
}
