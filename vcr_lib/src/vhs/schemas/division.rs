use serde::{Deserialize, Serialize};
use uuid::Uuid;
use vhs_diff::{Diff, Patch};
use crate::UuidShell;

#[derive(Diff, Patch, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Division {
    pub id: Uuid,
    pub name: String,
    pub teams: Vec<UuidShell>,
}
