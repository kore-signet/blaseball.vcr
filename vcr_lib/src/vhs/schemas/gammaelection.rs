
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, vhs_diff::Patch, vhs_diff::Diff)]
#[serde(rename_all = "camelCase")]
pub struct Gammaelection {
    pub choice_type: String,

    pub description: String,

    pub election_complete: bool,

    pub end_date: String,

    pub icon: String,

    pub id: String,

    pub maximum_allowed: Option<serde_json::Value>,

    pub name: String,

    pub start_date: String,
}
