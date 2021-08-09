mod db;

pub mod encoder;

pub use db::*;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde_json::{json, Value as JSONValue};

use json_patch::Patch as JSONPatch;

fn default_checkpoint() -> u32 {
    u32::MAX
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EntityData {
    pub data_offset: u64,
    pub patches: Vec<(u32, u64, u64)>, // timestamp, offset, end of patch
    pub path_map: HashMap<u16, String>, // path_id:path
    #[serde(default = "default_checkpoint")]
    pub checkpoint_every: u32,
    #[serde(default = "default_base")]
    pub base: JSONValue,
}

fn default_base() -> JSONValue {
    json!({})
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
pub enum Patch {
    ReplaceRoot(JSONValue),
    Normal(JSONPatch),
}
