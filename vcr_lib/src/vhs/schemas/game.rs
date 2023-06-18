#![allow(clippy::redundant_closure_call)]
#![allow(clippy::needless_lifetimes)]
#![allow(clippy::match_single_binding)]
#![allow(clippy::clone_on_copy)]

use serde::{Deserialize, Serialize};
use crate::UuidShell;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, vhs_diff::Diff, vhs_diff::Patch)]
pub struct GameUpdate {
    #[serde(rename = "atBatBalls")]
    pub at_bat_balls: i64,
    #[serde(rename = "atBatStrikes")]
    pub at_bat_strikes: i64,
    #[serde(rename = "awayBalls", default, skip_serializing_if = "Option::is_none")]
    pub away_balls: Option<i64>,
    #[serde(rename = "awayBases", default, skip_serializing_if = "Option::is_none")]
    pub away_bases: Option<i64>,
    #[serde(rename = "awayBatter", deserialize_with = "crate::uuid_shell::empty_string_as_none")]
    pub away_batter: Option<UuidShell>,
    #[serde(
        rename = "awayBatterMod",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub away_batter_mod: Option<String>,
    #[serde(rename = "awayBatterName")]
    pub away_batter_name: Option<String>,
    #[serde(rename = "awayOdds")]
    pub away_odds: Option<f64>,
    #[serde(rename = "awayOuts", default, skip_serializing_if = "Option::is_none")]
    pub away_outs: Option<i64>,
    #[serde(rename = "awayPitcher", deserialize_with = "crate::uuid_shell::empty_string_as_none")]
    pub away_pitcher: Option<UuidShell>,
    #[serde(
        rename = "awayPitcherMod",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub away_pitcher_mod: Option<String>,
    #[serde(rename = "awayPitcherName")]
    pub away_pitcher_name: Option<String>,
    #[serde(rename = "awayScore")]
    pub away_score: Option<f64>,
    #[serde(rename = "awayStrikes")]
    pub away_strikes: Option<i64>,
    #[serde(rename = "awayTeam")]
    pub away_team: UuidShell,
    #[serde(rename = "awayTeamBatterCount")]
    pub away_team_batter_count: Option<i64>,
    #[serde(rename = "awayTeamColor")]
    pub away_team_color: String,
    #[serde(rename = "awayTeamEmoji")]
    pub away_team_emoji: String,
    #[serde(rename = "awayTeamName")]
    pub away_team_name: String,
    #[serde(rename = "awayTeamNickname")]
    pub away_team_nickname: String,
    #[serde(rename = "awayTeamRuns", default)]
    pub away_team_runs: (),
    #[serde(
        rename = "awayTeamSecondaryColor",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub away_team_secondary_color: Option<String>,
    #[serde(
        rename = "baseRunnerMods",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub base_runner_mods: Vec<String>,
    #[serde(
        rename = "baseRunnerNames",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub base_runner_names: Vec<String>,
    #[serde(rename = "baseRunners")]
    pub base_runners: Vec<UuidShell>,
    #[serde(rename = "baserunnerCount")]
    pub baserunner_count: i64,
    #[serde(rename = "basesOccupied")]
    pub bases_occupied: Vec<i64>,
    #[serde(
        rename = "bottomInningScore",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub bottom_inning_score: Option<f64>,
    pub day: i64,
    #[serde(rename = "endPhase", default, skip_serializing_if = "Option::is_none")]
    pub end_phase: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub finalized: Option<bool>,
    #[serde(rename = "gameComplete")]
    pub game_complete: bool,
    #[serde(rename = "gameStart")]
    pub game_start: bool,
    #[serde(
        rename = "gameStartPhase",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub game_start_phase: Option<i64>,
    #[serde(rename = "halfInningOuts")]
    pub half_inning_outs: i64,
    #[serde(rename = "halfInningScore")]
    pub half_inning_score: f64,
    #[serde(rename = "homeBalls", default, skip_serializing_if = "Option::is_none")]
    pub home_balls: Option<i64>,
    #[serde(rename = "homeBases", default, skip_serializing_if = "Option::is_none")]
    pub home_bases: Option<i64>,
    #[serde(rename = "homeBatter", deserialize_with = "crate::uuid_shell::empty_string_as_none")]
    pub home_batter: Option<UuidShell>,
    #[serde(
        rename = "homeBatterMod",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub home_batter_mod: Option<String>,
    #[serde(rename = "homeBatterName")]
    pub home_batter_name: Option<String>,
    #[serde(rename = "homeOdds")]
    pub home_odds: Option<f64>,
    #[serde(rename = "homeOuts", default, skip_serializing_if = "Option::is_none")]
    pub home_outs: Option<i64>,
    #[serde(rename = "homePitcher", deserialize_with = "crate::uuid_shell::empty_string_as_none")]
    pub home_pitcher: Option<UuidShell>,
    #[serde(
        rename = "homePitcherMod",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub home_pitcher_mod: Option<String>,
    #[serde(rename = "homePitcherName")]
    pub home_pitcher_name: Option<String>,
    #[serde(rename = "homeScore")]
    pub home_score: Option<f64>,
    #[serde(rename = "homeStrikes")]
    pub home_strikes: Option<i64>,
    #[serde(rename = "homeTeam")]
    pub home_team: UuidShell,
    #[serde(rename = "homeTeamBatterCount")]
    pub home_team_batter_count: i64,
    #[serde(rename = "homeTeamColor")]
    pub home_team_color: String,
    #[serde(rename = "homeTeamEmoji")]
    pub home_team_emoji: String,
    #[serde(rename = "homeTeamName")]
    pub home_team_name: String,
    #[serde(rename = "homeTeamNickname")]
    pub home_team_nickname: String,
    #[serde(rename = "homeTeamRuns", default)]
    pub home_team_runs: (),
    #[serde(
        rename = "homeTeamSecondaryColor",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub home_team_secondary_color: Option<String>,
    #[serde(rename = "_id", default, skip_serializing_if = "Option::is_none")]
    pub old_id: Option<UuidShell>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<UuidShell>,
    pub inning: i64,
    #[serde(rename = "isPostseason")]
    pub is_postseason: bool,
    #[serde(
        rename = "isPrizeMatch",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_prize_match: Option<bool>,
    #[serde(
        rename = "isTitleMatch",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_title_match: Option<bool>,
    #[serde(
        rename = "lastUpdate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub last_update: Option<String>,
    #[serde(
        rename = "lastUpdateFull",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub last_update_full: Option<Vec<GameLastUpdateFullItem>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub loser: Option<String>,
    #[serde(
        rename = "newHalfInningPhase",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub new_half_inning_phase: Option<i64>,
    #[serde(
        rename = "newInningPhase",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub new_inning_phase: Option<i64>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub outcomes: Vec<String>,
    pub phase: i64,
    #[serde(rename = "playCount", default, skip_serializing_if = "Option::is_none")]
    pub play_count: Option<i64>,
    #[serde(
        rename = "queuedEvents",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub queued_events: Vec<GameQueuedEventsItem>,
    #[serde(
        rename = "repeatCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub repeat_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rules: Option<String>,
    #[serde(
        rename = "scoreLedger",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub score_ledger: Option<String>,
    #[serde(
        rename = "scoreUpdate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub score_update: Option<String>,
    pub season: i64,
    #[serde(rename = "seasonId", default, skip_serializing_if = "Option::is_none")]
    pub season_id: Option<String>,
    #[serde(
        rename = "secretBaserunner",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub secret_baserunner: Option<String>,
    #[serde(rename = "seriesIndex")]
    pub series_index: i64,
    #[serde(rename = "seriesLength")]
    pub series_length: i64,
    pub shame: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sim: Option<String>,
    #[serde(rename = "stadiumId", default, skip_serializing_if = "Option::is_none")]
    pub stadium_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<GameState>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub statsheet: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub terminology: Option<String>,
    #[serde(
        rename = "topInningScore",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub top_inning_score: Option<f64>,
    #[serde(rename = "topOfInning")]
    pub top_of_inning: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tournament: Option<i64>,
    #[serde(
        rename = "tournamentRound",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub tournament_round: Option<i64>,
    #[serde(
        rename = "tournamentRoundGameIndex",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub tournament_round_game_index: Option<i64>,
    pub weather: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub winner: Option<String>,
}
impl From<&GameUpdate> for GameUpdate {
    fn from(value: &GameUpdate) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct GameLastUpdateFullItem {
    pub blurb: String,
    pub category: i64,
    pub created: String,
    pub day: i64,
    pub description: String,
    #[serde(rename = "gameTags")]
    pub game_tags: Vec<serde_json::Value>,
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<GameLastUpdateFullItemMetadata>,
    pub nuts: i64,
    pub phase: i64,
    #[serde(rename = "playerTags")]
    pub player_tags: Vec<String>,
    pub season: i64,
    #[serde(rename = "teamTags")]
    pub team_tags: Vec<String>,
    pub tournament: i64,
    #[serde(rename = "type")]
    pub type_: i64,
}
impl From<&GameLastUpdateFullItem> for GameLastUpdateFullItem {
    fn from(value: &GameLastUpdateFullItem) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct GameLastUpdateFullItemMetadata {
    #[serde(rename = "aLocation", default, skip_serializing_if = "Option::is_none")]
    pub a_location: Option<i64>,
    #[serde(rename = "aPlayerId", default, skip_serializing_if = "Option::is_none")]
    pub a_player_id: Option<String>,
    #[serde(
        rename = "aPlayerName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub a_player_name: Option<String>,
    #[serde(rename = "aTeamId", default, skip_serializing_if = "Option::is_none")]
    pub a_team_id: Option<String>,
    #[serde(rename = "aTeamName", default, skip_serializing_if = "Option::is_none")]
    pub a_team_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub after: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub away: Option<String>,
    #[serde(rename = "awayEmoji", default, skip_serializing_if = "Option::is_none")]
    pub away_emoji: Option<String>,
    #[serde(rename = "awayScore", default, skip_serializing_if = "Option::is_none")]
    pub away_score: Option<i64>,
    #[serde(rename = "bLocation", default, skip_serializing_if = "Option::is_none")]
    pub b_location: Option<i64>,
    #[serde(rename = "bPlayerId", default, skip_serializing_if = "Option::is_none")]
    pub b_player_id: Option<String>,
    #[serde(
        rename = "bPlayerName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub b_player_name: Option<String>,
    #[serde(rename = "bTeamId", default, skip_serializing_if = "Option::is_none")]
    pub b_team_id: Option<String>,
    #[serde(rename = "bTeamName", default, skip_serializing_if = "Option::is_none")]
    pub b_team_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub before: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effect: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub home: Option<String>,
    #[serde(rename = "homeEmoji", default, skip_serializing_if = "Option::is_none")]
    pub home_emoji: Option<String>,
    #[serde(rename = "homeScore", default, skip_serializing_if = "Option::is_none")]
    pub home_score: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(
        rename = "inPlayerId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub in_player_id: Option<String>,
    #[serde(
        rename = "inPlayerName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub in_player_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ledger: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub lines: Vec<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<i64>,
    #[serde(rename = "mod", default, skip_serializing_if = "Option::is_none")]
    pub mod_: Option<String>,
    #[serde(
        rename = "outPlayerId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub out_player_id: Option<String>,
    #[serde(
        rename = "outPlayerName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub out_player_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(rename = "teamId", default, skip_serializing_if = "Option::is_none")]
    pub team_id: Option<String>,
    #[serde(rename = "teamName", default, skip_serializing_if = "Option::is_none")]
    pub team_name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub update: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub weather: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub winner: Option<String>,
}
impl From<&GameLastUpdateFullItemMetadata> for GameLastUpdateFullItemMetadata {
    fn from(value: &GameLastUpdateFullItemMetadata) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct GameQueuedEventsItem {
    pub delay: i64,
    #[serde(rename = "isSpecial")]
    pub is_special: bool,
    #[serde(rename = "logUpdates")]
    pub log_updates: Vec<String>,
    pub outcomes: Vec<serde_json::Value>,
    #[serde(rename = "type")]
    pub type_: i64,
}
impl From<&GameQueuedEventsItem> for GameQueuedEventsItem {
    fn from(value: &GameQueuedEventsItem) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct GameState {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ego_player_data: Vec<GameStateEgoPlayerDataItem>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub game_cancelled: Option<bool>,
    #[serde(
        rename = "holidayInning",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub holiday_inning: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub postseason: Option<GameStatePostseason>,
    #[serde(
        rename = "prizeMatch",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub prize_match: Option<GameStatePrizeMatch>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reload: Option<GameStateReload>,
    #[serde(
        rename = "snowfallEvents",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub snowfall_events: Option<i64>,
}
impl From<&GameState> for GameState {
    fn from(value: &GameState) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct GameStateEgoPlayerDataItem {
    #[serde(rename = "hallPlayer")]
    pub hall_player: bool,
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub team: Option<String>,
}
impl From<&GameStateEgoPlayerDataItem> for GameStateEgoPlayerDataItem {
    fn from(value: &GameStateEgoPlayerDataItem) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct GameStatePostseason {
    pub bracket: i64,
    pub matchup: String,
    #[serde(rename = "playoffId")]
    pub playoff_id: String,
}
impl From<&GameStatePostseason> for GameStatePostseason {
    fn from(value: &GameStatePostseason) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct GameStatePrizeMatch {
    #[serde(rename = "itemId")]
    pub item_id: String,
    #[serde(rename = "itemName")]
    pub item_name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub winner: Option<String>,
}
impl From<&GameStatePrizeMatch> for GameStatePrizeMatch {
    fn from(value: &GameStatePrizeMatch) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct GameStateReload {
    #[serde(rename = "BaserunnerCount")]
    pub baserunner_count: i64,
    #[serde(rename = "Baserunners")]
    pub baserunners: Vec<String>,
    #[serde(rename = "BasesOccupied")]
    pub bases_occupied: Vec<i64>,
    #[serde(rename = "Batter")]
    pub batter: String,
}
impl From<&GameStateReload> for GameStateReload {
    fn from(value: &GameStateReload) -> Self {
        value.clone()
    }
}
