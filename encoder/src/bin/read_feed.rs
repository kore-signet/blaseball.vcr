use blaseball_vcr::{feed::db::FeedDatabase, timestamp_to_millis, VCRResult};
use clap::clap_app;

fn main() -> VCRResult<()> {
    let matches = clap_app!(analyze_tape =>
        (version: "1.0")
        (author: "emily signet <emily@sibr.dev>")
        (about: "blaseball.vcr simple tape analyzer")
        (@arg INPUT: +required -i --input [FILE] "tape file")
    )
    .get_matches();

    let db = FeedDatabase::from_single(matches.value_of("INPUT").unwrap())?;
    let block = db.get_block_by_time(timestamp_to_millis(
        iso8601_timestamp::Timestamp::parse("2021-03-20T23:08:13.261Z").unwrap(),
    ))?;
    println!(
        "{}",
        serde_json::to_string_pretty(
            &block
                .event_at_time(timestamp_to_millis(
                    iso8601_timestamp::Timestamp::parse("2021-03-20T23:08:13.261Z").unwrap()
                ))
                // .event_at_index(5)
                .unwrap()
        )?
    );

    Ok(())
}
