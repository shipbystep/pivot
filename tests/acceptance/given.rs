use cucumber::given;

use crate::fixtures;
use crate::world::TaxiWorld;

#[given(expr = "a trips Parquet file with {int} trips")]
fn trips_file(world: &mut TaxiWorld, count: usize) {
    fixtures::write_trips(&world.path("trips.parquet"), count);
}

#[given("a zone lookup CSV with 2 zones")]
fn zones_file(world: &mut TaxiWorld) {
    fixtures::write_zones(&world.path("zones.csv"));
}

#[given("the trips Parquet file is missing")]
fn no_trips_file(_world: &mut TaxiWorld) {}
