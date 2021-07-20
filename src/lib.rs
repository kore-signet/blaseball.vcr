mod db;
mod err;
pub use db::*;
pub use err::*;

pub type VCRResult<T> = Result<T, VCRError>;
