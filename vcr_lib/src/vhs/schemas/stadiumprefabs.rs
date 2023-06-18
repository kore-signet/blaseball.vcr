
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, vhs_diff::Patch, vhs_diff::Diff)]
pub struct Stadiumprefabs {
    pub collection: Vec<Collection>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct Collection {
    pub description: String,

    pub name: String,
}
