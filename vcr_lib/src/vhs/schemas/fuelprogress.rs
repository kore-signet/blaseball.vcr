use serde::{Deserialize, Serialize};
use uuid::Uuid;
use vhs_diff::{Diff, Patch};

#[derive(Diff, Patch, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct FuelprogressWrapper {
    inner: Vec<FuelprogressElement>,
}

#[derive(Clone, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct FuelprogressElement {
    pub amount: f64,
    pub id: Uuid,
}
