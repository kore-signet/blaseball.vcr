use blaseball_vcr::{vhs::db, EntityDatabase, VCRError};

use vcr_schemas::GameUpdate;

fn main() -> Result<(), VCRError> {
    let db: db::Database<GameUpdate> = db::Database::from_single("./data/dist/gameupdate.vhs")?;
    let team = db
        .get_entity(
            uuid::uuid!("191e7bab-fcc2-4a9a-a993-ac22214ddb80").as_bytes(),
            i64::MAX,
        )?
        .unwrap();
    // dbg!(team.data.bench);
    println!("{}", serde_json::to_string_pretty(&team.data)?);

    Ok(())
}
