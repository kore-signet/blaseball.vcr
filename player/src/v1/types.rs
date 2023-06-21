use blaseball_vcr::timestamp_to_nanos;

#[derive(FromForm)]
pub struct GamesReq<'r> {
    pub before: Option<&'r str>,
    pub after: Option<&'r str>,
    pub count: Option<usize>,
    pub day: Option<i16>,
    pub game: Option<&'r str>,
    pub season: Option<i8>,
    pub tournament: Option<i8>,
    pub pitcher: Option<&'r str>,
    pub team: Option<&'r str>,
    pub weather: Option<&'r str>,
    pub page: Option<String>,
    pub sim: Option<&'r str>,
}

impl<'r> GamesReq<'r> {
    pub fn before_nanos(&self) -> Option<i64> {
        self.before
            .and_then(iso8601_timestamp::Timestamp::parse)
            .map(timestamp_to_nanos)
    }

    pub fn after_nanos(&self) -> Option<i64> {
        self.after
            .and_then(iso8601_timestamp::Timestamp::parse)
            .map(timestamp_to_nanos)
    }
}
