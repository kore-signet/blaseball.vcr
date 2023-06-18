
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, vhs_diff::Patch, vhs_diff::Diff)]
#[serde(transparent)]
#[repr(transparent)]
pub struct Nullified {
    inner: NullifiedInner
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
#[serde(untagged)]
pub enum NullifiedInner {
    NullifiedClass(NullifiedClass),

    StringArray(Vec<String>),
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct NullifiedClass {
    pub history: Option<Vec<History>>,

    pub rules: Vec<String>,

    pub size: Option<f64>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct History {
    pub day: i64,

    pub season: i64,

    pub size: f64,
}
