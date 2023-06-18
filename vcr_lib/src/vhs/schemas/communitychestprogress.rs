
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, vhs_diff::Patch, vhs_diff::Diff)]
#[serde(rename_all = "camelCase")]
pub struct CommunityChestProgress {
    pub chests_unlocked: i64,

    pub progress: Progress,

    pub runs: Progress,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
#[serde(untagged)]
pub enum Progress {
    Integer(i64),

    String(String),
}
