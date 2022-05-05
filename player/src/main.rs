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

use blaseball_vcr::vhs::{schemas::*};

use std::time::Duration;

#[macro_export]
macro_rules! call_method_by_type {
    ($obj:ident $sep:tt $method_name:ident, $args:tt, $ty:expr, $last_case:block) => {
        match $ty {
            "gameupdate" => $obj$sep$method_name::<GameUpdate>$args,
            "bossfight" => $obj$sep$method_name::<Bossfight>$args,
            "communitychestprogress" => $obj$sep$method_name::<CommunityChestProgress>$args,
            "division" => $obj$sep$method_name::<Division>$args,
            "league" => $obj$sep$method_name::<League>$args,
            "playoffmatchup" => $obj$sep$method_name::<Playoffmatchup>$args,
            "playoffround" => $obj$sep$method_name::<Playoffround>$args,
            "playoffs" => $obj$sep$method_name::<Playoffs>$args,
            "season" => $obj$sep$method_name::<Season>$args,
            "sim" => $obj$sep$method_name::<Sim>$args,
            "stadium" => $obj$sep$method_name::<Stadium>$args,
            "standings" => $obj$sep$method_name::<Standings>$args,
            "subleague" => $obj$sep$method_name::<Subleague>$args,
            "team" => $obj$sep$method_name::<Team>$args,
            "sunsun" => $obj$sep$method_name::<Sunsun>$args,
            "temporal" => $obj$sep$method_name::<Temporal>$args,
            "tiebreakers" => $obj$sep$method_name::<TiebreakerWrapper>$args,
            "tournament" => $obj$sep$method_name::<Tournament>$args,
            "bonusresult" => $obj$sep$method_name::<Bonusresult>$args,
            "decreeresult" => $obj$sep$method_name::<Decreeresult>$args,
            "eventresult" => $obj$sep$method_name::<Eventresult>$args,
            "fuelprogress" => $obj$sep$method_name::<FuelprogressWrapper>$args,
            "giftprogress" => $obj$sep$method_name::<Giftprogress>$args,
            "globalevents" => $obj$sep$method_name::<GlobaleventsWrapper>$args,
            "idols" => $obj$sep$method_name::<IdolsWrapper>$args,
            "item" => $obj$sep$method_name::<Item>$args,
            "librarystory" => $obj$sep$method_name::<LibrarystoryWrapper>$args,
            "nullified" => $obj$sep$method_name::<NullifiedWrapper>$args,
            "offseasonrecap" => $obj$sep$method_name::<Offseasonrecap>$args,
            "offseasonsetup" => $obj$sep$method_name::<Offseasonsetup>$args,
            "player" => $obj$sep$method_name::<Player>$args,
            "renovationprogress" => $obj$sep$method_name::<Renovationprogress>$args,
            "risingstars" => $obj$sep$method_name::<Risingstars>$args,
            "shopsetup" => $obj$sep$method_name::<Shopsetup>$args,
            "teamelectionstats" => $obj$sep$method_name::<Teamelectionstats>$args,
            "vault" => $obj$sep$method_name::<Vault>$args,
            _ => $last_case
        }
    }
}

// hack so we can use call_method_by_type for Database::from_single
mod db_wrapper {
    use blaseball_vcr::db_manager::*;
    use blaseball_vcr::vhs::db::Database;
    use blaseball_vcr::VCRResult;

    pub fn from_single_and_insert<
        T: Clone
            + vhs_diff::Patch
            + vhs_diff::Diff
            + serde::de::DeserializeOwned
            + Send
            + Sync
            + serde::Serialize
            + 'static,
    >(
        manager: &mut DatabaseManager,
        path: &std::path::Path,
    ) -> VCRResult<()> {
        let v: Database<T> = Database::from_single(path)?;
        manager.insert(v);
        Ok(())
    }
}

#[derive(Serialize)]
pub struct ChronResponse<T> {
    #[serde(rename = "nextPage")]
    next_page: Option<String>,
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

    rocket::build()
        .manage(db_manager)
        .attach(RequestTimer)
        .manage(PageManager::new(256, Duration::from_secs(10 * 60)))
        .mount(
            "/",
            routes![entities, versions, v1::games, v1::game_updates],
        )
}
