automod::dir!(pub "src/vhs/schemas");

pub use bossfight::Bossfight;
pub use communitychestprogress::CommunityChestProgress;
pub use division::Division;
pub use game::GameUpdate;
pub use league::League;
pub use playoffmatchup::Playoffmatchup;
pub use playoffround::Playoffround;
pub use playoffs::Playoffs;
pub use season::Season;
pub use sim::Sim;
pub use stadium::Stadium;
pub use standings::Standings;
pub use subleague::Subleague;
pub use sunsun::Sunsun;
pub use team::Team;
pub use temporal::Temporal;
pub use tiebreakers::TiebreakerWrapper;
pub use tournament::Tournament;

pub use bonusresult::Bonusresult;
pub use decreeresult::Decreeresult;
pub use eventresult::Eventresult;
pub use fuelprogress::FuelprogressWrapper;
pub use giftprogress::Giftprogress;
pub use globalevents::GlobaleventsWrapper;
pub use idols::IdolsWrapper;
pub use item::Item;
pub use librarystory::LibrarystoryWrapper;
pub use nullified::NullifiedWrapper;
pub use offseasonrecap::Offseasonrecap;
pub use offseasonsetup::Offseasonsetup;
pub use player::Player;
pub use renovationprogress::Renovationprogress;
pub use risingstars::Risingstars;
pub use shopsetup::Shopsetup;
pub use stream_data::*;
pub use teamelectionstats::Teamelectionstats;
pub use vault::Vault;

use serde::ser::{Serialize, Serializer};
use std::str::FromStr;

#[macro_export]
macro_rules! etypes {
    // "player" -> Player(Player)
    // "fuelprogress" -> FuelProgress(FuelProgressWrapper)
    ($($name:literal -> $variant:ident ($what:ty) ),*) => {
        pub enum DynamicEntity {
            $(
                $variant($what),
            )*
        }

        pub enum DynamicEntityType {
            $(
                $variant,
            )*
        }

        $(
            impl From<$what> for DynamicEntity {
                fn from(t: $what) -> Self {
                    DynamicEntity::$variant(t)
                }
            }
        )*

        impl Serialize for DynamicEntity {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer, {
                match self {
                    $(
                        DynamicEntity::$variant(data) => data.serialize(serializer),
                    )*
                }
            }
        }

        impl FromStr for DynamicEntityType {
            type Err = ();
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s.to_lowercase().as_str() {
                    $(
                        $name => Ok(DynamicEntityType::$variant),
                    )*
                    _ => Err(())
                }
            }
        }
    }
}

etypes! {
    "gameupdate" -> GameUpdate(GameUpdate),
    "bossfight" -> Bossfight(Bossfight),
    "communitychestprogress" -> CommunityChestProgress(CommunityChestProgress),
    "division" -> Division(Division),
    "league" -> League(League),
    "playoffmatchup" -> Playoffmatchup(Playoffmatchup),
    "playoffround" -> Playoffround(Playoffround),
    "playoffs" -> Playoffs(Playoffs),
    "season" -> Season(Season),
    "sim" -> Sim(Sim),
    "stadium" -> Stadium(Stadium),
    "standings" -> Standings(Standings),
    "subleague" -> Subleague(Subleague),
    "team" -> Team(Team),
    "sunsun" -> Sunsun(Sunsun),
    "temporal" -> Temporal(Temporal),
    "tiebreakers" -> Tiebreakers(TiebreakerWrapper),
    "tournament" -> Tournament(Tournament),
    "bonusresult" -> Bonusresult(Bonusresult),
    "decreeresult" -> Decreeresult(Decreeresult),
    "eventresult" -> Eventresult(Eventresult),
    "fuelprogress" -> FuelProgress(FuelprogressWrapper),
    "giftprogress" -> Giftprogress(Giftprogress),
    "globalevents" -> GlobalEvents(GlobaleventsWrapper),
    "idols" -> Idols(IdolsWrapper),
    "item" -> Item(Item),
    "librarystory" -> LibraryStory(LibrarystoryWrapper),
    "nullified" -> Nullified(NullifiedWrapper),
    "offseasonrecap" -> Offseasonrecap(Offseasonrecap),
    "offseasonsetup" -> Offseasonsetup(Offseasonsetup),
    "player" -> Player(Player),
    "renovationprogress" -> RenovationProgress(Renovationprogress),
    "risingstars" -> RisingStars(Risingstars),
    "shopsetup" -> ShopSetup(Shopsetup),
    "teamelectionstats" -> TeamElectionStats(Teamelectionstats),
    "vault" -> Vault(Vault),
    "stream" -> StreamData(StreamDataWrapper)
}
