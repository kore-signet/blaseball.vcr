use serde::{Deserialize, Serialize};
use uuid::Uuid;
use vhs_diff::{Diff, Patch};

#[derive(Diff, Patch, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct League {
    pub id: Uuid,
    pub name: String,
    pub subleagues: Vec<Uuid>,
    pub tiebreakers: Uuid,
}
