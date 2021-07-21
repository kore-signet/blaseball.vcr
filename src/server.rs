use blaseball_vcr::site::manager::ResourceManager;
use blaseball_vcr::*;
use chrono::DateTime;
use rocket::{get, http::ContentType, launch, routes, FromForm, State};
use serde_json::{json, Value as JSONValue};

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
fn site_updates(manager: &State<ResourceManager>) -> JSONValue {
    json!({
        "nextPage": "",
        "data": manager.expand_site_updates("/assets")
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

#[get("/v2/entities?<req..>")]
fn entities(req: EntityReq, db: &State<MultiDatabase>) -> VCRResult<JSONValue> {
    if let Some(ids) = req.ids {
        Ok(json!({
            "nextPage": "",
            "items": json!(
                db.get_entities(
                    &req.entity_type.to_lowercase(),
                    ids.split(",").map(|x| x.to_owned()).collect::<Vec<String>>(),
                    req.at.map_or(u32::MAX,|when|DateTime::parse_from_rfc3339(&when).unwrap().timestamp() as u32)
                )?
            )
        }))
    } else {
        Ok(json!({
            "nextPage": "",
            "items": json!(
                db.all_entities(
                    &req.entity_type.to_lowercase(),
                    req.at.map_or(u32::MAX,|when|DateTime::parse_from_rfc3339(&when).unwrap().timestamp() as u32)
                )?
            )
        }))
    }
}

#[launch]
fn rocket() -> _ {
    let dbs = MultiDatabase::from_files(vec![
        ("team", "datasets/teams_lookup.bin", "datasets/teams_db.bin"),
        (
            "player",
            "datasets/players_lookup.bin",
            "datasets/players_db.bin",
        ),
    ])
    .unwrap();

    let manager = ResourceManager::from_files(vec![
        (
            "2js",
            "datasets/site_data/2js.header.bin",
            "datasets/site_data/2js.bin",
        ),
        (
            "mainjs",
            "datasets/site_data/mainjs.header.bin",
            "datasets/site_data/mainjs.bin",
        ),
        (
            "index",
            "datasets/site_data/index.header.bin",
            "datasets/site_data/index.bin",
        ),
        (
            "maincss",
            "datasets/site_data/maincss.header.bin",
            "datasets/site_data/maincss.bin",
        ),
    ])
    .unwrap();

    rocket::build()
        .manage(dbs)
        .manage(manager)
        .mount("/", routes![entities, get_asset, site_updates])
}
