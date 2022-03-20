use serde::{Deserialize, Serialize};
use vhs_diff::{Diff, Patch};

#[derive(Diff, Patch, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct GlobaleventsWrapper {
    inner: Vec<Globalevent>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Globalevent {
    #[serde(rename = "__v")]
    pub v: Option<i64>,
    #[serde(rename = "_id")]
    pub id: Option<String>,
    pub expire: Option<String>,
    #[serde(rename = "id")]
    pub globalevent_id: Option<String>,
    #[serde(rename = "msg")]
    pub msg: String,
}
