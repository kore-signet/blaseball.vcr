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

use blaseball_vcr::site::AssetManager;
use serde::Serialize;

mod fairings;
mod paging;
mod site;
mod v1;
mod v2;

use fairings::*;
use paging::*;
use v2::*;

use blaseball_vcr::db_manager::DatabaseManager;
use blaseball_vcr::vhs::schemas::*;
use blaseball_vcr::{call_method_by_type, db_wrapper};
use std::time::Duration;

#[derive(Serialize)]
pub struct ChronV1Response<T> {
    #[serde(rename = "nextPage")]
    next_page: Option<String>,
    data: Vec<T>,
}

#[derive(Serialize)]
pub struct ChronResponse<T> {
    #[serde(rename = "nextPage")]
    next_page: Option<String>,
    #[serde(rename = "items")]
    data: Vec<T>,
}

pub type DynChronResponse = ChronResponse<DynamicChronEntity>;

#[launch]
fn rocket() -> _ {
    let mut db_manager = DatabaseManager::new();

    for entry in std::fs::read_dir("./vhs_tapes").unwrap() {
        if let Ok(entry) = entry {
            let path = entry.path();
            let stem = path.file_stem().unwrap().to_string_lossy().to_owned();
            println!("-> loading {}", stem);
            call_method_by_type!(
                db_wrapper::from_single_and_insert,
                (&mut db_manager, &entry.path()),
                stem.as_ref(),
                { continue }
            )
            .unwrap();
        }
    }

    let site_manager = AssetManager::from_single("vhs_tapes/site_assets.vhs").unwrap();

    rocket::build()
        .manage(db_manager)
        .attach(RequestTimer)
        .manage(site_manager)
        .manage(PageManager::new(256, Duration::from_secs(10 * 60)))
        .mount(
            "/",
            routes![
                entities,
                versions,
                v1::games,
                v1::game_updates,
                site::site_updates,
                site::site_download
            ],
        )
}
