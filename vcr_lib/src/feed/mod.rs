mod decode;
mod event;
pub use decode::*;
pub use event::*;

pub enum TagType {
    Team,
    Player,
    Game,
}
