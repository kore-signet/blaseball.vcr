use crate::chron_types::*;
use crate::{EntityDatabase, VCRResult};
use type_map::concurrent::TypeMap;

pub struct DatabaseManager {
    pub databases: TypeMap,
}

impl DatabaseManager {
    pub fn new() -> DatabaseManager {
        DatabaseManager {
            databases: TypeMap::new(),
        }
    }

    pub fn insert<T: 'static, D: 'static + Send + Sync + EntityDatabase<Record = T>>(
        &mut self,
        db: D,
    ) {
        let boxed_db: Box<dyn EntityDatabase<Record = T> + Send + Sync> = Box::new(db);
        self.databases.insert(boxed_db);
    }

    pub fn get_entity<E: 'static>(
        &self,
        id: &[u8; 16],
        at: i64,
    ) -> VCRResult<Option<ChroniclerEntity<E>>> {
        if let Some(db) = self
            .databases
            .get::<Box<dyn EntityDatabase<Record = E> + Send + Sync>>()
        {
            return db.get_entity(id, at);
        }

        Ok(None)
    }

    pub fn get_entities<E: 'static>(
        &self,
        ids: &[[u8; 16]],
        at: i64,
    ) -> VCRResult<Vec<Option<ChroniclerEntity<E>>>> {
        if let Some(db) = self
            .databases
            .get::<Box<dyn EntityDatabase<Record = E> + Send + Sync>>()
        {
            return db.get_entities(ids, at);
        }

        Ok(Vec::with_capacity(0))
    }

    pub fn get_versions<E: 'static>(
        &self,
        id: &[u8; 16],
        before: i64,
        after: i64,
    ) -> VCRResult<Option<Vec<ChroniclerEntity<E>>>> {
        if let Some(db) = self
            .databases
            .get::<Box<dyn EntityDatabase<Record = E> + Send + Sync>>()
        {
            return db.get_versions(id, before, after);
        }

        Ok(None)
    }

    pub fn all_entities<E: 'static>(&self, at: i64) -> VCRResult<Vec<Option<ChroniclerEntity<E>>>> {
        if let Some(db) = self
            .databases
            .get::<Box<dyn EntityDatabase<Record = E> + Send + Sync>>()
        {
            return db.get_entities(db.all_ids(), at);
        }

        Ok(Vec::with_capacity(0))
    }

    pub fn all_entity_ids<E: 'static>(&self) -> Option<&[[u8; 16]]> {
        if let Some(db) = self
            .databases
            .get::<Box<dyn EntityDatabase<Record = E> + Send + Sync>>()
        {
            Some(db.all_ids())
        } else {
            None
        }
    }

    pub fn get_db<E: 'static>(&self) -> Option<&Box<dyn EntityDatabase<Record = E> + Send + Sync>> {
        self.databases
            .get::<Box<dyn EntityDatabase<Record = E> + Send + Sync>>()
    }
}
