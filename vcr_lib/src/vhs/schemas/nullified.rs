use serde::{Deserialize, Serialize};
use vhs_diff::{Diff, Patch};

#[derive(PartialEq, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NullifiedClass {
    pub history: Option<Vec<History>>,

    pub rules: Vec<String>,

    pub size: Option<f64>,
}

#[derive(PartialEq, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct History {
    pub day: i64,

    pub season: i64,

    pub size: f64,
}

#[derive(PartialEq, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Nullified {
    NullifiedClass(NullifiedClass),

    StringArray(Vec<String>),
}

#[derive(Diff, Patch, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct NullifiedWrapper {
    inner: Nullified,
}
