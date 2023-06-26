pub mod db;
pub mod recorder;
pub mod tributes;

use std::{fs::File, io::Read, path::Path};

use memmap2::{Mmap, MmapOptions};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use zstd::dict::DecoderDictionary;

use crate::VCRResult;

#[derive(Serialize, Deserialize)]
pub struct DataHeader {
    pub id: [u8; 16],
    pub times: Vec<i64>,
    pub compressed_len: u32,
    pub decompressed_len: u32,
    pub offset: u32,
    pub checkpoint_every: usize,
    pub checkpoint_positions: Vec<usize>, // positions in the decompressed byte slice at which checkpoints happen
}

pub struct TapeComponents<T: DeserializeOwned> {
    pub dict: Option<DecoderDictionary<'static>>,
    pub header: T,
    pub store: Mmap,
}

impl<T: DeserializeOwned> TapeComponents<T> {
    pub fn split(path: impl AsRef<Path>) -> VCRResult<TapeComponents<T>> {
        let mut file = File::open(path)?;
        let mut len_bytes: [u8; 8] = [0; 8];
        file.read_exact(&mut len_bytes)?;

        let dict_len = u64::from_le_bytes(len_bytes) as usize;

        let dict = if dict_len > 0 {
            let mut dict = vec![0u8; dict_len];
            file.read_exact(&mut dict)?;
            Some(DecoderDictionary::copy(&dict[..]))
        } else {
            None
        };

        file.read_exact(&mut len_bytes)?;
        let header_len = u64::from_le_bytes(len_bytes) as usize;

        let mut header_bytes = vec![0u8; header_len];
        file.read_exact(&mut header_bytes)?;

        let header: T = rmp_serde::from_read(zstd::Decoder::new(&header_bytes[..])?)?;

        let total_len = file.metadata()?.len() as usize;

        let inner = unsafe {
            MmapOptions::new()
                .offset((dict_len + header_len + 16) as u64)
                .len(total_len - (dict_len + header_len + 16))
                .map(&file)?
        };

        Ok(TapeComponents {
            dict,
            header,
            store: inner,
        })
    }
}
