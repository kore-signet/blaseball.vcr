
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, vhs_diff::Patch, vhs_diff::Diff)]
pub struct Temporal {
    pub alpha: Option<i64>,

    pub beta: Option<i64>,

    pub delta: Option<bool>,

    pub doc: Option<Doc>,

    pub epsilon: Option<bool>,

    pub eta: Option<i64>,

    pub gamma: Option<i64>,

    pub id: Option<String>,

    pub iota: Option<i64>,

    pub theta: Option<String>,

    pub zeta: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
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
