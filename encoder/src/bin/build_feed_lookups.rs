use blaseball_vcr::VCRResult;
use clap::clap_app;
use perfect_map::PerfectMap;
use serde::Serialize;
use std::collections::HashMap;
use std::fs::File;
use std::hash::Hash;
use std::io::{BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};
use uuid::Uuid;
use vcr_lookups::IdLookUp;

#[derive(serde::Deserialize)]
struct IdIndex {
    games: Vec<Uuid>,
    teams: Vec<Uuid>,
    players: Vec<Uuid>,
}

fn main() -> VCRResult<()> {
    let matches = clap_app!(train_vhs_dict =>
        (version: "1.0")
        (author: "emily signet <emily@sibr.dev>")
        (about: "blaseball.vcr feed lookup table generator")
        (@arg INPUT: +required -i --input [FILE] "id index file")
        (@arg OUTPUT: +required -o --output [FILE] "output folder for indexes")
    )
    .get_matches();

    let mut player_tag_table: HashMap<Uuid, u32> = HashMap::new();
    let mut team_tag_table: HashMap<Uuid, u32> = HashMap::new();
    let mut game_tag_table: HashMap<Uuid, u32> = HashMap::new();

    let index: IdIndex = serde_json::from_reader(BufReader::new(File::open(
        matches.value_of("INPUT").unwrap(),
    )?))?;

    let path = PathBuf::from(matches.value_of("OUTPUT").unwrap());

    for game in index.games {
        if !game_tag_table.contains_key(&game) {
            game_tag_table.insert(game, game_tag_table.len() as u32);
        }
    }

    for team in index.teams {
        if !team_tag_table.contains_key(&team) {
            team_tag_table.insert(team, team_tag_table.len() as u32);
        }
    }

    for player in index.players {
        if !player_tag_table.contains_key(&player) {
            player_tag_table.insert(player, player_tag_table.len() as u32);
        }
    }

    write_map(player_tag_table, path.join("players.idmap"))?;
    write_map(team_tag_table, path.join("teams.idmap"))?;
    write_map(game_tag_table, path.join("games.idmap"))?;

    // let mut phf_players: PhfMap<[u8; 16]> = PhfMap::new();
    // let mut phf_games: PhfMap<[u8; 16]> = PhfMap::new();
    // let mut phf_teams: PhfMap<[u8; 16]> = PhfMap::new();

    // write_map(player_ids, player_tags.clone(), path.join("players.phfmap"))

    // let mut game_tag_table: Vec<(Uuid, u32)> = game_tag_table.into_iter().collect();
    // game_tag_table.sort_by_key(|&(_, v)| v);

    // let mut team_tag_table: Vec<(Uuid, u32)> = team_tag_table.into_iter().collect();
    // team_tag_table.sort_by_key(|&(_, v)| v);

    // let mut player_tag_array: Vec<Uuid> = Vec::new();
    // let mut game_tag_array: Vec<Uuid> = Vec::new();
    // let mut team_tag_array: Vec<Uuid> = Vec::new();

    // for (id, tag) in player_tag_table {
    //     player_tag_array.push(id);
    //     phf_players.entry(*id.as_bytes(), &tag.to_string());
    // }

    // for (id, tag) in game_tag_table {
    //     game_tag_array.push(id);
    //     phf_games.entry(*id.as_bytes(), &tag.to_string());
    // }

    // for (id, tag) in team_tag_table {
    //     team_tag_array.push(id);
    //     phf_teams.entry(*id.as_bytes(), &tag.to_string());
    // }

    // let mut out = BufWriter::new(File::create(matches.value_of("OUTPUT").unwrap())?);

    // writeln!(&mut out, "/*")?;
    // writeln!(&mut out, "this module contains the following tables:")?;
    // writeln!(&mut out, "UUID_TO_PLAYER: uuid -> u16 player tag; phf::Map")?;
    // writeln!(&mut out, "UUID_TO_GAME: uuid -> u16 game tag; phf::Map")?;
    // writeln!(&mut out, "UUID_TO_TEAM: uuid -> u16 team tag; phf::Map")?;
    // writeln!(&mut out, "-^~^-")?;
    // writeln!(&mut out, "PLAYER_TO_UUID: u32 player tag -> uuid; array")?;
    // writeln!(&mut out, "GAME_TO_UUID: u32 game tag -> uuid; array")?;
    // writeln!(&mut out, "TEAM_TO_UUID: u32 team tag -> uuid; array")?;
    // writeln!(&mut out, "*/")?;

    // writeln!(
    //     &mut out,
    //     "pub static UUID_TO_PLAYER: phf::Map<[u8; 16], u32> = {};\n",
    //     phf_players.build()
    // )?;
    // writeln!(
    //     &mut out,
    //     "pub static UUID_TO_GAME: phf::Map<[u8; 16], u32> = {};\n",
    //     phf_games.build()
    // )?;
    // writeln!(
    //     &mut out,
    //     "pub static UUID_TO_TEAM: phf::Map<[u8; 16], u32> = {};\n",
    //     phf_teams.build()
    // )?;

    // let player_tag_len = player_tag_array.len();
    // let player_tag_array = player_tag_array
    //     .into_iter()
    //     .map(|id| format!("uuid::Uuid::from_bytes({:?})", id.as_bytes()))
    //     .collect::<Vec<String>>();
    // writeln!(
    //     &mut out,
    //     "pub const PLAYER_TO_UUID: [uuid::Uuid; {player_tag_len}] = [{}];",
    //     player_tag_array.join(",")
    // )?;

    // let game_tag_len = game_tag_array.len();
    // let game_tag_array = game_tag_array
    //     .into_iter()
    //     .map(|id| format!("uuid::Uuid::from_bytes({:?})", id.as_bytes()))
    //     .collect::<Vec<String>>();
    // writeln!(
    //     &mut out,
    //     "pub const GAME_TO_UUID: [uuid::Uuid; {game_tag_len}] = [{}];",
    //     game_tag_array.join(",")
    // )?;

    // let team_tag_len = team_tag_array.len();
    // let team_tag_array = team_tag_array
    //     .into_iter()
    //     .map(|id| format!("uuid::Uuid::from_bytes({:?})", id.as_bytes()))
    //     .collect::<Vec<String>>();
    // writeln!(
    //     &mut out,
    //     "pub const TEAM_TO_UUID: [uuid::Uuid; {team_tag_len}] = [{}];",
    //     team_tag_array.join(",")
    // )?;

    // out.flush()?;

    Ok(())
}

fn write_map(map: HashMap<Uuid, u32>, to: impl AsRef<Path>) -> std::io::Result<()> {
    let mut out = BufWriter::new(File::create(to.as_ref())?);

    let mut table: Vec<(Uuid, u32)> = map.into_iter().collect();
    table.sort_by_key(|&(_, v)| v);

    let (ids, tags): (Vec<_>, Vec<_>) = table.into_iter().unzip();

    let map: PerfectMap<Uuid, u32> = PerfectMap::new(&ids, tags);

    rmp_serde::encode::write_named(
        &mut out,
        &IdLookUp {
            mapper: map,
            inverter: ids,
        },
    )
    .unwrap();
    // let map: PerfectMap<K, V> = PerfectMap::new(keys, vals);

    // rmp_serde::encode::write_named(&mut out, &map).unwrap();

    Ok(())
}
