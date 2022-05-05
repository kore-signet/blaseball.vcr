// mod paging;
// use blaseball_vcr::db_manager::*;
// use blaseball_vcr::*;
// use paging::*;

// fn main() -> VCRResult<()> {

//     let mut page = Page::entities(
//         u32::MAX,
//         db_manager.all_entity_ids::<Player>().unwrap().to_vec(),
//     );

//     let mut total = 0;

//     loop {
//         let v = page.take_n::<Player>(&db_manager, 100).unwrap();
//         if v.is_empty() {
//             break;
//         }

//         println!("iter:  got {}", v.len());
//         total += v.len();
//     }

//     println!("end: got {total}");

//     Ok(())
// }

#[macro_use]
extern crate rocket;

use serde::Serialize;

mod fairings;
mod paging;
mod v1;
mod v2;

use fairings::*;
use paging::*;
use v2::*;

use blaseball_vcr::db_manager::DatabaseManager;

use blaseball_vcr::vhs::{db::*, schemas::*};

use std::time::Duration;

#[derive(Serialize)]
pub struct ChronResponse {
    #[serde(rename = "nextPage")]
    next_page: Option<String>,
    data: Vec<DynamicChronEntity>,
}

#[launch]
fn rocket() -> _ {
    let mut db_manager = DatabaseManager::new();
    let player_db: Database<Player> = Database::from_single("./vhs_tapes/player.vhs").unwrap();

    db_manager.insert(player_db);

    let game_db: Database<GameUpdate> = Database::from_single("./vhs_tapes/games.vhs").unwrap();
    db_manager.insert(game_db);

    rocket::build()
        .manage(db_manager)
        .attach(RequestTimer)
        .manage(PageManager::new(256, Duration::from_secs(10 * 60)))
        .mount("/", routes![entities, versions, v1::games])
}
