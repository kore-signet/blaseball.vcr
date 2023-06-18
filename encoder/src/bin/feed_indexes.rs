// use blaseball_vcr::feed::event::FeedEvent;
// use blaseball_vcr::feed::lookup_tables::*;
// use blaseball_vcr::VCRResult;
// use clap::clap_app;
// use std::collections::HashMap;
// use std::collections::HashSet;
// use std::fs::File;
// use std::io::{BufRead, BufReader, BufWriter, Write};
// use uuid::Uuid;

// fn main() -> VCRResult<()> {
//     let matches = clap_app!(train_vhs_dict =>
//         (version: "1.0")
//         (author: "emily signet <emily@sibr.dev>")
//         (about: "blaseball.vcr gen 2 feed index maker")
//         (@arg INPUT: +required -i --input [FILE] "feed file (ndjson)")
//         (@arg OUTPUT: +required -o --output [FILE] "output file for index")
//         (@arg AUX_INPUT: +required -a --aux [FILE] "set input file for uuid -> internal id lookup table")
//     )
//     .get_matches();

//     let uuid_to_internal: HashMap<Uuid, (u16, u16)> = rmp_serde::from_read(BufReader::new(
//         File::open(matches.value_of("AUX_INPUT").unwrap())?,
//     ))?;

//     let input = BufReader::new(File::open(matches.value_of("INPUT").unwrap())?);

//     let mut table: Vec<(Vec<u8>, u64)> = Vec::new();

//     let stdout = std::io::stdout();
//     let mut stdout_handle = stdout.lock();

//     let mut seen_ids = HashSet::new();

//     let rng = fastrand::Rng::new();

//     for (i, line) in input.lines().enumerate() {
//         writeln!(stdout_handle, "#{i}")?;
//         let event: FeedEvent = serde_json::from_str::<FeedEvent>(&line?)?;

//         if !seen_ids.insert(event.id) {
//             continue;
//         }

//         let (block_index, event_index) = uuid_to_internal[&event.id];
//         let internal_id: u64 = (((block_index as u64) << 16) | event_index as u64).to_le();
//         let timestamp: u32 = event.created.timestamp() as u32;

//         let mut player_tags = event.player_tags.unwrap_or_default();
//         player_tags.sort();
//         player_tags.dedup();

//         for player_tag in player_tags {
//             let player_tag = UUID_TO_PLAYER[player_tag.as_bytes()];
//             let tagk = loop {
//                 let rand_byte = rng.u8(..);
//                 let nkey = key(player_tag, timestamp, &rand_byte.to_le_bytes());
//                 if table.iter().find(|(k, _)| k == &nkey).is_none() {
//                     break nkey;
//                 }
//             };

//             table.push((tagk, internal_id));
//         }
//     }

//     println!("index length: {}", table.len());

//     table.sort_by_key(|(k, _)| k.clone());

//     let out = BufWriter::new(File::create(matches.value_of("OUTPUT").unwrap())?);
//     let mut map_builder = fst::MapBuilder::new(out).unwrap();
//     map_builder.extend_iter(table).unwrap();
//     map_builder.finish().unwrap();

//     Ok(())
// }

// fn key(player_tag: u16, time: u32, extra: &[u8]) -> Vec<u8> {
//     let mut out = Vec::with_capacity(10);
//     out.extend_from_slice(&player_tag.to_be_bytes());
//     out.extend_from_slice(&time.to_be_bytes());
//     out.extend_from_slice(extra);
//     out
// }

fn main() {}