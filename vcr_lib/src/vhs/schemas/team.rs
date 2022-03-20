use serde::{Deserialize, Serialize};
use uuid::Uuid;
use vhs_diff::{Diff, Patch};

#[derive(Diff, Patch, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Team {
    #[serde(rename = "_id")]
    pub old_id: Option<Uuid>,
    pub id: Option<Uuid>,
    pub bench: Option<Vec<Uuid>>,
    pub bullpen: Option<Vec<Uuid>>,
    pub card: Option<i64>,
    pub championships: i64,
    pub deceased: Option<bool>,
    pub e_density: Option<f64>,
    pub emoji: String,
    pub e_velocity: Option<f64>,
    pub evolution: Option<i64>,
    pub full_name: String,
    pub game_attr: Option<Vec<String>>,
    pub im_position: Option<ImPosition>,
    pub level: Option<i64>,
    pub lineup: Vec<Uuid>,
    pub location: String,
    pub main_color: String,
    pub nickname: String,
    pub permanent_attributes: Option<Vec<Option<serde_json::Value>>>,
    pub perm_attr: Option<Vec<String>>,
    pub rotation: Vec<Uuid>,
    pub rotation_slot: Option<i64>,
    pub seas_attr: Option<Vec<String>>,
    pub season_attributes: Option<Vec<String>>,
    pub season_shames: i64,
    pub season_shamings: i64,
    pub secondary_color: String,
    pub shadows: Option<Vec<Uuid>>,
    pub shame_runs: f64,
    pub shorthand: String,
    pub slogan: String,
    pub stadium: Option<Uuid>,
    pub state: Option<State>,
    pub team_spirit: Option<i64>,
    pub total_shames: i64,
    pub total_shamings: i64,
    pub tournament_wins: Option<i64>,
    pub underchampionships: Option<i64>,
    pub week_attr: Option<Vec<String>>,
    pub win_streak: Option<i64>,
}

