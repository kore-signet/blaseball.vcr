
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, vhs_diff::Patch, vhs_diff::Diff)]
#[serde(rename_all = "camelCase")]
pub struct Renovationprogress {
    pub progress: Option<Progress>,

    pub stats: Option<Vec<Stat>>,

    pub to_next: Option<f64>,

    pub total: Option<i64>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Progress {
    pub to_next: f64,

    pub total: i64,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct Stat {
    pub id: String,

    pub percent: String,
}
