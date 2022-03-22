use blaseball_vcr::feed::event::FeedEvent;
use blaseball_vcr::feed::recorder::FeedDictTrainer;
use blaseball_vcr::VCRResult;
use clap::clap_app;
use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> VCRResult<()> {
    let matches = clap_app!(train_vhs_dict =>
        (version: "1.0")
        (author: "emily signet <emily@sibr.dev>")
        (about: "blaseball.vcr feed dict generator")
        (@arg INPUT: +required -i --input [FILE] "feed file")
        (@arg OUTPUT: +required -o --output [FILE] "output file for the trained dictionary")
    )
    .get_matches();

    let mut trainer = FeedDictTrainer::new();

    let input = BufReader::new(File::open(matches.value_of("INPUT").unwrap())?);

    for (i, line_chunk) in input.lines().chunks(100).into_iter().enumerate() {
        println!("processing chunk #{i}");

        let mut chunk = Vec::with_capacity(100);

        for line in line_chunk {
            chunk.push(serde_json::from_str::<FeedEvent>(&line?)?);
        }

        trainer.add_chunk(chunk);
    }

    std::fs::write(
        matches.value_of("OUTPUT").unwrap(),
        &trainer.train(112_000)?,
    )?;

    Ok(())
}
