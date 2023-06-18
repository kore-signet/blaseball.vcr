
use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, PartialEq, vhs_diff::Patch, vhs_diff::Diff)]
pub struct Subleague {
    pub divisions: Vec<Uuid>,

    pub id: Uuid,

    pub name: String,
}
