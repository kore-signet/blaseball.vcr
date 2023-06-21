
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, vhs_diff::Patch, vhs_diff::Diff)]
#[repr(transparent)]
#[serde(transparent)]
pub struct FuelProgressWrapper {
    inner: Fuelprogress
}

pub type Fuelprogress = Vec<FuelprogressElement>;

#[derive(Serialize, Deserialize, Clone, PartialEq, vhs_diff::Patch, vhs_diff::Diff)]
pub struct FuelprogressElement {
    pub amount: f64,

    pub id: String,
}
