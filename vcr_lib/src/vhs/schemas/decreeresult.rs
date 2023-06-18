
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, vhs_diff::Patch, vhs_diff::Diff)]
#[serde(rename_all = "camelCase")]
pub struct Decreeresult {
    pub decree_id: String,

    pub decree_title: String,

    pub description: String,

    pub id: String,

    pub total_votes: i64,
}
