use blaseball_vcr::vhs::recorder::merge_tape;
use blaseball_vcr::vhs::tributes::recorder::{
    DictTrainer, JsonTributes, TributeCompressor, TributesRecorder,
};
use blaseball_vcr::{RawChroniclerEntity, VCRResult};
use clap::clap_app;

use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::Path;

fn main() -> VCRResult<()> {
    let matches = clap_app!(train_vhs_dict =>
        (version: "1.0")
        (author: "emily signet <emily@sibr.dev>")
        (about: "blaseball.vcr gen 2 tributes encoder")
        (@arg INPUT: +required -i --input [FILE] "input NDJSON tributes file")
        (@arg OUTPUT: +required -o --output [FILE] "set output file for tape")
    )
    .get_matches();

    let versions = read_versions(matches.value_of("INPUT").unwrap()).unwrap();

    let dict = train_dict(&versions).unwrap();

    // let mut out = Cursor::new(Vec::new());
    let mut writer = TributesRecorder::new(TributeCompressor::new(
        BufWriter::new(tempfile::tempfile()?),
        &dict,
    )?);

    for version in &versions {
        writer.add_version(version)?;
    }

    let (header, compressor) = writer.finish()?;
    let mut tape_file = compressor.finish()?;

    tape_file.flush()?;

    let mut tape_file = tape_file.into_inner().unwrap();

    use std::io::Seek;
    tape_file.rewind()?;

    merge_tape(
        header,
        tape_file,
        Some(dict).as_deref(),
        File::create(matches.value_of("OUTPUT").unwrap())?,
    )?;
    // println!("{}", out.into_inner().len());

    Ok(())
}

fn train_dict(versions: &[RawChroniclerEntity<JsonTributes>]) -> VCRResult<Vec<u8>> {
    let mut dict_trainer = TributesRecorder::new(DictTrainer::default());

    for version in versions {
        dict_trainer.add_version(version)?;
    }

    let (_, dict_trainer) = dict_trainer.finish().unwrap();

    let dict = dict_trainer.into_dict(112_000).unwrap();

    Ok(dict)
}

fn read_versions(path: impl AsRef<Path>) -> VCRResult<Vec<RawChroniclerEntity<JsonTributes>>> {
    let input = BufReader::new(File::open(path.as_ref())?);
    let mut out = Vec::with_capacity(100_000);

    for line in input.lines() {
        out.push(serde_json::from_str(&line?)?);
    }

    Ok(out)
}
