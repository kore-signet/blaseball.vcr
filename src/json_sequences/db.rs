use crate::*;
use chrono::{DateTime, NaiveDateTime, Utc};
use json_patch::{
    patch as patch_json, AddOperation, CopyOperation, MoveOperation, Patch as JSONPatch,
    PatchOperation, PatchOperation::*, RemoveOperation, ReplaceOperation, TestOperation,
};
use serde_json::{json, Value as JSONValue};
use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader, SeekFrom};
use std::sync::Mutex;

pub struct Database {
    reader: BufReader<File>,
    entities: HashMap<String, EntityData>,
}

impl Database {
    pub fn from_files(entities_lookup_path: &str, db_path: &str) -> VCRResult<Database> {
        let entities_lookup_f = File::open(entities_lookup_path).map_err(VCRError::IOError)?;
        let db_f = File::open(db_path).map_err(VCRError::IOError)?;

        Ok(Database {
            reader: BufReader::new(db_f),
            entities: rmp_serde::from_read(entities_lookup_f).map_err(VCRError::MsgPackError)?,
        })
    }

    pub fn get_entity_data(
        &mut self,
        entity: &str,
        until: u32,
    ) -> VCRResult<Vec<(u32, JSONPatch)>> {
        let metadata = &self.entities[entity];
        let mut patches: Vec<(u32, JSONPatch)> = Vec::new();

        self.reader
            .seek(SeekFrom::Start(metadata.data_offset))
            .map_err(VCRError::IOError)?;

        for (time, patch_start, patch_end) in &metadata.patches {
            if time > &until {
                break;
            }

            let mut e_bytes: Vec<u8> = vec![0; (patch_end - patch_start) as usize];
            self.reader.read_exact(&mut e_bytes);

            let mut operations: Vec<PatchOperation> = Vec::new();

            while e_bytes.len() > 1 {
                let op_code = e_bytes.remove(0);

                let paths = if op_code == 3 || op_code == 4 {
                    vec![
                        metadata
                            .path_map
                            .get(&u8::from_be_bytes([e_bytes.remove(0)]))
                            .ok_or(VCRError::PathResolutionError)?,
                        metadata
                            .path_map
                            .get(&u8::from_be_bytes([e_bytes.remove(0)]))
                            .ok_or(VCRError::PathResolutionError)?,
                    ]
                } else {
                    vec![metadata
                        .path_map
                        .get(&u8::from_be_bytes([e_bytes.remove(0)]))
                        .ok_or(VCRError::PathResolutionError)?]
                };

                let value_length = u16::from_be_bytes([e_bytes.remove(0), e_bytes.remove(0)]);

                let value: Option<JSONValue> = if value_length > 0 {
                    let val_bytes: Vec<u8> = e_bytes.drain(..value_length as usize).collect();
                    Some(rmp_serde::from_read_ref(&val_bytes).map_err(VCRError::MsgPackError)?)
                } else {
                    None
                };

                operations.push(match op_code {
                    0 => Add(AddOperation {
                        path: paths[0].to_string(),
                        value: value.unwrap(),
                    }),
                    1 => Remove(RemoveOperation {
                        path: paths[0].to_string(),
                    }),
                    2 => Replace(ReplaceOperation {
                        path: paths[0].to_string(),
                        value: value.unwrap(),
                    }),
                    3 => Move(MoveOperation {
                        path: paths[0].to_string(),
                        from: paths[1].to_string(),
                    }),
                    4 => Copy(CopyOperation {
                        path: paths[0].to_string(),
                        from: paths[1].to_string(),
                    }),
                    5 => Test(TestOperation {
                        path: paths[0].to_string(),
                        value: value.unwrap(),
                    }),
                    _ => return Err(VCRError::InvalidOpCode),
                });
            }

            patches.push((*time, JSONPatch(operations)));
        }

        patches.sort_by_key(|x| x.0);
        Ok(patches)
    }

