use blaseball_vcr::{
    site::{chron::SiteUpdate, manager::ResourceManager},
    *,
};
use chrono::{DateTime, TimeZone, Utc};
use lru::LruCache;
use rand::Rng;
use rocket::{get, http::ContentType, serde::json::Json as RocketJson, State};
use serde_json::{json, Value as JSONValue};

use crate::types::{Order, UserAgent, V1GameUpdatesReq, V1GamesReq};

use std::sync::Mutex;

#[get("/site/updates")]
pub fn site_updates(
    manager: &State<ResourceManager>,
) -> RocketJson<ChroniclerV1Response<SiteUpdate>> {
    RocketJson(ChroniclerV1Response {
        next_page: None,
        data: manager.expand_site_updates("/assets"),
    })
}

#[get("/assets/<r_type>/<r_idx>")]
pub fn get_asset(
    r_type: &str,
    r_idx: u16,
    manager: &State<ResourceManager>,
) -> VCRResult<(ContentType, Vec<u8>)> {
    Ok((
        match r_type {
            "index" => ContentType::HTML,
            "maincss" => ContentType::CSS,
            "mainjs" | "2js" => ContentType::JavaScript,
            _ => panic!(), // TODO: result instead
        },
        manager.get_resource(r_type, r_idx)?,
    ))
}

// this is not a place of honor
#[get("/games?<req..>")]
pub fn games(
    req: Option<V1GamesReq>,
    user_agent: UserAgent,
    db: &State<MultiDatabase>,
) -> VCRResult<RocketJson<ChroniclerV1Response<ChronV1Game>>> {
    if user_agent.0.map_or(false, |v| {
        // if user agent is before, return only the game dates, not the game data itself. this is because before only uses the date information, and fetch operations are costly.
        v == "Before/1.0 (https://github.com/iliana/before; iliana@sibr.dev)"
    }) {
        Ok(RocketJson(ChroniclerV1Response {
            next_page: None,
            data: db.games_with_date(Utc.timestamp(0, 0))?,
        }))
    } else if let Some(req) = req {
        let before = req.before.as_ref().map_or(chrono::MAX_DATETIME, |d| {
            DateTime::parse_from_rfc3339(d).unwrap().with_timezone(&Utc)
        });
        let after = req.after.as_ref().map_or(Utc.timestamp(0, 0), |d| {
            DateTime::parse_from_rfc3339(d).unwrap().with_timezone(&Utc)
        });
        let weathers = req.weather.as_ref().map(|w| {
            w.split(',')
                .map(|v| json!(v.parse::<i64>().unwrap()))
                .collect::<Vec<JSONValue>>()
        });
        let teams = req
            .team
            .as_ref()
            .map(|v| v.split(',').map(|t| json!(t)).collect::<Vec<JSONValue>>());
        let pitchers = req
            .pitcher
            .as_ref()
            .map(|v| v.split(',').map(|t| json!(t)).collect::<Vec<JSONValue>>());
        let mut res = ChroniclerV1Response {
            next_page: None,
            data: db
                .game_index
                .iter()
                .filter(|(date, _)| {
                    (req.tournament.is_none()
                        || req.tournament.as_ref() == date.tournament.as_ref())
                        && req.day.as_ref().map_or(true, |d| d == &date.day)
                        && req.season.as_ref().map_or(true, |s| s == &date.season)
                })
                .flat_map(|(_, v)| v)
                .filter_map(|(id, start, end)| {
                    if start.is_some() && (start.unwrap() < after || start.unwrap() > before) {
                        return None;
                    }

                    match db.get_entity("game_updates", id, u32::MAX) {
                        Ok(g) => {
                            let game = g.data;
                            if (req.started.is_none()
                                || req.started.map(|v| json!(v)).as_ref() == game.get("gameStart"))
                                && (req.finished.is_none()
                                    || req.finished.map(|v| json!(v)).as_ref()
                                        == game.get("finalized"))
                                && (pitchers.is_none()
                                    || pitchers
                                        .as_ref()
                                        .unwrap()
                                        .contains(game.get("homePitcher").unwrap_or(&json!(null)))
                                    || pitchers
                                        .as_ref()
                                        .unwrap()
                                        .contains(game.get("awayPitcher").unwrap_or(&json!(""))))
                                && (teams.is_none()
                                    || teams
                                        .as_ref()
                                        .unwrap()
                                        .contains(game.get("homeTeam").unwrap_or(&json!(null)))
                                    || teams
                                        .as_ref()
                                        .unwrap()
                                        .contains(game.get("awayTeam").unwrap_or(&json!(""))))
                                && (weathers.is_none()
                                    || weathers
                                        .as_ref()
                                        .unwrap()
                                        .contains(game.get("weather").unwrap_or(&json!(null))))
                            {
                                Some(Ok(ChronV1Game {
                                    game_id: id.to_owned(),
                                    start_time: *start,
                                    end_time: *end,
                                    data: json!(game),
                                }))
                            } else {
                                None
                            }
                        }
                        Err(e) => Some(Err(e)),
                    }
                })
                .collect::<VCRResult<Vec<ChronV1Game>>>()?,
        };

        if let Some(ord) = req.order {
            res.data.sort_by_key(|v| v.start_time);
            if ord == Order::Desc {
                res.data.reverse();
            }
        }

        res.data.truncate(req.count.unwrap_or(usize::MAX));

        Ok(RocketJson(res))
    } else {
        Ok(RocketJson(ChroniclerV1Response {
            next_page: None,
            data: db
                .game_index
                .iter()
                .flat_map(|(_, v)| v)
                .map(
                    |(id, start, end)| match db.get_entity("game_updates", id, u32::MAX) {
                        Ok(g) => Ok(ChronV1Game {
                            game_id: id.to_owned(),
                            start_time: *start,
                            end_time: *end,
                            data: json!(g.data),
                        }),
                        Err(e) => Err(e),
                    },
                )
                .collect::<VCRResult<Vec<ChronV1Game>>>()?,
        }))
    }
}

