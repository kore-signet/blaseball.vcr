use std::{fs::File, io::Read};

use blaseball_vcr::VCRResult;
use clap::clap_app;
use humansize::{format_size, DECIMAL};

fn main() -> VCRResult<()> {
    let matches = clap_app!(analyze_tape =>
        (version: "1.0")
        (author: "emily signet <emily@sibr.dev>")
        (about: "blaseball.vcr simple tape analyzer")
        (@arg INPUT: +required -i --input [FILE] "tape file")
    )
    .get_matches();

    let mut file = File::open(matches.value_of("INPUT").unwrap())?;
    let mut len_bytes: [u8; 8] = [0; 8];
    file.read_exact(&mut len_bytes)?;

    let dict_len = u64::from_le_bytes(len_bytes) as usize;

    if dict_len > 0 {
        let mut dict = vec![0u8; dict_len];
        file.read_exact(&mut dict)?;
    };

    file.read_exact(&mut len_bytes)?;
    let header_len = u64::from_le_bytes(len_bytes) as usize;

    let mut header_bytes = vec![0u8; header_len];
    file.read_exact(&mut header_bytes)?;

    let total_len = file.metadata()?.len() as usize;
    let store_len: usize = total_len - (dict_len + header_len + 16);

    println!("total len: {}", format_size(total_len, DECIMAL));

    println!(
        "dictionary: {} ({:.2}%)",
        format_size(dict_len, DECIMAL),
        (dict_len as f64 / total_len as f64) * 100f64
    );
    println!(
        "header: {} ({:.2}%)",
        format_size(header_len, DECIMAL),
        (header_len as f64 / total_len as f64) * 100f64
    );
    println!(
        "store: {} ({:.2}%)",
        format_size(store_len, DECIMAL),
        (store_len as f64 / total_len as f64) * 100f64
    );

    // let header: T = rmp_serde::from_read(zstd::Decoder::new(&header_bytes[..])?)?;

    // let inner = unsafe {
    //     MmapOptions::new()
    //         .offset((dict_len + header_len + 16) as u64)
    //         .len()
    //         .map(&file)?
    // };

    // Ok(TapeComponents {
    //     dict,
    //     header,
    //     store: inner,
    // })

    Ok(())
}
