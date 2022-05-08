use std::io;

pub fn encode_varint(i: u16) -> Vec<u8> {
    if i >= 255 {
        vec![255_u8.to_be_bytes().to_vec(), i.to_be_bytes().to_vec()].concat()
    } else {
        (i as u8).to_be_bytes().to_vec()
    }
}

#[macro_export]
macro_rules! decode_varint {
    ($read:expr) => {{
        let mut first_byte: [u8; 1] = [0; 1];
        $read.read_exact(&mut first_byte)?;
        let length_byte = u8::from_be_bytes(first_byte);
        if length_byte == 255 {
            let mut longer_bytes: [u8; 2] = [0; 2];
            $read.read_exact(&mut longer_bytes)?;
            u16::from_be_bytes(longer_bytes)
        } else {
            length_byte as u16
        }
    }};
}

#[macro_export]
macro_rules! read_u8 {
    ($read:expr) => {{
        let mut byte: [u8; 1] = [0; 1];
        $read.read_exact(&mut byte)?;
        u8::from_be_bytes(byte)
    }};
}

#[macro_export]
macro_rules! read_i8 {
    ($read:expr) => {{
        let mut byte: [u8; 1] = [0; 1];
        $read.read_exact(&mut byte)?;
        i8::from_be_bytes(byte)
    }};
}

#[macro_export]
macro_rules! read_u16 {
    ($read:expr) => {{
        let mut bytes: [u8; 2] = [0; 2];
        $read.read_exact(&mut bytes)?;
        u16::from_be_bytes(bytes)
    }};
}

#[macro_export]
macro_rules! read_i16 {
    ($read:expr) => {{
        let mut bytes: [u8; 2] = [0; 2];
        $read.read_exact(&mut bytes)?;
        i16::from_be_bytes(bytes)
    }};
}

#[macro_export]
macro_rules! read_u32 {
    ($read:expr) => {{
        let mut bytes: [u8; 4] = [0; 4];
        $read.read_exact(&mut bytes)?;
        u32::from_be_bytes(bytes)
    }};
}

#[macro_export]
macro_rules! read_i64 {
    ($read:expr) => {{
        let mut bytes: [u8; 8] = [0; 8];
        $read.read_exact(&mut bytes)?;
        i64::from_be_bytes(bytes)
    }};
}

pub fn is_eof<T>(err: &io::Result<T>) -> bool {
    match err {
        Ok(_) => false,
        Err(e) => e.kind() == io::ErrorKind::UnexpectedEof,
    }
}

#[macro_export]
macro_rules! call_method_by_type {
    ($obj:ident $sep:tt $method_name:ident, $args:tt, $ty:expr, $last_case:block) => {
        match $ty {
            "gameupdate" => $obj$sep$method_name::<GameUpdate>$args,
            "bossfight" => $obj$sep$method_name::<Bossfight>$args,
            "communitychestprogress" => $obj$sep$method_name::<CommunityChestProgress>$args,
            "division" => $obj$sep$method_name::<Division>$args,
            "league" => $obj$sep$method_name::<League>$args,
            "playoffmatchup" => $obj$sep$method_name::<Playoffmatchup>$args,
            "playoffround" => $obj$sep$method_name::<Playoffround>$args,
            "playoffs" => $obj$sep$method_name::<Playoffs>$args,
            "season" => $obj$sep$method_name::<Season>$args,
            "sim" => $obj$sep$method_name::<Sim>$args,
            "stadium" => $obj$sep$method_name::<Stadium>$args,
            "standings" => $obj$sep$method_name::<Standings>$args,
            "subleague" => $obj$sep$method_name::<Subleague>$args,
            "team" => $obj$sep$method_name::<Team>$args,
            "sunsun" => $obj$sep$method_name::<Sunsun>$args,
            "temporal" => $obj$sep$method_name::<Temporal>$args,
            "tiebreakers" => $obj$sep$method_name::<TiebreakerWrapper>$args,
            "tournament" => $obj$sep$method_name::<Tournament>$args,
            "bonusresult" => $obj$sep$method_name::<Bonusresult>$args,
            "decreeresult" => $obj$sep$method_name::<Decreeresult>$args,
            "eventresult" => $obj$sep$method_name::<Eventresult>$args,
            "fuelprogress" => $obj$sep$method_name::<FuelprogressWrapper>$args,
            "giftprogress" => $obj$sep$method_name::<Giftprogress>$args,
            "globalevents" => $obj$sep$method_name::<GlobaleventsWrapper>$args,
            "idols" => $obj$sep$method_name::<IdolsWrapper>$args,
            "item" => $obj$sep$method_name::<Item>$args,
            "librarystory" => $obj$sep$method_name::<LibrarystoryWrapper>$args,
            "nullified" => $obj$sep$method_name::<NullifiedWrapper>$args,
            "offseasonrecap" => $obj$sep$method_name::<Offseasonrecap>$args,
            "offseasonsetup" => $obj$sep$method_name::<Offseasonsetup>$args,
            "player" => $obj$sep$method_name::<Player>$args,
            "renovationprogress" => $obj$sep$method_name::<Renovationprogress>$args,
            "risingstars" => $obj$sep$method_name::<Risingstars>$args,
            "shopsetup" => $obj$sep$method_name::<Shopsetup>$args,
            "teamelectionstats" => $obj$sep$method_name::<Teamelectionstats>$args,
            "vault" => $obj$sep$method_name::<Vault>$args,
            _ => $last_case
        }
    }
}
