mod err;
pub mod site;
#[macro_use]
pub mod utils;
pub mod feed;
pub use err::*;
pub mod db_manager;
pub mod vhs;

mod chron_types;
pub use chron_tpes::*;

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
    ) -> VCRResult<Option<Vec<Self::Record>>>;

    fn all_ids(&self) -> &[[u8; 16]];
}
