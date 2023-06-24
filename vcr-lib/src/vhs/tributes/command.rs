use modular_bitfield::BitfieldSpecifier;
use mycelium_bitfield::{bitfield, FromBits};
use std::convert::Infallible;

bitfield! {
    pub struct TributeCommand<u32> {
        pub const ID: u16;
        pub const KIND: CommandKind;
        pub const AFFECTS: TributeAffects;
        pub const EMBEDDED_DELTA: bool;
        pub const DELTA: i16;
    }
}

#[derive(BitfieldSpecifier, Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum CommandKind {
    Delta,
    Remove,
}

#[repr(u8)]
#[derive(BitfieldSpecifier, Debug, PartialEq, Eq, Clone, Copy)]
pub enum TributeAffects {
    Team,
    Player,
}

// from https://docs.rs/mycelium-bitfield/0.1.2/src/mycelium_bitfield/lib.rs.html#111-113
macro_rules! frombits_for_enum {
    (impl FromBits<$($F:ty),+> for $en:ty [$false:path, $true:path] {}) => {
        $(
            impl FromBits<$F> for $en {
                const BITS: u32 = 1;
                type Error = Infallible;

                fn try_from_bits(f: $F) -> Result<Self, Self::Error> {
                    Ok(if f == 0 { $false } else { $true })
                }

                fn into_bits(self) -> $F {
                    match self {
                        $false => 0,
                        $true => 1
                    }
                }
            }
        )+
    }
}

frombits_for_enum!(impl FromBits<u8, u16, u32, u64, usize> for CommandKind [CommandKind::Delta, CommandKind::Remove] {});
frombits_for_enum!(impl FromBits<u8, u16, u32, u64, usize> for TributeAffects [TributeAffects::Team, TributeAffects::Player] {});
