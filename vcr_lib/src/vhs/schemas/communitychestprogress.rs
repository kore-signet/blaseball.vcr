use serde::{Deserialize, Serialize};
use vhs_diff::{Diff, Patch};

#[derive(Diff, Patch, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
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
