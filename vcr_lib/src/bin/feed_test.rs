use blaseball_vcr::feed::db::*;
use blaseball_vcr::feed::event::*;
use blaseball_vcr::*;

fn main() -> VCRResult<()> {
    let db = FeedDatabase::from_single("./vhs_tapes/feed.vhs")?;
    println!(
        "read whole block: {}",
        easybench::bench(|| db.get_block_by_time(1627669026894))
    );

    let block = db.get_block_by_time(1627669026894)?;

    println!(
        "read from block: {}",
        easybench::bench(|| block.event_at_time(1627669026894))
    );
    let event = block.event_at_time(1627669026894);

    println!("{}", serde_json::to_string(&event)?);

    Ok(())
}
