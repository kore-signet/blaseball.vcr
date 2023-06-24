use std::{
    collections::BTreeMap,
    io::{self, Seek, Write},
};

use dusk_varint::VarInt;

use serde::{Deserialize, Serialize};
use vcr_lookups::UuidShell;
use zstd::bulk::Compressor;

use crate::{timestamp_to_nanos, RawChroniclerEntity};

use super::{
    command::{
        CommandKind::{self, Delta},
        TributeAffects, TributeCommand,
    },
    TributePosition, TributesTapeHeader,
};

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum JsonTributes {
    Players(Vec<PlayerTribute>),
    PlayersAndTeams {
        players: Vec<PlayerTribute>,
        teams: Vec<TeamTribute>,
    },
}

impl JsonTributes {
    // (players, teams)
    pub fn split(&self) -> (BTreeMap<u16, i64>, BTreeMap<u16, i64>) {
        match self {
            JsonTributes::Players(players) => (
                players
                    .iter()
                    .copied()
                    .map(|PlayerTribute { player_id, peanuts }| {
                        (player_id.as_tag_value().unwrap(), peanuts)
                    })
                    .collect(),
                BTreeMap::new(),
            ),
            JsonTributes::PlayersAndTeams { players, teams } => (
                players
                    .iter()
                    .copied()
                    .map(|PlayerTribute { player_id, peanuts }| {
                        (player_id.as_tag_value().unwrap(), peanuts)
                    })
                    .collect(),
                teams
                    .iter()
                    .copied()
                    .map(|TeamTribute { team_id, peanuts }| {
                        (team_id.as_tag_value().unwrap(), peanuts)
                    })
                    .collect(),
            ),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub struct TeamTribute {
    team_id: UuidShell,
    peanuts: i64,
}

#[derive(Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub struct PlayerTribute {
    player_id: UuidShell,
    peanuts: i64,
}

pub struct TributesRecorder<W: TributeWriter> {
    times: Vec<i64>,
    positions: Vec<TributePosition>,
    output: W,
    players: BTreeMap<u16, i64>,
    teams: BTreeMap<u16, i64>,
}

impl<W: TributeWriter> TributesRecorder<W> {
    pub fn new(output: W) -> TributesRecorder<W> {
        TributesRecorder {
            positions: Vec::with_capacity(2000),
            times: Vec::with_capacity(2000),
            output,
            players: BTreeMap::new(),
            teams: BTreeMap::new(),
        }
    }

    pub fn add_version(&mut self, ver: &RawChroniclerEntity<JsonTributes>) -> io::Result<()> {
        let RawChroniclerEntity {
            valid_from, data, ..
        } = ver;
        let (players, teams) = data.split();

        let mut block = BlockWriter::default();

        for old_player in self.players.keys() {
            if !players.contains_key(old_player) {
                block.remove(*old_player, TributeAffects::Player);
            }
        }

        for old_team in self.teams.keys() {
            if !teams.contains_key(old_team) {
                block.remove(*old_team, TributeAffects::Team);
            }
        }

        for (new_player, new_val) in players {
            match self.players.get(&new_player) {
                Some(old_val) => {
                    // assert!(new_val > *old_val);
                    block.delta(new_player, new_val - old_val, TributeAffects::Player)
                }
                None => {
                    block.delta(new_player, new_val, TributeAffects::Player);
                }
            }

            self.players.insert(new_player, new_val);
        }

        for (new_team, new_val) in teams {
            match self.teams.get(&new_team) {
                Some(old_val) => {
                    // assert!(new_val > *old_val);
                    // assert!(new_val == old_val + (new_val - old_val));
                    block.delta(new_team, new_val - old_val, TributeAffects::Team)
                }
                None => {
                    block.delta(new_team, new_val, TributeAffects::Team);
                }
            }

            self.teams.insert(new_team, new_val);
        }

        self.times.push(timestamp_to_nanos(*valid_from));
        self.positions.push(self.output.write_block(&block.bytes)?);

        Ok(())
    }

    pub fn finish(self) -> io::Result<(Vec<u8>, W)> {
        let header_serialized = rmp_serde::to_vec_named(&TributesTapeHeader {
            times: self.times,
            positions: self.positions,
        })
        .unwrap();

        Ok((header_serialized, self.output))
    }
}

#[derive(Default)]
pub struct BlockWriter {
    bytes: Vec<u8>,
}

impl BlockWriter {
    pub fn delta(&mut self, id: u16, delta: i64, affects: TributeAffects) {
        let mut cmd = TributeCommand::new()
            .with(TributeCommand::ID, id)
            .with(TributeCommand::KIND, Delta)
            .with(TributeCommand::AFFECTS, affects)
            .with(TributeCommand::EMBEDDED_DELTA, false);

        let embedded = if ((delta as i16) as u32) <= TributeCommand::DELTA.max_value() {
            cmd.set(TributeCommand::EMBEDDED_DELTA, true);
            cmd.set(TributeCommand::DELTA, delta as i16);
            true
        } else {
            false
        };
        // let embedded = false;

        self.write_command(&cmd);

        if !embedded {
            let mut bytes = vec![0u8; delta.required_space()];
            delta.encode_var(&mut bytes[..]);
            self.bytes.extend_from_slice(&bytes);
        }
    }

    fn write_command(&mut self, cmd: &TributeCommand) {
        let cmd_ptr: *const TributeCommand = cmd;

        self.bytes
            .extend_from_slice(unsafe { std::slice::from_raw_parts(cmd_ptr as *const u8, 4) });
    }

    pub fn remove(&mut self, id: u16, affects: TributeAffects) {
        self.write_command(
            &TributeCommand::new()
                .with(TributeCommand::ID, id)
                .with(TributeCommand::KIND, CommandKind::Remove)
                .with(TributeCommand::AFFECTS, affects),
        );
    }
}

#[derive(Default)]
pub struct DictTrainer {
    samples: Vec<Vec<u8>>,
    position: u64,
}

impl DictTrainer {
    pub fn into_dict(self, size: usize) -> io::Result<Vec<u8>> {
        zstd::dict::from_samples(&self.samples, size)
    }
}

pub trait TributeWriter {
    fn write_block(&mut self, block: &[u8]) -> io::Result<TributePosition>;
}

impl TributeWriter for DictTrainer {
    fn write_block(&mut self, block: &[u8]) -> io::Result<TributePosition> {
        assert!(!block.is_empty());
        self.samples.push(block.to_vec());
        self.position += block.len() as u64;
        Ok(TributePosition {
            start: 0,
            end: 0,
            uncompressed_len: 0,
        })
    }
}

pub struct TributeCompressor<W: Write + Seek> {
    compressor: Compressor<'static>,
    output: W,
    lens: Vec<usize>,
    compressed_lens: Vec<usize>,
}

impl<W: Write + Seek> TributeCompressor<W> {
    pub fn new(output: W, dict: &[u8]) -> io::Result<TributeCompressor<W>> {
        Ok(TributeCompressor {
            compressor: Compressor::with_dictionary(23, dict)?,
            output,
            lens: Vec::new(),
            compressed_lens: Vec::new(),
        })
    }

    pub fn finish(mut self) -> io::Result<W> {
        self.output.flush()?;
        Ok(self.output)
    }
}

impl<W: Write + Seek> TributeWriter for TributeCompressor<W> {
    fn write_block(&mut self, block: &[u8]) -> io::Result<TributePosition> {
        let start = self.output.stream_position()?;

        let compressed = self.compressor.compress(block)?;

        self.lens.push(block.len());
        self.compressed_lens.push(compressed.len());

        self.output.write_all(&compressed)?;

        self.output.flush()?;

        let end = self.output.stream_position()?;

        Ok(TributePosition {
            start,
            end,
            uncompressed_len: block.len() as u64,
        })
    }
}
