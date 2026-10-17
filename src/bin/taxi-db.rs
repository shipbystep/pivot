//! Builds data/taxi.db from the TLC files that `just data` downloads.

use std::path::Path;
use std::time::Instant;

use pivot::app::{Sources, build_taxi_db};

fn main() -> pivot::Result<()> {
    let sources = Sources {
        trips: "data/yellow_tripdata_2024-01.parquet".into(),
        zones: "data/taxi_zone_lookup.csv".into(),
    };
    let start = Instant::now();
    let counts = build_taxi_db(&sources, Path::new("data/taxi.db"))?;
    println!(
        "data/taxi.db: {} trips, {} zones in {:.1?}",
        counts.trips,
        counts.zones,
        start.elapsed()
    );
    Ok(())
}
