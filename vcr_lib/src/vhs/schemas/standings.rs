use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use crate::UuidShell;
use vhs_diff::{Diff, Patch};

#[derive(Diff, Patch, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Standings {
    #[serde(rename = "__v")]
    pub v: Option<i64>,
    #[serde(rename = "_id")]
    pub old_id: Option<Uuid>,
    pub id: Option<Uuid>,
    pub games_played: Option<HashMap<UuidShell, Option<i64>>>,
    pub losses: Option<HashMap<UuidShell, Option<i64>>>,
    pub runs: Option<HashMap<UuidShell, Option<f64>>>,
    pub wins: Option<HashMap<UuidShell, Option<i64>>>,
}

impl Standings {
    pub fn id(&self) -> Option<Uuid> {
        self.id.or(self.old_id)
    }
}
