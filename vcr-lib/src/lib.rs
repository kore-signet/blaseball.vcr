mod err;
pub mod site;
#[macro_use]
pub mod utils;
use std::path::Path;

pub use utils::*;
pub mod feed;
pub use err::*;
pub mod db_manager;
pub mod vhs;

mod chron_types;
pub use chron_types::*;

pub mod stream_data;

// use chrono::{DateTime, Utc};
// use rocket::FromFormField;
// use serde::{Deserialize, Serialize};
// use serde_json::Value as JSONValue;

pub type VCRResult<T> = Result<T, VCRError>;
pub type OptionalEntity<T> = Option<ChroniclerEntity<T>>;

pub trait EntityDatabase {
    type Record;

    fn from_single(path: impl AsRef<Path>) -> VCRResult<Self>
    where
        Self: Sized;

    fn get_entity(&self, id: &[u8; 16], at: i64) -> VCRResult<OptionalEntity<Self::Record>>;

    fn get_first_entity(&self, id: &[u8; 16]) -> VCRResult<OptionalEntity<Self::Record>>;

    fn get_first_entities(&self, ids: &[[u8; 16]]) -> VCRResult<Vec<OptionalEntity<Self::Record>>>;

    fn get_entities(
        &self,
        ids: &[[u8; 16]],
        at: i64,
    ) -> VCRResult<Vec<OptionalEntity<Self::Record>>> {
        ids.iter()
            .map(|id| self.get_entity(id, at))
            .collect::<VCRResult<Vec<OptionalEntity<Self::Record>>>>()
    }

    fn get_next_time(&self, id: &[u8; 16], at: i64) -> Option<i64>;

    fn get_versions(
        &self,
        id: &[u8; 16],
        before: i64,
        after: i64,
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

// hack so we can use call_method_by_type for Database::from_single
pub mod db_wrapper {
    use crate::vhs::db::Database;
    use crate::VCRResult;
    use crate::{db_manager::*, EntityDatabase};

    pub fn from_single_and_insert<
        T: Clone
            + vhs_diff::Patch
            + vhs_diff::Diff
            + serde::de::DeserializeOwned
            + Send
            + Sync
            + serde::Serialize
            + 'static,
    >(
        manager: &mut DatabaseManager,
        path: &std::path::Path,
    ) -> VCRResult<()> {
        let v: Database<T> = Database::from_single(path)?;
        manager.insert(v);
        Ok(())
    }
}

pub struct SliceReader<'a> {
    bytes: &'a [u8],
}

impl<'a> SliceReader<'a> {
    pub fn read_array<const N: usize>(&mut self) -> &'a [u8; N] {
        let (lhs, rhs) = self.bytes.split_at(N);
        self.bytes = rhs;

        unsafe { &*(lhs.as_ptr() as *const [u8; N]) }
    }

    pub fn read_slice(&mut self, len: usize) -> &'a [u8] {
        let (lhs, rhs) = self.bytes.split_at(len);
        self.bytes = rhs;
        lhs
    }

    pub fn read_str(&mut self) -> &'a str {
        let len = u16::from_le_bytes(*self.read_array::<2>());
        unsafe { std::str::from_utf8_unchecked(self.read_slice(len as usize)) }
    }

    pub fn read_varlen_slice(&mut self) -> &'a [u8] {
        let len = u16::from_le_bytes(*self.read_array::<2>());
        self.read_slice(len as usize)
    }
}

pub fn write_str(st: &str, out: &mut Vec<u8>) {
    out.extend_from_slice(&(st.len() as u16).to_le_bytes());
    out.extend_from_slice(st.as_bytes());
}

pub fn write_slice(st: &[u8], out: &mut Vec<u8>) {
    out.extend_from_slice(&(st.len() as u16).to_le_bytes());
    out.extend_from_slice(st);
}
