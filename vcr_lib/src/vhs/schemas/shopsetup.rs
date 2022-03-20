use serde::{Deserialize, Serialize};
use vhs_diff::{Diff, Patch};

#[derive(Diff, Patch, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Shopsetup {
    pub menu: Vec<String>,
    pub snack_data: SnackData,
}

#[derive(PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SnackData {
    pub black_hole_tiers: Vec<SnackTier>,

    pub consumer_tiers: Vec<SnackTier>,

    pub flood_clear_tiers: Vec<SnackTier>,

    pub idol_hits_tiers: Vec<SnackTier>,

    pub idol_homer_allowed_tiers: Vec<SnackTier>,

    pub idol_homers_tiers: Vec<SnackTier>,

    pub idol_pitcher_lose_tiers: Vec<SnackTier>,

    pub idol_pitcher_win_tiers: Vec<SnackTier>,

    pub idol_shutouts_tiers: Vec<SnackTier>,

    pub idol_steal_tiers: Vec<SnackTier>,

    pub idol_strikeouts_tiers: Vec<SnackTier>,

    pub incineration_tiers: Vec<SnackTier>,

    pub max_bet_tiers: Vec<SnackTier>,

    pub sun_two_tiers: Vec<SnackTier>,

    pub team_loss_coin_tiers: Vec<SnackTier>,

    pub team_shamed_tiers: Vec<SnackTier>,

    pub team_shaming_tiers: Vec<SnackTier>,

    pub team_win_coin_tiers: Vec<SnackTier>,

    pub time_off_tiers: Vec<SnackTier>,
}

#[derive(PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SnackTier {
    pub amount: i64,

    pub price: i64,
}
