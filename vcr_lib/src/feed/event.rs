use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value as JSONValue;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FeedEvent {
    pub id: Uuid,
    pub category: i8,
    pub created: DateTime<Utc>,
    pub day: i16,
    pub description: String,
    #[serde(default)]
    pub nuts: u16,
    pub phase: u8,
    pub player_tags: Option<Vec<Uuid>>,
    pub game_tags: Option<Vec<Uuid>>,
    pub team_tags: Option<Vec<Uuid>>,
    #[serde(rename = "type")]
    pub etype: i16,
    pub tournament: i8,
    pub season: i8,
    #[serde(default)]
    pub metadata: JSONValue,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CompactedFeedEvent {
    pub id: Uuid,
    pub category: i8,
    pub day: i16,
    pub description: String,
    #[serde(default)]
    pub player_tags: Vec<u16>,
    pub game_tags: Vec<u16>,
    pub team_tags: Vec<u8>,
    #[serde(rename = "type")]
    pub etype: i16,
    pub tournament: i8,
    #[serde(default)]
    pub metadata: JSONValue,
}

impl FeedEvent {
    pub fn generate_id(&self) -> Vec<u8> {
        [
            self.season.to_be_bytes().to_vec(),
            self.phase.to_be_bytes().to_vec(),
            (self.created.timestamp() as u32).to_be_bytes().to_vec(),
            self.id.as_bytes()[0..2].to_vec(),
        ]
        .concat()
    }
}

impl CompactedFeedEvent {
    pub fn encode(&self) -> Vec<u8> {
        let player_tag_bytes: Vec<u8> = {
            let player_tags: Vec<u8> = self
                .player_tags
                .iter()
                .map(|id| id.to_be_bytes())
                .flatten()
                .collect();
            [
                (self.player_tags.len() as u8).to_be_bytes().to_vec(),
                player_tags,
            ]
            .concat()
        };

        let team_tag_bytes: Vec<u8> = {
            let team_tags: Vec<u8> = self
                .team_tags
                .iter()
                .map(|id| id.to_be_bytes())
                .flatten()
                .collect();
            [
                (self.team_tags.len() as u8).to_be_bytes().to_vec(),
                team_tags,
            ]
            .concat()
        };

        let game_tag_bytes: Vec<u8> = {
            let game_tags: Vec<u8> = self
                .game_tags
                .iter()
                .map(|id| id.to_be_bytes())
                .flatten()
                .collect();
            [
                (self.game_tags.len() as u8).to_be_bytes().to_vec(),
                game_tags,
            ]
            .concat()
        };

        let description_bytes = {
            let description = self.description.as_bytes().to_vec();
            [
                (description.len() as u16).to_be_bytes().to_vec(),
                description,
            ]
            .concat()
        };

        [
            self.id.as_bytes().to_vec(),
            self.category.to_be_bytes().to_vec(),
            self.etype.to_be_bytes().to_vec(),
            self.day.to_be_bytes().to_vec(),
            self.tournament.to_be_bytes().to_vec(),
            description_bytes,
            player_tag_bytes,
            team_tag_bytes,
            game_tag_bytes,
            rmp_serde::to_vec(&self.metadata).unwrap(),
        ]
        .concat()
    }
}
