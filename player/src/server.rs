use blaseball_vcr::site::{chron::SiteUpdate, manager::ResourceManager};
use blaseball_vcr::{feed::*, *};
use chrono::{DateTime, TimeZone, Utc};
use lru::LruCache;
use rand::Rng;
use rocket::{
    get,
    http::{ContentType, Status},
    launch, routes,
    serde::json::Json as RocketJson,
    FromForm, State,
};
use serde_json::json;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Mutex;
use std::time::Instant;

pub struct RequestTimer;

#[derive(Copy, Clone)]
struct TimerStart(Option<Instant>);

#[rocket::async_trait]
impl rocket::fairing::Fairing for RequestTimer {
    fn info(&self) -> rocket::fairing::Info {
        rocket::fairing::Info {
            name: "Request Timer",
            kind: rocket::fairing::Kind::Request | rocket::fairing::Kind::Response,
        }
    }

    async fn on_request(&self, request: &mut rocket::Request<'_>, _: &mut rocket::Data<'_>) {
        request.local_cache(|| TimerStart(Some(Instant::now())));
    }

    async fn on_response<'r>(&self, req: &'r rocket::Request<'_>, _: &mut rocket::Response<'r>) {
        let start_time = req.local_cache(|| TimerStart(None));
        if let Some(duration) = start_time.0.map(|st| st.elapsed()) {
            println!("\x1b[1m{}\x1b[m took {:?}", req.uri(), duration);
        }
    }
}

#[derive(FromForm)]
struct EntityReq {
    #[field(name = "type")]
    entity_type: String,
    #[field(name = "id")]
    ids: Option<String>,
    at: Option<String>,
    count: Option<usize>,
    page: Option<String>,
    order: Option<String>,
}

#[derive(FromForm)]
struct VersionsReq {
    #[field(name = "type")]
    entity_type: String,
    #[field(name = "id")]
    ids: Option<String>,
    before: Option<String>,
    after: Option<String>,
    count: Option<usize>,
    order: Option<String>,
    page: Option<String>,
}

#[get("/v1/site/updates")]
fn site_updates(manager: &State<ResourceManager>) -> RocketJson<ChroniclerV1Response<SiteUpdate>> {
    RocketJson(ChroniclerV1Response {
        next_page: None,
        data: manager.expand_site_updates("/assets"),
    })
}

#[get("/v1/assets/<r_type>/<r_idx>")]
fn get_asset(
    r_type: &str,
    r_idx: u16,
    manager: &State<ResourceManager>,
) -> VCRResult<(ContentType, Vec<u8>)> {
    Ok((
        match r_type {
            "index" => ContentType::HTML,
            "maincss" => ContentType::CSS,
            "mainjs" | "2js" => ContentType::JavaScript,
            _ => panic!(), // TODO: result instead
        },
        manager.get_resource(r_type, r_idx)?,
    ))
}

#[get("/v1/games?<after>")]
fn all_games(
    after: Option<String>,
    db: &State<MultiDatabase>,
) -> VCRResult<RocketJson<ChroniclerV1Response<ChronV1Game>>> {
    Ok(RocketJson(ChroniclerV1Response {
        next_page: None,
        data: db.games_with_date(after.map_or(Utc.timestamp(0, 0), |d| {
            DateTime::parse_from_rfc3339(&d)
                .unwrap()
                .with_timezone(&Utc)
        }))?,
    }))
}

#[get("/feed/global?<time>&<limit>&<phase>&<season>")]
fn feed(
    time: Option<i64>,
    limit: Option<usize>,
    phase: Option<u8>,
    season: Option<i8>,
    db: &State<Mutex<FeedDatabase>>,
) -> VCRResult<RocketJson<Vec<FeedEvent>>> {
    let mut feed = db.lock().unwrap();
    if phase.is_some() && season.is_some() {
        Ok(RocketJson(feed.events_by_phase(
            season.unwrap(),
            phase.unwrap(),
            limit.unwrap_or(1000),
        )?))
    } else {
        Ok(RocketJson(feed.events_before(
            time.map_or(Utc::now(), |d| Utc.timestamp_millis(d)),
            limit.unwrap_or(100),
        )?))
    }
}

