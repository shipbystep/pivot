//! BDD scenarios from features/*.feature, run with `just bdd`.

mod fixtures;
mod given;
mod then;
mod when;
mod world;

use cucumber::World as _;

#[tokio::main]
async fn main() {
    world::TaxiWorld::run("features").await;
}
