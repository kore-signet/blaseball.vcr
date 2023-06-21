
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, vhs_diff::Patch, vhs_diff::Diff)]
#[repr(transparent)]
#[serde(transparent)]
pub struct Idols {
    inner: IdolsInner
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
#[serde(untagged)]
pub enum IdolsInner {
    IdolArray(Vec<Idol>),

    IdolsClass(IdolsClass),
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Idol {
    pub id: Option<String>,

    pub player_id: String,

    pub total: Option<i64>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct IdolsClass {
    pub data: Data,

    pub idols: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    pub strictly_confidential: i64,
}
