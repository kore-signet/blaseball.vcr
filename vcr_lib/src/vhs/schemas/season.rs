use serde::{Deserialize, Serialize};
use uuid::Uuid;
use vhs_diff::{Diff, Patch};

#[derive(Diff, Patch, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Season {
    // look i'm not gonna question it
    #[serde(rename = "__v")]
    pub v: Option<i64>,

    #[serde(rename = "_id")]
    pub old_id: Option<Uuid>,
    pub id: Option<Uuid>,

    pub league: Uuid,

    pub rules: Uuid,

    pub schedule: Uuid,

    pub season_number: i64,

    pub standings: Uuid,

    pub stats: String,

    pub terminology: Uuid,
}

impl Season {
    pub fn id(&self) -> Option<Uuid> {
        self.id.or(self.old_id)
    }
}
