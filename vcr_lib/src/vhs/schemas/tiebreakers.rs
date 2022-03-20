use serde::{Deserialize, Serialize};
use uuid::Uuid;
use vhs_diff::{Diff, Patch};

#[derive(Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Tiebreaker {
    pub id: Uuid,
    pub order: Vec<Uuid>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TiebreakersClass {
    #[serde(rename = "id")]
    pub id: Uuid,

    #[serde(rename = "order")]
    pub order: Vec<Uuid>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
#[serde(untagged)]
// [shrugs fearfully]
pub enum Tiebreakers {
    TiebreakerArray(Vec<Tiebreaker>),

    TiebreakersClass(TiebreakersClass),
}

#[derive(Diff, Patch, Clone, Serialize, Deserialize, PartialEq)]
#[serde(transparent)]
pub struct TiebreakerWrapper {
    inner: Tiebreakers,
}