#[get("/v2/versions?<req..>")]
fn versions(
    req: VersionsReq,
    db: &State<MultiDatabase>,
    page_map: &State<Mutex<LruCache<String, InternalPaging>>>,
) -> VCRResult<RocketJson<ChroniclerResponse<ChroniclerEntity>>> {
    let mut res: ChroniclerResponse<ChroniclerEntity> = {
        if let Some(page_token) = req.page {
            let mut page_cache = page_map.lock().unwrap();
            if let Some(ref mut p) = page_cache.get_mut(&page_token) {
                let results: Vec<ChroniclerEntity> =
                    db.fetch_page(&req.entity_type.to_lowercase(), p, req.count.unwrap_or(100))?;
                if results.len() < req.count.unwrap_or(100) {
                    ChroniclerResponse {
                        next_page: None,
                        items: results,
                    }
                } else {
                    ChroniclerResponse {
                        next_page: Some(page_token),
                        items: results,
                    }
                }
            } else {
                panic!() // TODO|:| RESULT
            }
        } else {
            let mut page = if req.entity_type.to_lowercase() == "stream" {
                let start_time = req.after.as_ref().map_or(
                    req.before.as_ref().map_or(u32::MAX, |x| {
                        DateTime::parse_from_rfc3339(&x).unwrap().timestamp() as u32
                    }) - ((req.count.unwrap_or(1) as u32) * 5),
                    |y| DateTime::parse_from_rfc3339(&y).unwrap().timestamp() as u32,
                );

                let end_time = req.before.map_or(
                    req.after.map_or(u32::MIN, |x| {
                        DateTime::parse_from_rfc3339(&x).unwrap().timestamp() as u32
                    }) + ((req.count.unwrap_or(1) as u32) * 5),
                    |y| DateTime::parse_from_rfc3339(&y).unwrap().timestamp() as u32,
                );

                InternalPaging {
                    remaining_data: db.stream_data_versions(end_time, start_time)?,
                    remaining_ids: vec![],
                    kind: ChronV2EndpointKind::Versions(end_time, start_time),
                }
            } else {
                let start_time = req.after.as_ref().map_or(u32::MIN, |y| {
                    DateTime::parse_from_rfc3339(&y).unwrap().timestamp() as u32
                });

                let end_time = req.before.map_or(u32::MAX, |y| {
                    DateTime::parse_from_rfc3339(&y).unwrap().timestamp() as u32
                });

                if let Some(ids) = req
                    .ids
                    .map(|i| i.split(",").map(|x| x.to_owned()).collect::<Vec<String>>())
                {
                    InternalPaging {
                        remaining_data: vec![],
                        remaining_ids: ids,
                        kind: ChronV2EndpointKind::Versions(end_time, start_time),
                    }
                } else {
                    InternalPaging {
                        remaining_data: vec![],
                        remaining_ids: db.all_ids(&req.entity_type.to_lowercase()),
                        kind: ChronV2EndpointKind::Versions(end_time, start_time),
                    }
                }
            };

            let res = db.fetch_page(
                &req.entity_type.to_lowercase(),
                &mut page,
                req.count.unwrap_or(100),
            )?;
            if !(res.len() < req.count.unwrap_or(100)) {
                let mut page_cache = page_map.lock().unwrap();
                let key = {
                    let mut k = String::new();
                    let mut rng = rand::thread_rng();

                    loop {
                        let chars: String = std::iter::repeat(())
                            .map(|()| rng.sample(rand::distributions::Alphanumeric))
                            .map(char::from)
                            .take(16)
                            .collect();
                        if !page_cache.contains(&chars) {
                            k = chars;
                            break;
                        }
                    }

                    k
                };

                page_cache.put(key.clone(), page);

                ChroniclerResponse {
                    next_page: Some(key),
                    items: res,
                }
            } else {
                ChroniclerResponse {
                    next_page: None,
                    items: res,
                }
            }
        }
    };

    if let Some(ord) = req.order {
        if ord.to_lowercase() == "asc" {
            res.items.sort_by_key(|x| x.valid_from);
        } else if ord.to_lowercase() == "desc" {
            res.items.sort_by_key(|x| x.valid_from);
            res.items.reverse();
        }
    }

    Ok(RocketJson(res))
}

