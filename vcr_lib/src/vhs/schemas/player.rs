use serde::{Deserialize, Serialize};
use vhs_diff::{Diff, Patch};

#[derive(Diff, Patch, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Player {
    #[serde(rename = "_id")]
    pub id: Option<String>,

    pub a_baserunning_rating: Option<i64>,

    pub a_defense_rating: Option<i64>,

    pub a_hitting_rating: Option<i64>,

    pub anticapitalism: f64,

    pub a_pitching_rating: Option<i64>,

    pub armor: Option<String>,

    pub baserunning_rating: Option<f64>,

    pub base_thirst: f64,

    pub bat: Option<String>,

    pub blood: Option<i64>,

    pub buoyancy: f64,

    pub chasiness: f64,

    pub cinnamon: Option<f64>,

    pub coffee: Option<i64>,

    pub coldness: f64,

    pub consecutive_hits: Option<i64>,

    pub continuation: f64,

    pub deceased: Option<bool>,

    pub defense_rating: Option<f64>,

    pub divinity: f64,

    pub e_density: Option<f64>,

    pub evolution: Option<i64>,

    pub fate: Option<i64>,

    pub game_attr: Option<Vec<String>>,

    pub ground_friction: f64,

    pub hit_streak: Option<i64>,

    pub hitting_rating: Option<f64>,

    #[serde(rename = "id")]
    pub player_id: Option<String>,

    pub indulgence: f64,

    pub item_attr: Option<Vec<String>>,

    pub items: Option<Vec<ItemElement>>,

    pub laserlikeness: f64,

    pub league_team_id: Option<String>,

    pub martyrdom: f64,

    pub moxie: f64,

    pub musclitude: f64,

    pub name: String,

    pub omniscience: f64,

    pub overpowerment: f64,

    pub patheticism: f64,

    pub peanut_allergy: Option<bool>,

    pub perm_attr: Option<Vec<String>>,

    pub pitching_rating: Option<f64>,

    pub pressurization: f64,

    pub ritual: Option<String>,

    pub ruthlessness: f64,

    pub seas_attr: Option<Vec<String>>,

    pub shakespearianism: f64,

    pub soul: i64,

    pub spin: Option<i64>,

    pub state: Option<PlayerState>,

    pub suppression: f64,

    pub tenaciousness: f64,

    pub thwackability: f64,

    pub total_fingers: i64,

    pub tournament_team_id: Option<String>,

    pub tragicness: f64,

    pub unthwackability: f64,

    pub watchfulness: f64,

    pub week_attr: Option<Vec<String>>,
}

#[derive(PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ItemClass {
    pub baserunning_rating: Option<f64>,

    pub defense_rating: Option<f64>,

    pub durability: i64,

    pub forger: Option<serde_json::Value>,

    pub forger_name: Option<serde_json::Value>,

    pub health: i64,

    pub hitting_rating: Option<f64>,

    pub id: String,

    pub name: String,

    pub pitching_rating: Option<f64>,

    pub post_prefix: Option<PostPrefix>,

    pub prefixes: Option<Vec<Prefix>>,

    pub pre_prefix: Option<serde_json::Value>,

    pub root: Root,

    pub state: Option<ItemState>,

    pub suffix: Option<Suffix>,
}

#[derive(PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PostPrefix {
    pub adjustments: Vec<PostPrefixAdjustment>,

    pub name: String,
}

#[derive(PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PostPrefixAdjustment {
    #[serde(rename = "mod")]
    pub adjustment_mod: Option<String>,

    pub stat: Option<i64>,

    #[serde(rename = "type")]
    pub adjustment_type: i64,

    pub value: Option<f64>,
}

#[derive(PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Prefix {
    pub adjustments: Vec<PrefixAdjustment>,

    pub name: String,
}

#[derive(PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PrefixAdjustment {
    #[serde(rename = "mod")]
    pub adjustment_mod: Option<String>,

    pub stat: Option<i64>,

    #[serde(rename = "type")]
    pub adjustment_type: i64,

    pub value: Option<f64>,
}

#[derive(PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Root {
    pub adjustments: Vec<RootAdjustment>,

    pub name: String,
}

#[derive(PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct RootAdjustment {
    pub stat: i64,

    #[serde(rename = "type")]
    pub adjustment_type: i64,

    pub value: f64,
}

#[derive(PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ItemState {
    pub original: Option<String>,
}