impl Team {
    pub fn id(&self) -> Option<Uuid> {
        self.id.or(self.old_id)
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct State {
    donated_shame: Option<f64>,
    fax_machine: Option<FaxMachine>,
    game_mod_sources: Option<GameModSources>,
    #[serde(rename = "halfinning_plays")]
    halfinning_plays: Option<i64>,
    #[serde(rename = "imp_motion")]
    imp_motion: Option<Vec<ImpMotion>>,
    nullified: Option<bool>,
    #[serde(rename = "overflowRuns")]
    overflow_runs: Option<f64>,
    #[serde(rename = "permModSources")]
    perm_mod_sources: Option<PermModSources>,
    #[serde(rename = "redacted")]
    redacted: Option<bool>,
    #[serde(rename = "scattered")]
    scattered: Option<Scattered>,
    #[serde(rename = "stolenPlayers")]
    stolen_players: Option<Vec<StolenPlayer>>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct FaxMachine {
    runs_needed: i64,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
// SURE I GUESS? ? ??
pub struct GameModSources {
    #[serde(rename = "0")]
    the_0: Option<Vec<String>>,

    #[serde(rename = "A")]
    a: Option<Vec<String>>,

    #[serde(rename = "AA")]
    aa: Option<Vec<String>>,

    #[serde(rename = "AAA")]
    aaa: Option<Vec<String>>,

    #[serde(rename = "ACIDIC")]
    acidic: Option<Vec<String>>,

    #[serde(rename = "AFFINITY_FOR_CROWS")]
    affinity_for_crows: Option<Vec<String>>,

    #[serde(rename = "AMBUSH")]
    ambush: Option<Vec<String>>,

    #[serde(rename = "BASE_INSTINCTS")]
    base_instincts: Option<Vec<String>>,

    #[serde(rename = "BIRD_SEED")]
    bird_seed: Option<Vec<String>>,

    #[serde(rename = "BLACKHOLE_PAYOUTS")]
    blackhole_payouts: Option<Vec<String>>,

    #[serde(rename = "BOTTOM_DWELLER")]
    bottom_dweller: Option<Vec<String>>,

    #[serde(rename = "CARCINIZATION")]
    carcinization: Option<Vec<String>>,

    #[serde(rename = "EARLY_TO_PARTY")]
    early_to_party: Option<Vec<String>>,

    #[serde(rename = "ELECTRIC")]
    electric: Option<Vec<String>>,

    #[serde(rename = "EXTRA_STRIKE")]
    extra_strike: Option<Vec<String>>,

    #[serde(rename = "FIERY")]
    fiery: Option<Vec<String>>,

    #[serde(rename = "FIREPROOF")]
    fireproof: Option<Vec<String>>,

    #[serde(rename = "FREE_GIFT")]
    free_gift: Option<Vec<String>>,

    #[serde(rename = "GOOD_RIDDANCE")]
    good_riddance: Option<Vec<String>>,

    #[serde(rename = "GROWTH")]
    growth: Option<Vec<String>>,

    #[serde(rename = "H20")]
    h20: Option<Vec<String>>,

    #[serde(rename = "HEAVY_HANDED")]
    heavy_handed: Option<Vec<String>>,

    #[serde(rename = "HIGH_PRESSURE")]
    high_pressure: Option<Vec<String>>,

    #[serde(rename = "HOME_FIELD")]
    home_field: Option<Vec<String>>,

    #[serde(rename = "LATE_TO_PARTY")]
    late_to_party: Option<Vec<String>>,

    #[serde(rename = "LIFE_OF_PARTY")]
    life_of_party: Option<Vec<String>>,

    #[serde(rename = "LIGHT_HANDED")]
    light_handed: Option<Vec<String>>,

    #[serde(rename = "LOVE")]
    love: Option<Vec<String>>,

    #[serde(rename = "MAINTENANCE_MODE")]
    maintenance_mode: Option<Vec<String>>,

    #[serde(rename = "MIDDLING")]
    middling: Option<Vec<String>>,

    #[serde(rename = "MODERATION")]
    moderation: Option<Vec<String>>,

    #[serde(rename = "O_NO")]
    o_no: Option<Vec<String>>,

    #[serde(rename = "OVERPERFORMING")]
    overperforming: Option<Vec<String>>,

    #[serde(rename = "PARTY_TIME")]
    party_time: Option<Vec<String>>,

    #[serde(rename = "POPCORN_PAYOUTS")]
    popcorn_payouts: Option<Vec<String>>,

    #[serde(rename = "PSYCHIC")]
    psychic: Option<Vec<String>>,

    #[serde(rename = "SEALANT")]
    sealant: Option<Vec<String>>,

    #[serde(rename = "SHAME_GIVER")]
    shame_giver: Option<Vec<String>>,

    #[serde(rename = "SINKING_SHIP")]
    sinking_ship: Option<Vec<String>>,

    #[serde(rename = "STALEPOPCORN_PAYOUTS")]
    stalepopcorn_payouts: Option<Vec<String>>,

    #[serde(rename = "SUN2_PAYOUTS")]
    sun2_payouts: Option<Vec<String>>,

    #[serde(rename = "SUN_KISSED")]
    sun_kissed: Option<Vec<String>>,

    #[serde(rename = "TRAVELING")]
    traveling: Option<Vec<String>>,

    #[serde(rename = "UNDERPERFORMING")]
    underperforming: Option<Vec<String>>,

    #[serde(rename = "UNDERSEA")]
    undersea: Option<Vec<String>>,

    #[serde(rename = "UNHOLEY")]
    unholey: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ImpMotion {
    day: i64,
    im_position: Vec<f64>,
    season: i64,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct PermModSources {
    #[serde(rename = "OVERPERFORMING")]
    overperforming: Option<Vec<String>>,

    #[serde(rename = "UNDERPERFORMING")]
    underperforming: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Scattered {
    full_name: String,
    location: String,
    nickname: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct StolenPlayer {
    id: String,
    victim_index: i64,
    victim_location: i64,
    victim_team_id: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
#[serde(untagged)]
pub enum ImPosition {
    Double(f64),

    DoubleArray(Vec<f64>),
}
