use blaseball_vcr::vhs::recorder::*;
use blaseball_vcr::vhs::schemas::game::GameUpdate;
use blaseball_vcr::VCRResult;
use new_encoder::ChronV1GameUpdate;

use std::fs::File;
use std::io::{self, BufReader, Read};

use clap::clap_app;
use zstd::bulk::Decompressor;

fn main() -> VCRResult<()> {
    let matches = clap_app!(train_vhs_dict =>
        (version: "1.0")
        (author: "emily signet <emily@sibr.dev>")
        (about: "blaseball.vcr gen 2 games dict trainer")
        (@arg INPUT: +required -i --input [INPUT] "input games file")
        (@arg OUTPUT: +required -o --output [FILE] "output file for zstd dictionary")
    )
    .get_matches();

    let mut reader = BufReader::new(File::open(matches.value_of("INPUT").unwrap())?);
    let mut decompressor = Decompressor::new()?;

    let mut trainer = DictTrainer::new(u16::MAX as usize);

    let mut i = 0;

    loop {
        let mut len_buf: [u8; 8] = [0; 8];
        if let Err(e) = reader.read_exact(&mut len_buf) {
            if e.kind() == io::ErrorKind::UnexpectedEof {
                break;
            } else {
                return Err(blaseball_vcr::VCRError::IOError(e));
            }
        }

        let compressed_len = u64::from_le_bytes(len_buf);
        reader.read_exact(&mut len_buf)?;
        let decompressed_len = u64::from_le_bytes(len_buf);

        let mut buf: Vec<u8> = vec![0; compressed_len as usize];
        reader.read_exact(&mut buf)?;
        let decompressed = decompressor.decompress(&buf, decompressed_len as usize)?;

        let deser_mrow = &mut serde_json::Deserializer::from_slice(&decompressed[..]);

        // let game_data: Vec<ChronV1GameUpdate<GameUpdate>> =
        // serde_json::from_slice(&decompressed[..]).unwrap();

        let game_data: Vec<ChronV1GameUpdate<GameUpdate>> =
            match serde_path_to_error::deserialize(deser_mrow) {
                Ok(v) => v,
                Err(e) => {
                    println!("{}", e.path().to_string());
                    panic!()
                }
            };

        let data: Vec<GameUpdate> = game_data.into_iter().map(|v| v.data).collect();

        trainer.add_entity(data)?;

        i += 1;

        println!("game #{i}");
    }

    std::fs::write(matches.value_of("OUTPUT").unwrap(), trainer.train(112000)?)?;

    Ok(())
}
