
use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, PartialEq, vhs_diff::Patch, vhs_diff::Diff)]
pub struct League {
    pub id: Uuid,

    pub name: String,

    pub subleagues: Vec<Uuid>,

    pub tiebreakers: Uuid,
}
