use crate::types::FeedReq;
use blaseball_vcr::{feed::*, MultiDatabase, VCRError, VCRResult};
use chrono::{DateTime, TimeZone, Utc};
use rocket::{get, serde::json::Json as RocketJson, State};
use serde_json::Value as JSONValue;
use uuid::Uuid;

use std::sync::Mutex;

#[get("/feed/<kind>?<req..>")]
pub fn feed(
    kind: &str,
    db: &State<Mutex<FeedDatabase>>,
    req: FeedReq,
) -> VCRResult<RocketJson<Vec<FeedEvent>>> {
    let mut feed = db.lock().unwrap();

    let time = req
        .start
        .as_ref()
        .map(|s| s.parse::<DateTime<Utc>>().unwrap())
        .unwrap_or_else(|| req.time.map_or(Utc::now(), |d| Utc.timestamp_millis(d)));

    let category: i8 = req.category.unwrap_or(-3);

    match kind {
        "global" => {
            if req.phase.is_some() && req.season.is_some() {
                Ok(RocketJson(feed.events_by_phase(
                    req.season.unwrap(),
                    req.phase.unwrap(),
                    req.limit.unwrap_or(1000),
                )?))
            } else if req.etype.is_some() {
                Ok(RocketJson(feed.events_by_type_and_time(
                    time,
                    req.etype.unwrap(),
                    req.limit.unwrap_or(100),
                )?))
            } else {
                Ok(RocketJson(feed.events_before(
                    time,
                    req.limit.unwrap_or(100),
                    category,
                )?))
            }
        }
        "player" => {
            Ok(RocketJson(feed.events_by_tag_and_time(
                time,
                &Uuid::parse_str(&req.id.ok_or(VCRError::EntityNotFound)?).unwrap(), // wrong sort of error. oop. also do n't unwrap
                TagType::Player,
                req.limit.unwrap_or(100),
                category,
                req.etype.unwrap_or(-1),
            )?))
        }
        "team" => {
            Ok(RocketJson(feed.events_by_tag_and_time(
                time,
                &Uuid::parse_str(&req.id.ok_or(VCRError::EntityNotFound)?).unwrap(), // wrong sort of error. oop. also do n't unwrap
                TagType::Team,
                req.limit.unwrap_or(100),
                category,
                req.etype.unwrap_or(-1),
            )?))
        }
        "game" => {
            Ok(RocketJson(feed.events_by_tag_and_time(
                time,
                &Uuid::parse_str(&req.id.ok_or(VCRError::EntityNotFound)?).unwrap(), // wrong sort of error. oop. also do n't unwrap
                TagType::Game,
                req.limit.unwrap_or(100),
                category,
                req.etype.unwrap_or(-1),
            )?))
        }
        _ => Err(VCRError::EntityTypeNotFound),
    }
}

#[get("/feed/story?<time>&<id>")]
pub fn library(
    time: Option<i64>,
    id: &str,
    db: &State<MultiDatabase>,
) -> VCRResult<RocketJson<Vec<JSONValue>>> {
    Ok(RocketJson(
        serde_json::from_value::<Vec<JSONValue>>(
            db.get_entity(
                "librarystory",
                id,
                time.map_or(Utc::now().timestamp() as u32, |d| {
                    Utc.timestamp_millis(d).timestamp() as u32
                }),
            )?
            .data,
        )
        .unwrap(),
    ))
}
