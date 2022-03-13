#![allow(unused_assignments)]

mod api;
pub use api::*;

pub mod types;
pub use types::*;

#[cfg(feature = "gui")]
pub mod gui;

pub enum RunState {
    Preparing,
    ReadingEntities,
    ReadingSiteAssets,
    ReadingFeed,
    Running(rocket::Config, Option<rocket::Shutdown>),
    ShuttingDown,
}
