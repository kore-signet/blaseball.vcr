// // use blaseball_vcr::feed::db::*;
// // use blaseball_vcr::feed::event::*;
// // use blaseball_vcr::*;

// // fn main() -> VCRResult<()> {
// //     let db = FeedDatabase::from_single("./vhs_tapes/feed.vhs")?;
// //     println!(
// //         "read whole block: {}",
// //         easybench::bench(|| db.get_block_by_time(1627669026894))
// //     );

// //     let block = db.get_block_by_time(1627669026894)?;

// //     println!(
// //         "read from block: {}",
// //         easybench::bench(|| block.event_at_time(1627669026894))
// //     );
// //     let event = block.event_at_time(1627669026894);

// //     println!("{}", serde_json::to_string(&event)?);

// //     Ok(())
// // }

// use blaseball_vcr::feed::db::*;

// use blaseball_vcr::feed::index::{unpack_event_index, Index as FeedIndex, IndexType};
// use blaseball_vcr::feed::lookup_tables::*;
// use blaseball_vcr::*;
// use fst::Map as FstMap;
// use fst::Streamer;
// use uuid::Uuid;

// fn main() -> VCRResult<()> {
//     let db = FeedDatabase::from_single("./vhs_tapes/feed.vhs")?;
//     let memory = std::fs::read("feed_playertags.fst")?;
//     let idx = FstMap::new(memory).unwrap();

//     let mut idx_manager = FeedIndex::new();
//     idx_manager.add_index(IndexType::PlayerTags, idx);

//     // patty fox
//     let player_tag = UUID_TO_PLAYER[Uuid::parse_str("81d7d022-19d6-427d-aafc-031fcb79b29e")
//         .unwrap()
//         .as_bytes()]
//     .to_be_bytes();

//     let after = 1617837026; // 2021-04-07T23:10:26.874Z
//     let before = 1617858011; // 2021-04-08T05:00:11.241Z
//                              // println!("uwu");
//     let after_ms: u64 = after as u64 * 1000;
//     println!(
//         "{}",
//         serde_json::to_string_pretty(&db.get_events_after(after_ms, 201).into_serializable())?
//     );

//     Ok(())
//     // let mut stream = idx_manager
//     //     .get_by_tag_and_time(IndexType::PlayerTags, &player_tag[..], after, before)
//     //     .unwrap();

//     // while let Some((_, key)) = stream.next() {
//     //     let (block_index, event_index) = unpack_event_index(key);
//     //     let block = db.get_block_by_index(block_index)?;
//     //     println!(
//     //         "{}",
//     //         serde_json::to_string_pretty(&block.event_at_index(event_index))?
//     //     );
//     // }

//     // Ok(())
// }

fn main() {}