#[get("/games/updates?<req..>")]
pub fn game_updates(
    req: V1GameUpdatesReq,
    db: &State<MultiDatabase>,
    page_map: &State<Mutex<LruCache<String, InternalPaging>>>,
) -> VCRResult<RocketJson<ChroniclerV1Response<ChronV1GameUpdate>>> {
    let mut res = if let Some(page_token) = req.page {
        let mut page_cache = page_map.lock().unwrap();
        if let Some(ref mut p) = page_cache.get_mut(&page_token) {
            let results: Vec<ChroniclerEntity> =
                db.fetch_page("game_updates", p, req.count.unwrap_or(100))?;
            if results.len() < req.count.unwrap_or(100) {
                ChroniclerV1Response {
                    next_page: None,
                    data: results
                        .into_iter()
                        .map(|e| ChronV1GameUpdate {
                            game_id: e.entity_id,
                            timestamp: e.valid_from,
                            hash: String::new(),
                            data: e.data,
                        })
                        .collect::<Vec<ChronV1GameUpdate>>(),
                }
            } else {
                ChroniclerV1Response {
                    next_page: Some(page_token),
                    data: results
                        .into_iter()
                        .map(|e| ChronV1GameUpdate {
                            game_id: e.entity_id,
                            timestamp: e.valid_from,
                            hash: String::new(),
                            data: e.data,
                        })
                        .collect::<Vec<ChronV1GameUpdate>>(),
                }
            }
        } else {
            return Err(VCRError::InvalidPageToken);
        }
    } else {
        let ids = if let Some(games) = req.game {
            games.split(',').map(|v| v.to_owned()).collect()
        } else {
            let before = req.before.as_ref().map_or(chrono::MAX_DATETIME, |d| {
                DateTime::parse_from_rfc3339(d).unwrap().with_timezone(&Utc)
            });
            let after = req.after.as_ref().map_or(Utc.timestamp(0, 0), |d| {
                DateTime::parse_from_rfc3339(d).unwrap().with_timezone(&Utc)
            });

            db.game_index
                .iter()
                .filter(|(date, _)| {
                    (req.tournament.is_none()
                        || req.tournament.as_ref() == date.tournament.as_ref())
                        && req.day.as_ref().map_or(true, |d| d == &date.day)
                        && req.season.as_ref().map_or(true, |s| s == &date.season)
                })
                .flat_map(|(_, v)| v)
                .take(req.count.unwrap_or(usize::MAX))
                .filter_map(|(id, start, _)| {
                    if start.is_some() && (start.unwrap() < after || start.unwrap() > before) {
                        None
                    } else {
                        Some(id.to_owned())
                    }
                })
                .collect()
        };

        let start_time = req.after.as_ref().map_or(u32::MIN, |y| {
            DateTime::parse_from_rfc3339(y).unwrap().timestamp() as u32
        });

        let end_time = req.before.map_or(u32::MAX, |y| {
            DateTime::parse_from_rfc3339(&y).unwrap().timestamp() as u32
        });

        let mut page = InternalPaging {
            remaining_data: vec![],
            remaining_ids: ids,
            kind: ChronV2EndpointKind::Versions(end_time, start_time),
        };

        let res = db.fetch_page("game_updates", &mut page, req.count.unwrap_or(100))?;
        if res.len() >= req.count.unwrap_or(100) {
            let mut page_cache = page_map.lock().unwrap();
            let key = {
                let mut k = String::new();
                let mut rng = rand::thread_rng();

                loop {
                    let chars: String = std::iter::repeat(())
                        .map(|()| rng.sample(rand::distributions::Alphanumeric))
                        .map(char::from)
                        .take(16)
                        .collect();
                    if !page_cache.contains(&chars) {
                        k = chars;
                        break;
                    }
                }

                k
            };

            page_cache.put(key.clone(), page);

            ChroniclerV1Response {
                next_page: Some(key),
                data: res
                    .into_iter()
                    .map(|e| ChronV1GameUpdate {
                        game_id: e.entity_id,
                        timestamp: e.valid_from,
                        hash: String::new(),
                        data: e.data,
                    })
                    .collect::<Vec<ChronV1GameUpdate>>(),
            }
        } else {
            ChroniclerV1Response {
                next_page: None,
                data: res
                    .into_iter()
                    .map(|e| ChronV1GameUpdate {
                        game_id: e.entity_id,
                        timestamp: e.valid_from,
                        hash: String::new(),
                        data: e.data,
                    })
                    .collect::<Vec<ChronV1GameUpdate>>(),
            }
        }
    };

    if let Some(ord) = req.order {
        res.data.sort_by_key(|v| v.timestamp);
        if ord == Order::Desc {
            res.data.reverse();
        }
    }

    Ok(RocketJson(res))
}
