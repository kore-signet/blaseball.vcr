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

#[derive(FromFormField)]
pub enum Order {
    Asc,
    Desc,
}
