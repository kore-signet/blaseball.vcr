use blaseball_vcr::*;
use chrono::DateTime;
use json_patch::{diff, PatchOperation::*};
use serde_json::{json, Value as JSONValue};
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Read, Seek, Write};

type EntityPatch = (u32, Vec<Vec<u8>>);

struct Op {
    paths: Vec<String>,
    op_code: u8,
    value: Option<JSONValue>,
}

fn encode(entity: Vec<(u32, JSONValue)>) -> (Vec<EntityPatch>, HashMap<u8, String>) {
    let mut last = json!({});
    let mut paths: HashMap<String, u8> = HashMap::new();
    (
        entity
            .into_iter()
            .map(|(time, obj)| {
                let diff: Vec<Vec<u8>> = diff(&last, &obj)
                    .0
                    .into_iter()
                    .map(|r_op| {
                        let op = match r_op {
                            Add(add_op) => Op {
                                paths: vec![add_op.path],
                                op_code: 0,
                                value: Some(add_op.value),
                            },
                            Remove(rm_op) => Op {
                                paths: vec![rm_op.path],
                                op_code: 1,
                                value: None,
                            },
                            Replace(re_op) => Op {
                                paths: vec![re_op.path],
                                op_code: 2,
                                value: Some(re_op.value),
                            },
                            Move(mv_op) => Op {
                                paths: vec![mv_op.path, mv_op.from],
                                op_code: 3,
                                value: None,
                            },
                            Copy(cp_op) => Op {
                                paths: vec![cp_op.path, cp_op.from],
                                op_code: 4,
                                value: None,
                            },
                            Test(te_op) => Op {
                                paths: vec![te_op.path],
                                op_code: 5,
                                value: Some(te_op.value),
                            },
                        };

                        let mut bytes: Vec<u8> = Vec::new();

                        bytes.push(op.op_code.to_be());

                        for path in &op.paths {
                            if !paths.contains_key(path) {
                                paths.insert(path.to_string(), paths.len() as u8);
                            }

                            bytes.push(paths[path].to_be());
                        }

                        if let Some(value) = op.value {
                            let mut val_bytes = rmp_serde::to_vec_named(&value).unwrap();
                            bytes.extend((val_bytes.len() as u16).to_be_bytes());
                            bytes.append(&mut val_bytes);
                        } else {
                            bytes.extend(0_u16.to_be_bytes());
                        }

                        bytes
                    })
                    .collect();

                last = obj;

                (time, diff)
            })
            .collect::<Vec<EntityPatch>>(),
        paths
            .into_iter()
            .map(|(k, v)| (v, k))
            .collect::<HashMap<u8, String>>(),
    )
}

fn main() {
    // arguments: input out lookup_out
    let args: Vec<String> = env::args().collect();

    let mut entities_file = File::open(&args[1]).unwrap();
    let mut out_file = File::create(&args[2]).unwrap();

    let mut out = BufWriter::new(out_file);

    let mut entity_lookup_table: HashMap<String, EntityData> = HashMap::new();

    let mut entities_reader = BufReader::new(entities_file);
    let mut all_entities: JSONValue = serde_json::from_reader(entities_reader).unwrap();

    for (k, entity) in all_entities.as_object().unwrap().into_iter() {
        let mut entries: Vec<(u32, JSONValue)> = Vec::new();

        let entity_start_pos = out.stream_position().unwrap();
        println!("processing entity {}", &k);

        for (i, value) in entity.as_array().unwrap().into_iter().enumerate() {
            println!("#{}", i);
            let time = DateTime::parse_from_rfc3339(value["validFrom"].as_str().unwrap())
                .unwrap()
                .timestamp() as u32;
            entries.push((time, value["data"].clone()));
        }

        let (patches, path_map) = encode(entries);
        let mut bytes: Vec<u8> = Vec::new();

        let mut offsets: Vec<(u32, u64, u64)> = Vec::new(); // timestamp:start_position:end_position

        for (time, patch) in patches {
            let start_pos = out.stream_position().unwrap();

            for op in patch {
                out.write_all(&op).unwrap();
            }

            let end_pos = out.stream_position().unwrap();
            offsets.push((time, start_pos, end_pos));
        }

        entity_lookup_table.insert(
            k.to_owned(),
            EntityData {
                data_offset: entity_start_pos,
                patches: offsets,
                path_map: path_map,
            },
        );
    }

    out.flush().unwrap();

    let mut entity_table_f = File::create(&args[3]).unwrap();
    entity_table_f
        .write_all(&rmp_serde::to_vec(&entity_lookup_table).unwrap())
        .unwrap();
}
