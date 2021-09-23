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
