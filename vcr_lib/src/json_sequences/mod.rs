mod db;
mod header;
mod tributes;

pub mod encoder;

pub use db::*;
pub use header::*;
pub use tributes::*;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use serde_json::{json, Value as JSONValue};

use json_patch::Patch as JSONPatch;

fn default_checkpoint() -> u16 {
    u16::MAX
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EntityData {
    pub patches: Vec<(u32, u32, u32)>, // timestamp, offset, end of patch
    pub path_map: HashMap<u16, String>, // path_id:path
    #[serde(default = "default_checkpoint")]
    pub checkpoint_every: u16,
    #[serde(default = "default_base")]
    pub base: JSONValue,
}

fn default_base() -> JSONValue {
    json!({})
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Patch {
    ReplaceRoot(JSONValue),
    Normal(JSONPatch),
}
