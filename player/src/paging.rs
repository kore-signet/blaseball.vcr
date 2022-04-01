use blaseball_vcr::db_manager::DatabaseManager;
use blaseball_vcr::ChroniclerEntity;

type DynamicChronEntity = ChroniclerEntity<Box<dyn erased_serde::Serialize>>;

pub enum PageFetchParameters {
    Versions {
        before: u32,
        after: u32
    },
    Entities {
        at: u32
    }
}

pub struct Page {
    // ids still not fetched
    remaining_ids: Vec<[u8; 16]>,
    remaining_data: Vec<DynamicChronEntity>,
    parameters: PageFetchParameters
}

impl Page {
    // fetch a block of n entites
    #[inline]
    fn fetch_next_entities<T>(&mut self, db: &DatabaseManager, count: usize, at: u32) -> VCRResult<()> {
        let ids: Vec<[u8; 16]> = self.remaining_ids.drain(..count).collect();
        let mut data: Vec<Option<ChroniclerEntity<T>>> = db.get_entities(&ids[..], at)?;
        self.remaining_data.extend(data.into_iter().filter_map(|entity| {
            entity.map(ChroniclerEntity::erase)
        }));

        Ok(())
    }
}