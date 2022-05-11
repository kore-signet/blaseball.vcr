use blaseball_vcr::feed::db::*;
use blaseball_vcr::*;
use chrono::{DateTime, Utc};
use rocket::http::ContentType;
use rocket::State;

#[get("/feed/global?<start>&<time>&<limit>")]
pub fn feed(
    db: &State<FeedDatabase>,
    start: Option<&str>,
    time: Option<i64>,
    limit: Option<usize>,
) -> VCRResult<(ContentType, String)> {
    let time = start
        .as_ref()
        .and_then(|s| {
            s.parse::<DateTime<Utc>>()
                .ok()
                .map(|t| t.timestamp_millis())
        })
        .or(time)
        .unwrap_or(Utc::now().timestamp_millis());

    Ok((
        ContentType::JSON,
        serde_json::to_string(&db.get_events_before(time as u64, limit.unwrap_or(100)))?,
    ))
}
