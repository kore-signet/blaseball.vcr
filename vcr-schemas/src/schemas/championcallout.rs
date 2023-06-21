
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, vhs_diff::Patch, vhs_diff::Diff)]
pub struct Championcallout {
    pub champion: Option<Champion>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct Champion {
    pub team: String,
}
