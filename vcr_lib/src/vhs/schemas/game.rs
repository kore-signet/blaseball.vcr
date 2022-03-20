use serde::{Deserialize, Serialize};
use uuid::Uuid;
use vhs_diff::{Diff, Patch};

#[derive(Clone, Serialize, Deserialize, Patch, Diff)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct GameUpdate {
    #[serde(rename = "_id")]
    pub old_id: Option<Uuid>,
    pub id: Option<Uuid>,
    pub at_bat_balls: i64,
    pub at_bat_strikes: i64,
    pub away_balls: Option<i64>,
    pub away_bases: Option<i64>,
    pub away_batter: Option<String>,
    pub away_batter_mod: Option<String>,
    pub away_batter_name: Option<String>,
    pub away_odds: f64,
    pub away_outs: Option<i64>,
    pub away_pitcher: Option<String>,
    pub away_pitcher_mod: Option<String>,
    pub away_pitcher_name: Option<String>,
    pub away_score: f64,
    pub away_strikes: Option<i64>,
    pub away_team: String,
    pub away_team_batter_count: i64,
    pub away_team_color: String,
    pub away_team_emoji: String,
    pub away_team_name: String,
    pub away_team_nickname: String,
    pub away_team_secondary_color: Option<String>,
    pub baserunner_count: i64,
    pub base_runner_mods: Option<Vec<String>>,
    pub base_runner_names: Option<Vec<String>>,
    pub base_runners: Vec<String>,
    pub bases_occupied: Vec<i64>,
    pub bottom_inning_score: Option<f64>,
    pub day: i64,
    pub end_phase: Option<i64>,
    pub finalized: bool,
    pub game_complete: bool,
    pub game_start: bool,
    pub game_start_phase: Option<i64>,
    pub half_inning_outs: i64,
    pub half_inning_score: f64,
    pub home_balls: Option<i64>,
    pub home_bases: Option<i64>,
    pub home_batter: Option<String>,
    pub home_batter_mod: Option<String>,
    pub home_batter_name: Option<String>,
    pub home_odds: f64,
    pub home_outs: Option<i64>,
    pub home_pitcher: Option<String>,
    pub home_pitcher_mod: Option<String>,
    pub home_pitcher_name: Option<String>,
    pub home_score: f64,
    pub home_strikes: Option<i64>,
    pub home_team: String,
    pub home_team_batter_count: i64,
    pub home_team_color: String,
    pub home_team_emoji: String,
    pub home_team_name: String,
    pub home_team_nickname: String,
    pub home_team_secondary_color: Option<String>,
    pub inning: i64,
    pub is_postseason: bool,
    pub is_title_match: Option<bool>,
    pub last_update: String,
    pub new_half_inning_phase: Option<i64>,
    pub new_inning_phase: Option<i64>,
    pub outcomes: Vec<String>,
    pub phase: i64,
    pub play_count: Option<i64>,
    pub queued_events: Option<Vec<QueuedEvent>>,
    pub repeat_count: Option<i64>,
    pub rules: String,
    pub score_ledger: Option<String>,
    pub score_update: Option<String>,
    pub season: i64,
    pub secret_baserunner: Option<String>,
    pub series_index: i64,
    pub series_length: i64,
    pub shame: bool,
    pub stadium_id: Option<String>,
    pub state: Option<State>,
    pub statsheet: String,
    pub terminology: String,
    pub top_inning_score: Option<f64>,
    pub top_of_inning: bool,
    pub tournament: Option<i64>,
    pub weather: Option<i64>,
}

impl GameUpdate {
    pub fn id(&self) -> Option<Uuid> {
        self.id.or(self.old_id)
    }
}

#[derive(Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct QueuedEvent {
    delay: Option<i64>,
    is_special: Option<bool>,
    log_updates: Option<Vec<String>>,
    outcomes: Vec<Option<serde_json::Value>>,
    queued_event_type: Option<i64>,
    #[serde(rename = "type")]
    ty: Option<i64>,
}

#[derive(Clone, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct State {
    ego_player_data: Option<Vec<EgoPlayerDatum>>,
    game_cancelled: Option<bool>,
    #[serde(rename = "holidayInning")]
    holiday_inning: Option<bool>,
    postseason: Option<Postseason>,
    #[serde(rename = "prizeMatch")]
    prize_match: Option<PrizeMatch>,
    reload: Option<Reload>,
}

#[derive(Clone, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct EgoPlayerDatum {
    #[serde(rename = "hallPlayer")]
    hall_player: bool,
    id: String,
    location: Option<i64>,
    team: Option<String>,
}

#[derive(Clone, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Postseason {
    bracket: i64,
    matchup: String,
    #[serde(rename = "playoffId")]
    playoff_id: String,
}

#[derive(Clone, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct PrizeMatch {
    #[serde(rename = "itemId")]
    item_id: String,
    #[serde(rename = "itemName")]
    item_name: String,
    winner: Option<String>,
}

#[derive(Clone, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Reload {
    #[serde(rename = "BaserunnerCount")]
    baserunner_count: i64,
    #[serde(rename = "Baserunners")]
    baserunners: Vec<String>,
    #[serde(rename = "BasesOccupied")]
    bases_occupied: Vec<i64>,
    #[serde(rename = "Batter")]
    batter: String,
}
