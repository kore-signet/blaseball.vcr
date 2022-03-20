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

    // db.get_versions_inner(&Uuid::nil().as_bytes(), 1596647381, 1596266001)?;

    Ok(())
}
