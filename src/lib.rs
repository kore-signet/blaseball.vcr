mod err;
pub mod feed;
mod json_sequences;
pub mod site;
pub use err::*;
pub use json_sequences::*;

use chrono::{DateTime, Utc};
use serde::{Serialize,Deserialize};
use serde_json::Value as JSONValue;

pub type VCRResult<T> = Result<T, VCRError>;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChroniclerResponse<T> {
    pub next_page: Option<String>,
    pub items: Vec<T>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChroniclerV1Response<T> {
    pub next_page: Option<String>,
    pub data: Vec<T>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChroniclerEntity {
    pub entity_id: String,
    pub hash: String,
    pub valid_from: DateTime<Utc>,
    pub valid_to: Option<DateTime<Utc>>,
    pub data: JSONValue,
}
