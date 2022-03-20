use serde::{Deserialize, Serialize};
use uuid::Uuid;
use vhs_diff::{Diff, Patch};

#[derive(Diff, Patch, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Decreeresult {
    pub decree_id: String,
    pub decree_title: String,
    pub description: String,
    pub id: Uuid,
    pub total_votes: i64,
}
