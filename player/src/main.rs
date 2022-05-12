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

use blaseball_vcr::feed::db::FeedDatabase;
use blaseball_vcr::site::AssetManager;
use serde::Serialize;

mod fairings;
mod feed;
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
use rocket::figment::{
    providers::{Env, Format, Toml},
    Figment, Profile,
};
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

#[cfg(not(feature = "bundle_before"))]
async fn build_rocket(figment: Figment) -> rocket::Rocket<rocket::Build> {
    rocket::custom(figment)
}

#[cfg(feature = "bundle_before")]
async fn build_rocket(figment: Figment) -> rocket::Rocket<rocket::Build> {
    use rocket::figment::{providers::Serialized, util::map};

    let profile = Profile::from_env_or("VCR_PROFILE", "default");
    let figment = figment
        .merge(Serialized::from(
            &map![
                "chronicler_base_url" => "{addr}/vcr/",
                "upnuts_base_url" => "{addr}/vcr/",
            ],
            profile.as_str(),
        ))
        .merge(Serialized::from(
            &map![
                "siesta_mode" => true,
                "chronplete" => true,
            ],
            profile.as_str(),
        ));
    before::build(&figment).await.unwrap()
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let figment = Figment::from(rocket::Config::default())
        .merge(Toml::file("Vcr.toml").nested())
        .merge(Env::prefixed("VCR_"))
        .select(Profile::from_env_or("VCR_PROFILE", "default"));
    let mut rocket = build_rocket(figment).await;
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
    if false {
        for asset_kind in site_manager.assets.keys() {
            let (total, failures) = site_manager.check_asset(asset_kind).unwrap();
            println!(
                "{} ~ {} checksum matches ({} failures)",
                asset_kind,
                total - failures.len(),
                failures.len()
            );
        }
    }

    let feed_db = FeedDatabase::from_single("./vhs_tapes/feed.vhs").unwrap();

    let rocket = rocket
        .manage(db_manager)
        .attach(RequestTimer)
        .manage(site_manager)
        .manage(feed_db)
        .manage(PageManager::new(256, Duration::from_secs(10 * 60)))
        .mount(
            "/vcr",
            routes![
                entities,
                versions,
                v1::games,
                v1::game_updates,
                site::site_updates,
                site::site_download,
                feed::api::feed
            ],
        );
    rocket.launch().await?;
    Ok(())
}
