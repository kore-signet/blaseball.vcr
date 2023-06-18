
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, vhs_diff::Patch, vhs_diff::Diff)]
pub struct Gammabracket {
    pub bracket: Vec<Vec<Bracket>>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct Bracket {
    pub away: Option<Away>,

    pub home: Home,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Away {
    pub day_number: i64,

    pub id: String,

    pub initial_ruleset: String,

    pub phase_id: String,

    pub previous_round_number: i64,

    pub round_game_index: i64,

    pub round_number: i64,

    pub round_score: i64,

    pub season_id: String,

    pub season_number: i64,

    pub sim_id: String,

    pub team_id: String,

    pub tournament: i64,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Home {
    pub day_number: i64,

    pub id: String,

    pub initial_ruleset: String,

    pub phase_id: String,

    pub previous_round_number: i64,

    pub round_game_index: i64,

    pub round_number: i64,

    pub round_score: i64,

    pub season_id: String,

    pub season_number: i64,

    pub sim_id: String,

    pub team_id: String,

    pub tournament: i64,
}
