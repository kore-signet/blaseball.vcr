use crate::{timestamp_from_nanos, VCRError, VCRResult};
use bsdiff::patch;
use faster_hex::hex_string;
use memmap2::{Mmap, MmapOptions};
use serde::{
    ser::{SerializeStruct, Serializer},
    Deserialize, Serialize,
};
use std::collections::BTreeMap;
use std::fmt;
use std::fs::File;
use std::io::Read;
use std::mem;
use std::path::Path;
use std::str::FromStr;
use zstd::bulk::Decompressor;

static DOWNLOAD_PATH: &str = "/site/download";

#[repr(u8)]
#[derive(Copy, Clone, Hash, PartialEq, Eq, Serialize, Deserialize, PartialOrd, Ord)]
pub enum AssetType {
    #[serde(rename = "mainjs")]
    MainJs,
    #[serde(rename = "2js")]
    TwoJs,
    #[serde(rename = "indexhtml")]
    Index,
    #[serde(rename = "maincss")]
    MainCss,
}

impl fmt::Display for AssetType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                AssetType::MainJs => "mainjs",
                AssetType::Index => "indexhtml",
                AssetType::MainCss => "maincss",
                AssetType::TwoJs => "2js",
            }
        )
    }
}

impl FromStr for AssetType {
    type Err = VCRError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "mainjs" => AssetType::MainJs,
            "maincss" => AssetType::MainCss,
            "indexhtml" => AssetType::Index,
            "2js" => AssetType::TwoJs,
            _ => return Err(VCRError::InvalidAssetKind),
        })
    }
}

#[derive(Serialize, Deserialize)]
pub struct PatchSet {
    pub initial: Vec<u8>,
    pub patches: Vec<PatchHeader>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PatchHeader {
    pub path: String,
    pub timestamp: i64,                 // millis
    pub hash: [u8; 16],                 // blake2sp
    pub offset: u32,                    // offset from start of file
    pub length: u32,                    // compressed length of patch
    pub uncompressed_length: u32,       // uncompressed length of file
    pub uncompressed_patch_length: u32, // uncompressed length of patch
}

pub struct ChronSiteUpdate {
    pub path: String,
    pub kind: AssetType,
    pub timestamp: i64,
    pub hash: [u8; 16], // blake2sp
    pub idx: usize,
    pub uncompressed_length: u32,
}

impl Serialize for ChronSiteUpdate {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut ser = serializer.serialize_struct("ChroniclerSiteUpdate", 5)?;
        ser.serialize_field("timestamp", &timestamp_from_nanos(self.timestamp))?;
        ser.serialize_field("path", &self.path)?;
        ser.serialize_field("hash", &hex_string(&self.hash))?;
        ser.serialize_field("size", &self.uncompressed_length)?;
        ser.serialize_field(
            "downloadUrl",
            &format!("{}/{}/{}", DOWNLOAD_PATH, self.kind, self.idx),
        )?;
        ser.end()
    }
}

pub struct AssetManager {
    pub assets: BTreeMap<AssetType, PatchSet>,
    inner: Mmap,
}

impl AssetManager {
    pub fn from_single(path: impl AsRef<Path>) -> VCRResult<AssetManager> {
        let mut input = File::open(path)?;
        let mut len: [u8; 4] = [0; 4];
        input.read_exact(&mut len)?;
        let header_len = u32::from_le_bytes(len) as u64;
        let mut header_buffer = vec![0; header_len as usize];
        input.read_exact(&mut header_buffer)?;
        let header: BTreeMap<AssetType, PatchSet> = rmp_serde::from_slice(&header_buffer)?;
        let total_len = input.metadata()?.len();

        Ok(AssetManager {
            assets: header,
            inner: unsafe {
                MmapOptions::new()
                    .offset(header_len + 4)
                    .len((total_len - (header_len + 4)) as usize)
                    .populate()
                    .map(&input)?
            },
        })
    }

    pub fn get_resources(&self) -> Vec<ChronSiteUpdate> {
        self.assets
            .iter()
            .flat_map(|(key, set)| {
                set.patches
                    .iter()
                    .enumerate()
                    .map(|(idx, patch)| ChronSiteUpdate {
                        path: patch.path.clone(),
                        kind: *key,
                        timestamp: patch.timestamp,
                        hash: patch.hash,
                        uncompressed_length: patch.uncompressed_length,
                        idx,
                    })
            })
            .collect()
    }

    pub fn read_asset(&self, asset: &AssetType, idx: usize) -> VCRResult<Vec<u8>> {
        let patch_set = &self.assets[asset];
        let mut cur = patch_set.initial.clone();
        let mut working_copy = Vec::new();
        let mut decompressor = Decompressor::new()?;

        for patch in &patch_set.patches[..=idx] {
            self.read_patch_inner(&mut decompressor, patch, &mut cur, &mut working_copy)?;
        }

        Ok(cur)
    }

    #[inline(always)]
    fn read_patch_inner<'a>(
        &self,
        decompressor: &mut Decompressor,
        patch: &PatchHeader,
        cur: &'a mut Vec<u8>,
        working_copy: &'a mut Vec<u8>,
    ) -> VCRResult<()> {
        let patch_data = decompressor.decompress(
            &self.inner[patch.offset as usize..patch.offset as usize + patch.length as usize],
            patch.uncompressed_patch_length as usize,
        )?;

        // pre-allocate extra space if needed
        working_copy.clear();
        unsafe {
            working_copy.reserve(patch.uncompressed_length as usize);
            working_copy.set_len(patch.uncompressed_length as usize);
        }
        // working_copy.append(&mut vec![0; patch.uncompressed_length as usize]);

        // apply patch to cur, saving result to working_copy
        patch::patch(cur, &mut patch_data.as_slice(), working_copy)?;

        mem::swap(cur, working_copy); // swap the result with the current working copy (which contains the result)

        Ok(())
    }

    // reads asset, checks it, and returns (TOTAL_ASSETS, Vec<usize>); where Vec<usize> is the index of the failures
    pub fn check_asset(&self, asset: &AssetType) -> VCRResult<(usize, Vec<usize>)> {
        let patch_set = &self.assets[asset];
        let mut cur = patch_set.initial.clone();
        let mut working_copy = Vec::new();
        let mut decompressor = Decompressor::new()?;
        let mut failures: Vec<usize> = Vec::new();

        let mut hash_params = blake2s_simd::blake2sp::Params::new();
        hash_params.hash_length(16);

        for (i, patch) in patch_set.patches.iter().enumerate() {
            self.read_patch_inner(&mut decompressor, patch, &mut cur, &mut working_copy)?;
            // calculate checksum of patched result
            let hash = hash_params.hash(&cur);

            if hash.as_bytes()[..] != patch.hash[..] {
                failures.push(i);
            }
        }

        Ok((patch_set.patches.len(), failures))
    }
}
