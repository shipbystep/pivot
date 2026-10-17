//! Tiny copies of the TLC files, with the same column names and types.

use std::fs::{self, File};
use std::path::Path;
use std::sync::Arc;

use arrow::array::{ArrayRef, Float64Array, Int32Array, Int64Array, TimestampMicrosecondArray};
use arrow::record_batch::RecordBatch;
use parquet::arrow::ArrowWriter;

/// 2024-01-01 00:57:55, the first pickup in the real January 2024 file.
const FIRST_PICKUP_US: i64 = 1_704_070_675_000_000;
const ONE_MINUTE_US: i64 = 60_000_000;

pub fn write_trips(path: &Path, count: usize) {
    let batch = trips_batch(count);
    let mut writer =
        ArrowWriter::try_new(File::create(path).unwrap(), batch.schema(), None).unwrap();
    writer.write(&batch).unwrap();
    writer.close().unwrap();
}

pub fn write_zones(path: &Path) {
    let csv = "\"LocationID\",\"Borough\",\"Zone\",\"service_zone\"\n\
               1,\"EWR\",\"Newark Airport\",\"EWR\"\n\
               2,\"Queens\",\"Jamaica Bay\",\"Boro Zone\"\n";
    fs::write(path, csv).unwrap();
}

fn trips_batch(count: usize) -> RecordBatch {
    let pickups: Vec<i64> = (0..count as i64)
        .map(|i| FIRST_PICKUP_US + i * ONE_MINUTE_US)
        .collect();
    let dropoffs: Vec<i64> = pickups.iter().map(|at| at + 10 * ONE_MINUTE_US).collect();
    // Trip 2 has no passenger count, like 140,162 trips in the real file.
    let passengers: Vec<Option<i64>> = (0..count).map(|i| (i != 1).then_some(1)).collect();
    RecordBatch::try_from_iter([
        ("VendorID", ints32(count, 2)),
        (
            "tpep_pickup_datetime",
            Arc::new(TimestampMicrosecondArray::from(pickups)) as ArrayRef,
        ),
        (
            "tpep_dropoff_datetime",
            Arc::new(TimestampMicrosecondArray::from(dropoffs)) as ArrayRef,
        ),
        (
            "passenger_count",
            Arc::new(Int64Array::from(passengers)) as ArrayRef,
        ),
        ("trip_distance", floats(count, 1.7)),
        ("RatecodeID", ints64(count, 1)),
        ("PULocationID", ints32(count, 1)),
        ("DOLocationID", ints32(count, 2)),
        ("payment_type", ints64(count, 1)),
        ("fare_amount", floats(count, 12.5)),
        ("tip_amount", floats(count, 2.0)),
        ("total_amount", floats(count, 16.0)),
    ])
    .unwrap()
}

fn ints32(count: usize, value: i32) -> ArrayRef {
    Arc::new(Int32Array::from(vec![value; count]))
}

fn ints64(count: usize, value: i64) -> ArrayRef {
    Arc::new(Int64Array::from(vec![value; count]))
}

fn floats(count: usize, value: f64) -> ArrayRef {
    Arc::new(Float64Array::from(vec![value; count]))
}
