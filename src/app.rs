//! App layer: the steps of a job, in order. Each step lives in a lower layer.

use std::fs;
use std::path::{Path, PathBuf};

use rusqlite::Connection;

use crate::Result;
use crate::data::{read, sqlite};
use crate::model::{TRIPS, ZONES};

pub struct Sources {
    pub trips: PathBuf,
    pub zones: PathBuf,
}

pub struct Counts {
    pub trips: u64,
    pub zones: u64,
}

/// Builds the database next to `db_path`, then renames it into place.
/// If anything fails, the old database (if any) stays as it was.
pub fn build_taxi_db(sources: &Sources, db_path: &Path) -> Result<Counts> {
    let draft = db_path.with_extension("db.tmp");
    let _ = fs::remove_file(&draft);
    match write_db(sources, &draft) {
        Ok(counts) => {
            fs::rename(&draft, db_path)?;
            Ok(counts)
        }
        Err(error) => {
            let _ = fs::remove_file(&draft);
            Err(error)
        }
    }
}

/// One transaction: SQLite writes to disk once, at commit.
fn write_db(sources: &Sources, path: &Path) -> Result<Counts> {
    let mut db = Connection::open(path)?;
    let tx = db.transaction()?;
    sqlite::create_table(&tx, &TRIPS)?;
    sqlite::create_table(&tx, &ZONES)?;
    let trip_batches = read::parquet_batches(&sources.trips, &TRIPS)?;
    let zone_batches = read::csv_batches(&sources.zones, &ZONES)?;
    let trips = sqlite::insert_batches(&tx, &TRIPS, trip_batches)?;
    let zones = sqlite::insert_batches(&tx, &ZONES, zone_batches)?;
    tx.commit()?;
    Ok(Counts { trips, zones })
}
