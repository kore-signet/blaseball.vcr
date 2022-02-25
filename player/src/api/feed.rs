use crate::types::{EventuallyCountReq, FeedReq};
use blaseball_vcr::{feed::*, MultiDatabase, VCRError, VCRResult};
use chrono::{DateTime, TimeZone, Utc};
use rocket::{get, serde::json::Json as RocketJson, State};
use serde_json::Value as JSONValue;
use uuid::Uuid;

#[get("/feed/<kind>?<req..>")]
pub fn feed(
    kind: &str,
    feed: &State<FeedDatabase>,
    req: FeedReq,
) -> VCRResult<RocketJson<Vec<FeedEvent>>> {
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

//api.sibr.dev/eventually/count?before=2021-04-08T19%3A00%3A08.47507619Z&after=2021-04-08T17%3A05%3A42.223899933Z&type=0
#[get("/feed_count?<req..>")]
pub fn feed_count(
    feed: &State<FeedDatabase>,
    req: EventuallyCountReq,
) -> VCRResult<RocketJson<JSONValue>> {
    let (tag_t, tag) = if let Some(u) = req.player_tags.and_then(|v| Uuid::parse_str(&v).ok()) {
        (Some(TagType::Player), Some(u)) // wow! i hate this
    } else if let Some(u) = req.team_tags.and_then(|v| Uuid::parse_str(&v).ok()) {
        (Some(TagType::Team), Some(u))
    } else {
        (None, None)
    };

    Ok(RocketJson(serde_json::json!({
        "count": feed.get_event_ids(
        req.after.parse().unwrap(),
        req.before.parse().unwrap(),
        tag.as_ref(),
        tag_t,
        req.etype,
    )?
    .len()})))
}
