use blaseball_vcr::timestamp_to_nanos;
use uuid::Uuid;

#[derive(FromForm)]
pub struct EntitiesRequest<'r> {
    #[field(name = "type")]
    pub ty: &'r str,
    pub id: Option<Uuid>,
    pub page: Option<String>,
    pub at: Option<&'r str>,
    pub count: Option<usize>,
}

impl<'r> EntitiesRequest<'r> {
    pub fn at_nanos(&self) -> Option<i64> {
        self.at
            .and_then(iso8601_timestamp::Timestamp::parse)
            .map(timestamp_to_nanos)
    }
}

#[derive(FromForm)]
pub struct VersionsRequest<'r> {
    #[field(name = "type")]
    pub ty: &'r str,
    pub id: Option<Uuid>,
    pub page: Option<String>,
    pub before: Option<&'r str>,
    pub after: Option<&'r str>,
    pub count: Option<usize>,
    pub order: Option<Order>,
}

impl<'r> VersionsRequest<'r> {
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

#[derive(FromFormField)]
pub enum Order {
    Asc,
    Desc,
}
