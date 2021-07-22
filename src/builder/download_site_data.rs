use blaseball_vcr::{
    site::{chron, delta},
    ChroniclerV1Response, VCRError, VCRResult,
};
use reqwest::blocking;
use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

fn main() -> VCRResult<()> {
    let chron_res: ChroniclerV1Response<chron::SiteUpdate> =
        blocking::get("https://api.sibr.dev/chronicler/v1/site/updates")
            .map_err(VCRError::ReqwestError)?
            .json()
            .map_err(VCRError::ReqwestError)?;
    let all_steps = chron::updates_to_steps(chron_res.data);
    let args: Vec<String> = env::args().collect();

    for (name, steps) in all_steps {
        println!("Recording asset {}", name);
        let main_path = Path::new(&args[1]).join(&format!("{}.bin", name));
        let header_path = Path::new(&args[1]).join(&format!("{}.header.bin", name));

        let main_f = File::create(main_path).map_err(VCRError::IOError)?;
        let mut main_out = BufWriter::new(main_f);

        let header = delta::encode_resource(steps, &mut main_out)?;

        let mut header_f = File::create(header_path).map_err(VCRError::IOError)?;
        rmp_serde::encode::write(&mut header_f, &header).map_err(VCRError::MsgPackEncError)?;
    }

    Ok(())
}