#[derive(PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Suffix {
    pub adjustments: Vec<SuffixAdjustment>,

    pub name: String,
}

#[derive(PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SuffixAdjustment {
    #[serde(rename = "mod")]
    pub adjustment_mod: Option<String>,

    pub stat: Option<i64>,

    #[serde(rename = "type")]
    pub adjustment_type: i64,

    pub value: Option<f64>,
}

#[derive(PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PlayerState {
    pub elsewhere: Option<Elsewhere>,

    pub game_mod_sources: Option<GameModSources>,

    pub hunches: Option<Vec<i64>>,

    pub investigations: Option<i64>,

    pub item_mod_sources: Option<ItemModSources>,

    pub necromancied_this_election: Option<bool>,

    pub original: Option<String>,

    pub perm_mod_sources: Option<PermModSources>,

    pub pre_investigation_index: Option<i64>,

    pub pre_investigation_location: Option<i64>,

    pub pre_investigation_team: Option<String>,

    pub redacted: Option<bool>,

    pub seas_mod_sources: Option<SeasModSources>,

    pub unscattered_name: Option<String>,
}

#[derive(PartialEq, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Elsewhere {
    #[serde(rename = "day")]
    pub day: i64,

    #[serde(rename = "season")]
    pub season: i64,
}

#[derive(PartialEq, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GameModSources {
    #[serde(rename = "OVERPERFORMING")]
    pub overperforming: Vec<String>,
}

#[derive(PartialEq, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ItemModSources {
    #[serde(rename = "AMBITIOUS")]
    pub ambitious: Option<Vec<String>>,

    #[serde(rename = "BIRD_SEED")]
    pub bird_seed: Option<Vec<String>>,

    #[serde(rename = "BLASERUNNING")]
    pub blaserunning: Option<Vec<String>>,

    #[serde(rename = "CAREFUL")]
    pub careful: Option<Vec<String>>,

    #[serde(rename = "CHUNKY")]
    pub chunky: Option<Vec<String>>,

    #[serde(rename = "COASTING")]
    pub coasting: Option<Vec<String>>,

    #[serde(rename = "COFFEE_PERIL")]
    pub coffee_peril: Option<Vec<String>>,

    #[serde(rename = "CONTAINMENT")]
    pub containment: Option<Vec<String>>,

    #[serde(rename = "CURSE_OF_CROWS")]
    pub curse_of_crows: Option<Vec<String>>,

    #[serde(rename = "DOUBLE_PAYOUTS")]
    pub double_payouts: Option<Vec<String>>,

    #[serde(rename = "ENTANGLED")]
    pub entangled: Option<Vec<String>>,

    #[serde(rename = "EXTRA_STRIKE")]
    pub extra_strike: Option<Vec<String>>,

    #[serde(rename = "FIRE_EATER")]
    pub fire_eater: Option<Vec<String>>,

    #[serde(rename = "FIRE_PROTECTOR")]
    pub fire_protector: Option<Vec<String>>,

    #[serde(rename = "FIREPROOF")]
    pub fireproof: Option<Vec<String>>,

    #[serde(rename = "FLICKERING")]
    pub flickering: Option<Vec<String>>,

    #[serde(rename = "FLIICKERRRIIING")]
    pub fliickerrriiing: Option<Vec<String>>,

    #[serde(rename = "FORCE")]
    pub force: Option<Vec<String>>,

    #[serde(rename = "GRAVITY")]
    pub gravity: Option<Vec<String>>,

    #[serde(rename = "HIGH_PRESSURE")]
    pub high_pressure: Option<Vec<String>>,

    #[serde(rename = "MAXIMALIST")]
    pub maximalist: Option<Vec<String>>,

    #[serde(rename = "MINIMIZED")]
    pub minimized: Option<Vec<String>>,

    #[serde(rename = "NIGHT_VISION")]
    pub night_vision: Option<Vec<String>>,

    #[serde(rename = "OFFWORLD")]
    pub offworld: Option<Vec<String>>,

    #[serde(rename = "PARASITE")]
    pub parasite: Option<Vec<String>>,

    #[serde(rename = "PRO_SKATER")]
    pub pro_skater: Option<Vec<String>>,

    #[serde(rename = "REPEATING")]
    pub repeating: Option<Vec<String>>,

    #[serde(rename = "SMOOTH")]
    pub smooth: Option<Vec<String>>,

    #[serde(rename = "SOUNDPROOF")]
    pub soundproof: Option<Vec<String>>,

    #[serde(rename = "SPICY")]
    pub spicy: Option<Vec<String>>,

    #[serde(rename = "SQUIDDISH")]
    pub squiddish: Option<Vec<String>>,

    #[serde(rename = "STEELED")]
    pub steeled: Option<Vec<String>>,

    #[serde(rename = "SUBTRACTOR")]
    pub subtractor: Option<Vec<String>>,

    #[serde(rename = "SUPERWANDERER")]
    pub superwanderer: Option<Vec<String>>,

    #[serde(rename = "TRADER")]
    pub trader: Option<Vec<String>>,

    #[serde(rename = "TRAITOR")]
    pub traitor: Option<Vec<String>>,

    #[serde(rename = "TRAVELING")]
    pub traveling: Option<Vec<String>>,

    #[serde(rename = "UNAMBITIOUS")]
    pub unambitious: Option<Vec<String>>,

    #[serde(rename = "UNCERTAIN")]
    pub uncertain: Option<Vec<String>>,

    #[serde(rename = "UNDERHANDED")]
    pub underhanded: Option<Vec<String>>,
}

