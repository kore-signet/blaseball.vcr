
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, vhs_diff::Patch, vhs_diff::Diff)]
pub struct Division {
    pub id: String,

    pub name: String,

    pub teams: Vec<String>,
}
