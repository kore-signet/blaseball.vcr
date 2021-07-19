use json_patch::{
    patch as patch_json, AddOperation, CopyOperation, MoveOperation, Patch as JSONPatch, PatchOperation,
    PatchOperation::*, RemoveOperation, ReplaceOperation, TestOperation,
};
use serde_json::{json, Value as JSONValue};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, SeekFrom, prelude::*};
use std::time::Instant;

fn decode(
    header: HashMap<u32, (u64, u64)>,
    path_map: HashMap<u8, String>,
    bytes: Vec<u8>,
) -> Vec<(u32, JSONPatch)> {
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
                        .unwrap(),
                    path_map
                        .get(&u8::from_be_bytes([e_bytes.remove(0)]))
                        .unwrap(),
                ]
            } else {
                vec![path_map
                    .get(&u8::from_be_bytes([e_bytes.remove(0)]))
                    .unwrap()]
            };

            let value_length = u16::from_be_bytes([e_bytes.remove(0), e_bytes.remove(0)]);

            let value: Option<JSONValue> = if value_length > 0 {
                let val_bytes: Vec<u8> = e_bytes.drain(..value_length as usize).collect();
                Some(rmp_serde::from_read_ref(&val_bytes).unwrap())
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
                _ => {
                    panic!("Couldn't decode operation - invalid op code")
                }
            });
        }

        patches.push((time, JSONPatch(operations)));
    }

    patches.sort_by_key(|x| x.0);
    patches
}

fn main() {
    let entities_lookup_f = File::open("entities.bin").unwrap();
    let db_f = File::open("out.bin").unwrap();
    let mut db = BufReader::new(db_f);

    let entities_lookup: HashMap<String, (u64, u64)> =
        rmp_serde::from_read(entities_lookup_f).unwrap();

    let start_ops = Instant::now();

    let (offset_start, entity_len) = entities_lookup
        .get("083d09d4-7ed3-4100-b021-8fbe30dd43e8")
        .unwrap();
    db.seek(SeekFrom::Start(*offset_start)).unwrap();

    let mut buffer: Vec<u8> = Vec::new();

    loop {
        db.read_until(0, &mut buffer).unwrap();
        let mut end = [0; 23];
        db.read_exact(&mut end).unwrap();
        if end.iter().all(|&x| x == 0) {
            buffer.remove(buffer.len() - 1);
            break;
        } else {
            buffer.extend(end);
        }
    }

    let header: (HashMap<u32, (u64, u64)>, HashMap<u8, String>) =
        rmp_serde::from_read_ref(&buffer).unwrap();
    let mut patch_bytes =
        vec![0; (offset_start + entity_len) as usize - db.stream_position().unwrap() as usize];

    db.read_exact(&mut patch_bytes).unwrap();

    let mut jt = json!({});
    let patches = decode(header.0, header.1, patch_bytes);
    for (_, patch) in patches {
        patch_json(&mut jt, &patch).unwrap();
    }

    println!("{:?}",start_ops.elapsed().as_micros());
}
