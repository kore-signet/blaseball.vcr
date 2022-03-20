use serde::{Deserialize, Serialize};
use uuid::Uuid;
use vhs_diff::{Diff, Patch};

#[derive(Diff, Patch, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Playoffs {
    #[serde(rename = "__v")]
    pub v: Option<i64>,
    #[serde(rename = "_id")]
    pub old_id: Option<Uuid>,
    pub id: Option<Uuid>,

    pub bracket: Option<i64>,

    pub name: String,

    pub number_of_rounds: i64,

    pub playoff_day: i64,

    pub round: Option<i64>,

    pub rounds: Vec<Uuid>,

    pub season: i64,

    pub tomorrow_round: Option<i64>,

    pub tournament: Option<i64>,

    pub winner: Option<Uuid>,
}

impl Playoffs {
    pub fn id(&self) -> Option<Uuid> {
        self.id.or(self.old_id)
    }
}
