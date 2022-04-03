use crate::vhs::schemas::DynamicEntity;
use chrono::{DateTime, TimeZone, Utc};
use serde::ser::{SerializeStruct, Serializer};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub struct ChroniclerEntity<T> {
    pub entity_id: [u8; 16],
    // this is a tiny optimization that saves a few bytes during paging:
    // chrono uses >12 bytes per datetime; we can get away with 4
    pub valid_from: u32,
    pub data: T,
}

impl<T: Into<DynamicEntity>> ChroniclerEntity<T> {
    #[inline(always)]
    pub fn erase(self) -> ChroniclerEntity<DynamicEntity> {
        ChroniclerEntity {
            entity_id: self.entity_id,
            valid_from: self.valid_from,
            data: self.data.into(),
        }
    }
}

impl<T: Serialize> Serialize for ChroniclerEntity<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut ser = serializer.serialize_struct("ChroniclerEntity", 5)?;
        ser.serialize_field("entityId", &Uuid::from_bytes(self.entity_id))?;
        ser.serialize_field("validFrom", &Utc.timestamp(self.valid_from as i64, 0))?;
        // we don't store these
        ser.serialize_field("validTo", &())?;
        ser.serialize_field("hash", "")?; // there's probably a way to add hashing here behind a compile feature - i'm not sure it's worth it, tho
                                          // -
        ser.serialize_field("data", &self.data)?;
        ser.end()
    }
}

// as returned from the actual chron api
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RawChroniclerEntity<T> {
    pub entity_id: String,
    pub hash: String,
    pub valid_from: DateTime<Utc>,
    pub valid_to: Option<String>,
    pub data: T,
}
