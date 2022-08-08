use serde::{Deserialize, Serialize};
use uuid::Uuid;
use vhs_diff::{Diff, Patch};
use crate::UuidShell;

#[derive(Diff, Patch, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Tournament {
    pub description: String,
    pub finals_name: String,
    pub id: Uuid,
    pub index: i64,
    pub name: String,
    pub playoffs: Uuid,
    pub teams: Vec<UuidShell>,
}
