
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, vhs_diff::Patch, vhs_diff::Diff)]
pub struct Feedseasonlist {
    pub collection: Vec<Collection>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct Collection {
    pub index: i64,

    pub name: String,

    pub seasons: Vec<i64>,

    pub sim: String,
}
