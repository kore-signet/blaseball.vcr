use std::fmt;
use std::io;

#[derive(Debug)]
pub enum VCRError {
    EntityNotFound,
    PathResolutionError,
    InvalidOpCode,
    MsgPackError(rmp_serde::decode::Error),
    IOError(io::Error),
    JSONPatchError(json_patch::PatchError),
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

impl From<json_patch::PatchError> for VCRError {
    fn from(err: json_patch::PatchError) -> VCRError {
        VCRError::JSONPatchError(err)
    }
}

impl fmt::Display for VCRError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IOError(err) => write!(f, "{}", err),
            MsgPackError(err) => write!(f, "{}", err),
            JSONPatchError(err) => write!(f, "{}", err),
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
