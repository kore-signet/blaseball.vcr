use serde::{Deserialize, Serialize};
use vhs_diff::{Diff, Patch};

#[derive(Diff, Patch, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Vault {
    #[serde(rename = "legendaryPlayers")]
    pub legendary_players: Vec<String>,
}
