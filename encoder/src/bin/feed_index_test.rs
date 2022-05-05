use blaseball_vcr::feed::db::*;

use blaseball_vcr::feed::lookup_tables::*;
use blaseball_vcr::*;
use fst::{automaton::Str as StrAutomaton, Automaton, IntoStreamer, Map as FstMap, Streamer};

use uuid::Uuid;

fn main() -> VCRResult<()> {
    let _db = FeedDatabase::from_single("./vhs_tapes/feed.vhs")?;

    let memory = std::fs::read("feed_playertags.fst")?;
    let idx = FstMap::new(memory).unwrap();

    let mut prefix: Vec<u8> = Vec::new();
    prefix.extend_from_slice(
        &UUID_TO_PLAYER[Uuid::parse_str("81d7d022-19d6-427d-aafc-031fcb79b29e")
            .unwrap()
            .as_bytes()]
        .to_be_bytes(),
    );
    prefix.extend_from_slice(&(1627669026u32.to_be_bytes()));
    println!(
        "{}",
        easybench::bench(|| {
            let mut stream = idx
                .search(
                    StrAutomaton::new(unsafe { std::str::from_utf8_unchecked(&prefix[..]) })
                        .starts_with(),
                )
                .into_stream();
            while let Some((_k, v)) = stream.next() {
                let v = v.to_le();

                let _block_idx = (v >> 16) as u16;
                let _event_idx = (v & (u16::MAX as u64)) as usize;

                // let block = db.get_block_by_index(block_idx).unwrap();
                // let event = block.event_at_index(event_idx);
            }
        })
    );
    // println!("{}", i);

    Ok(())
}
