
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, vhs_diff::Patch, vhs_diff::Diff)]
pub struct Teamelectionstats {
    pub wills: Vec<Will>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct Will {
    pub id: String,

    pub percent: String,
}
