use blaseball_vcr::*;
use chrono::{DateTime, TimeZone, Utc};
use lru::LruCache;
use rand::Rng;
use rocket::{get, serde::json::Json as RocketJson, State};
use serde_json::json;

use crate::types::{EntityReq, Order, StreamDataStep, VersionsReq};

use std::sync::Mutex;

#[get("/versions?<req..>")]
pub fn versions(
    req: VersionsReq,
    step: &State<StreamDataStep>,
    db: &State<MultiDatabase>,
    page_map: &State<Mutex<LruCache<String, InternalPaging>>>,
) -> VCRResult<RocketJson<ChroniclerResponse<ChroniclerEntity>>> {
    let mut res: ChroniclerResponse<ChroniclerEntity> =
        if req.entity_type.to_lowercase() == "stream" {
            let start_time = req.after.as_ref().map_or(
                req.before.as_ref().map_or(u32::MAX, |x| {
                    DateTime::parse_from_rfc3339(x).unwrap().timestamp() as u32
                }) - ((req.count.unwrap_or(1) as u32) * step.0),
                |y| DateTime::parse_from_rfc3339(y).unwrap().timestamp() as u32,
            );

            let step = if req.after.is_some() && (1596747150..1596747270).contains(&start_time) {
                // grand unslam workaround
                1
            } else {
                step.0
            };

            let end_time = req.before.map_or(
                req.after.map_or(u32::MIN, |x| {
                    DateTime::parse_from_rfc3339(&x).unwrap().timestamp() as u32
                }) + ((req.count.unwrap_or(1) as u32) * step),
                |y| DateTime::parse_from_rfc3339(&y).unwrap().timestamp() as u32,
            );

            let mut results: Vec<ChroniclerEntity> = Vec::new();
            for at in (start_time..end_time).into_iter().step_by(step as usize) {
                results.push(ChroniclerEntity {
                    entity_id: "00000000-0000-0000-0000-000000000000".to_owned(),
                    valid_from: Utc.timestamp(at as i64, 0),
                    valid_to: Some(Utc.timestamp((at + step) as i64, 0).to_rfc3339()),
                    hash: String::new(),
                    data: db.stream_data(at)?,
                });
            }

            ChroniclerResponse {
                next_page: None,
                items: results,
            }
        } else if let Some(page_token) = req.page {
            let mut page_cache = page_map.lock().unwrap();
            if let Some(ref mut p) = page_cache.get_mut(&page_token) {
                let results: Vec<ChroniclerEntity> =
                    db.fetch_page(&req.entity_type.to_lowercase(), p, req.count.unwrap_or(100))?;
                if results.len() < req.count.unwrap_or(100) {
                    ChroniclerResponse {
                        next_page: None,
                        items: results,
                    }
                } else {
                    ChroniclerResponse {
                        next_page: Some(page_token),
                        items: results,
                    }
                }
            } else {
                return Err(VCRError::InvalidPageToken);
            }
        } else {
            let start_time = req.after.as_ref().map_or(u32::MIN, |y| {
                DateTime::parse_from_rfc3339(y).unwrap().timestamp() as u32
            });

            let end_time = req.before.map_or(u32::MAX, |y| {
                DateTime::parse_from_rfc3339(&y).unwrap().timestamp() as u32
            });

            let mut page = if let Some(ids) = req
                .ids
                .map(|i| i.split(',').map(|x| x.to_owned()).collect::<Vec<String>>())
            {
                InternalPaging {
                    remaining_data: vec![],
                    remaining_ids: ids,
                    kind: ChronV2EndpointKind::Versions(end_time, start_time),
                }
            } else {
                InternalPaging {
                    remaining_data: vec![],
                    remaining_ids: db.all_ids(&req.entity_type.to_lowercase())?,
                    kind: ChronV2EndpointKind::Versions(end_time, start_time),
                }
            };

            let res = db.fetch_page(
                &req.entity_type.to_lowercase(),
                &mut page,
                req.count.unwrap_or(100),
            )?;
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

                ChroniclerResponse {
                    next_page: Some(key),
                    items: res,
                }
            } else {
                ChroniclerResponse {
                    next_page: None,
                    items: res,
                }
            }
        };

    if let Some(ord) = req.order {
        res.items.sort_by_key(|x| x.valid_from);
        if ord == Order::Desc {
            res.items.reverse();
        }
    }

    Ok(RocketJson(res))
}

#[get("/entities?<req..>")]
pub fn entities(
    req: EntityReq,
    db: &State<MultiDatabase>,
    page_map: &State<Mutex<LruCache<String, InternalPaging>>>,
) -> VCRResult<RocketJson<ChroniclerResponse<ChroniclerEntity>>> {
    let mut res = if let Some(page_token) = req.page {
        let mut page_cache = page_map.lock().unwrap();
        if let Some(ref mut p) = page_cache.get_mut(&page_token) {
            let results: Vec<ChroniclerEntity> = db
                .fetch_page(&req.entity_type.to_lowercase(), p, req.count.unwrap_or(100))?
                .into_iter()
                .filter(|x| x.data != json!({}))
                .collect();
            if results.len() < req.count.unwrap_or(100) {
                ChroniclerResponse {
                    next_page: None,
                    items: results,
                }
            } else {
                ChroniclerResponse {
                    next_page: Some(page_token),
                    items: results,
                }
            }
        } else {
            return Err(VCRError::InvalidPageToken);
        }
    } else {
        let at = req.at.map_or(u32::MAX, |when| {
            DateTime::parse_from_rfc3339(&when).unwrap().timestamp() as u32
        });

        let mut page = if let Some(ids) = req
            .ids
            .map(|i| i.split(',').map(|x| x.to_owned()).collect::<Vec<String>>())
        {
            InternalPaging {
                remaining_data: vec![],
                remaining_ids: ids,
                kind: ChronV2EndpointKind::Entities(at),
            }
        } else {
            InternalPaging {
                remaining_data: vec![],
                remaining_ids: db.all_ids(&req.entity_type.to_lowercase())?,
                kind: ChronV2EndpointKind::Entities(at),
            }
        };

        let res: Vec<ChroniclerEntity> = db
            .fetch_page(
                &req.entity_type.to_lowercase(),
                &mut page,
                req.count.unwrap_or(100),
            )?
            .into_iter()
            .filter(|x| x.data != json!({}))
            .collect();
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

            ChroniclerResponse {
                next_page: Some(key),
                items: res,
            }
        } else {
            ChroniclerResponse {
                next_page: None,
                items: res,
            }
        }
    };

    if let Some(ord) = req.order {
        res.items.sort_by_key(|x| x.valid_from);
        if ord == Order::Desc {
            res.items.reverse();
        }
    }

    Ok(RocketJson(res))
}
