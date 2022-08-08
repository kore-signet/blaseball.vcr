use crate::feed::lookup_tables::*;
use modular_bitfield::prelude::*;
use serde::{
    de::{self, Deserialize, IntoDeserializer, Visitor},
    Serialize, Serializer,
};
use std::hash::Hash;
use uuid::Uuid;

#[derive(Clone, Debug, Copy)]
pub enum UuidShell {
    RawUuid(Uuid),
    Tagged(UuidTag),
}

impl PartialEq for UuidShell {
    fn eq(&self, other: &Self) -> bool {
        self.as_uuid() == other.as_uuid()
    }
}

impl Eq for UuidShell {}

impl Hash for UuidShell {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.as_uuid().hash(state)
    }
}

impl UuidShell {
    pub fn find_tag(self) -> UuidShell {
        match self {
            Self::RawUuid(id) => {
                let v = id.as_bytes();
                if let Some(tag) = UUID_TO_PLAYER.get(v) {
                    UuidShell::Tagged(UuidTag::Player(*tag))
                } else if let Some(tag) = UUID_TO_GAME.get(v) {
                    UuidShell::Tagged(UuidTag::Game(*tag))
                } else if let Some(tag) = UUID_TO_TEAM.get(v) {
                    UuidShell::Tagged(UuidTag::Team(*tag))
                } else {
                    UuidShell::RawUuid(id)
                }
            }
            Self::Tagged(_) => self,
        }
    }

    pub fn as_uuid(&self) -> &Uuid {
        match self {
            UuidShell::RawUuid(id) => id,
            UuidShell::Tagged(v) => v.as_uuid(),
        }
    }
}

impl Serialize for UuidShell {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // if it's human readable, we serialize it as a formatted string
        if serializer.is_human_readable() {
            let mut encode_buffer = Uuid::encode_buffer();
            return serializer.serialize_str(
                self.as_uuid()
                    .as_hyphenated()
                    .encode_lower(&mut encode_buffer),
            );
        }

        // if it's not human readable..
        match self {
            UuidShell::RawUuid(tag) => tag.serialize(serializer),
            UuidShell::Tagged(tag) => tag.serialize(serializer),
        }
    }
}

impl<'de> Deserialize<'de> for UuidShell {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let id = if deserializer.is_human_readable() {
            deserializer.deserialize_str(UuidShellVisitor)?
        } else {
            deserializer.deserialize_any(UuidShellVisitor)?
        };

        Ok(id.find_tag())
    }
}

#[derive(Copy, Clone, Debug)]
pub enum UuidTag {
    Team(u8),
    Player(u16),
    Game(u16),
}

impl UuidTag {
    pub fn as_uuid(&self) -> &Uuid {
        (match self {
            UuidTag::Team(val) => TEAM_TO_UUID.get((*val) as usize),
            UuidTag::Player(val) => PLAYER_TO_UUID.get((*val) as usize),
            UuidTag::Game(val) => GAME_TO_UUID.get((*val) as usize),
        })
        .unwrap()
    }
}

impl Serialize for UuidTag {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            UuidTag::Team(val) => serializer.serialize_u8(*val),
            UuidTag::Player(val) => serializer.serialize_u16(u16::from_ne_bytes(
                PackedTag::new()
                    .with_tag_val(*val)
                    .with_kind(PackedTagKind::Player)
                    .into_bytes(),
            )),
            UuidTag::Game(val) => serializer.serialize_u16(u16::from_ne_bytes(
                PackedTag::new()
                    .with_tag_val(*val)
                    .with_kind(PackedTagKind::Game)
                    .into_bytes(),
            )),
        }
    }
}

#[bitfield]
struct PackedTag {
    tag_val: B15,
    kind: PackedTagKind,
}

#[derive(BitfieldSpecifier)]
#[bits = 1]
enum PackedTagKind {
    Player,
    Game,
}

struct UuidShellVisitor;

impl<'de> Visitor<'de> for UuidShellVisitor {
    type Value = UuidShell;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a uuid tag (either u8 or u16)")
    }

    fn visit_u8<E>(self, value: u8) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(UuidShell::Tagged(UuidTag::Team(value)))
    }

    fn visit_u16<E>(self, value: u16) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let tag = PackedTag::from_bytes(value.to_ne_bytes());

        Ok(UuidShell::Tagged(match tag.kind() {
            PackedTagKind::Player => UuidTag::Player(tag.tag_val()),
            PackedTagKind::Game => UuidTag::Game(tag.tag_val()),
        }))
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        value
            .parse::<Uuid>()
            .map(|v| UuidShell::RawUuid(v))
            .map_err(|e| E::custom(e))
    }

    fn visit_bytes<E: de::Error>(self, value: &[u8]) -> Result<Self::Value, E> {
        Uuid::from_slice(value)
            .map(|v| UuidShell::RawUuid(v))
            .map_err(|e| E::custom(e))
    }
}

// thanks https://github.com/serde-rs/serde/issues/1425#issuecomment-462282398 !
pub fn empty_string_as_none<'de, D, T>(de: D) -> Result<Option<T>, D::Error>
where
    D: serde::Deserializer<'de>,
    T: serde::Deserialize<'de>,
{
    if de.is_human_readable() {
        let opt = Option::<String>::deserialize(de)?;
        let opt = opt.as_ref().map(String::as_str);
        match opt {
            None | Some("") => Ok(None),
            Some(s) => T::deserialize(s.into_deserializer()).map(Some),
        }
    } else {
        Option::<T>::deserialize(de)
    }
}