#[derive(PartialEq, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PermModSources {
    #[serde(rename = "AFFINITY_FOR_CROWS")]
    pub affinity_for_crows: Option<Vec<String>>,

    #[serde(rename = "ALTERNATE")]
    pub alternate: Option<Vec<String>>,

    #[serde(rename = "ATTRACTOR")]
    pub attractor: Option<Vec<String>>,

    #[serde(rename = "CAREFUL")]
    pub careful: Option<Vec<String>>,

    #[serde(rename = "CHUNKY")]
    pub chunky: Option<Vec<String>>,

    #[serde(rename = "CLUTTERED")]
    pub cluttered: Option<Vec<String>>,

    #[serde(rename = "CREDIT_TO_THE_TEAM")]
    pub credit_to_the_team: Option<Vec<String>>,

    #[serde(rename = "DEBT_THREE")]
    pub debt_three: Option<Vec<String>>,

    #[serde(rename = "DOUBLE_PAYOUTS")]
    pub double_payouts: Option<Vec<String>>,

    #[serde(rename = "EGO1")]
    pub ego1: Option<Vec<String>>,

    #[serde(rename = "EGO2")]
    pub ego2: Option<Vec<String>>,

    #[serde(rename = "EGO3")]
    pub ego3: Option<Vec<String>>,

    #[serde(rename = "EGO4")]
    pub ego4: Option<Vec<String>>,

    #[serde(rename = "ELSEWHERE")]
    pub elsewhere: Option<Vec<String>>,

    #[serde(rename = "FIRE_EATER")]
    pub fire_eater: Option<Vec<String>>,

    #[serde(rename = "FLINCH")]
    pub flinch: Option<Vec<String>>,

    #[serde(rename = "FRIEND_OF_CROWS")]
    pub friend_of_crows: Option<Vec<String>>,

    #[serde(rename = "GAUDY")]
    pub gaudy: Option<Vec<String>>,

    #[serde(rename = "HARD_BOILED")]
    pub hard_boiled: Option<Vec<String>>,

    #[serde(rename = "HOMEBODY")]
    pub homebody: Option<Vec<String>>,

    #[serde(rename = "HONEY_ROASTED")]
    pub honey_roasted: Option<Vec<String>>,

    #[serde(rename = "MAGMATIC")]
    pub magmatic: Option<Vec<String>>,

    #[serde(rename = "MINIMALIST")]
    pub minimalist: Option<Vec<String>>,

    #[serde(rename = "NEGATIVE")]
    pub negative: Option<Vec<String>>,

    #[serde(rename = "NON_IDOLIZED")]
    pub non_idolized: Option<Vec<String>>,

    #[serde(rename = "OVERPERFORMING")]
    pub overperforming: Option<Vec<String>>,

    #[serde(rename = "PERK")]
    pub perk: Option<Vec<String>>,

    #[serde(rename = "RETURNED")]
    pub returned: Option<Vec<String>>,

    #[serde(rename = "REVERBERATING")]
    pub reverberating: Option<Vec<String>>,

    #[serde(rename = "SCATTERED")]
    pub scattered: Option<Vec<String>>,

    #[serde(rename = "SCRAMBLED")]
    pub scrambled: Option<Vec<String>>,

    #[serde(rename = "SEEKER")]
    pub seeker: Option<Vec<String>>,

    #[serde(rename = "SHELLED")]
    pub shelled: Option<Vec<String>>,

    #[serde(rename = "SIPHON")]
    pub siphon: Option<Vec<String>>,

    #[serde(rename = "SKIPPING")]
    pub skipping: Option<Vec<String>>,

    #[serde(rename = "SMOOTH")]
    pub smooth: Option<Vec<String>>,

    #[serde(rename = "SPICY")]
    pub spicy: Option<Vec<String>>,

    #[serde(rename = "SQUIDDISH")]
    pub squiddish: Option<Vec<String>>,

    #[serde(rename = "SUPERALLERGIC")]
    pub superallergic: Option<Vec<String>>,

    #[serde(rename = "SUPERYUMMY")]
    pub superyummy: Option<Vec<String>>,

    #[serde(rename = "SWIM_BLADDER")]
    pub swim_bladder: Option<Vec<String>>,

    #[serde(rename = "TRIPLE_THREAT")]
    pub triple_threat: Option<Vec<String>>,

    #[serde(rename = "UNCERTAIN")]
    pub uncertain: Option<Vec<String>>,

    #[serde(rename = "UNDEFINED")]
    pub undefined: Option<Vec<String>>,

    #[serde(rename = "UNDERPERFORMING")]
    pub underperforming: Option<Vec<String>>,

    #[serde(rename = "UNDERTAKER")]
    pub undertaker: Option<Vec<String>>,

    #[serde(rename = "WALK_IN_THE_PARK")]
    pub walk_in_the_park: Option<Vec<String>>,

    #[serde(rename = "WANDERER")]
    pub wanderer: Option<Vec<String>>,

    #[serde(rename = "WILD")]
    pub wild: Option<Vec<String>>,

    #[serde(rename = "YOLKED")]
    pub yolked: Option<Vec<String>>,
}

