use blaseball_vcr::VCRError;
use rocket::{
    http::{Header, Status},
    options,
};
use std::time::Instant;

pub struct RequestTimer;

pub struct UserAgent(pub Option<String>);

#[rocket::async_trait]
impl<'r> rocket::request::FromRequest<'r> for UserAgent {
    type Error = VCRError;

    async fn from_request(
        req: &'r rocket::Request<'_>,
    ) -> rocket::request::Outcome<Self, Self::Error> {
        rocket::request::Outcome::Success(UserAgent(
            req.headers().get_one("User-Agent").map(|s| s.to_owned()),
        ))
    }
}

#[derive(Copy, Clone)]
struct TimerStart(Option<Instant>);

pub struct CORS;
#[rocket::async_trait]
impl rocket::fairing::Fairing for CORS {
    fn info(&self) -> rocket::fairing::Info {
        rocket::fairing::Info {
            name: "CORS headers",
            kind: rocket::fairing::Kind::Response,
        }
    }

    async fn on_response<'r>(
        &self,
        _: &'r rocket::Request<'_>,
        response: &mut rocket::Response<'r>,
    ) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Methods", "GET"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
    }
}

#[rocket::async_trait]
impl<'r> rocket::response::Responder<'r, 'static> for CORS {
    fn respond_to(self, _: &'r rocket::Request<'_>) -> rocket::response::Result<'static> {
        rocket::Response::build()
            .header(Header::new("Access-Control-Allow-Origin", "*"))
            .header(Header::new("Access-Control-Allow-Methods", "GET"))
            .header(Header::new("Access-Control-Allow-Headers", "*"))
            .header(Header::new("Access-Control-Max-Age", "86400"))
            .header(Header::new("Allow", "OPTIONS, GET"))
            .status(Status::NoContent)
            .ok()
    }
}

#[options("/<_..>")]
pub async fn cors_preflight() -> CORS {
    CORS
}

#[rocket::async_trait]
impl rocket::fairing::Fairing for RequestTimer {
    fn info(&self) -> rocket::fairing::Info {
        rocket::fairing::Info {
            name: "Request Timer",
            kind: rocket::fairing::Kind::Request | rocket::fairing::Kind::Response,
        }
    }

    async fn on_request(&self, request: &mut rocket::Request<'_>, _: &mut rocket::Data<'_>) {
        request.local_cache(|| TimerStart(Some(Instant::now())));
    }

    async fn on_response<'r>(&self, req: &'r rocket::Request<'_>, _: &mut rocket::Response<'r>) {
        let start_time = req.local_cache(|| TimerStart(None));
        if let Some(duration) = start_time.0.map(|st| st.elapsed()) {
            if let Some(route) = req.route() {
                let query_params = if let Some(query) = req.uri().query() {
                    query
                        .segments()
                        .fold(String::new(), |acc, (k, v)| format!("{}={} {}", k, v, acc))
                } else {
                    "no params".to_owned()
                };
                println!(
                    "\x1b[31;1m{}\x1b[m\x1b[1m + {}\x1b[m-> \x1b[4m{:?}\x1b[m",
                    route.name.as_ref().unwrap(),
                    query_params,
                    duration
                );
            }
        }
    }
}
