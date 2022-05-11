use crate::*;
use blaseball_vcr::site::*;
use blaseball_vcr::VCRResult;
use rocket::http::ContentType;
use rocket::serde::json::Json as RocketJSON;
use rocket::State;
use std::str::FromStr;

#[get("/v1/site/updates")]
pub fn site_updates(
    asset_manager: &State<AssetManager>,
) -> RocketJSON<ChronV1Response<ChronSiteUpdate>> {
    RocketJSON(ChronV1Response {
        next_page: None,
        data: asset_manager.get_resources(),
    })
}

#[get("/v1/site/download/<kind>/<idx>")]
pub fn site_download(
    asset_manager: &State<AssetManager>,
    kind: &str,
    idx: usize,
) -> VCRResult<(ContentType, Vec<u8>)> {
    let kind = AssetType::from_str(kind)?;
    let content_type = match kind {
        AssetType::TwoJs | AssetType::MainJs => ContentType::JavaScript,
        AssetType::Index => ContentType::HTML,
        AssetType::MainCss => ContentType::CSS,
    };

    Ok((content_type, asset_manager.read_asset(&kind, idx)?))
}
