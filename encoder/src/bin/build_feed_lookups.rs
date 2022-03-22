use blaseball_vcr::VCRResult;
use clap::clap_app;
use phf_codegen::Map as PhfMap;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use uuid::Uuid;

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct FeedEvent {
    player_tags: Option<Vec<Uuid>>,
    game_tags: Option<Vec<Uuid>>,
    team_tags: Option<Vec<Uuid>>,
}

fn main() -> VCRResult<()> {
    let matches = clap_app!(train_vhs_dict =>
        (version: "1.0")
        (author: "emily signet <emily@sibr.dev>")
        (about: "blaseball.vcr feed lookup table generator")
        (@arg INPUT: +required -i --input [FILE] "feed file")
        (@arg OUTPUT: +required -o --output [FILE] "output file for indexes")
    )
    .get_matches();

    let mut player_tag_table: HashMap<Uuid, u16> = HashMap::new();
    let mut team_tag_table: HashMap<Uuid, u8> = HashMap::new();
    let mut game_tag_table: HashMap<Uuid, u16> = HashMap::new();

    let reader = BufReader::new(File::open(matches.value_of("INPUT").unwrap())?);

    for line in reader.lines() {
        let event: FeedEvent = serde_json::from_str(&line?)?;
        for player in event.player_tags.unwrap_or(vec![]) {
            if !player_tag_table.contains_key(&player) {
                player_tag_table.insert(player, player_tag_table.len() as u16);
            }
        }

        for game in event.game_tags.unwrap_or(vec![]) {
            if !game_tag_table.contains_key(&game) {
                game_tag_table.insert(game, game_tag_table.len() as u16);
            }
        }

        for team in event.team_tags.unwrap_or(vec![]) {
            if !team_tag_table.contains_key(&team) {
                team_tag_table.insert(team, team_tag_table.len() as u8);
            }
        }
    }

    let mut phf_players: PhfMap<[u8; 16]> = PhfMap::new();
    let mut phf_games: PhfMap<[u8; 16]> = PhfMap::new();
    let mut phf_teams: PhfMap<[u8; 16]> = PhfMap::new();

    let mut player_tag_table: Vec<(Uuid, u16)> = player_tag_table.into_iter().collect();
    player_tag_table.sort_by_key(|&(_, v)| v);

    let mut game_tag_table: Vec<(Uuid, u16)> = game_tag_table.into_iter().collect();
    game_tag_table.sort_by_key(|&(_, v)| v);

    let mut team_tag_table: Vec<(Uuid, u8)> = team_tag_table.into_iter().collect();
    team_tag_table.sort_by_key(|&(_, v)| v);

    let mut player_tag_array: Vec<Uuid> = Vec::new();
    let mut game_tag_array: Vec<Uuid> = Vec::new();
    let mut team_tag_array: Vec<Uuid> = Vec::new();

    for (id, tag) in player_tag_table {
        player_tag_array.push(id);
        phf_players.entry(*id.as_bytes(), &tag.to_string());
    }

    for (id, tag) in game_tag_table {
        game_tag_array.push(id);
        phf_games.entry(*id.as_bytes(), &tag.to_string());
    }

    for (id, tag) in team_tag_table {
        team_tag_array.push(id);
        phf_teams.entry(*id.as_bytes(), &tag.to_string());
    }

    let mut out = BufWriter::new(File::create(matches.value_of("OUTPUT").unwrap())?);

    writeln!(&mut out, "/*")?;
    writeln!(&mut out, "this module contains the following tables:")?;
    writeln!(&mut out, "UUID_TO_PLAYER: uuid -> u16 player tag; phf::Map")?;
    writeln!(&mut out, "UUID_TO_GAME: uuid -> u16 game tag; phf::Map")?;
    writeln!(&mut out, "UUID_TO_TEAM: uuid -> u8 team tag; phf::Map")?;
    writeln!(&mut out, "-^~^-")?;
    writeln!(&mut out, "PLAYER_TO_UUID: u16 player tag -> uuid; array")?;
    writeln!(&mut out, "GAME_TO_UUID: u16 game tag -> uuid; array")?;
    writeln!(&mut out, "TEAM_TO_UUID: u8 team tag -> uuid; array")?;
    writeln!(&mut out, "*/")?;

    writeln!(
        &mut out,
        "pub static UUID_TO_PLAYER: phf::Map<[u8; 16], u16> = {};\n",
        phf_players.build()
    )?;
    writeln!(
        &mut out,
        "pub static UUID_TO_GAME: phf::Map<[u8; 16], u16> = {};\n",
        phf_games.build()
    )?;
    writeln!(
        &mut out,
        "pub static UUID_TO_TEAM: phf::Map<[u8; 16], u8> = {};\n",
        phf_teams.build()
    )?;

    let player_tag_len = player_tag_array.len();
    let player_tag_array = player_tag_array
        .into_iter()
        .map(|id| format!("uuid::Uuid::from_bytes({:?})", id.as_bytes()))
        .collect::<Vec<String>>();
    writeln!(
        &mut out,
        "pub const PLAYER_TO_UUID: [uuid::Uuid; {player_tag_len}] = [{}];",
        player_tag_array.join(",")
    )?;

    let game_tag_len = game_tag_array.len();
    let game_tag_array = game_tag_array
        .into_iter()
        .map(|id| format!("uuid::Uuid::from_bytes({:?})", id.as_bytes()))
        .collect::<Vec<String>>();
    writeln!(
        &mut out,
        "pub const GAME_TO_UUID: [uuid::Uuid; {game_tag_len}] = [{}];",
        game_tag_array.join(",")
    )?;

    let team_tag_len = team_tag_array.len();
    let team_tag_array = team_tag_array
        .into_iter()
        .map(|id| format!("uuid::Uuid::from_bytes({:?})", id.as_bytes()))
        .collect::<Vec<String>>();
    writeln!(
        &mut out,
        "pub const TEAM_TO_UUID: [uuid::Uuid; {team_tag_len}] = [{}];",
        team_tag_array.join(",")
    )?;

    out.flush()?;

    Ok(())
}
