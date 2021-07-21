mod err;
mod json_sequences;
pub mod site;
pub use err::*;
pub use json_sequences::*;

pub type VCRResult<T> = Result<T, VCRError>;
