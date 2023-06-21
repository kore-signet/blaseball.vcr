
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, vhs_diff::Patch, vhs_diff::Diff)]
pub struct Glossarywords {
    pub collection: Vec<Collection>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct Collection {
    pub definition: Vec<String>,

    pub name: String,
}
