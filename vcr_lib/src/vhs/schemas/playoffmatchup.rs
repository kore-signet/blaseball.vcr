use serde::{Deserialize, Serialize};
use uuid::Uuid;
use vhs_diff::{Diff, Patch};

#[derive(Diff, Patch, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Playoffmatchup {
    #[serde(rename = "__v")]
    pub v: Option<i64>,

    #[serde(rename = "_id")]
    pub old_id: Option<Uuid>,
    pub id: Option<Uuid>,

    pub away_seed: Option<i64>,

    pub away_team: Option<Uuid>,

    pub away_wins: i64,

    pub games_needed: Option<String>,

    pub games_played: Option<i64>,

    pub home_seed: i64,

    pub home_team: Uuid,

    pub home_wins: i64,

    pub name: Option<serde_json::Value>,
}

impl Playoffmatchup {
    pub fn id(&self) -> Option<Uuid> {
        self.id.or(self.old_id)
    }
}
