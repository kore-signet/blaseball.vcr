use serde::ser::{Serialize, Serializer, SerializeStruct};

pub struct ChroniclerEntity<T> {
    pub entity_id: [u8; 16],
    // this is a tiny optimization that saves a few bytes during paging:
    // chrono uses >12 bytes per datetime; we can get away with 4
    pub valid_from: u32,
    pub data: T
}

impl<T: Serialize> ChroniclerEntity<T> {
    #[inline(always)]
    pub fn erase(self) -> ChroniclerEntity<Box<dyn erased_serde::Serialize>> {
        ChroniclerEntity {
            entity_id: self.entity_id,
            valid_from: self.valid_from,
            data: self.data
        }
    }
}

impl<T: Serialize> Serialize for ChroniclerEntity<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let mut ser = serializer.serialize_struct("ChroniclerEntity", 5)?;
        ser.serialize_field("entityId", &self.entity_id)?;
        ser.serialize_field("validFrom", &self.valid_from)?;
        // we don't store these
        ser.serialize_field("validTo", ())?;
        ser.serialize_field("hash", "")?; // there's probably a way to add hashing here behind a compile feature - i'm not sure it's worth it, tho
        // -
        ser.serialize_field("data", &self.data)?;
        ser.finish()
    }
}