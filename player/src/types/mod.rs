mod fairings;
mod requests;
pub use fairings::*;
pub use requests::*;

#[derive(Debug)]
pub struct StreamDataStep(pub u32);

#[derive(Debug)]
pub struct ParallelizeStreamData(pub bool);
