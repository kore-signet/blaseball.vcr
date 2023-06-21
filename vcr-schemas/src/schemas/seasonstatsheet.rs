use serde::*;
use vhs_diff::*;

#[derive(Serialize, Deserialize, Diff, Patch, Clone, Debug)]
pub struct Seasonstatsheet {
    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "teamStats")]
    pub team_stats: Vec<String>,
}
