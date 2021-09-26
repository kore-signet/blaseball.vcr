use super::EventDescription;
use crate::utils::encode_varint;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value as JSONValue;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
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
    pub season: u8,
    #[serde(default)]
    pub metadata: JSONValue,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CompactedFeedEvent {
    pub id: Uuid,
    pub created: DateTime<Utc>,
    pub category: i8,
    pub day: u8,
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
    pub season: u8,
    pub phase: u8,
}

impl FeedEvent {
    pub fn generate_id(&self, millis_epoch: Option<u32>) -> Vec<u8> {
        let timestamp = match millis_epoch {
            Some(epoch) => {
                let epoch = (epoch as i64) * 1000;
                (self.created.timestamp_millis() - epoch) as u32
            }
            None => self.created.timestamp() as u32,
        };
        [
            self.season.to_be_bytes().to_vec(),
            self.phase.to_be_bytes().to_vec(),
            timestamp.to_be_bytes().to_vec(),
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
            match EventDescription::from_type(self.etype) {
                EventDescription::Constant(s) => {
                    assert_eq!(self.description, s);
                    vec![]
                }
                EventDescription::ConstantVariant(possibilities) => {
                    vec![(possibilities
                        .iter()
                        .position(|&d| d == self.description)
                        .unwrap_or_else(|| panic!("{}", self.etype))
                        as u8)
                        .to_be()]
                }
                EventDescription::Suffix(s) => {
                    let description = self
                        .description
                        .strip_suffix(s)
                        .unwrap()
                        .as_bytes()
                        .to_vec();
                    [encode_varint(description.len() as u16), description].concat()
                }
                EventDescription::Prefix(s) => {
                    let description = self
                        .description
                        .strip_prefix(s)
                        .unwrap()
                        .as_bytes()
                        .to_vec();
                    [encode_varint(description.len() as u16), description].concat()
                }
                EventDescription::Variable => {
                    let description = self.description.as_bytes().to_vec();
                    [encode_varint(description.len() as u16), description].concat()
                }
            }
        };

        [
            self.category.to_be_bytes().to_vec(),
            self.etype.to_be_bytes().to_vec(),
            self.day.to_be_bytes().to_vec(),
            ((self.season - 10) | (self.phase << 4))
                .to_be_bytes()
                .to_vec(),
            if self.phase == 13 {
                self.id.as_bytes().to_vec()
            } else {
                vec![]
            },
            description_bytes,
            player_tag_bytes,
            team_tag_bytes,
            game_tag_bytes,
            rmp_serde::to_vec(&self.metadata).unwrap(),
        ]
        .concat()
    }
}
