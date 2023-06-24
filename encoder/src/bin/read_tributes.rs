use blaseball_vcr::{vhs::tributes::TributesDatabase, EntityDatabase};
use uuid::Uuid;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let players:Database::<Player> = Database::from_single("./data/dist/vhs_tapes/player.vhs")?;
    let tributes = TributesDatabase::from_single("tributes.vhs")?;

    let v = tributes
        .get_entity(
            Uuid::nil().as_bytes(),
            i64::MAX, // timestamp_to_nanos(
                      //     iso8601_timestamp::Timestamp::parse("2022-01-26T10:33:01.000344Z").unwrap(),
                      // ),
        )?
        .unwrap();
    println!("{}", serde_json::to_string_pretty(&v)?);

    Ok(())
}
