
use serde::{Serialize, Deserialize};

// #[derive(Serialize, Deserialize, Clone, PartialEq, vhs_diff::Patch, vhs_diff::Diff)]
// #[repr(transparent)]
// #[serde(transparent)]
// pub struct Gammaelectiondetails {
//     inner:  Vec<Gammaelectiondetail>
// }

// // pub type Gammaelectiondetails = Vec<Gammaelectiondetail>;

#[derive(Serialize, Deserialize, Clone, PartialEq, vhs_diff::Patch, vhs_diff::Diff)]
#[serde(transparent)]
#[repr(transparent)]
pub struct Gammaelectiondetails {
    inner: serde_json::Value
}