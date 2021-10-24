pub mod feed;
pub mod v1;
pub mod v2;

use blaseball_vcr::{ChroniclerEntity, ChroniclerResponse, ChroniclerV1Response, VCRResult};
use rocket::serde::json::Json as RocketJson;
use serde_json::value::RawValue;

pub type JSONResponse<T> = VCRResult<RocketJson<T>>;
pub type ChronV1Res<T> = JSONResponse<ChroniclerV1Response<T>>;
pub type ChronV2Res<T> = JSONResponse<ChroniclerResponse<T>>;
pub type RawChronEntity = ChroniclerEntity<Box<RawValue>>;
