use serde::{Deserialize, Serialize};
use vhs_diff::{Diff, Patch};

#[derive(Diff, Patch, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Sunsun {
    pub current: f64,
    pub maximum: i64,
    pub recharge: i64,
}
