use serde::{Deserialize, Serialize};
use vhs_diff::{Diff, Patch};

#[derive(Diff, Patch, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Temporal {
    pub doc: Option<Doc>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Doc {
    pub alpha: i64,
    pub beta: i64,
    pub delta: bool,
    pub epsilon: bool,
    pub eta: Option<i64>,
    pub gamma: i64,
    pub id: String,
    pub iota: Option<i64>,
    pub theta: Option<String>,
    pub zeta: String,
}
