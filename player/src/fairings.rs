use rocket::{
    http::{Header, Status},
    options,
};
use std::time::Instant;

pub struct RequestTimer;

#[derive(Copy, Clone)]
struct TimerStart(Option<Instant>);

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
