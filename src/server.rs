use blaseball_vcr::*;
use chrono::DateTime;
use rocket::{get, launch, routes, FromForm, State};
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

#[get("/entities?<req..>")]
fn entities(req: EntityReq, db: &State<MultiDatabase>) -> Result<JSONValue, VCRError> {
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
        ("player", "datasets/players_lookup.bin", "datasets/players_db.bin")
    ])
    .unwrap();
    rocket::build().manage(dbs).mount("/v2/", routes![entities])
}
