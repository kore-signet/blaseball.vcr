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
}
