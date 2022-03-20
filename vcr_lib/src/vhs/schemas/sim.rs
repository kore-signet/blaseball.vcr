use serde::{Deserialize, Serialize};
use uuid::Uuid;
use vhs_diff::{Diff, Patch};

#[derive(Diff, Patch, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Sim {
    // ...fear
    #[serde(rename = "__v")]
    pub v: Option<i64>,

    #[serde(rename = "_id")]
    pub old_id: Option<String>,
    pub id: Option<String>,

    #[serde(rename = "agitations")]
    pub agitations: Option<i64>,

    #[serde(rename = "attr")]
    pub attr: Option<Vec<String>>,

    #[serde(rename = "day")]
    pub day: i64,

    #[serde(rename = "doTheThing")]
    pub do_the_thing: Option<bool>,

    #[serde(rename = "earlpostseasonDate")]
    pub earlpostseason_date: Option<String>,

    #[serde(rename = "earlseasonDate")]
    pub earlseason_date: Option<String>,

    #[serde(rename = "earlsiestaDate")]
    pub earlsiesta_date: Option<String>,

    #[serde(rename = "eatTheRich")]
    pub eat_the_rich: Option<bool>,

    #[serde(rename = "electionDate")]
    pub election_date: Option<String>,

    #[serde(rename = "endseasonDate")]
    pub endseason_date: Option<String>,

    #[serde(rename = "eraColor")]
    pub era_color: Option<String>,

    #[serde(rename = "eraTitle")]
    pub era_title: String,

    #[serde(rename = "godsDayDate")]
    pub gods_day_date: Option<String>,

    #[serde(rename = "labourOne")]
    pub labour_one: Option<i64>,

    #[serde(rename = "latepostseasonDate")]
    pub latepostseason_date: Option<String>,

    #[serde(rename = "lateseasonDate")]
    pub lateseason_date: Option<String>,

    #[serde(rename = "latesiestaDate")]
    pub latesiesta_date: Option<String>,

    #[serde(rename = "league")]
    pub league: String,

    #[serde(rename = "menu")]
    pub menu: Option<String>,

    #[serde(rename = "midseasonDate")]
    pub midseason_date: Option<String>,

    #[serde(rename = "nextElectionEnd")]
    pub next_election_end: Option<String>,

    #[serde(rename = "nextPhaseTime")]
    pub next_phase_time: String,

    #[serde(rename = "nextSeasonStart")]
    pub next_season_start: Option<String>,

    #[serde(rename = "openedBook")]
    pub opened_book: Option<bool>,

    #[serde(rename = "phase")]
    pub phase: i64,

    #[serde(rename = "playOffRound")]
    pub play_off_round: Option<i64>,

    #[serde(rename = "playoffs")]
    pub playoffs: Playoffs,

    #[serde(rename = "preseasonDate")]
    pub preseason_date: Option<String>,

    #[serde(rename = "rules")]
    pub rules: Uuid,

    #[serde(rename = "salutations")]
    pub salutations: Option<i64>,

    #[serde(rename = "season")]
    pub season: i64,

    #[serde(rename = "seasonId")]
    pub season_id: Uuid,

    #[serde(rename = "state")]
    pub state: Option<State>,

    #[serde(rename = "subEraColor")]
    pub sub_era_color: Option<String>,

    #[serde(rename = "subEraTitle")]
    pub sub_era_title: Option<String>,

    #[serde(rename = "terminology")]
    pub terminology: Uuid,

    #[serde(rename = "tournament")]
    pub tournament: Option<i64>,

    #[serde(rename = "tournamentRound")]
    pub tournament_round: Option<i64>,

    #[serde(rename = "twgo")]
    pub twgo: Option<String>,

    #[serde(rename = "unlockedInterviews")]
    pub unlocked_interviews: Option<bool>,

    #[serde(rename = "unlockedPeanuts")]
    pub unlocked_peanuts: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct State {
    #[serde(rename = "phaseOnHold")]
    phase_on_hold: Option<i64>,
    #[serde(rename = "scheduled_game_event")]
    scheduled_game_event: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
#[serde(untagged)]
pub enum Playoffs {
    String(Uuid),
    StringArray(Vec<Uuid>),
}
