use blaseball_vcr::feed::lookup_tables::*;
use blaseball_vcr::*;
use clap::clap_app;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufWriter, Write};
use uuid::Uuid;

macro_rules! build_index {
    ($table:expr, $key_ty:ty, $out:expr, $declr:literal) => {{
        let mut generator: phf_codegen::Map<$key_ty> = phf_codegen::Map::new();
        for (key, val) in $table {
            let val_array = val
                .into_iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>()
                .join(",");
            generator.entry(key, &format!("&[{val_array}]"));
        }

        writeln!($out, $declr, generator.build())?;
    }};
}

#[derive(Deserialize)]
struct GameIndex {
    by_pitcher: HashMap<Uuid, Vec<Uuid>>,
    by_team: HashMap<Uuid, Vec<Uuid>>,
    by_date: HashMap<String, Vec<Uuid>>,
    by_weather: HashMap<i32, Vec<Uuid>>,
}

fn main() -> VCRResult<()> {
    let matches = clap_app!(train_vhs_dict =>
        (version: "1.0")
        (author: "emily signet <emily@sibr.dev>")
        (about: "blaseball.vcr game lookup table generator")
        (@arg INPUT: +required -i --input [FILE] "game index file")
        (@arg OUTPUT: +required -o --output [FILE] "output file for indexes")
    )
    .get_matches();

    let index: GameIndex = serde_json::from_reader(BufReader::new(File::open(
        matches.value_of("INPUT").unwrap(),
    )?))?;

    let mut output_file = BufWriter::new(File::create(matches.value_of("OUTPUT").unwrap())?);

    let pitcher_index: Vec<(u16, Vec<u16>)> = index
        .by_pitcher
        .into_iter()
        .map(|(k, v)| {
            (
                UUID_TO_PLAYER[k.as_bytes()],
                v.into_iter().map(|g| UUID_TO_GAME[g.as_bytes()]).collect(),
            )
        })
        .collect();

    let team_index: Vec<(u8, Vec<u16>)> = index
        .by_team
        .into_iter()
        .map(|(k, v)| {
            (
                UUID_TO_TEAM[k.as_bytes()],
                v.into_iter().map(|g| UUID_TO_GAME[g.as_bytes()]).collect(),
            )
        })
        .collect();

    let date_index: Vec<([u8; 4], Vec<u16>)> = index
        .by_date
        .into_iter()
        .map(|(k, v)| {
            let date_components: Vec<i16> = k
                .splitn(3, ':')
                .map(|v| v.parse::<i16>().unwrap())
                .collect();
            (
                (GameDate {
                    day: date_components[0],
                    season: date_components[1].try_into().unwrap(),
                    tournament: date_components[2].try_into().unwrap(),
                })
                .to_bytes(),
                v.into_iter().map(|g| UUID_TO_GAME[g.as_bytes()]).collect(),
            )
        })
        .collect();

    let weather_index: Vec<(u8, Vec<u16>)> = index
        .by_weather
        .into_iter()
        .map(|(k, v)| {
            (
                k.try_into().unwrap(),
                v.into_iter().map(|g| UUID_TO_GAME[g.as_bytes()]).collect(),
            )
        })
        .collect();

    writeln!(&mut output_file, "/*")?;
    writeln!(
        &mut output_file,
        "this module contains the following tables:"
    )?;
    writeln!(
        &mut output_file,
        "PITCHER_TO_GAMES: u16 -> u16 game tag; phf::Map"
    )?;
    writeln!(
        &mut output_file,
        "TEAMS_TO_GAMES: u8 -> u16 game tag; phf::Map"
    )?;
    writeln!(
        &mut output_file,
        "DATES_TO_GAMES: GameDate as [u8; 4] -> u16 game tag; phf::Map"
    )?;
    writeln!(
        &mut output_file,
        "WEATHER_TO_GAMES: u8-> u16 game tag; phf::Map"
    )?;
    writeln!(&mut output_file, "-^~^-")?;
    writeln!(&mut output_file, "*/")?;

    build_index!(
        pitcher_index,
        u16,
        &mut output_file,
        "pub static PITCHER_TO_GAMES: phf::Map<u16, &'static [u16]> = \n{};\n"
    );

    build_index!(
        team_index,
        u8,
        &mut output_file,
        "pub static TEAMS_TO_GAMES: phf::Map<u8, &'static [u16]> = \n{};\n"
    );

    build_index!(
        date_index,
        [u8; 4],
        &mut output_file,
        "pub static DATES_TO_GAMES: phf::Map<[u8; 4], &'static [u16]> = \n{};\n"
    );

    build_index!(
        weather_index,
        u8,
        &mut output_file,
        "pub static WEATHER_TO_GAMES: phf::Map<u8, &'static [u16]> = \n{};\n"
    );

    output_file.flush()?;

    Ok(())
}
