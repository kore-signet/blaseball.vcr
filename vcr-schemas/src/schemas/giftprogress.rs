use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use vhs_diff::{Diff, Patch};

#[derive(Diff, Patch, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Giftprogress {
    pub team_progress: HashMap<Uuid, TeamProgress>,
    pub team_wish_lists: HashMap<Uuid, Vec<WishlistProgress>>,
}

#[derive(PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TeamProgress {
    total: i64,
    to_next: f64,
}

#[derive(PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct WishlistProgress {
    bonus: String,
    percent: f64,
}
