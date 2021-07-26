use blaseball_vcr::site::{chron::SiteUpdate, manager::ResourceManager};
use blaseball_vcr::*;
use chrono::DateTime;
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

#[get("/v2/versions?type=Stream")]
fn fake_versions() -> RocketJson<ChroniclerResponse<ChroniclerEntity>> {
    RocketJson(ChroniclerResponse {
        next_page: None,
        items: Vec::new(),
    })
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
