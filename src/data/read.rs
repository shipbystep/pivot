//! Reads the TLC files as Arrow batches, already in the shape of our table.

use std::fs::File;
use std::path::Path;
use std::sync::Arc;

use arrow::array::ArrayRef;
use arrow::compute::{CastOptions, cast_with_options};
use arrow::csv::ReaderBuilder;
use arrow::datatypes::{DataType, Field, Schema};
use arrow::record_batch::RecordBatch;
use arrow::util::display::FormatOptions;
use parquet::arrow::ProjectionMask;
use parquet::arrow::arrow_reader::ParquetRecordBatchReaderBuilder;

use crate::Result;
use crate::model::{ColumnType, Table};

/// Batches stream one by one, so a big file never sits in memory at once.
pub type Batches = Box<dyn Iterator<Item = Result<RecordBatch>>>;

const TIME_FORMAT: &str = "%Y-%m-%d %H:%M:%S";

pub fn parquet_batches(path: &Path, table: &'static Table) -> Result<Batches> {
    let builder = ParquetRecordBatchReaderBuilder::try_new(File::open(path)?)?;
    let names = table.columns.iter().map(|column| column.source);
    let only_our_columns = ProjectionMask::columns(builder.parquet_schema(), names);
    let reader = builder.with_projection(only_our_columns).build()?;
    Ok(Box::new(reader.map(move |batch| conform(&batch?, table))))
}

pub fn csv_batches(path: &Path, table: &'static Table) -> Result<Batches> {
    let reader = ReaderBuilder::new(Arc::new(arrow_schema(table)))
        .with_header(true)
        .build(File::open(path)?)?;
    Ok(Box::new(reader.map(move |batch| conform(&batch?, table))))
}

/// Picks our columns in our order and casts each one to its Arrow type.
fn conform(batch: &RecordBatch, table: &Table) -> Result<RecordBatch> {
    let mut columns = Vec::new();
    for column in table.columns {
        let source = batch
            .column_by_name(column.source)
            .ok_or_else(|| format!("{}: no column {}", table.name, column.source))?;
        columns.push(cast_to(source, column.kind)?);
    }
    Ok(RecordBatch::try_new(
        Arc::new(arrow_schema(table)),
        columns,
    )?)
}

fn arrow_schema(table: &Table) -> Schema {
    let fields = table
        .columns
        .iter()
        .map(|column| Field::new(column.source, arrow_type(column.kind), true));
    Schema::new(fields.collect::<Vec<_>>())
}

/// Fails on a value that does not fit, instead of turning it into NULL.
fn cast_to(array: &ArrayRef, kind: ColumnType) -> Result<ArrayRef> {
    let options = CastOptions {
        safe: false,
        format_options: FormatOptions::new().with_timestamp_format(Some(TIME_FORMAT)),
    };
    Ok(cast_with_options(array, &arrow_type(kind), &options)?)
}

fn arrow_type(kind: ColumnType) -> DataType {
    match kind {
        ColumnType::Integer => DataType::Int64,
        ColumnType::Real => DataType::Float64,
        ColumnType::Text => DataType::Utf8,
    }
}

#[cfg(test)]
mod tests {
    use arrow::array::{AsArray, Int32Array, TimestampMicrosecondArray};
    use arrow::datatypes::Int64Type;

    use super::*;

    #[test]
    fn a_timestamp_becomes_sqlite_style_text() {
        // 2024-01-01 00:57:55, the first pickup in the January 2024 file.
        let at: ArrayRef = Arc::new(TimestampMicrosecondArray::from(vec![1_704_070_675_000_000]));
        let text = cast_to(&at, ColumnType::Text).unwrap();
        assert_eq!(text.as_string::<i32>().value(0), "2024-01-01 00:57:55");
    }

    #[test]
    fn a_small_integer_becomes_int64() {
        let id: ArrayRef = Arc::new(Int32Array::from(vec![132]));
        let wide = cast_to(&id, ColumnType::Integer).unwrap();
        assert_eq!(wide.as_primitive::<Int64Type>().value(0), 132);
    }

    #[test]
    fn text_that_is_not_a_number_is_an_error() {
        let text: ArrayRef = Arc::new(arrow::array::StringArray::from(vec!["Queens"]));
        assert!(cast_to(&text, ColumnType::Integer).is_err());
    }
}
