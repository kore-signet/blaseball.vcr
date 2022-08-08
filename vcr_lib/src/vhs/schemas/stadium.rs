use serde::{Deserialize, Serialize};
use uuid::Uuid;
use vhs_diff::{Diff, Patch};
use crate::UuidShell;

#[derive(Diff, Patch, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Stadium {
    pub birds: i64,
    pub elongation: f64,
    pub filthiness: f64,
    pub fortification: f64,
    pub forwardness: f64,
    pub grandiosity: f64,
    pub hype: f64,
    pub id: Uuid,
    pub inconvenience: f64,
    pub luxuriousness: i64,
    pub main_color: String,
    pub model: Option<i64>,
    pub mods: Vec<String>,
    pub mysticism: f64,
    pub name: String,
    pub nickname: String,
    pub obtuseness: f64,
    pub ominousness: f64,
    pub reno_cost: i64,
    pub reno_discard: Vec<String>,
    pub reno_hand: Vec<String>,
    pub reno_log: RenoLog,
    pub secondary_color: String,
    pub state: State,
    pub team_id: UuidShell,
    pub tertiary_color: String,
    pub viscosity: f64,
    pub weather: Weather,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct RenoLog {
    #[serde(rename = "air_balloons_mod")]
    air_balloons_mod: Option<i64>,
    #[serde(rename = "anti_flood_pumps_mod")]
    anti_flood_pumps_mod: Option<i64>,
    #[serde(rename = "anti_graphene_mod")]
    anti_graphene_mod: Option<i64>,
    #[serde(rename = "big_bucket_mod")]
    big_bucket_mod: Option<i64>,
    #[serde(rename = "bird_hotel_mod")]
    bird_hotel_mod: Option<i64>,
    #[serde(rename = "birdhouses_mod")]
    birdhouses_mod: Option<i64>,
    #[serde(rename = "climate_control_mod")]
    climate_control_mod: Option<i64>,
    #[serde(rename = "condensed_floor_plan_mod")]
    condensed_floor_plan_mod: Option<i64>,
    #[serde(rename = "cycling_mod")]
    cycling_mod: Option<i64>,
    #[serde(rename = "echo_chamber_mod")]
    echo_chamber_mod: Option<i64>,
    #[serde(rename = "event_horizon_mod")]
    event_horizon_mod: Option<i64>,
    #[serde(rename = "fax_machine_mod")]
    fax_machine_mod: Option<i64>,
    #[serde(rename = "fire_insurance_mod")]
    fire_insurance_mod: Option<i64>,
    #[serde(rename = "flood_balloons_mod")]
    flood_balloons_mod: Option<i64>,
    #[serde(rename = "flood_pumps_mod")]
    flood_pumps_mod: Option<i64>,
    #[serde(rename = "graphene_mod")]
    graphene_mod: Option<i64>,
    #[serde(rename = "grind_rail_mod")]
    grind_rail_mod: Option<i64>,
    #[serde(rename = "hoops_mod")]
    hoops_mod: Option<i64>,
    #[serde(rename = "hot_air_balloons_mod")]
    hot_air_balloons_mod: Option<i64>,
    #[serde(rename = "hotel_motel_mod")]
    hotel_motel_mod: Option<i64>,
    #[serde(rename = "light_switch_toggle")]
    light_switch_toggle: Option<i64>,
    #[serde(rename = "open_floor_plan_mod")]
    open_floor_plan_mod: Option<i64>,
    #[serde(rename = "peanut_mister_mod")]
    peanut_mister_mod: Option<i64>,
    #[serde(rename = "psychoacoustics_mod")]
    psychoacoustics_mod: Option<i64>,
    #[serde(rename = "salmon_cannons_mod")]
    salmon_cannons_mod: Option<i64>,
    #[serde(rename = "secret_base_mod")]
    secret_base_mod: Option<i64>,
    #[serde(rename = "secret_tunnels_mod")]
    secret_tunnels_mod: Option<i64>,
    #[serde(rename = "solar_panels_mod")]
    solar_panels_mod: Option<i64>,
    #[serde(rename = "soundsystem_mod")]
    soundsystem_mod: Option<i64>,
    #[serde(rename = "stables_mod")]
    stables_mod: Option<i64>,
    #[serde(rename = "sweetener_mod")]
    sweetener_mod: Option<i64>,
    // what the actual fuck
    #[serde(rename = "sweeupdate stadiums seon_cannons_mod")]
    sweeupdate_stadiums_seon_cannons_mod: Option<i64>,
    #[serde(rename = "thieves_guild_mod")]
    thieves_guild_mod: Option<i64>,
    #[serde(rename = "turntables_mod")]
    turntables_mod: Option<i64>,
    #[serde(rename = "very_foul_balls_mod")]
    very_foul_balls_mod: Option<i64>,
    #[serde(rename = "voicemail_mod")]
    voicemail_mod: Option<i64>,
    #[serde(rename = "weather_reports_mod")]
    weather_reports_mod: Option<i64>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct State {
    air_balloons: Option<i64>,
    #[serde(rename = "eventHorizon")]
    event_horizon: Option<bool>,
    #[serde(rename = "faxMachine")]
    fax_machine: Option<FaxMachine>,
    flood_balloons: Option<i64>,
    #[serde(rename = "solarPanels")]
    solar_panels: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct FaxMachine {
    runs_needed: f64,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
// lol. lmao
pub struct Weather {
    #[serde(rename = "1")]
    the_1: Option<i64>,

    #[serde(rename = "10")]
    the_10: Option<i64>,

    #[serde(rename = "11")]
    the_11: Option<i64>,

    #[serde(rename = "13")]
    the_13: Option<i64>,

    #[serde(rename = "14")]
    the_14: Option<i64>,

    #[serde(rename = "15")]
    the_15: Option<i64>,

    #[serde(rename = "16")]
    the_16: Option<i64>,

    #[serde(rename = "17")]
    the_17: Option<i64>,

    #[serde(rename = "18")]
    the_18: Option<i64>,

    #[serde(rename = "19")]
    the_19: Option<i64>,

    #[serde(rename = "29")]
    the_29: Option<i64>,

    #[serde(rename = "7")]
    the_7: Option<i64>,

    #[serde(rename = "8")]
    the_8: Option<i64>,

    #[serde(rename = "9")]
    the_9: Option<i64>,
}
