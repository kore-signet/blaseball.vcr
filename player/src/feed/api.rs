use blaseball_vcr::feed::db::*;
use blaseball_vcr::*;
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
        .and_then(iso8601_timestamp::Timestamp::parse)
        .map(timestamp_to_millis)
        .or(time)
        .unwrap_or_else(|| timestamp_to_millis(iso8601_timestamp::Timestamp::now_utc()));

    Ok((
        ContentType::JSON,
        serde_json::to_string(&db.get_events_before(time, limit.unwrap_or(100)))?,
    ))
}
