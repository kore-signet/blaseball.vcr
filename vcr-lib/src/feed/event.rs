use arrayvec::ArrayVec;
use bytemuck::bytes_of;
use serde::ser::SerializeStruct;

use vcr_lookups::{UuidShell, UuidTag, GAME_ID_TABLE, PLAYER_ID_TABLE, TEAM_ID_TABLE};

use uuid::Uuid;

use crate::{timestamp_from_millis, write_slice, write_str, SliceReader};

use super::BlockMetadata;

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
    pub metadata: Option<rmpv::Value>,
}

pub struct EncodedFeedEvent<'a> {
    pub created: i64,
    block: &'a BlockMetadata,
    bytes: &'a [u8],
}

impl<'a> EncodedFeedEvent<'a> {
    pub fn new(created: i64, bytes: &'a [u8], block: &'a BlockMetadata) -> EncodedFeedEvent<'a> {
        EncodedFeedEvent {
            created,
            block,
            bytes,
        }
    }
}

impl<'a> serde::Serialize for EncodedFeedEvent<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut event = serializer.serialize_struct("CompactedFeedEvent", 13)?;
        let mut reader = SliceReader { bytes: self.bytes };

        if self.block.phase == 13 {
            event.serialize_field("id", Uuid::from_bytes_ref(reader.read_array::<16>()))?;
        } else {
            event.serialize_field("id", &Uuid::nil())?;
        };

        event.serialize_field("created", &timestamp_from_millis(self.created))?;

        let metadata: EventMetadata = bytemuck::cast(*reader.read_array::<4>());

        let category = metadata.get(EventMetadata::CATEGORY);
        let category = if category == 0b111 {
            -1i8
        } else {
            category as i8
        };

        event.serialize_field("category", &category)?;
        let day = metadata.get(EventMetadata::DAY);
        event.serialize_field("day", &(if day == 255 { 1522 } else { day }))?;

        let event_type: i16 = metadata.get(EventMetadata::EVENT_TYPE).try_into().unwrap();
        event.serialize_field("type", &(event_type - 1))?;

        event.serialize_field("season", &self.block.season)?;
        event.serialize_field("phase", &self.block.phase)?;
        event.serialize_field("tournament", &self.block.tournament)?;

        let mut team_tags: ArrayVec<UuidShell, 24> = ArrayVec::new();
        for _ in 0..TEAM_TAG_LENS[metadata.get(EventMetadata::TEAM_TAGS_LEN) as usize] {
            team_tags.push(UuidShell::Tagged(UuidTag::Team(u16::from_le_bytes(
                *reader.read_array::<2>(),
            ))));
        }

        let mut player_tags: ArrayVec<UuidShell, 55> = ArrayVec::new();
        for _ in 0..PLAYER_TAG_LENS[metadata.get(EventMetadata::PLAYER_TAGS_LEN) as usize] {
            player_tags.push(UuidShell::Tagged(UuidTag::Player(u16::from_le_bytes(
                *reader.read_array::<2>(),
            ))));
        }

        let mut game_tags: ArrayVec<UuidShell, 1> = ArrayVec::new();
        if dbg!(metadata.get(EventMetadata::HAS_GAME_TAG)) {
            game_tags.push(UuidShell::Tagged(UuidTag::Game(u16::from_le_bytes(
                *reader.read_array::<2>(),
            ))));
        }

        event.serialize_field("teamTags", &team_tags)?;
        event.serialize_field("gameTags", &game_tags)?;
        event.serialize_field("playerTags", &player_tags)?;

        event.serialize_field("description", &reader.read_str())?;

        let mut metadata = reader.read_varlen_slice();
        if !metadata.is_empty() {
            let metadata_val = rmpv::decode::read_value(&mut metadata).unwrap();
            event.serialize_field("metadata", &metadata_val)?;
        } else {
            event.serialize_field("metadata", &None::<()>)?;
        }

        event.end()
    }
}

mycelium_bitfield::bitfield! {
    #[derive(bytemuck::Pod, bytemuck::Zeroable)]
    pub struct EventMetadata<u32> {
        pub const PLAYER_TAGS_LEN = 5;
        pub const TEAM_TAGS_LEN = 4;
        pub const HAS_GAME_TAG: bool;
        pub const EVENT_TYPE = 9;
        pub const CATEGORY = 3;
        pub const DAY = 9;
    }
}

pub const TEAM_TAG_LENS: [usize; 7] = [0, 1, 2, 3, 4, 6, 24];
pub const PLAYER_TAG_LENS: [usize; 17] =
    [0, 1, 2, 3, 4, 5, 6, 8, 10, 13, 15, 21, 24, 27, 30, 35, 55];

pub fn encode_event(event: FeedEvent) -> Vec<u8> {
    let mut out = Vec::new();

    if event.phase == 13 {
        out.extend_from_slice(event.id.as_bytes());
    }

    let wrapped_day: u8 = event.day.try_into().unwrap_or(255);
    let wrapped_ty: u32 = (event.etype + 1).try_into().unwrap();
    let wrapped_category: u8 = event.category.try_into().unwrap_or(0b111);

    let metadata = EventMetadata::new()
        .with(EventMetadata::CATEGORY, wrapped_category as u32)
        .with(
            EventMetadata::TEAM_TAGS_LEN,
            TEAM_TAG_LENS
                .binary_search(&event.team_tags.as_ref().map_or(0, |v| v.len()))
                .unwrap() as u32,
        )
        .with(
            EventMetadata::PLAYER_TAGS_LEN,
            PLAYER_TAG_LENS
                .binary_search(&event.player_tags.as_ref().map_or(0, |v| v.len()))
                .unwrap() as u32,
        )
        .with(
            EventMetadata::HAS_GAME_TAG,
            event.game_tags.as_ref().map_or(false, |v| !v.is_empty()),
        )
        .with(EventMetadata::DAY, wrapped_day as u32)
        .with(EventMetadata::EVENT_TYPE, wrapped_ty);

    assert_eq!(
        TEAM_TAG_LENS[metadata.get(EventMetadata::TEAM_TAGS_LEN) as usize],
        event.team_tags.as_deref().unwrap_or(&[]).len()
    );
    assert_eq!(
        PLAYER_TAG_LENS[metadata.get(EventMetadata::PLAYER_TAGS_LEN) as usize],
        event.player_tags.as_deref().unwrap_or(&[]).len()
    );

    out.extend_from_slice(bytes_of(&metadata));

    for team_tag in event.team_tags.as_deref().unwrap_or(&[]) {
        out.extend_from_slice(&TEAM_ID_TABLE.map(team_tag).unwrap().to_le_bytes())
    }

    for player_tag in event.player_tags.as_deref().unwrap_or(&[]) {
        out.extend_from_slice(&PLAYER_ID_TABLE.map(player_tag).unwrap().to_le_bytes())
    }

    if let Some(&[game_tag]) = event.game_tags.as_deref() {
        out.extend_from_slice(&GAME_ID_TABLE.map(&game_tag).unwrap().to_le_bytes());
    }

    write_str(&event.description, &mut out);

    let mut metadata_bytes = Vec::new();
    if let Some(metadata) = event.metadata {
        rmpv::encode::write_value(&mut metadata_bytes, &metadata).unwrap();
    }

    write_slice(&metadata_bytes, &mut out);

    out
}
