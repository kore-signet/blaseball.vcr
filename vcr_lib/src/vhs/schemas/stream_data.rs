use serde::Serialize;
use super::*;

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PlayoffData {
    pub round: Option<Playoffround>,
    pub matchups: Vec<Playoffmatchup>,
    pub playoffs: Playoffs,
    pub all_rounds: Vec<Playoffround>,
    pub all_matchups: Vec<Playoffmatchup>,
    pub tomorrow_round: Option<Playoffround>,
    pub tomorrow_matchups: Vec<Playoffmatchup>,
}

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[repr(transparent)]
pub struct StreamDataWrapper {
    pub value: StreamData
}

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StreamData {
    pub games: GameData,
    pub leagues: LeagueData,
    pub fights: FightData,
    pub temporal: Option<Temporal>
}

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GameData {
    pub sim: Sim,
    pub season: Season,
    pub schedule: Vec<GameUpdate>,
    pub tomorrow_schedule: Vec<GameUpdate>,
    pub tournament: Option<Tournament>,
    pub standings: Option<Standings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postseason: Option<PlayoffData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postseasons: Option<Vec<PlayoffData>>
}

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LeagueData {
    pub teams: Vec<Team>,
    pub subleagues: Vec<Subleague>,
    pub divisions: Vec<Division>,
    pub leagues: Vec<League>,
    pub tiebreakers: Vec<TiebreakerWrapper>,
    pub stadiums: Vec<Stadium>,
    pub stats: StatData
}

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StatData {
    pub sunsun: Option<Sunsun>,
    pub community_chest: Option<CommunityChestProgress>
}

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FightData {
    pub boss_fights: Vec<Bossfight>
}