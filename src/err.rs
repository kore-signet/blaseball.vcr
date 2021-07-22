use std::fmt;
use std::io;

#[derive(Debug)]
pub enum VCRError {
    EntityNotFound,
    PathResolutionError,
    InvalidOpCode,
    MsgPackError(rmp_serde::decode::Error),
    MsgPackEncError(rmp_serde::encode::Error),
    IOError(io::Error),
    JSONPatchError(json_patch::PatchError),
    ReqwestError(reqwest::Error),
}

use VCRError::*;

impl std::error::Error for VCRError {}

impl From<io::Error> for VCRError {
    fn from(err: io::Error) -> VCRError {
        VCRError::IOError(err)
    }
}

impl From<rmp_serde::decode::Error> for VCRError {
    fn from(err: rmp_serde::decode::Error) -> VCRError {
        VCRError::MsgPackError(err)
    }
}

impl From<rmp_serde::encode::Error> for VCRError {
    fn from(err: rmp_serde::encode::Error) -> VCRError {
        VCRError::MsgPackEncError(err)
    }
}

impl From<json_patch::PatchError> for VCRError {
    fn from(err: json_patch::PatchError) -> VCRError {
        VCRError::JSONPatchError(err)
    }
}

impl From<reqwest::Error> for VCRError {
    fn from(err: reqwest::Error) -> VCRError {
        VCRError::ReqwestError(err)
    }
}

impl fmt::Display for VCRError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IOError(err) => write!(f, "IO/ERR {}", err),
            MsgPackError(err) => write!(f, "MSGPACKERR {}", err),
            MsgPackEncError(err) => write!(f, "MSGPACKENCERR {}", err),
            JSONPatchError(err) => write!(f, "JSONPATCHERR {}", err),
            ReqwestError(err) => write!(f, "REQWESTERR {}", err),
            EntityNotFound => write!(f, "entity not found"),
            PathResolutionError => write!(f, "could not resolve patch path"),
            InvalidOpCode => write!(f, "invalid patch opcode"),
        }
    }
}

use rocket::{
    http::Status,
    response::{self, Responder},
    Request, Response,
};
use std::io::Cursor;
impl<'r> Responder<'r, 'static> for VCRError {
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {
        let res = format!("{}", self);
        Response::build()
            .status(Status::InternalServerError)
            .sized_body(res.len(), Cursor::new(res))
            .ok()
    }
}
