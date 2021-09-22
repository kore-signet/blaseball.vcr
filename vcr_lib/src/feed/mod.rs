mod decode;
mod desc;
mod event;
pub use decode::*;
pub use desc::*;
pub use event::*;

pub enum TagType {
    Team,
    Player,
    Game,
}
