use super::*;
use crate::*;
use rocket::serde::json::Json as RocketJSON;

use rocket::State;

use blaseball_vcr::db_manager::DatabaseManager;

use blaseball_vcr::vhs::schemas::*;
use blaseball_vcr::*;
use chrono::DateTime;

#[get("/v2/entities?<req..>")]
pub fn entities(
    req: EntitiesRequest<'_>,
    db_manager: &State<DatabaseManager>,
    page_manager: &State<PageManager>,
) -> VCRResult<RocketJSON<DynChronResponse>> {
    let ety = req.ty.to_lowercase();

    if let Some(page_token) = req
        .page
        .as_ref()
        .and_then(|v| u64::from_str_radix(v, 16).ok())
    {
        let page_mutex = page_manager
            .get_page(&page_token)
            .ok_or(VCRError::InvalidPageToken)?;
        let mut page = page_mutex.lock();
        let data = call_method_by_type!(
            page.take_n,
            (db_manager, req.count.unwrap_or(100)),
            ety.as_str(),
            { return Err(VCRError::EntityTypeNotFound) }
        )?;

        Ok(RocketJSON(ChronResponse {
            next_page: if page.is_empty() {
                None
            } else {
                Some(req.page.unwrap())
            },
            data,
        }))
    } else {
        let at = req
            .at
            .and_then(|v| DateTime::parse_from_rfc3339(v).ok())
            .map(|v| v.timestamp() as u32)
            .unwrap_or(u32::MAX);
        let ids = if let Some(id) = req.id {
            vec![*id.as_bytes()]
        } else {
            call_method_by_type!(db_manager.all_entity_ids, (), ety.as_str(), {
                return Err(VCRError::EntityTypeNotFound);
            })
            .ok_or(VCRError::EntityTypeNotFound)?
            .to_vec()
        };

        let mut page = Page::entities(at, ids);
        let data = call_method_by_type!(
            page.take_n,
            (db_manager, req.count.unwrap_or(100)),
            ety.as_str(),
            { return Err(VCRError::EntityTypeNotFound) }
        )?;

        // if the page isn't empty, add it to the manager
        let token = if !page.is_empty() {
            Some(page_manager.add_page(page))
        } else {
            None
        };

        Ok(RocketJSON(ChronResponse {
            next_page: token.map(|v| format!("{:X}", v)),
            data,
        }))
    }
}

#[get("/v2/versions?<req..>")]
pub fn versions(
    req: VersionsRequest<'_>,
    db_manager: &State<DatabaseManager>,
    page_manager: &State<PageManager>,
) -> VCRResult<RocketJSON<DynChronResponse>> {
    let ety = req.ty.to_lowercase();

    if let Some(page_token) = req
        .page
        .as_ref()
        .and_then(|v| u64::from_str_radix(v, 16).ok())
    {
        let page_mutex = page_manager
            .get_page(&page_token)
            .ok_or(VCRError::InvalidPageToken)?;
        let mut page = page_mutex.lock();
        let data = call_method_by_type!(
            page.take_n,
            (db_manager, req.count.unwrap_or(100)),
            ety.as_str(),
            { return Err(VCRError::EntityTypeNotFound) }
        )?;

        Ok(RocketJSON(ChronResponse {
            next_page: if page.is_empty() {
                None
            } else {
                Some(req.page.unwrap())
            },
            data,
        }))
    } else {
        let before = req
            .before
            .and_then(|v| DateTime::parse_from_rfc3339(v).ok())
            .map(|v| v.timestamp() as u32)
            .unwrap_or(u32::MAX);
        let after = req
            .after
            .and_then(|v| DateTime::parse_from_rfc3339(v).ok())
            .map(|v| v.timestamp() as u32)
            .unwrap_or(0);
        let ids = if let Some(id) = req.id {
            vec![*id.as_bytes()]
        } else {
            call_method_by_type!(db_manager.all_entity_ids, (), ety.as_str(), {
                return Err(VCRError::EntityTypeNotFound);
            })
            .ok_or(VCRError::EntityTypeNotFound)?
            .to_vec()
        };

        let mut page = Page::versions(before, after, ids);
        let data = call_method_by_type!(
            page.take_n,
            (db_manager, req.count.unwrap_or(100)),
            ety.as_str(),
            { return Err(VCRError::EntityTypeNotFound) }
        )?;

        // if the page isn't empty, add it to the manager
        let token = if !page.is_empty() {
            Some(page_manager.add_page(page))
        } else {
            None
        };

        Ok(RocketJSON(ChronResponse {
            next_page: token.map(|v| format!("{:X}", v)),
            data,
        }))
    }
}
