use serde::{Deserialize, Serialize};
use uuid::Uuid;
use vhs_diff::{Diff, Patch};
use crate::UuidShell;

#[derive(Diff, Patch, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Bonusresult {
    pub bonus_id: String,
    pub bonus_title: String,
    pub description: String,
    pub highest_team: UuidShell,
    pub highest_team_votes: i64,
    pub id: Uuid,
    pub team_id: UuidShell,
    pub team_votes: i64,
    pub total_votes: i64,
}
