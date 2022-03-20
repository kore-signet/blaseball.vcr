use serde::{Deserialize, Serialize};
use uuid::Uuid;
use vhs_diff::{Diff, Patch};

#[derive(Diff, Patch, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Playoffround {
    #[serde(rename = "__v")]
    pub v: Option<i64>,
    #[serde(rename = "_id")]
    pub old_id: Option<Uuid>,
    pub id: Option<Uuid>,
    pub game_index: i64,
    pub games: Vec<Vec<String>>,
    pub matchups: Vec<Uuid>,
    pub name: String,
    pub round_number: i64,
    pub special: bool,
    pub winners: Vec<String>,
    pub winner_seeds: Vec<i64>,
}

impl Playoffround {
    pub fn id(&self) -> Option<Uuid> {
        self.id.or(self.old_id)
    }
}
