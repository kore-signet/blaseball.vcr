use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value as JSONValue;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FeedEvent {
    category: i16,
    created: DateTime<Utc>,
    day: u16,
    description: String,
    #[serde(default)]
    nuts: u16,
    phase: u8,
    player_tags: Option<Vec<String>>,
    game_tags: Option<Vec<String>>,
    team_tags: Option<Vec<String>>,
    #[serde(rename = "type")]
    etype: i16,
    tournament: i8,
    season: i16,
    #[serde(default)]
    metadata: JSONValue,
}
