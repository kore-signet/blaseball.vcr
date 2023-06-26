use std::ops::Bound;

use blaseball_vcr::{feed::db::FeedDatabase, VCRResult};
use clap::clap_app;

fn main() -> VCRResult<()> {
    let matches = clap_app!(analyze_tape =>
        (version: "1.0")
        (author: "emily signet <emily@sibr.dev>")
        (about: "blaseball.vcr simple feed reader")
        (@arg INPUT: +required -i --input [FILE] "tape file")
        (@arg INDEX: +required --idx [FILE] "feed index")

    )
    .get_matches();

    let db = FeedDatabase::from_tape(
        matches.value_of("INPUT").unwrap(),
        Some(matches.value_of("INDEX").unwrap()),
    )?;

    // let block = db.get_block_by_time(timestamp_to_millis(
    //     iso8601_timestamp::Timestamp::parse("2021-03-20T23:08:13.261Z").unwrap(),
    // ))?;
    // println!(
    //     "{}",
    //     serde_json::to_string_pretty(
    //         &block
    //             .event_at_time(timestamp_to_millis(
    //                 iso8601_timestamp::Timestamp::parse("2021-03-20T23:08:13.261Z").unwrap()
    //             ))
    //             // .event_at_index(5)
    //             .unwrap()
    //     )?
    // );

    // let indexes: FeedIndexCollection = rmp_serde::from_read(BufReader::new(File::open(
    //     matches.value_of("INDEX").unwrap(),
    // )?))?;

    // let after = timestamp_to_millis(Timestamp::parse("2021-10-05T17:00:00.508Z").unwrap());
    // let before = timestamp_to_millis(Timestamp::parse("2021-10-05T17:00:41.028Z").unwrap());

    // let game_iter = indexes.by_game(
    //     uuid::uuid!("b560d844-d6b0-42cb-a7d7-259bfd3f61ad"),
    //         after..=before,
    // ).unwrap();

    // for (_, chunk) in game_iter {
    //     let block = db.get_block_by_index(chunk.chunk)?;
    //     for id in &chunk.ids {
    //         let event = block.event_at_index(*id as usize).unwrap();
    //         if event.created >= after && event.created <= before {
    //             println!("{}", serde_json::to_string_pretty(&event)?);
    //         }
    //     }
    // }

    let events = db
        .events_by_game(
            uuid::uuid!("b560d844-d6b0-42cb-a7d7-259bfd3f61ad"),
            (Bound::Included(i64::MIN), Bound::Included(i64::MAX)),
            100,
            0,
        )
        .unwrap();
    println!("{}", serde_json::to_string_pretty(&events)?);

    Ok(())
}
