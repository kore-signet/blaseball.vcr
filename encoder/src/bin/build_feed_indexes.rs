use blaseball_vcr::feed::event::FeedEvent;
use blaseball_vcr::feed::index::{FeedIndexBuilder, FeedIndexCollection};
use blaseball_vcr::feed::BlockMetadata;
use blaseball_vcr::VCRResult;
use clap::clap_app;
use vcr_lookups::{GAME_ID_TABLE, PLAYER_ID_TABLE, TEAM_ID_TABLE};

use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};

fn main() -> VCRResult<()> {
    let matches = clap_app!(train_vhs_dict =>
        (version: "1.0")
        (author: "emily signet <emily@sibr.dev>")
        (about: "blaseball.vcr gen 2 feed encoder")
        (@arg INPUT: +required -i --input [FILE] "feed file (ndjson)")
        (@arg OUTPUT: +required -o --output [FILE] "set output file for feed indexes")

    )
    .get_matches();

    let mut input = BufReader::new(File::open(matches.value_of("INPUT").unwrap())?).lines();

    let mut current_meta = BlockMetadata {
        tournament: i8::MIN,
        season: i8::MIN,
        phase: u8::MAX,
    };

    let mut chunk_idx = 0;
    let mut event_idx = 0;
    let mut event_within_chunk_idx = 0;

    // let mut indexes = FeedIndexCollection::new(GAME_ID_TABLE.len(), TEAM_ID_TABLE.len(), PLAYER_ID_TABLE.len());
    let mut games = FeedIndexBuilder::create(GAME_ID_TABLE.len());
    let mut teams = FeedIndexBuilder::create(TEAM_ID_TABLE.len());
    let mut players = FeedIndexBuilder::create(PLAYER_ID_TABLE.len());

    loop {
        let Some(line) = input.next() else { break };

        let event = serde_json::from_str::<FeedEvent>(&line?)?;
        let new_meta = BlockMetadata::from_event(&event);

        if event_within_chunk_idx > 0 && (event_within_chunk_idx >= 255 || current_meta != new_meta)
        {
            println!("processing chunk #{chunk_idx} (event #{event_idx})");
            if event_within_chunk_idx >= 255 {
                println!("reason: chunklen")
            } else if current_meta.season != new_meta.season {
                println!("reason: season")
            } else if current_meta.phase != new_meta.phase {
                println!("reason: phase");
            } else if current_meta.tournament != new_meta.tournament {
                println!("reason: tournament")
            }

            chunk_idx += 1;
            event_within_chunk_idx = 0;
        }

        current_meta = new_meta;

        for player in event.player_tags.as_deref().unwrap_or(&[]) {
            players.add(
                *PLAYER_ID_TABLE.map(player).unwrap() as usize,
                event.created,
                chunk_idx,
                event_within_chunk_idx,
            );
        }

        for team in event.team_tags.as_deref().unwrap_or(&[]) {
            teams.add(
                *TEAM_ID_TABLE.map(team).unwrap() as usize,
                event.created,
                chunk_idx,
                event_within_chunk_idx,
            );
        }

        for game in event.game_tags.as_deref().unwrap_or(&[]) {
            games.add(
                *GAME_ID_TABLE.map(game).unwrap() as usize,
                event.created,
                chunk_idx,
                event_within_chunk_idx,
            );
        }

        event_idx += 1;
        event_within_chunk_idx += 1;
    }

    println!("serializing...");
    let indexes = FeedIndexCollection {
        games: games.finish(),
        teams: teams.finish(),
        players: players.finish(),
    };

    let bytes = rmp_serde::to_vec(&indexes)?;

    let mut encoder = zstd::Encoder::new(
        BufWriter::new(File::create(matches.value_of("OUTPUT").unwrap())?),
        21,
    )?;
    encoder.long_distance_matching(true)?;
    encoder.set_pledged_src_size(Some(bytes.len() as u64))?;

    encoder.write_all(&bytes)?;

    encoder.finish()?.flush()?;

    Ok(())
}

// fn flush_chunk(recorder: &mut FeedRecorder<impl Write, impl Write>, chunk: &mut Vec<FeedEvent>, meta: BlockMetadata) -> VCRResult<()> {

//     Ok(())
// }
