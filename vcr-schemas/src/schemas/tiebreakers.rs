
use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, PartialEq, vhs_diff::Patch, vhs_diff::Diff)]
#[repr(transparent)]
#[serde(transparent)]
pub struct Tiebreakers {
    inner: TiebreakersInner
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
#[serde(untagged)]
pub enum TiebreakersInner {
    TiebreakerArray(Vec<Tiebreaker>),

    TiebreakersClass(TiebreakersClass),
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct Tiebreaker {
    pub id: Uuid,

    pub order: Vec<Uuid>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct TiebreakersClass {
    pub id: Uuid,

    pub order: Vec<Uuid>,
}
