mod decode;
mod desc;
mod event;
pub use decode::*;
pub use desc::*;
pub use event::*;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

pub enum TagType {
    Team,
    Player,
    Game,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct MetaIndex {
    pub player_tags: HashMap<u16, Uuid>,
    pub game_tags: HashMap<u16, Uuid>,
    pub team_tags: HashMap<u8, Uuid>,
    pub reverse_player_tags: HashMap<Uuid, u16>,
    pub reverse_game_tags: HashMap<Uuid, u16>,
    pub reverse_team_tags: HashMap<Uuid, u8>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct EventIndex {
    pub player_index: HashMap<u16, Vec<(u32, (u32, u16))>>,
    pub game_index: HashMap<u16, Vec<(u32, (u32, u16))>>,
    pub team_index: HashMap<u8, Vec<(u32, (u32, u16))>>,
    pub phase_index: HashMap<(i8, u8), Vec<(i64, (u32, u16))>>,
}
