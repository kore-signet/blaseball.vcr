use crate::UuidShell;
use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, PartialEq, vhs_diff::Patch, vhs_diff::Diff)]
#[serde(rename_all = "camelCase")]
pub struct Tournament {
    pub description: String,

    pub finals_name: String,

    pub id: String,

    pub index: i64,

    pub name: String,

    pub playoffs: Uuid,

    pub teams: Vec<UuidShell>,
}
