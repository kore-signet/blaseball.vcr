use blaseball_vcr::{feed::*, MultiDatabase, VCRError, VCRResult};
use chrono::{DateTime, TimeZone, Utc};
use rocket::{get, serde::json::Json as RocketJson, State};
use serde_json::Value as JSONValue;
use uuid::Uuid;

#[get("/feed/<kind>?<id>&<time>&<start>&<category>&<limit>&<phase>&<season>")]
pub fn feed(
    kind: &str,
    id: Option<String>,
    time: Option<i64>,
    start: Option<String>,
    limit: Option<usize>,
    phase: Option<u8>,
    season: Option<u8>,
    category: Option<i8>,
    feed: &State<FeedDatabase>,
) -> VCRResult<RocketJson<Vec<FeedEvent>>> {
    let time = start
        .map(|s| s.parse::<DateTime<Utc>>().unwrap())
        .unwrap_or_else(|| time.map_or(Utc::now(), |d| Utc.timestamp_millis(d)));

    let category: i8 = category.unwrap_or(-3);

    match kind {
        "global" => {
            if phase.is_some() && season.is_some() {
                Ok(RocketJson(feed.events_by_phase(
                    season.unwrap(),
                    phase.unwrap(),
                    limit.unwrap_or(1000),
                )?))
            } else {
                Ok(RocketJson(feed.events_before(
                    time,
                    limit.unwrap_or(100),
                    category,
                )?))
            }
        }
        "player" => {
            Ok(RocketJson(feed.events_by_tag_and_time(
                time,
                &Uuid::parse_str(&id.ok_or(VCRError::EntityNotFound)?).unwrap(), // wrong sort of error. oop. also do n't unwrap
                TagType::Player,
                limit.unwrap_or(100),
                category,
            )?))
        }
        "team" => {
            Ok(RocketJson(feed.events_by_tag_and_time(
                time,
                &Uuid::parse_str(&id.ok_or(VCRError::EntityNotFound)?).unwrap(), // wrong sort of error. oop. also do n't unwrap
                TagType::Team,
                limit.unwrap_or(100),
                category,
            )?))
        }
        "game" => {
            Ok(RocketJson(feed.events_by_tag_and_time(
                time,
                &Uuid::parse_str(&id.ok_or(VCRError::EntityNotFound)?).unwrap(), // wrong sort of error. oop. also do n't unwrap
                TagType::Game,
                limit.unwrap_or(100),
                category,
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
