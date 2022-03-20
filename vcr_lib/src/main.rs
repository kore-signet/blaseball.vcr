use blaseball_vcr::vhs::schemas::*;
use blaseball_vcr::{
    db_manager,
    vhs::{self, db},
    EntityDatabase, VCRResult,
};
use std::time::Instant;
use uuid::Uuid;

fn main() -> VCRResult<()> {
    let db: db::Database<Sim> = db::Database::from_single("./vhs_tapes/sim.vhs")?;

    // 0 - 3
    // println!("{}", serde_json::to_string_pretty(&db.get_versions_inner(&Uuid::nil().as_bytes(), 1596294001, 1596266001)?).unwrap());

    // 98 - 100
    // println!("{}", serde_json::to_string_pretty(&db.get_versions_inner(&Uuid::nil().as_bytes(), 1596650400, 1596646801)?).unwrap());

    // 0 - 272
    let after = 1596266001;
    let before = 1598443202;
    let v = db
        .get_versions(&Uuid::nil().as_bytes(), before, after)?
        .unwrap();

    println!("{}", v.len());
    println!("{}", serde_json::to_string_pretty(&v.last())?);

    Ok(())
}
