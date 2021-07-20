use chrono::DateTime;
use json_patch::{diff, PatchOperation::*};
use serde_json::{json, Value as JSONValue};
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;
use std::io::Seek;
use std::io::Write;

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

    let mut file = File::open(&args[0]).unwrap();
    let mut out = File::create(&args[1]).unwrap();

    let mut entity_table: HashMap<String, (u64, u64)> = HashMap::new();

    let mut contents = String::new();
    file.read_to_string(&mut contents);

    let mut players = serde_json::from_str::<JSONValue>(&contents).unwrap();

    for (k, player) in players.as_object().unwrap().into_iter() {
        let mut entities: Vec<(u32, JSONValue)> = Vec::new();
        let start_pos = out.stream_position().unwrap();
        println!("processing entity {}", &k);

        for (i, value) in player.as_array().unwrap().into_iter().enumerate() {
            println!("#{}", i);
            let time = DateTime::parse_from_rfc3339(value["validFrom"].as_str().unwrap())
                .unwrap()
                .timestamp() as u32;
            entities.push((time, value["data"].clone()));
        }

        let (patches, path_map) = encode(entities);
        let mut bytes: Vec<u8> = Vec::new();

        let mut offset = 0_u64;
        let mut offset_table: HashMap<u32, (u64, u64)> = HashMap::new(); // timestamp:(offset,length)

        for (time, patch) in &patches {
            offset_table.insert(*time, (offset, patch.concat().len() as u64));
            offset += patch.concat().len() as u64;
        }

        let header = rmp_serde::to_vec_named(&(&offset_table, path_map)).unwrap();
        out.write_all(&header).unwrap();
        out.write_all(&vec![
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ])
        .unwrap();
        for (_, patch) in patches {
            for op in patch {
                out.write_all(&op).unwrap();
            }
        }

        let end_pos = out.stream_position().unwrap();

        entity_table.insert(k.to_string(), (start_pos, end_pos - start_pos));
    }

    let mut entity_table_f = File::create(&args[3]).unwrap();
    entity_table_f
        .write_all(&rmp_serde::to_vec_named(&entity_table).unwrap())
        .unwrap();
}
