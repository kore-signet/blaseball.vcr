
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, vhs_diff::Patch, vhs_diff::Diff)]
pub struct Sunsun {
    pub current: f64,

    pub maximum: i64,

    pub recharge: i64,
}
