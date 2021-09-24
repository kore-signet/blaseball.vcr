mod err;
mod json_sequences;
pub mod site;
#[macro_use]
pub mod utils;
pub mod feed;
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
pub struct ChroniclerEntity<T> {
    pub entity_id: String,
    pub hash: String,
    pub valid_from: DateTime<Utc>,
    pub valid_to: Option<String>,
    pub data: T,
}

pub struct InternalPaging<T> {
    pub remaining_ids: Vec<String>,
    pub remaining_data: Vec<ChroniclerEntity<T>>,
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
    #[serde(default)]
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
pub struct ChronV1GameUpdate<T> {
    pub game_id: String,
    pub timestamp: DateTime<Utc>,
    pub hash: String,
    pub data: T,
}
