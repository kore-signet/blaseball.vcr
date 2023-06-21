use vcr_lookups::{GAME_ID_TABLE, PLAYER_ID_TABLE, TEAM_ID_TABLE};

use std::ops::Deref;
use uuid::Uuid;

use crate::timestamp_from_millis;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FeedEvent {
    pub id: Uuid,
    pub category: i8,
    pub created: i64,
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
    pub metadata: Option<Box<serde_json::value::RawValue>>,
}

#[derive(rkyv::Archive, rkyv::Serialize)]
#[archive_attr(derive(Debug))]
pub struct CompactedFeedEvent {
    pub id: Option<Uuid>,
    pub category: i8,
    pub day: u8,
    pub description: String,
    pub player_tags: Vec<u16>,
    pub game_tags: Vec<u16>,
    pub team_tags: Vec<u16>,
    pub etype: i16,
    pub tournament: i8,
    pub metadata: Option<String>,
    pub season: i8,
    pub phase: u8,
}

impl CompactedFeedEvent {
    pub fn convert(ev: FeedEvent) -> CompactedFeedEvent {
        CompactedFeedEvent {
            id: if ev.phase == 13 { Some(ev.id) } else { None },
            category: ev.category,
            day: ev.day.try_into().unwrap_or(255),
            etype: ev.etype,
            tournament: ev.tournament,
            season: ev.season,
            phase: ev.phase,
            description: ev.description,
            metadata: ev.metadata.map(|v| v.get().to_owned()),
            player_tags: ev
                .player_tags
                .unwrap_or_default()
                .into_iter()
                .map(|id| PLAYER_ID_TABLE.mapper[&id] as u16)
                .collect(),
            game_tags: ev
                .game_tags
                .unwrap_or_default()
                .into_iter()
                .map(|id| match GAME_ID_TABLE.map(&id) {
                    Some(v) => *v as u16,
                    None => panic!("{}", id),
                })
                .collect(),
            team_tags: ev
                .team_tags
                .unwrap_or_default()
                .into_iter()
                .map(|id| TEAM_ID_TABLE.mapper[&id] as u16)
                .collect(),
        }
    }
}

// a simple wrapper around a the archived version that allows for serialization, and adds the event timestamp timestamp
pub struct ArchivedEventWithTimestamp<'a> {
    pub(crate) created: i64,
    inner: &'a ArchivedCompactedFeedEvent,
}

impl<'a> ArchivedEventWithTimestamp<'a> {
    pub fn new(
        created: i64,
        inner: &'a ArchivedCompactedFeedEvent,
    ) -> ArchivedEventWithTimestamp<'a> {
        ArchivedEventWithTimestamp { created, inner }
    }
}

impl<'a> Deref for ArchivedEventWithTimestamp<'a> {
    type Target = ArchivedCompactedFeedEvent;

    fn deref(&self) -> &Self::Target {
        self.inner
    }
}

impl<'a> serde::Serialize for ArchivedEventWithTimestamp<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use rkyv::rend::LittleEndian;
        use serde::ser::{Serialize, SerializeSeq, SerializeStruct, Serializer};

        #[repr(transparent)]
        struct RawValueWrapper<'a>(&'a str);

        impl<'a> Serialize for RawValueWrapper<'a> {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                let mut s = serializer.serialize_struct("$serde_json::private::RawValue", 1)?;
                s.serialize_field("$serde_json::private::RawValue", &self.0)?;
                s.end()
            }
        }

        #[repr(transparent)]
        struct PlayerTags<'a>(&'a [LittleEndian<u16>]);

        impl<'a> Serialize for PlayerTags<'a> {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                let mut s = serializer.serialize_seq(Some(self.0.len()))?;
                for v in self.0 {
                    s.serialize_element(&PLAYER_ID_TABLE.inverter[v.value() as usize])?;
                }
                s.end()
            }
        }

        #[repr(transparent)]
        struct GameTags<'a>(&'a [LittleEndian<u16>]);

        impl<'a> Serialize for GameTags<'a> {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                let mut s = serializer.serialize_seq(Some(self.0.len()))?;
                for v in self.0 {
                    s.serialize_element(&GAME_ID_TABLE.inverter[v.value() as usize])?;
                }
                s.end()
            }
        }

        #[repr(transparent)]
        struct TeamTags<'a>(&'a [LittleEndian<u16>]);

        impl<'a> Serialize for TeamTags<'a> {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                let mut s = serializer.serialize_seq(Some(self.0.len()))?;
                for v in self.0 {
                    s.serialize_element(&TEAM_ID_TABLE.inverter[v.value() as usize])?;
                }
                s.end()
            }
        }

        let mut event = serializer.serialize_struct("CompactedFeedEvent", 13)?;
        event.serialize_field("id", self.id.as_ref().unwrap_or(&Uuid::nil()))?;
        event.serialize_field("created", &timestamp_from_millis(self.created))?;
        event.serialize_field("category", &self.category)?;
        event.serialize_field(
            "day",
            &(if self.day == 255 {
                1522i16
            } else {
                self.day.into()
            }),
        )?;
        event.serialize_field("type", &self.etype.value())?;
        event.serialize_field("season", &self.season)?;
        event.serialize_field("phase", &self.phase)?;
        event.serialize_field("tournament", &self.tournament)?;
        event.serialize_field("description", self.description.as_str())?;
        event.serialize_field(
            "metadata",
            &self.metadata.as_ref().map(|m| RawValueWrapper(m.as_str())),
        )?;
        event.serialize_field("playerTags", &PlayerTags(self.player_tags.as_slice()))?;
        event.serialize_field("gameTags", &GameTags(self.game_tags.as_slice()))?;
        event.serialize_field("teamTags", &TeamTags(self.team_tags.as_slice()))?;
        event.end()
    }
}