    pub fn get_entity_versions(
        &mut self,
        entity: &str,
        before: u32,
        after: u32,
    ) -> VCRResult<Vec<(u32, JSONValue)>> {
        let mut entity_value = json!({});
        let patches = self.get_entity_data(entity, before)?;
        let mut results: Vec<(u32, JSONValue)> = Vec::with_capacity(patches.len());

        for (time, patch) in patches {
            patch_json(&mut entity_value, &patch).map_err(VCRError::JSONPatchError)?;

            if time > after {
                results.push((time, entity_value.clone()));
            }
        }

        Ok(results)
    }

    pub fn get_entity(&mut self, entity: &str, at: u32) -> VCRResult<JSONValue> {
        let mut entity_value = json!({});
        let mut last_time = 0;

        for (time, patch) in self.get_entity_data(entity, at)? {
            patch_json(&mut entity_value, &patch).map_err(VCRError::JSONPatchError)?;
            last_time = time;
        }

        Ok(json!({
            "data": entity_value,
            "entityId": entity,
            "validFrom": DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(last_time as i64, 0), Utc).to_rfc3339()
        }))
    }

    pub fn get_entities(&mut self, entities: Vec<String>, at: u32) -> VCRResult<Vec<JSONValue>> {
        let mut results = Vec::with_capacity(entities.len());
        for e in entities {
            results.push(self.get_entity(&e, at)?);
        }

        Ok(results)
    }

    pub fn get_entities_versions(
        &mut self,
        entities: Vec<String>,
        before: u32,
        after: u32,
    ) -> VCRResult<Vec<(u32, JSONValue)>> {
        let mut results = Vec::with_capacity(entities.len());
        for e in entities {
            results.append(&mut self.get_entity_versions(&e, before, after)?);
        }

        Ok(results)
    }

    pub fn all_entities(&mut self, at: u32) -> VCRResult<Vec<JSONValue>> {
        let mut results = Vec::with_capacity(self.entities.len());
        let keys: Vec<String> = self.entities.keys().cloned().collect();
        for entity in keys {
            results.push(self.get_entity(&entity, at)?);
        }

        Ok(results)
    }
}

pub struct MultiDatabase {
    dbs: HashMap<String, Mutex<Database>>, // entity_type:db
}

impl MultiDatabase {
    pub fn from_files(files: Vec<(&str, &str, &str)>) -> VCRResult<MultiDatabase> {
        let mut dbs: HashMap<String, Mutex<Database>> = HashMap::new();
        for (e_type, lookup_file, main_file) in files {
            dbs.insert(
                e_type.to_owned(),
                Mutex::new(Database::from_files(lookup_file, main_file)?),
            );
        }

        Ok(MultiDatabase { dbs: dbs })
    }

    pub fn get_entity(&self, e_type: &str, entity: &str, at: u32) -> VCRResult<JSONValue> {
        let mut db = self.dbs[e_type].lock().unwrap();
        Ok(db.get_entity(entity, at)?)
    }

    pub fn get_entity_versions(
        &self,
        e_type: &str,
        entity: &str,
        before: u32,
        after: u32,
    ) -> VCRResult<Vec<(u32, JSONValue)>> {
        let mut db = self.dbs[e_type].lock().unwrap();
        Ok(db.get_entity_versions(entity, before, after)?)
    }

    pub fn get_entities(
        &self,
        e_type: &str,
        entities: Vec<String>,
        at: u32,
    ) -> VCRResult<Vec<JSONValue>> {
        let mut db = self.dbs[e_type].lock().unwrap();
        Ok(db.get_entities(entities, at)?)
    }

    pub fn get_entities_versions(
        &self,
        e_type: &str,
        entities: Vec<String>,
        before: u32,
        after: u32,
    ) -> VCRResult<Vec<(u32, JSONValue)>> {
        let mut db = self.dbs[e_type].lock().unwrap();
        Ok(db.get_entities_versions(entities, before, after)?)
    }

    pub fn all_entities(&self, e_type: &str, at: u32) -> VCRResult<Vec<JSONValue>> {
        let mut db = self.dbs[e_type].lock().unwrap();
        Ok(db.all_entities(at)?)
    }
}
