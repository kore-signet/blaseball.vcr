
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, vhs_diff::Patch, vhs_diff::Diff)]
pub struct Attributes {
    pub collection: Vec<Collection>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Collection {
    pub background: String,

    pub color: String,

    pub description: String,

    pub descriptions: Option<Descriptions>,

    pub id: String,

    pub text_color: String,

    pub title: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct Descriptions {
    pub ballpark: Option<String>,

    pub league: Option<String>,

    pub player: Option<String>,

    pub team: Option<String>,
}
