use blaseball_vcr::site::manager::ResourceManager;
use blaseball_vcr::{feed::FeedDatabase, InternalPaging, MultiDatabase};
use lru::LruCache;
use rocket::figment::{
    providers::{Env, Format, Toml},
    Figment, Profile,
};
use rocket::{
    get,
    http::{uri::Origin, ContentType, Status},
    response::Redirect,
    routes, State,
};
use std::collections::HashMap;
use std::io::Write;
use std::path::PathBuf;
use std::sync::Mutex;

use player::{types::*, v1, v2};

use serde_json::value::RawValue;

#[cfg(feature = "bundle_before")]
use rocket::{response::content::Html, Either};

#[get("/database/coffee")]
fn coffee() -> (Status, (ContentType, &'static str)) {
    (Status::ImATeapot, (ContentType::Plain, "Coffee?"))
}

#[cfg(not(feature = "bundle_before"))]
#[get("/youtube/<id>")]
fn embed(id: &str, origin: &Origin) -> Redirect {
    Redirect::to(format!(
        "https://www.youtube.com/embed/{}?{}",
        id,
        origin.query().map(|q| q.as_str()).unwrap_or_default(),
    ))
}

#[cfg(feature = "bundle_before")]
#[get("/youtube/<id>")]
fn embed(
    id: &str,
    origin: &Origin,
    config: &State<before::Config>,
) -> Either<Html<String>, Redirect> {
    println!("{:?}", config.static_dir.join(format!("{}.webm", id)));
    if config.static_dir.join(format!("{}.webm", id)).exists() {
        Either::Left(Html(format!(include_str!("video.html"), id = id)))
    } else {
        Either::Right(Redirect::to(format!(
            "https://www.youtube.com/embed/{}?{}",
            id,
            origin.query().map(|q| q.as_str()).unwrap_or_default(),
        )))
    }
}

#[cfg(not(feature = "bundle_before"))]
async fn build_rocket(figment: Figment) -> rocket::Rocket<rocket::Build> {
    rocket::custom(figment)
}

#[cfg(feature = "bundle_before")]
async fn build_rocket(figment: Figment) -> rocket::Rocket<rocket::Build> {
    use rocket::figment::{providers::Serialized, util::map};

    let profile = Profile::from_env_or("VCR_PROFILE", "default");
    let figment = figment
        .merge(Serialized::from(
            &map![
                "chronicler_base_url" => "{addr}/vcr/",
                "upnuts_base_url" => "{addr}/vcr/",
            ],
            profile.as_str(),
        ))
        .merge(Serialized::from(
            &map![
                "siesta_mode" => true,
                "chronplete" => true,
            ],
            profile.as_str(),
        ));
    before::build(&figment).await.unwrap()
}

async fn spinny(formatting: &str, msg: &str) {
    for frame in vec![
        "[    ]", "[=   ]", "[==  ]", "[=== ]", "[ ===]", "[  ==]", "[   =]", "[    ]", "[   =]",
        "[  ==]", "[ ===]", "[====]", "[=== ]", "[==  ]", "[=   ]",
    ]
    .into_iter()
    .cycle()
    {
        print!("\x1b[1000D{}{} {}\x1b[0m", formatting, frame, msg);
        std::io::stdout().flush().unwrap();
        rocket::tokio::time::sleep(std::time::Duration::from_millis(80)).await;
    }
}

#[rocket::launch]
async fn build_vcr() -> rocket::Rocket<rocket::Build> {
    #[derive(serde::Deserialize, Debug)]
    struct VCRConfig {
        tapes: String,
        site_assets: String,
        zstd_dictionaries: Option<String>,
        feed: Option<FeedConfig>,
        cached_page_capacity: Option<usize>,
        entities_cache_size: Option<usize>,
        time_responses: Option<bool>,
        cors: Option<bool>,
        stream_data_step: Option<u32>,
        parallelize_stream_data: Option<bool>,
        #[cfg(feature = "bundle_before")]
        open_in_browser: Option<bool>,
    }

    #[derive(serde::Deserialize, Debug)]
    struct FeedConfig {
        index: String,
        path: String,
        dict: String,
        id_table: String,
        tag_table: String,
        cache_size: Option<usize>,
    }

    if let Some((_, path)) = std::env::vars().find(|(k, _)| k == "APPDIR") {
        std::env::set_current_dir(path).unwrap();
    } else {
        // traverse from the directory where we live up until we find a Vcr.toml, then chdir there.
        if let Ok(dir) = std::env::current_exe() {
            if let Some(new_dir) = dir.ancestors().find(|d| d.join("Vcr.toml").exists()) {
                std::env::set_current_dir(new_dir).unwrap();
            }
        }
    };

    let figment = Figment::from(rocket::Config::default())
        .merge(Toml::file("Vcr.toml").nested())
        .merge(Env::prefixed("VCR_"))
        .select(Profile::from_env_or("VCR_PROFILE", "default"));
    let config: VCRConfig = figment.extract_inner("vcr").expect("missing vcr config!");
    let mut rocket = build_rocket(figment).await;

    let dicts = if let Some(dicts_folder) = config.zstd_dictionaries {
        std::fs::read_dir(dicts_folder)
            .unwrap()
            .map(|res| res.map(|e| e.path()))
            .collect::<Result<Vec<PathBuf>, std::io::Error>>()
            .unwrap()
            .into_iter()
            .filter_map(|path| {
                if let Some(ext) = path.extension() {
                    if ext == "dict" {
                        Some((
                            path.file_stem().unwrap().to_string_lossy().to_string(),
                            path,
                        ))
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect::<HashMap<String, PathBuf>>()
    } else {
        HashMap::new()
    };

    let blahaj = rocket::tokio::task::spawn(spinny("\x1b[1m", "reading entities database"));
    let dbs = MultiDatabase::from_folder(
        PathBuf::from(config.tapes),
        dicts,
        config.entities_cache_size.unwrap_or(30),
    )
    .unwrap();
    blahaj.abort();

    println!();

    let blahaj = rocket::tokio::task::spawn(spinny("\x1b[1m", "reading site assets"));
    let manager = ResourceManager::from_folder(&config.site_assets).unwrap();
    blahaj.abort();
    println!();

    if let Some(feed_config) = config.feed {
        let blahaj = rocket::tokio::task::spawn(spinny("\x1b[1m", "reading feed data"));
        let feed_db = FeedDatabase::from_files(
            feed_config.index,
            feed_config.path,
            feed_config.dict,
            feed_config.id_table,
            feed_config.tag_table,
            feed_config.cache_size.unwrap_or(50),
        )
        .unwrap();
        blahaj.abort();
        println!();
        rocket = rocket
            .manage(feed_db)
            .mount("/vcr", routes![player::feed::feed]);
    }

    if config.time_responses.unwrap_or(false) {
        rocket = rocket.attach(RequestTimer);
    }

    if config.cors.unwrap_or(false) {
        rocket = rocket.attach(CORS);
    }

    let cache: LruCache<String, InternalPaging<Box<RawValue>>> =
        LruCache::new(config.cached_page_capacity.unwrap_or(20));

    #[cfg(feature = "bundle_before")]
    if config.open_in_browser.unwrap_or(false) {
        rocket = rocket.attach(rocket::fairing::AdHoc::on_liftoff("Open in browser", |r| {
            Box::pin(async move {
                let url = format!(
                    "{}://{}:{}",
                    if r.config().tls_enabled() {
                        "https"
                    } else {
                        "http"
                    },
                    r.config().address,
                    r.config().port,
                );
                if open::that(&url).is_err() {
                    println!("Couldn't open before in default browser");
                }
            })
        }));
    }

    rocket
        .manage(dbs)
        .manage(manager)
        .manage(Mutex::new(cache))
        .manage(StreamDataStep(config.stream_data_step.unwrap_or(5)))
        .manage(ParallelizeStreamData(
            config.parallelize_stream_data.unwrap_or(false),
        ))
        .mount("/vcr/v2", routes![v2::entities, v2::versions])
        .mount(
            "/vcr/v1",
            routes![v1::get_asset, v1::site_updates, v1::games, v1::game_updates],
        )
        .mount(
            "/vcr",
            routes![coffee, embed, cors_preflight, player::feed::library],
        )
}
