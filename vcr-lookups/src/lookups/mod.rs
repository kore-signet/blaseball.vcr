// pub struct LookUpMap<K,V> {
//     mapper: PerfectMap<K,V>
// }

use perfect_map::PerfectMap;
use uuid::Uuid;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct IdLookUp {
    pub mapper: PerfectMap<Uuid, u16>,
    pub inverter: Vec<Uuid>,
}

impl IdLookUp {
    pub fn map(&self, id: &Uuid) -> Option<&u16> {
        self.mapper.get(id)
        // .filter(|k| self.inverter[**k as usize] == *id)
    }

    pub fn invert(&self, tag: u16) -> Option<&Uuid> {
        self.inverter.get(tag as usize)
    }
}

#[static_init::dynamic]
pub static GAME_ID_TABLE: IdLookUp =
    rmp_serde::from_slice(include_bytes!("./games.idmap")).unwrap();

#[static_init::dynamic]
pub static TEAM_ID_TABLE: IdLookUp =
    rmp_serde::from_slice(include_bytes!("./teams.idmap")).unwrap();

#[static_init::dynamic]
pub static PLAYER_ID_TABLE: IdLookUp =
    rmp_serde::from_slice(include_bytes!("./players.idmap")).unwrap();
// IdLookUp { mapper: PerfectMap::<Uuid, u16>::new::<u16>(&[], vec![]), inverter: vec![] };

// PITCHER_TO_GAMES: u16 -> u16 game tag; phf::Map
// TEAMS_TO_GAMES: u16 -> u16 game tag; phf::Map
// DATES_TO_GAMES: GameDate as [u8; 4] -> u16 game tag; phf::Map
// WEATHER_TO_GAMES: u8-> u16 game tag; phf::Map

#[static_init::dynamic]
pub static PITCHER_TO_GAMES: PerfectMap<u16, Vec<u16>> = {
    // PerfectMap::new(&[], Vec::<Vec<u32>>::new())
    rmp_serde::from_slice(include_bytes!("./games/pitchers.index")).unwrap()
};

#[static_init::dynamic]
pub static TEAMS_TO_GAMES: PerfectMap<u16, Vec<u16>> = {
    // PerfectMap::new(&[], Vec::<Vec<u32>>::new())

    rmp_serde::from_slice(include_bytes!("./games/teams.index")).unwrap()
};

#[static_init::dynamic]
pub static DATES_TO_GAMES: PerfectMap<[u8; 4], Vec<u16>> = {
    // PerfectMap::new(&[], Vec::<Vec<u32>>::new())

    rmp_serde::from_slice(include_bytes!("./games/dates.index")).unwrap()
};

#[static_init::dynamic]
pub static WEATHER_TO_GAMES: PerfectMap<u8, Vec<u16>> = {
    // PerfectMap::new(&[], Vec::<Vec<u32>>::new())

    rmp_serde::from_slice(include_bytes!("./games/weather.index")).unwrap()
};
