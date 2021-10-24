use blaseball_vcr::Order;
use rocket::FromForm;

#[derive(FromForm)]
pub struct EntityReq {
    #[field(name = "type")]
    pub entity_type: String,
    #[field(name = "id")]
    pub ids: Option<String>,
    pub at: Option<String>,
    pub count: Option<usize>,
    pub page: Option<String>,
    pub order: Option<Order>,
}

#[derive(FromForm)]
pub struct VersionsReq {
    #[field(name = "type")]
    pub entity_type: String,
    #[field(name = "id")]
    pub ids: Option<String>,
    pub before: Option<String>,
    pub after: Option<String>,
    pub count: Option<usize>,
    pub order: Option<Order>,
    pub page: Option<String>,
}

#[derive(Debug, FromForm)]
pub struct V1GamesReq {
    pub after: Option<String>,
    pub before: Option<String>,
    pub count: Option<usize>,
    pub day: Option<i32>,
    pub season: Option<i32>,
    pub finished: Option<bool>,
    pub order: Option<Order>,
    pub pitcher: Option<String>,
    pub started: Option<bool>,
    pub team: Option<String>,
    pub tournament: Option<i32>,
    pub weather: Option<String>,
}

#[derive(FromForm)]
pub struct V1GameUpdatesReq {
    pub after: Option<String>,
    pub before: Option<String>,
    pub count: Option<usize>,
    pub day: Option<i32>,
    pub season: Option<i32>,
    pub order: Option<Order>,
    pub tournament: Option<i32>,
    pub game: Option<String>,
    pub page: Option<String>,
}

#[derive(Debug, FromForm)]
pub struct FeedReq {
    pub id: Option<String>,
    pub time: Option<i64>,
    pub start: Option<String>,
    pub limit: Option<usize>,
    pub phase: Option<u8>,
    pub season: Option<u8>,
    pub category: Option<i8>,
    #[field(name = "type")]
    pub etype: Option<i16>,
}
