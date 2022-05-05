mod err;
pub mod site;
#[macro_use]
pub mod utils;
pub mod feed;
pub use err::*;
pub mod db_manager;
pub mod vhs;
#[rustfmt::skip]
pub mod game_lookup_tables;

mod chron_types;
pub use chron_types::*;

// use chrono::{DateTime, Utc};
// use rocket::FromFormField;
// use serde::{Deserialize, Serialize};
// use serde_json::Value as JSONValue;

pub type VCRResult<T> = Result<T, VCRError>;
pub type OptionalEntity<T> = Option<ChroniclerEntity<T>>;

pub trait EntityDatabase {
    type Record;

    fn get_entity(&self, id: &[u8; 16], at: u32) -> VCRResult<OptionalEntity<Self::Record>>;

    fn get_entities(
        &self,
        ids: &[[u8; 16]],
        at: u32,
    ) -> VCRResult<Vec<OptionalEntity<Self::Record>>> {
        ids.iter()
            .map(|id| self.get_entity(id, at))
            .collect::<VCRResult<Vec<OptionalEntity<Self::Record>>>>()
    }

    fn get_versions(
        &self,
        id: &[u8; 16],
        before: u32,
        after: u32,
    ) -> VCRResult<Option<Vec<ChroniclerEntity<Self::Record>>>>;

    fn all_ids(&self) -> &[[u8; 16]];

    fn as_any(&self) -> &dyn std::any::Any;
}

pub struct GameDate {
    pub day: i16,
    pub season: i8,
    pub tournament: i8,
}

impl GameDate {
    pub const fn to_bytes(&self) -> [u8; 4] {
        let [day_a, day_b] = self.day.to_le_bytes();
        [
            day_a,
            day_b,
            self.season.to_le_bytes()[0],
            self.tournament.to_le_bytes()[0],
        ]
    }

    pub const fn from_bytes([day_a, day_b, season, tournament]: [u8; 4]) -> GameDate {
        GameDate {
            day: i16::from_le_bytes([day_a, day_b]),
            season: i8::from_le_bytes([season]),
            tournament: i8::from_le_bytes([tournament]),
        }
    }
}
