use serde::{Deserialize, Serialize};
use vhs_diff::{Diff, Patch};

#[derive(Diff, Patch, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Offseasonrecap {
    pub bonus_results: Vec<String>,
    pub decree_results: Vec<String>,
    pub event_results: Vec<String>,
    pub id: String,
    pub name: String,
    pub season: i64,
    pub total_bonus_votes: i64,
    pub total_decree_votes: i64,
    pub total_will_votes: Option<i64>,
    pub vote_count: i64,
    pub will_results: Option<Vec<String>>,
}
