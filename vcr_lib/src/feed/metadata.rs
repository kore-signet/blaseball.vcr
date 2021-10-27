use crate::{err::VCRError, utils::*, VCRResult};
use serde_json::{json, Value as JSONValue};
use std::io::{Read, Write};
use uuid::Uuid;

// encode event with only play, subPlay, and (possibly) children
pub fn encode_simple_event(metadata: &JSONValue) -> Vec<u8> {
    let mut res: Vec<u8> = Vec::new();
    res.write_all(&(metadata["play"].as_i64().unwrap() as u16).to_be_bytes())
        .unwrap();
    res.write_all(&(metadata["subPlay"].as_i64().unwrap() as i8).to_be_bytes())
        .unwrap();
    if let Some(children) = metadata.get("children").and_then(|v| v.as_array()) {
        for child in children {
            res.write_all(Uuid::parse_str(child.as_str().unwrap()).unwrap().as_bytes())
                .unwrap();
        }
    }

    res
}

pub fn decode_simple_event(mut bytes: impl Read) -> VCRResult<JSONValue> {
    let play = read_u16!(bytes);
    let sub_play = read_i8!(bytes);
    let mut children: Vec<Uuid> = Vec::new();

    loop {
        let mut uuid_buffer: [u8; 16] = [0; 16];
        let read_res = bytes.read_exact(&mut uuid_buffer);
        if is_eof(&read_res) {
            break;
        }

        read_res?;
        children.push(Uuid::from_bytes(uuid_buffer));
    }

    if children.len() > 1 {
        Ok(json!({
            "play": play,
            "subPlay": sub_play,
            "children": children
        }))
    } else {
        Ok(json!({
            "play": play,
            "subPlay": sub_play
        }))
    }
}

pub fn encode_metadata(etype: i16, metadata: &JSONValue) -> Vec<u8> {
    match etype {
        1..=10
        | 12..=25
        | 27
        | 28
        | 30..=53
        | 62..=79
        | 84..=99
        | 165
        | 169
        | 170
        | 173
        | 174
        | 177
        | 178
        | 181
        | 183
        | 189
        | 191
        | 192
        | 193
        | 195
        | 198
        | 208
        | 213
        | 216
        | 226
        | 228
        | 230
        | 231
        | 233
        | 237
        | 239
        | 243
        | 246
        | 247
        | 250
        | 251
        | 252
        | 254
        | 255 => encode_simple_event(metadata),
        _ => rmp_serde::to_vec(metadata).unwrap(),
    }
}

pub fn decode_metadata(etype: i16, metadata: impl Read) -> VCRResult<JSONValue> {
    match etype {
        1..=10
        | 12..=25
        | 27
        | 28
        | 30..=53
        | 62..=79
        | 84..=99
        | 165
        | 169
        | 170
        | 173
        | 174
        | 177
        | 178
        | 181
        | 183
        | 189
        | 191
        | 192
        | 193
        | 195
        | 198
        | 208
        | 213
        | 216
        | 226
        | 228
        | 230
        | 231
        | 233
        | 237
        | 239
        | 243
        | 246
        | 247
        | 249
        | 250
        | 251
        | 252
        | 254
        | 255 => decode_simple_event(metadata),
        _ => rmp_serde::from_read(metadata).map_err(VCRError::MsgPackDecError),
    }
}
// play max: 1100, play min: 0
// subPlay max: 64, subPlay min: -1
