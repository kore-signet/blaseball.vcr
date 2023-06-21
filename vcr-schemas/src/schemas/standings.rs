
use std::collections::HashMap;

use serde::{Serialize, Deserialize};
use vcr_lookups::UuidShell;

#[derive(Deserialize, Serialize, Copy, Clone, PartialEq)]
#[serde(untagged)]
pub enum FloatOrI64 {
    F64(f64),
    I64(i64)
}

#[derive(Serialize, Deserialize, Clone, PartialEq, vhs_diff::Patch, vhs_diff::Diff)]
#[serde(rename_all = "camelCase")]
pub struct Standings {
    #[serde(rename = "__v")]
    pub v: Option<i64>,

    #[serde(rename = "_id")]
    pub id: Option<String>,

    pub games_played: Option<HashMap<UuidShell, Option<FloatOrI64>>>,

    #[serde(rename = "id")]
    pub standings_id: Option<String>,

    pub losses: Option<HashMap<UuidShell, Option<FloatOrI64>>>,

    pub runs: Option<HashMap<UuidShell, Option<FloatOrI64>>>,
    pub wins: HashMap<UuidShell, Option<FloatOrI64>>
}

