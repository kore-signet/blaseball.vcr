use json_patch::{diff, PatchOperation, PatchOperation::*};
use serde_json::{json, Value as JSONValue};
use std::collections::HashMap;

type EntityPatch = (u32, Vec<Vec<u8>>);

struct Op {
    paths: Vec<String>,
    op_code: u8,
    value: Option<JSONValue>,
}

pub fn encode(
    entity: Vec<(u32, JSONValue)>,
    checkpoint_every: u32,
) -> (Vec<EntityPatch>, HashMap<u16, String>) {
    let mut last = json!({});
    let mut paths: HashMap<String, u16> = HashMap::new();
    (
        entity
            .into_iter()
            .enumerate()
            .map(|(iter, (time, obj))| {
                let diff_ops: Vec<PatchOperation> = if iter as u32 % checkpoint_every == 0 {
                    diff(&json!({}), &obj).0
                } else {
                    diff(&last, &obj).0
                };

                let diff: Vec<Vec<u8>> = diff_ops
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
                                paths.insert(path.to_string(), paths.len() as u16);
                            }

                            bytes.extend(paths[path].to_be_bytes());
                        }

                        if let Some(value) = op.value {
                            let mut val_bytes = rmp_serde::to_vec(&value).unwrap();
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
            .collect::<HashMap<u16, String>>(),
    )
}
