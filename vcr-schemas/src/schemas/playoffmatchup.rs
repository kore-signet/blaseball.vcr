use vcr_lookups::UuidShell;
use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, PartialEq, vhs_diff::Patch, vhs_diff::Diff)]
#[serde(rename_all = "camelCase")]
pub struct Playoffmatchup {
    #[serde(rename = "__v")]
    pub v: Option<i64>,

    #[serde(alias = "_id")]
    pub id: Uuid,

    pub away_seed: Option<i64>,

    pub away_team: Option<UuidShell>,

    pub away_wins: i64,

    pub games_needed: Option<String>,

    pub games_played: Option<i64>,

    pub home_seed: i64,

    pub home_team: UuidShell,

    pub home_wins: i64,


    pub name: Option<serde_json::Value>,
}