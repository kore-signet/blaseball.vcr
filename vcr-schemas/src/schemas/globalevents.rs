
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, vhs_diff::Patch, vhs_diff::Diff)]
#[repr(transparent)]
#[serde(transparent)]
pub struct GlobaleventsWrapper {
    inner: Globalevents
}

pub type Globalevents = Vec<Globalevent>;

#[derive(Serialize, Deserialize, Clone, PartialEq, vhs_diff::Patch, vhs_diff::Diff)]
pub struct Globalevent {
    #[serde(rename = "__v")]
    pub v: Option<i64>,

    #[serde(rename = "_id")]
    pub id: Option<String>,

    pub expire: Option<String>,

    #[serde(rename = "id")]
    pub globalevent_id: Option<String>,

    pub msg: String,
}