#[get("/v2/entities?<req..>")]
fn entities(
    req: EntityReq,
    db: &State<MultiDatabase>,
    page_map: &State<Mutex<LruCache<String, InternalPaging>>>,
) -> VCRResult<RocketJson<ChroniclerResponse<ChroniclerEntity>>> {
    let mut res = if let Some(page_token) = req.page {
        let mut page_cache = page_map.lock().unwrap();
        if let Some(ref mut p) = page_cache.get_mut(&page_token) {
            let results: Vec<ChroniclerEntity> = db
                .fetch_page(&req.entity_type.to_lowercase(), p, req.count.unwrap_or(100))?
                .into_iter()
                .filter(|x| x.data != json!({}))
                .collect();
            if results.len() < req.count.unwrap_or(100) {
                ChroniclerResponse {
                    next_page: None,
                    items: results,
                }
            } else {
                ChroniclerResponse {
                    next_page: Some(page_token),
                    items: results,
                }
            }
        } else {
            panic!() // TODO|:| RESULT
        }
    } else {
        let at = req.at.map_or(u32::MAX, |when| {
            DateTime::parse_from_rfc3339(&when).unwrap().timestamp() as u32
        });

        let mut page = if let Some(ids) = req
            .ids
            .map(|i| i.split(",").map(|x| x.to_owned()).collect::<Vec<String>>())
        {
            InternalPaging {
                remaining_data: vec![],
                remaining_ids: ids,
                kind: ChronV2EndpointKind::Entities(at),
            }
        } else {
            InternalPaging {
                remaining_data: vec![],
                remaining_ids: db.all_ids(&req.entity_type.to_lowercase()),
                kind: ChronV2EndpointKind::Entities(at),
            }
        };

        let res: Vec<ChroniclerEntity> = db
            .fetch_page(
                &req.entity_type.to_lowercase(),
                &mut page,
                req.count.unwrap_or(100),
            )?
            .into_iter()
            .filter(|x| x.data != json!({}))
            .collect();
        if !(res.len() < req.count.unwrap_or(100)) {
            let mut page_cache = page_map.lock().unwrap();
            let key = {
                let mut k = String::new();
                let mut rng = rand::thread_rng();

                loop {
                    let chars: String = std::iter::repeat(())
                        .map(|()| rng.sample(rand::distributions::Alphanumeric))
                        .map(char::from)
                        .take(16)
                        .collect();
                    if !page_cache.contains(&chars) {
                        k = chars;
                        break;
                    }
                }

                k
            };

            page_cache.put(key.clone(), page);

            ChroniclerResponse {
                next_page: Some(key),
                items: res,
            }
        } else {
            ChroniclerResponse {
                next_page: None,
                items: res,
            }
        }
    };

    if let Some(ord) = req.order {
        if ord.to_lowercase() == "asc" {
            res.items.sort_by_key(|x| x.valid_from);
        } else if ord.to_lowercase() == "desc" {
            res.items.sort_by_key(|x| x.valid_from);
            res.items.reverse();
        }
    }

    Ok(RocketJson(res))
}

#[get("/database/coffee")]
fn coffee() -> (Status, (ContentType, &'static str)) {
    (Status::ImATeapot, (ContentType::Plain, "Coffee?"))
}

#[launch]
fn rocket() -> _ {
    let mut rocket = rocket::build();

    #[derive(serde::Deserialize)]
    struct VCRConfig {
        tapes: String,
        site_assets: String,
        zstd_dictionaries: Option<String>,
        feed: Option<FeedConfig>,
        cached_page_capacity: Option<usize>,
    }

    #[derive(serde::Deserialize)]
    struct FeedConfig {
        index: String,
        path: String,
        dict: String,
        id_table: String,
    }

    let figment = rocket.figment();
    let config: VCRConfig = figment.extract_inner("vcr").expect("missing vcr config!");

    let dicts = if let Some(dicts_folder) = config.zstd_dictionaries {
        std::fs::read_dir(dicts_folder)
            .unwrap()
            .map(|res| res.map(|e| e.path()))
            .collect::<Result<Vec<PathBuf>, std::io::Error>>()
            .unwrap()
            .into_iter()
            .filter_map(|path| {
                if let Some(ext) = path.extension() {
                    if ext == "dict" {
                        Some((
                            path.file_stem().unwrap().to_string_lossy().to_string(),
                            path,
                        ))
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect::<HashMap<String, PathBuf>>()
    } else {
        HashMap::new()
    };

    let dbs = MultiDatabase::from_folder(PathBuf::from(config.tapes), dicts).unwrap();
    let manager = ResourceManager::from_folder(&config.site_assets).unwrap();

    if let Some(feed_config) = config.feed {
        let feed_db = Mutex::new(
            FeedDatabase::from_files(
                feed_config.index,
                feed_config.path,
                feed_config.dict,
                feed_config.id_table,
            )
            .unwrap(),
        );
        rocket = rocket.manage(feed_db).mount("/", routes![feed]);
    }

    let cache: LruCache<String, InternalPaging> =
        LruCache::new(config.cached_page_capacity.unwrap_or(20));

    rocket
        .manage(dbs)
        .manage(manager)
        .manage(Mutex::new(cache))
        .attach(RequestTimer)
        .mount(
            "/",
            routes![
                all_games,
                entities,
                get_asset,
                site_updates,
                versions,
                coffee
            ],
        )
}