#[derive(PartialEq, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SeasModSources {
    #[serde(rename = "ALTERNATE")]
    pub alternate: Option<Vec<String>>,

    #[serde(rename = "CHUNKY")]
    pub chunky: Option<Vec<String>>,

    #[serde(rename = "DEBT_THREE")]
    pub debt_three: Option<Vec<String>>,

    #[serde(rename = "EGO2")]
    pub ego2: Option<Vec<String>>,

    #[serde(rename = "ELSEWHERE")]
    pub elsewhere: Option<Vec<String>>,

    #[serde(rename = "FIRE_EATER")]
    pub fire_eater: Option<Vec<String>>,

    #[serde(rename = "FRIEND_OF_CROWS")]
    pub friend_of_crows: Option<Vec<String>>,

    #[serde(rename = "HONEY_ROASTED")]
    pub honey_roasted: Option<Vec<String>>,

    #[serde(rename = "MINIMALIST")]
    pub minimalist: Option<Vec<String>>,

    #[serde(rename = "OVERPERFORMING")]
    pub overperforming: Option<Vec<String>>,

    #[serde(rename = "PERK")]
    pub perk: Option<Vec<String>>,

    #[serde(rename = "RETURNED")]
    pub returned: Option<Vec<String>>,

    #[serde(rename = "SCATTERED")]
    pub scattered: Option<Vec<String>>,

    #[serde(rename = "SHELLED")]
    pub shelled: Option<Vec<String>>,

    #[serde(rename = "SIPHON")]
    pub siphon: Option<Vec<String>>,

    #[serde(rename = "SMOOTH")]
    pub smooth: Option<Vec<String>>,

    #[serde(rename = "SPICY")]
    pub spicy: Option<Vec<String>>,

    #[serde(rename = "SUPERALLERGIC")]
    pub superallergic: Option<Vec<String>>,

    #[serde(rename = "SUPERYUMMY")]
    pub superyummy: Option<Vec<String>>,

    #[serde(rename = "SWIM_BLADDER")]
    pub swim_bladder: Option<Vec<String>>,

    #[serde(rename = "TRIPLE_THREAT")]
    pub triple_threat: Option<Vec<String>>,

    #[serde(rename = "UNDERPERFORMING")]
    pub underperforming: Option<Vec<String>>,

    #[serde(rename = "WANDERER")]
    pub wanderer: Option<Vec<String>>,
}

#[derive(PartialEq, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ItemElement {
    ItemClass(ItemClass),

    String(String),
}
