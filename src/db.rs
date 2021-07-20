use super::*;
use easybench::bench;
use json_patch::{
    patch as patch_json, AddOperation, CopyOperation, MoveOperation, Patch as JSONPatch,
    PatchOperation, PatchOperation::*, RemoveOperation, ReplaceOperation, TestOperation,
};
use serde_json::{json, Value as JSONValue};
use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader, SeekFrom};
use std::sync::Mutex;

fn decode(
    header: HashMap<u32, (u64, u64)>,
    path_map: HashMap<u8, String>,
    bytes: Vec<u8>,
) -> Result<Vec<(u32, JSONPatch)>, VCRError> {
    let mut patches: Vec<(u32, JSONPatch)> = Vec::new();

    for (time, (offset, len)) in header {
        let mut e_bytes: Vec<u8> = (&bytes[offset as usize..offset as usize + len as usize])
            .into_iter()
            .copied()
            .collect();

        let mut operations: Vec<PatchOperation> = Vec::new();

        while e_bytes.len() > 1 {
            let op_code = e_bytes.remove(0);

            let paths = if op_code == 3 || op_code == 4 {
                vec![
                    path_map
                        .get(&u8::from_be_bytes([e_bytes.remove(0)]))
                        .ok_or(VCRError::PathResolutionError)?,
                    path_map
                        .get(&u8::from_be_bytes([e_bytes.remove(0)]))
                        .ok_or(VCRError::PathResolutionError)?,
                ]
            } else {
                vec![path_map
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

        patches.push((time, JSONPatch(operations)));
    }

    patches.sort_by_key(|x| x.0);
    Ok(patches)
}

pub struct Database {
    reader: BufReader<File>,
    entities: HashMap<String, (u64, u64)>,
}

impl Database {
    pub fn from_files(entities_lookup_path: &str, db_path: &str) -> Result<Database, VCRError> {
        let entities_lookup_f = File::open(entities_lookup_path).map_err(VCRError::IOError)?;
        let db_f = File::open(db_path).map_err(VCRError::IOError)?;

        Ok(Database {
            reader: BufReader::new(db_f),
            entities: rmp_serde::from_read(entities_lookup_f).map_err(VCRError::MsgPackError)?,
        })
    }

    pub fn get_entity_data(&mut self, entity: &str) -> Result<Vec<(u32, JSONPatch)>, VCRError> {
        let (offset_start, entity_len) = self.entities.get(entity).unwrap();

        self.reader
            .seek(SeekFrom::Start(*offset_start))
            .map_err(VCRError::IOError)?;

        let mut buffer: Vec<u8> = Vec::new();

        loop {
            self.reader
                .read_until(0, &mut buffer)
                .map_err(VCRError::IOError)?;
            let mut end = [0; 23];
            self.reader
                .read_exact(&mut end)
                .map_err(VCRError::IOError)?;
            if end.iter().all(|&x| x == 0) {
                buffer.remove(buffer.len() - 1);
                break;
            } else {
                buffer.extend(end);
            }
        }

        let header: (HashMap<u32, (u64, u64)>, HashMap<u8, String>) =
            rmp_serde::from_read_ref(&buffer).map_err(VCRError::MsgPackError)?;
        let mut patch_bytes = vec![
            0;
            (offset_start + entity_len) as usize
                - self.reader.stream_position().map_err(VCRError::IOError)?
                    as usize
        ];

        self.reader
            .read_exact(&mut patch_bytes)
            .map_err(VCRError::IOError)?;
        decode(header.0, header.1, patch_bytes)
    }

    pub fn get_entity_versions(
        &mut self,
        entity: &str,
        before: u32,
        after: u32,
    ) -> Result<Vec<(u32, JSONValue)>, VCRError> {
        let mut entity_value = json!({});
        let patches = self.get_entity_data(entity)?;
        let mut results: Vec<(u32, JSONValue)> = Vec::with_capacity(patches.len());

        for (time, patch) in patches {
            if time >= before {
                break;
            }

            if time <= after {
                continue;
            }

            patch_json(&mut entity_value, &patch).map_err(VCRError::JSONPatchError)?;
            results.push((time, entity_value.clone()));
        }

        Ok(results)
    }

    pub fn get_entity(&mut self, entity: &str, at: u32) -> Result<JSONValue, VCRError> {
        let mut entity_value = json!({});
        for (time, patch) in self.get_entity_data(entity)? {
            if time > at {
                break;
            }

            patch_json(&mut entity_value, &patch).map_err(VCRError::JSONPatchError)?;
        }

        Ok(entity_value)
    }

    pub fn get_entities(
        &mut self,
        entities: Vec<String>,
        at: u32,
    ) -> Result<Vec<JSONValue>, VCRError> {
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
    ) -> Result<Vec<(u32, JSONValue)>, VCRError> {
        let mut results = Vec::with_capacity(entities.len());
        for e in entities {
            results.append(&mut self.get_entity_versions(&e, before, after)?);
        }

        Ok(results)
    }
}

pub struct MultiDatabase {
    dbs: HashMap<String, Mutex<Database>>, // entity_type:db
}

impl MultiDatabase {
    pub fn from_files(files: Vec<(String,String,String)>) -> Result<MultiDatabase,VCRError> {
        let mut dbs: HashMap<String, Mutex<Database>> = HashMap::new();
        for (e_type, lookup_file, main_file) in files {
            dbs.insert(e_type, Mutex::new(Database::from_files(&lookup_file, &main_file)?));
        }

        Ok(MultiDatabase {
            dbs: dbs
        })
    }

    pub fn get_entity(
        &mut self,
        e_type: &str,
        entity: &str,
        at: u32,
    ) -> Result<JSONValue, VCRError> {
        let mut db = self.dbs[e_type].lock().unwrap();
        Ok(db.get_entity(entity, at)?)
    }

    pub fn get_entity_versions(
        &mut self,
        e_type: &str,
        entity: &str,
        before: u32,
        after: u32,
    ) -> Result<Vec<(u32,JSONValue)>, VCRError> {
        let mut db = self.dbs[e_type].lock().unwrap();
        Ok(db.get_entity_versions(entity, before, after)?)
    }

    pub fn get_entities(
        &mut self,
        e_type: &str,
        entities: Vec<String>,
        at: u32,
    ) -> Result<Vec<JSONValue>, VCRError> {
        let mut db = self.dbs[e_type].lock().unwrap();
        Ok(db.get_entities(entities, at)?)
    }

    pub fn get_entities_versions(
        &mut self,
        e_type: &str,
        entities: Vec<String>,
        before: u32,
        after: u32,
    ) -> Result<Vec<(u32, JSONValue)>, VCRError> {
        let mut db = self.dbs[e_type].lock().unwrap();
        Ok(db.get_entities_versions(entities, before, after)?)
    }
}
