//! The shape of the taxi database: tables, columns and column types.
//! Only descriptions live here. No files, no database.

use ColumnType::{Integer, Real, Text};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ColumnType {
    Integer,
    Real,
    Text,
}

/// One column: its name in the source file and its name in our database.
pub struct Column {
    pub source: &'static str,
    pub name: &'static str,
    pub kind: ColumnType,
}

pub struct Table {
    pub name: &'static str,
    pub columns: &'static [Column],
}

/// 12 of the 19 TLC columns: enough for time, place, payment and money pivots.
/// Times are local New York time, stored as text "YYYY-MM-DD HH:MM:SS".
pub const TRIPS: Table = Table {
    name: "trips",
    columns: &[
        column("VendorID", "vendor_id", Integer),
        column("tpep_pickup_datetime", "pickup_at", Text),
        column("tpep_dropoff_datetime", "dropoff_at", Text),
        column("passenger_count", "passenger_count", Integer),
        column("trip_distance", "trip_distance", Real),
        column("RatecodeID", "rate_code", Integer),
        column("PULocationID", "pickup_zone_id", Integer),
        column("DOLocationID", "dropoff_zone_id", Integer),
        column("payment_type", "payment_type", Integer),
        column("fare_amount", "fare_amount", Real),
        column("tip_amount", "tip_amount", Real),
        column("total_amount", "total_amount", Real),
    ],
};

pub const ZONES: Table = Table {
    name: "zones",
    columns: &[
        column("LocationID", "zone_id", Integer),
        column("Borough", "borough", Text),
        column("Zone", "zone", Text),
        column("service_zone", "service_zone", Text),
    ],
};

const fn column(source: &'static str, name: &'static str, kind: ColumnType) -> Column {
    Column { source, name, kind }
}
