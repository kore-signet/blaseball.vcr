use blaseball_vcr::site::{chron::SiteUpdate, manager::ResourceManager};
use blaseball_vcr::*;
use chrono::{DateTime,Utc, TimeZone};
use rocket::serde::json::Json as RocketJson;
use rocket::{get, http::ContentType, launch, routes, FromForm, State};

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
struct VersionReq {
    #[field(name = "type")]
    entity_type: String,
    count: Option<u32>,
    before: Option<String>,
    after: Option<String>,
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

#[get("/v2/versions?type=Stream&<before>&<after>&<count>&<order>")]
fn fake_versions(
    before: Option<String>,
    after: Option<String>,
    count: Option<u32>,
    order: Option<String>,
    db: &State<MultiDatabase>,
) -> VCRResult<RocketJson<ChroniclerResponse<ChroniclerEntity>>> {
    let start_time = after.as_ref().map_or(
        before.as_ref().map_or(
            Utc::now().timestamp() as u32,
            |x| DateTime::parse_from_rfc3339(&x).unwrap().timestamp() as u32
        ) - (count.unwrap_or(1) * 5),
        |y| DateTime::parse_from_rfc3339(&y).unwrap().timestamp() as u32
    );

    let end_time = before.map_or(
        after.map_or(
            Utc::now().timestamp() as u32,
            |x| DateTime::parse_from_rfc3339(&x).unwrap().timestamp() as u32,
        ) + (count.unwrap_or(1) * 5),
        |y| DateTime::parse_from_rfc3339(&y).unwrap().timestamp() as u32,
    );

    let mut results: Vec<ChroniclerEntity> = Vec::new();
    for at in (start_time..end_time).into_iter().step_by(5) {
        results.push(
            ChroniclerEntity {
                entity_id: "00000000-0000-0000-0000-000000000000".to_owned(),
                valid_from: Utc.timestamp(at as i64,0),
                valid_to: Some(Utc.timestamp((at+5) as i64,0).to_rfc3339()),
                hash: String::new(),
                data: db.stream_data(at)?
            }
        );
    }

    if let Some(ord) = order {
        if ord.to_lowercase() == "asc" {
            results.sort_by_key(|x| x.valid_from);
        } else if ord.to_lowercase() == "desc" {
            results.sort_by_key(|x| x.valid_from);
            results.reverse();
        }
    }

    Ok(RocketJson(ChroniclerResponse {
        next_page: None,
        items: results
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
            items: db.get_entities(
                &req.entity_type.to_lowercase(),
                ids.split(",")
                    .map(|x| x.to_owned())
                    .collect::<Vec<String>>(),
                req.at.map_or(u32::MAX, |when| {
                    DateTime::parse_from_rfc3339(&when).unwrap().timestamp() as u32
                }),
            )?,
        }))
    } else {
        Ok(RocketJson(ChroniclerResponse {
            next_page: None,
            items: db.all_entities(
                &req.entity_type.to_lowercase(),
                req.at.map_or(u32::MAX, |when| {
                    DateTime::parse_from_rfc3339(&when).unwrap().timestamp() as u32
                }),
            )?,
        }))
    }
}

#[launch]
fn rocket() -> _ {
    let dbs = MultiDatabase::from_folder("./tapes/").unwrap();

    let manager = ResourceManager::from_folder("./tapes/site_data/").unwrap();

    rocket::build().manage(dbs).manage(manager).mount(
        "/",
        routes![entities, get_asset, site_updates, fake_versions],
    )
}
