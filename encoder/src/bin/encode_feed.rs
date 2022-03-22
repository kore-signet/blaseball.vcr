use blaseball_vcr::feed::event::FeedEvent;
use blaseball_vcr::feed::recorder::FeedRecorder;
use blaseball_vcr::vhs::recorder::merge_tape;
use blaseball_vcr::VCRResult;
use clap::clap_app;
use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> VCRResult<()> {
    let matches = clap_app!(train_vhs_dict =>
        (version: "1.0")
        (author: "emily signet <emily@sibr.dev>")
        (about: "blaseball.vcr gen 2 feed encoder")
        (@arg COMPRESSION_LEVEL: -l --level [LEVEL] "set compression level")
        (@arg ZSTD_DICT: -d --dict [DICT] "set dict for tape")
        (@arg INPUT: +required -i --input [FILE] "feed file (ndjson)")
        (@arg OUTPUT: +required -o --output [FILE] "set output file for tape")
    )
    .get_matches();

    let dict = if let Some(path) = matches.value_of("ZSTD_DICT") {
        Some(std::fs::read(path)?)
    } else {
        None
    };

    let mut recorder: FeedRecorder<File, File> = FeedRecorder::new(
        tempfile::tempfile()?,
        tempfile::tempfile()?,
        dict.clone(),
        matches
            .value_of("COMPRESSION_LEVEL")
            .and_then(|v| v.parse::<i32>().ok())
            .unwrap_or(11),
    )?;

    let input = BufReader::new(File::open(matches.value_of("INPUT").unwrap())?);

    for (i, line_chunk) in input.lines().chunks(100).into_iter().enumerate() {
        println!("processing chunk #{i}");

        let mut chunk = Vec::with_capacity(100);

        for line in line_chunk {
            chunk.push(serde_json::from_str::<FeedEvent>(&line?)?);
        }

        recorder.add_chunk(chunk)?;
    }

    let (mut header, mut main) = recorder.finish()?;
    let out = std::fs::File::create(matches.value_of("OUTPUT").unwrap())?;

    use std::io::Seek;
    header.rewind()?;
    main.rewind()?;

    merge_tape(header, main, dict.as_deref(), out)?;

    Ok(())
}
