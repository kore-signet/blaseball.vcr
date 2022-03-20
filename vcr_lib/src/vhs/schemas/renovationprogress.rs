use serde::{Deserialize, Serialize};
use vhs_diff::{Diff, Patch};

#[derive(Diff, Patch, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Renovationprogress {
    pub progress: Option<Progress>,
    pub stats: Option<Vec<Stat>>,
    pub to_next: Option<f64>,
    pub total: Option<i64>,
}

#[derive(PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Progress {
    pub to_next: f64,
    pub total: i64,
}

#[derive(PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Stat {
    pub id: String,
    pub percent: String,
}
