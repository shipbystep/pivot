use cucumber::when;

use crate::world::TaxiWorld;

#[when("I build the taxi database")]
fn build(world: &mut TaxiWorld) {
    let sources = pivot::app::Sources {
        trips: world.path("trips.parquet"),
        zones: world.path("zones.csv"),
    };
    let result = pivot::app::build_taxi_db(&sources, &world.path("taxi.db"));
    world.built = Some(result.map(|_| ()).map_err(|e| e.to_string()));
}
