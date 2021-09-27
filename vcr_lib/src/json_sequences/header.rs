use crate::{read_u8, utils::is_eof, EntityData, VCRError, VCRResult};
use integer_encoding::{VarIntReader, VarIntWriter};
use serde_json::{json, Value as JSONValue};
use std::collections::HashMap;
use std::io::{Cursor, Read, Seek, Write};
use uuid::Uuid;

pub struct HeaderEncoder<W: Write> {
    writer: W,
}

impl<W: Write> HeaderEncoder<W> {
    pub fn new(
        base: JSONValue,
        checkpoint_every: u16,
        path_map: HashMap<u16, String>,
        start_pos: u32,
        mut writer: W,
    ) -> VCRResult<HeaderEncoder<W>> {
        writer.write_varint(start_pos)?;
        writer.write_varint(checkpoint_every)?;
        let mut path_bytes: Vec<u8> = Vec::new();
        for (path, string) in path_map {
            let s_bytes = string.as_bytes();
            path_bytes.write_all(&(s_bytes.len() as u8).to_be_bytes())?;
            path_bytes.write_all(s_bytes)?;
            path_bytes.write_varint(path)?;
        }

        writer.write_varint(path_bytes.len() as u32)?;
        writer.write_all(&path_bytes)?;

        writer.write_all(
            &(match base {
                JSONValue::Null => 0_u8,
                JSONValue::Bool(_) => 1_u8,
                JSONValue::Number(_) => 2_u8,
                JSONValue::String(_) => 3_u8,
                JSONValue::Array(_) => 4_u8,
                JSONValue::Object(_) => 5_u8,
            })
            .to_be_bytes(),
        )?;

        Ok(HeaderEncoder { writer })
    }

    pub fn write_patch(&mut self, time: u32, position_delta: u32) -> VCRResult<()> {
        self.writer.write_all(&time.to_be_bytes())?;
        self.writer.write_varint(position_delta)?;
        Ok(())
    }

    pub fn release(self) -> W {
        self.writer
    }
}

pub fn decode_header<R: Read>(mut reader: R) -> VCRResult<HashMap<String, EntityData>> {
    let mut entities: HashMap<String, EntityData> = HashMap::new();
    loop {
        let len_res = reader.read_varint::<u32>();
        if is_eof(&len_res) {
            break;
        }

        let end_position = reader.read_varint::<u32>()?;

        let mut uuid_bytes: [u8; 16] = [0; 16];
        reader.read_exact(&mut uuid_bytes)?;

        let mut header: Vec<u8> = vec![0; len_res? as usize];
        reader.read_exact(&mut header)?;
        let mut header = Cursor::new(header);

        let mut last_position = header.read_varint::<u32>()?;
        let checkpoint_every = header.read_varint::<u16>()?;

        let path_map = {
            let mut paths: HashMap<u16, String> = HashMap::new();

            let path_bytes_len = header.read_varint::<u32>()?;

            let start_path_pos = header.stream_position()? as u32;

            while (header.stream_position()? as u32 - start_path_pos) < path_bytes_len {
                let s_len = read_u8!(header);
                let mut s_bytes: Vec<u8> = vec![0; s_len as usize];
                header.read_exact(&mut s_bytes)?;

                let path_id = header.read_varint::<u16>()?;

                paths.insert(path_id, String::from_utf8(s_bytes)?);
            }

            paths
        };

        let base_val = match read_u8!(header) {
            0 => json!(null),
            1 => json!(false),
            2 => json!(0),
            3 => json!(""),
            4 => json!([]),
            5 => json!({}),
            _ => return Err(VCRError::InvalidPatchData),
        };

        let mut offsets: Vec<(u32, u32, u32)> = Vec::new();

        loop {
            let mut time_bytes: [u8; 4] = [0; 4];
            let time_res = header.read_exact(&mut time_bytes);
            if is_eof(&time_res) {
                break;
            } else {
                time_res?;
            }

            let time = u32::from_be_bytes(time_bytes);
            let position_delta = header.read_varint::<u32>()?;

            let start_pos = last_position + position_delta as u32;

            if !offsets.is_empty() {
                let idx = offsets.len() - 1;
                let mut a = offsets[idx];
                a.2 = start_pos as u32 - a.1;
                offsets[idx] = a;
            }

            offsets.push((time, start_pos, 0));

            last_position = start_pos;
        }

        if !offsets.is_empty() {
            let idx = offsets.len() - 1;
            let mut a = offsets[idx];
            a.2 = end_position as u32 - a.1;
            offsets[idx] = a;
        }

        entities.insert(
            Uuid::from_bytes(uuid_bytes).to_string(),
            EntityData {
                patches: offsets,
                checkpoint_every,
                base: base_val,
                path_map,
            },
        );
    }

    Ok(entities)
}

// pub struct HeaderDecoder<R: Read> {

// }

// #[derive(Serialize, Deserialize, Debug)]
// pub struct EntityData {
//     pub patches: Vec<(u32, u32, u32)>, // timestamp, offset, end of patch
//     pub path_map: HashMap<u16, String>, // path_id:path
//     #[serde(default = "default_checkpoint")]
//     pub checkpoint_every: u16,
//     #[serde(default = "default_base")]
//     pub base: JSONValue,
// }
