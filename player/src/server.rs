use blaseball_vcr::site::{chron::SiteUpdate, manager::ResourceManager};
use blaseball_vcr::{feed::*, *};
use chrono::{DateTime, TimeZone, Utc};
use rocket::serde::json::Json as RocketJson;
use rocket::{get, http::{ContentType, Status}, launch, routes, FromForm, State};
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
            kind: rocket::fairing::Kind::Request | rocket::fairing::Kind::Response
        }
    }

    async fn on_request(&self, request: &mut rocket::Request<'_>, _: &mut rocket::Data<'_>) {
        request.local_cache(|| TimerStart(Some(Instant::now())));
    }

    async fn on_response<'r>(&self, req: &'r rocket::Request<'_>, res: &mut rocket::Response<'r>) {
        let start_time = req.local_cache(|| TimerStart(None));
        if let Some(duration) = start_time.0.map(|st| st.elapsed()) {
            println!("\x1b[1m{}\x1b[m took {:?}",req.uri(),duration);
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
    count: Option<u32>,
}

#[derive(FromForm)]
struct VersionsReq {
    #[field(name = "type")]
    entity_type: String,
    #[field(name = "id")]
    ids: Option<String>,
    before: Option<String>,
    after: Option<String>,
    count: Option<u32>,
    order: Option<String>
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
) -> VCRResult<RocketJson<ChroniclerResponse<ChroniclerEntity>>> {
    let mut res: Vec<ChroniclerEntity> = if req.entity_type.to_lowercase() == "stream" {
        let start_time = req.after.as_ref().map_or(
            req.before.as_ref().map_or(u32::MAX, |x| {
                DateTime::parse_from_rfc3339(&x).unwrap().timestamp() as u32
            }) - (req.count.unwrap_or(1) * 5),
            |y| DateTime::parse_from_rfc3339(&y).unwrap().timestamp() as u32,
        );

        let end_time = req.before.map_or(
            req.after.map_or(u32::MIN, |x| {
                DateTime::parse_from_rfc3339(&x).unwrap().timestamp() as u32
            }) + (req.count.unwrap_or(1) * 5),
            |y| DateTime::parse_from_rfc3339(&y).unwrap().timestamp() as u32,
        );


        let mut results: Vec<ChroniclerEntity> = Vec::new();
        for at in (start_time..end_time).into_iter().step_by(5) {
            results.push(ChroniclerEntity {
                entity_id: "00000000-0000-0000-0000-000000000000".to_owned(),
                valid_from: Utc.timestamp(at as i64, 0),
                valid_to: Some(Utc.timestamp((at + 5) as i64, 0).to_rfc3339()),
                hash: String::new(),
                data: db.stream_data(at)?,
            });
        }

        results
    } else {
        let start_time = req.after.as_ref().map_or(
            u32::MIN,
            |y| DateTime::parse_from_rfc3339(&y).unwrap().timestamp() as u32,
        );

        let end_time = req.before.map_or(
            u32::MAX,
            |y| DateTime::parse_from_rfc3339(&y).unwrap().timestamp() as u32,
        );

        if let Some(ids) = req.ids.map(|i| i.split(",").map(|x| x.to_owned()).collect::<Vec<String>>()) {
            db.get_entities_versions(&req.entity_type, ids, end_time, start_time)?
        } else {
            db.all_entities_versions(&req.entity_type, end_time, start_time)?
        }
    };

    if let Some(ord) = req.order {
        if ord.to_lowercase() == "asc" {
            res.sort_by_key(|x| x.valid_from);
        } else if ord.to_lowercase() == "desc" {
            res.sort_by_key(|x| x.valid_from);
            res.reverse();
        }
    }

    Ok(RocketJson(ChroniclerResponse {
        next_page: None,
        items: res,
    }))
}

#[get("/v2/entities?<req..>")]
fn entities(
    req: EntityReq,
    db: &State<MultiDatabase>,
) -> VCRResult<RocketJson<ChroniclerResponse<ChroniclerEntity>>> {
    if let Some(ids) = req.ids {
        Ok(RocketJson(ChroniclerResponse {
            next_page: None,
            items: db
                .get_entities(
                    &req.entity_type.to_lowercase(),
                    ids.split(",")
                        .map(|x| x.to_owned())
                        .collect::<Vec<String>>(),
                    req.at.map_or(u32::MAX, |when| {
                        DateTime::parse_from_rfc3339(&when).unwrap().timestamp() as u32
                    }),
                )?
                .into_iter()
                .filter(|x| x.data != json!({}))
                .collect(),
        }))
    } else {
        Ok(RocketJson(ChroniclerResponse {
            next_page: None,
            items: db
                .all_entities(
                    &req.entity_type.to_lowercase(),
                    req.at.map_or(u32::MAX, |when| {
                        DateTime::parse_from_rfc3339(&when).unwrap().timestamp() as u32
                    }),
                )?
                .into_iter()
                .filter(|x| x.data != json!({}))
                .collect(),
        }))
    }
}

#[get("/database/coffee")]
fn coffee() -> (Status, (ContentType, &'static str)) {
    (Status::ImATeapot, (ContentType::Plain, "Coffee?"))
}

#[launch]
fn rocket() -> _ {
    let rocket = rocket::build();

    #[derive(serde::Deserialize)]
    struct VCRConfig {
        tapes: String,
        site_assets: String,
        zstd_dictionaries: Option<String>,
        feed_index: String,
        feed_path: String,
        feed_dict: String,
        feed_id_table: String,
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
    let feed_db = Mutex::new(
        FeedDatabase::from_files(
            config.feed_index,
            config.feed_path,
            config.feed_dict,
            config.feed_id_table,
        )
        .unwrap(),
    );

    rocket.manage(dbs).manage(manager).manage(feed_db).mount(
        "/",
        routes![
            all_games,
            entities,
            feed,
            get_asset,
            site_updates,
            versions,
            coffee
        ],
    )
}
