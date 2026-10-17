//! Writes Arrow batches into SQLite tables.

use arrow::array::{Array, ArrayRef, AsArray};
use arrow::datatypes::{Float64Type, Int64Type};
use arrow::record_batch::RecordBatch;
use rusqlite::types::Value;
use rusqlite::{Connection, params_from_iter};

use crate::Result;
use crate::model::{ColumnType, Table};

pub fn create_table(db: &Connection, table: &Table) -> Result<()> {
    db.execute(&create_sql(table), [])?;
    Ok(())
}

/// Inserts every row of every batch. Returns how many rows were written.
pub fn insert_batches(
    db: &Connection,
    table: &Table,
    batches: impl Iterator<Item = Result<RecordBatch>>,
) -> Result<u64> {
    let mut insert = db.prepare(&insert_sql(table))?;
    let mut rows = 0;
    for batch in batches {
        let batch = batch?;
        for row in 0..batch.num_rows() {
            insert.execute(params_from_iter(row_values(table, batch.columns(), row)))?;
        }
        rows += batch.num_rows() as u64;
    }
    Ok(rows)
}

fn create_sql(table: &Table) -> String {
    let columns: Vec<String> = table
        .columns
        .iter()
        .map(|column| format!("{} {}", column.name, sql_type(column.kind)))
        .collect();
    format!("CREATE TABLE {} ({})", table.name, columns.join(", "))
}

fn insert_sql(table: &Table) -> String {
    let names: Vec<&str> = table.columns.iter().map(|column| column.name).collect();
    let placeholders = vec!["?"; names.len()].join(", ");
    format!(
        "INSERT INTO {} ({}) VALUES ({placeholders})",
        table.name,
        names.join(", ")
    )
}

fn sql_type(kind: ColumnType) -> &'static str {
    match kind {
        ColumnType::Integer => "INTEGER",
        ColumnType::Real => "REAL",
        ColumnType::Text => "TEXT",
    }
}

fn row_values(table: &Table, columns: &[ArrayRef], row: usize) -> Vec<Value> {
    table
        .columns
        .iter()
        .zip(columns)
        .map(|(column, array)| value_at(array, column.kind, row))
        .collect()
}

/// Expects the array type that `data::read` gives each column type.
fn value_at(array: &ArrayRef, kind: ColumnType, row: usize) -> Value {
    if array.is_null(row) {
        return Value::Null;
    }
    match kind {
        ColumnType::Integer => Value::Integer(array.as_primitive::<Int64Type>().value(row)),
        ColumnType::Real => Value::Real(array.as_primitive::<Float64Type>().value(row)),
        ColumnType::Text => Value::Text(array.as_string::<i32>().value(row).to_string()),
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use arrow::array::{Int64Array, StringArray};

    use super::*;
    use crate::model::ZONES;

    #[test]
    fn create_sql_names_every_column_with_its_type() {
        let expected =
            "CREATE TABLE zones (zone_id INTEGER, borough TEXT, zone TEXT, service_zone TEXT)";
        assert_eq!(create_sql(&ZONES), expected);
    }

    #[test]
    fn insert_sql_has_one_placeholder_per_column() {
        let expected =
            "INSERT INTO zones (zone_id, borough, zone, service_zone) VALUES (?, ?, ?, ?)";
        assert_eq!(insert_sql(&ZONES), expected);
    }

    #[test]
    fn a_missing_value_becomes_sql_null() {
        let array: ArrayRef = Arc::new(Int64Array::from(vec![Some(1), None]));
        assert_eq!(value_at(&array, ColumnType::Integer, 0), Value::Integer(1));
        assert_eq!(value_at(&array, ColumnType::Integer, 1), Value::Null);
    }

    #[test]
    fn text_stays_text() {
        let array: ArrayRef = Arc::new(StringArray::from(vec!["Queens"]));
        assert_eq!(
            value_at(&array, ColumnType::Text, 0),
            Value::Text("Queens".into())
        );
    }
}
