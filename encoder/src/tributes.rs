use blaseball_vcr::*;
use integer_encoding::VarIntWriter;
use serde_json::Value as JSONValue;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufWriter, Seek, Write};
use uuid::Uuid;

pub fn main() {
    // let f = File::open("tributes.json").unwrap();
    // let reader = BufReader::new(f);
    let client = reqwest::blocking::Client::new();

    let mut ids: HashMap<Uuid, (u16, bool)> = HashMap::new();
    let mut times: Vec<(u32, u32, u16)> = Vec::new();
    let mut vals: HashMap<Uuid, u64> = HashMap::new();
    let mut last_seen: Vec<Uuid> = Vec::new();

    let mut next_page: Option<String> = None;

    let out_f = File::create("./tapes/tributes.riv").unwrap();
    let mut out_writer = BufWriter::new(out_f);

    let mut page = 1;

    loop {
        println!("#{}", page);

        let parameters = if let Some(ref page) = next_page {
            vec![("type", "tributes"), ("count", "1000"), ("page", page)]
        } else {
            vec![("type", "tributes"), ("count", "1000")]
        };

        let tributes: ChroniclerResponse<ChroniclerEntity<JSONValue>> = client
            .get("https://api.sibr.dev/chronicler/v2/versions")
            .query(&parameters)
            .send()
            .unwrap()
            .json()
            .unwrap();

        for version in tributes.items {
            let start_pos = out_writer.stream_position().unwrap() as u32;
            let mut seen_ids: Vec<Uuid> = Vec::new();

            if let Some(players) = version.data.as_array() {
                for i in players {
                    let id = Uuid::parse_str(i["playerId"].as_str().unwrap()).unwrap();
                    seen_ids.push(id);
                    let l = (ids.len() as u16) + 1;
                    let idx = ids.entry(id).or_insert((l, false));
                    let n = i["peanuts"].as_u64().unwrap();

                    if let Some(x) = vals.get_mut(&id) {
                        if *x != n {
                            out_writer.write_varint(idx.0).unwrap();
                            out_writer.write_varint(n).unwrap();
                            *x = n;
                        }
                    } else {
                        out_writer.write_varint(idx.0).unwrap();
                        out_writer.write_varint(n).unwrap();
                        vals.insert(id, n);
                    }
                }
            } else if let Some(obj) = version.data.as_object() {
                for i in obj["teams"].as_array().unwrap() {
                    let id = Uuid::parse_str(i["teamId"].as_str().unwrap()).unwrap();
                    seen_ids.push(id);
                    let l = (ids.len() as u16) + 1;
                    let idx = ids.entry(id).or_insert((l, true));
                    let n = i["peanuts"].as_u64().unwrap();

                    if let Some(x) = vals.get_mut(&id) {
                        if *x != n {
                            out_writer.write_varint(idx.0).unwrap();
                            out_writer.write_varint(n).unwrap();
                            *x = n;
                        }
                    } else {
                        out_writer.write_varint(idx.0).unwrap();
                        out_writer.write_varint(n).unwrap();
                        vals.insert(id, n);
                    }
                }

                for i in obj["players"].as_array().unwrap() {
                    let id = Uuid::parse_str(i["playerId"].as_str().unwrap()).unwrap();
                    seen_ids.push(id);

                    let l = (ids.len() as u16) + 1;
                    let idx = ids.entry(id).or_insert((l, false));
                    let n = i["peanuts"].as_u64().unwrap();

                    if let Some(x) = vals.get_mut(&id) {
                        if *x != n {
                            out_writer.write_varint(idx.0).unwrap();
                            out_writer.write_varint(n).unwrap();
                            *x = n;
                        }
                    } else {
                        out_writer.write_varint(idx.0).unwrap();
                        out_writer.write_varint(n).unwrap();
                        vals.insert(id, n);
                    }
                }
            }

            let removed = last_seen
                .iter()
                .filter(|i| !seen_ids.contains(i))
                .copied()
                .collect::<Vec<Uuid>>();
            last_seen = seen_ids;

            if !removed.is_empty() {
                if removed.len() > (u8::MAX as usize) {
                    println!("{}", removed.len());
                }
                out_writer.write_varint(0).unwrap();
                out_writer.write_varint(removed.len() as u8).unwrap();
                for r in removed {
                    let idx = ids[&r];
                    out_writer.write_varint(idx.0).unwrap();
                }
            }

            let out_pos = out_writer.stream_position().unwrap() as u32;
            times.push((
                version.valid_from.timestamp() as u32,
                start_pos,
                (out_pos - start_pos) as u16,
            ));
        }

        page += 1;

        if let Some(page) = tributes.next_page {
            next_page = Some(page);
        } else {
            break;
        }
    }

    // Ok(results)
    // let mut bytes: File = File::create("tributes.bin").unwrap();

    // for t in tributes.items {
    //     if let Some(ts) = t.data.as_array() {
    //         for i in ts {}
    //     } else if let Some(ts) = t.data.as_object() {
    //     }
    // }

    let mut header_f = File::create("./tapes/tributes.header.riv").unwrap();

    let idx_ids: Vec<u8> = ids
        .into_iter()
        .flat_map(|(k, v)| {
            vec![
                k.as_bytes().to_vec(),
                (v.0 | ((v.1 as u16) << 15)).to_be_bytes().to_vec(),
            ]
            .concat()
        })
        .collect();

    header_f
        .write_all(&(idx_ids.len() as u32).to_be_bytes())
        .unwrap();
    header_f.write_all(&idx_ids).unwrap();

    let times: Vec<u8> = times
        .into_iter()
        .flat_map(|(time, start, len)| {
            vec![
                time.to_be_bytes().to_vec(),
                start.to_be_bytes().to_vec(),
                len.to_be_bytes().to_vec(),
            ]
            .concat()
        })
        .collect();

    header_f.write_all(&times).unwrap();
    // loop {
    //     let idx = cursor.read_varint::<u16>();
    //     if idx.is_err() {
    //         break;
    //     }

    //     let val = cursor.read_varint::<u64>().unwrap();

    //     new_vals.insert(idx_ids[&idx.unwrap()].clone(), val);
    // }
    //    let a: u16 = 512_u16 | mask;
    //    println!("{:?}", a & 0xFFF); // original
    //    println!("{:?}", (a >> 15) & (0xFFF)); // flag
}
