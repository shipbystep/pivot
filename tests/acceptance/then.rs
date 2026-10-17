use cucumber::then;

use crate::world::TaxiWorld;

#[then(expr = "the {string} table has {int} rows")]
fn table_rows(world: &mut TaxiWorld, table: String, expected: i64) {
    let sql = format!("SELECT COUNT(*) FROM {table}");
    let count: i64 = world.db().query_row(&sql, [], |row| row.get(0)).unwrap();
    assert_eq!(count, expected);
}

#[then(expr = "trip {int} was picked up at {string}")]
fn picked_up_at(world: &mut TaxiWorld, trip: i64, expected: String) {
    let sql = "SELECT pickup_at FROM trips WHERE rowid = ?1";
    let at: String = world.db().query_row(sql, [trip], |row| row.get(0)).unwrap();
    assert_eq!(at, expected);
}

#[then(expr = "trip {int} has no passenger count")]
fn no_passenger_count(world: &mut TaxiWorld, trip: i64) {
    let sql = "SELECT passenger_count FROM trips WHERE rowid = ?1";
    let count: Option<i64> = world.db().query_row(sql, [trip], |row| row.get(0)).unwrap();
    assert_eq!(count, None);
}

#[then("the build fails")]
fn build_fails(world: &mut TaxiWorld) {
    assert!(matches!(world.built, Some(Err(_))), "{:?}", world.built);
}

#[then("there is no taxi database")]
fn no_database(world: &mut TaxiWorld) {
    assert!(!world.path("taxi.db").exists());
    assert!(!world.path("taxi.db.tmp").exists());
}
