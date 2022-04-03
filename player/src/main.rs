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

use rocket::serde::json::Json as RocketJSON;
use rocket::FromForm;
use rocket::State;

use serde::Serialize;
use uuid::Uuid;
mod fairings;
mod paging;
use fairings::*;
use paging::*;

use blaseball_vcr::db_manager::DatabaseManager;
use blaseball_vcr::vhs::{db::*, schemas::*};
use blaseball_vcr::*;
use chrono::DateTime;
use std::time::Duration;

#[derive(FromForm)]
struct EntitiesRequest<'r> {
    #[field(name = "type")]
    ty: &'r str,
    id: Option<Uuid>,
    page: Option<String>,
    at: Option<&'r str>,
    count: Option<usize>,
}

#[derive(FromForm)]
struct VersionsRequest<'r> {
    #[field(name = "type")]
    ty: &'r str,
    id: Option<Uuid>,
    page: Option<String>,
    before: Option<&'r str>,
    after: Option<&'r str>,
    count: Option<usize>,
}

#[derive(Serialize)]
struct ChronResponse {
    #[serde(rename = "nextPage")]
    next_page: Option<String>,
    data: Vec<DynamicChronEntity>,
}

#[get("/v2/entities?<req..>")]
fn entities(
    req: EntitiesRequest<'_>,
    db_manager: &State<DatabaseManager>,
    page_manager: &State<PageManager>,
) -> VCRResult<RocketJSON<ChronResponse>> {
    if let Some(page_token) = req
        .page
        .as_ref()
        .and_then(|v| u64::from_str_radix(v, 16).ok())
    {
        let page_mutex = page_manager
            .get_page(&page_token)
            .ok_or(VCRError::InvalidPageToken)?;
        let mut page = page_mutex.lock();
        let data = page.take_n::<Player>(db_manager, req.count.unwrap_or(100))?;

        Ok(RocketJSON(ChronResponse {
            next_page: if page.is_empty() {
                None
            } else {
                Some(req.page.unwrap())
            },
            data,
        }))
    } else {
        let at = req
            .at
            .and_then(|v| DateTime::parse_from_rfc3339(v).ok())
            .map(|v| v.timestamp() as u32)
            .unwrap_or(u32::MAX);
        let ids = if let Some(id) = req.id {
            vec![*id.as_bytes()]
        } else {
            db_manager
                .all_entity_ids::<Player>()
                .ok_or(VCRError::EntityTypeNotFound)?
                .to_vec()
        };

        let mut page = Page::entities(at, ids);
        let data = page.take_n::<Player>(db_manager, req.count.unwrap_or(100))?;

        // if the page isn't empty, add it to the manager
        let token = if !page.is_empty() {
            Some(page_manager.add_page(page))
        } else {
            None
        };

        Ok(RocketJSON(ChronResponse {
            next_page: token.map(|v| format!("{:X}", v)),
            data,
        }))
    }
}

#[get("/v2/versions?<req..>")]
fn versions(
    req: VersionsRequest<'_>,
    db_manager: &State<DatabaseManager>,
    page_manager: &State<PageManager>,
) -> VCRResult<RocketJSON<ChronResponse>> {
    if let Some(page_token) = req
        .page
        .as_ref()
        .and_then(|v| u64::from_str_radix(v, 16).ok())
    {
        let page_mutex = page_manager
            .get_page(&page_token)
            .ok_or(VCRError::InvalidPageToken)?;
        let mut page = page_mutex.lock();
        let data = page.take_n::<Player>(db_manager, req.count.unwrap_or(100))?;

        Ok(RocketJSON(ChronResponse {
            next_page: if page.is_empty() {
                None
            } else {
                Some(req.page.unwrap())
            },
            data,
        }))
    } else {
        let before = req
            .before
            .and_then(|v| DateTime::parse_from_rfc3339(v).ok())
            .map(|v| v.timestamp() as u32)
            .unwrap_or(u32::MAX);
        let after = req
            .after
            .and_then(|v| DateTime::parse_from_rfc3339(v).ok())
            .map(|v| v.timestamp() as u32)
            .unwrap_or(0);
        let ids = if let Some(id) = req.id {
            vec![*id.as_bytes()]
        } else {
            db_manager
                .all_entity_ids::<Player>()
                .ok_or(VCRError::EntityTypeNotFound)?
                .to_vec()
        };

        let mut page = Page::versions(before, after, ids);
        let data = page.take_n::<Player>(db_manager, req.count.unwrap_or(100))?;

        // if the page isn't empty, add it to the manager
        let token = if !page.is_empty() {
            Some(page_manager.add_page(page))
        } else {
            None
        };

        Ok(RocketJSON(ChronResponse {
            next_page: token.map(|v| format!("{:X}", v)),
            data,
        }))
    }
}

#[launch]
fn rocket() -> _ {
    let mut db_manager = DatabaseManager::new();
    let player_db: Database<Player> = Database::from_single("./vhs_tapes/player.vhs").unwrap();

    db_manager.insert(player_db);

    rocket::build()
        .manage(db_manager)
        .attach(RequestTimer)
        .manage(PageManager::new(256, Duration::from_secs(10 * 60)))
        .mount("/", routes![entities, versions])
}
