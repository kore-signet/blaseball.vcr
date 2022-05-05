use uuid::Uuid;

pub struct GameUpdatesReq<'r> {
    pub before: Option<&'r str>,
    pub after: Option<&'r str>,
    pub count: Option<usize>,
    pub day: Option<i32>,
    pub game: Option<Uuid>,
    pub season: Option<i32>,
    pub tournament: Option<i32>,
}

#[derive(FromForm)]
pub struct GamesReq<'r> {
    pub before: Option<&'r str>,
    pub after: Option<&'r str>,
    pub count: Option<usize>,
    pub day: Option<i16>,
    pub game: Option<Uuid>,
    pub season: Option<i8>,
    pub tournament: Option<i8>,
    pub pitcher: Option<&'r str>,
    pub team: Option<&'r str>,
    pub weather: Option<&'r str>,
}
