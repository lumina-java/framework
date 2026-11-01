//! MSSQL Execution Adapter untuk Lumina ORM & QueryBuilder
//! Mengonversi baris data Tiberius TDS menjadi Struct Model melalui deserialisasi JSON cerdas.

use crate::database::connection::DatabasePool;
#[allow(unused_imports)]
use serde::de::DeserializeOwned;

#[cfg(feature = "mssql")]
use serde_json::Value;

#[cfg(feature = "mssql")]
use tiberius::ColumnData;

/// Konversi baris `tiberius::Row` ke `serde_json::Value` (Map JSON)
#[cfg(feature = "mssql")]
pub fn row_to_json(row: &tiberius::Row) -> Value {
    let mut map = serde_json::Map::new();
    for (col, data) in row.cells() {
        let key = col.name().to_string();
        let val = match data {
            ColumnData::U8(v) => v.map(|x| Value::Number(x.into())).unwrap_or(Value::Null),
            ColumnData::I16(v) => v.map(|x| Value::Number(x.into())).unwrap_or(Value::Null),
            ColumnData::I32(v) => v.map(|x| Value::Number(x.into())).unwrap_or(Value::Null),
            ColumnData::I64(v) => v.map(|x| Value::Number(x.into())).unwrap_or(Value::Null),
            ColumnData::F32(v) => v
                .and_then(|x| serde_json::Number::from_f64(x as f64))
                .map(Value::Number)
                .unwrap_or(Value::Null),
            ColumnData::F64(v) => v
                .and_then(serde_json::Number::from_f64)
                .map(Value::Number)
                .unwrap_or(Value::Null),
            ColumnData::Bit(v) => v.map(Value::Bool).unwrap_or(Value::Null),
            ColumnData::String(v) => v
                .as_deref()
                .map(|s| Value::String(s.to_string()))
                .unwrap_or(Value::Null),
            ColumnData::Guid(v) => v.map(|g| Value::String(g.to_string())).unwrap_or(Value::Null),
            ColumnData::Numeric(v) => v.map(|n| Value::String(n.to_string())).unwrap_or(Value::Null),
            ColumnData::DateTime(v) => v
                .map(|dt| Value::String(format!("{:?}", dt)))
                .unwrap_or(Value::Null),
            ColumnData::SmallDateTime(v) => v
                .map(|dt| Value::String(format!("{:?}", dt)))
                .unwrap_or(Value::Null),
            ColumnData::DateTime2(v) => v
                .map(|dt| Value::String(format!("{:?}", dt)))
                .unwrap_or(Value::Null),
            ColumnData::Date(v) => v.map(|d| Value::String(format!("{:?}", d))).unwrap_or(Value::Null),
            ColumnData::Time(v) => v.map(|t| Value::String(format!("{:?}", t))).unwrap_or(Value::Null),
            _ => Value::Null,
        };
        map.insert(key, val);
    }
    Value::Object(map)
}

/// Konversi satu baris `tiberius::Row` langsung menjadi instance Model `T`
#[cfg(feature = "mssql")]
pub fn row_to_model<T: DeserializeOwned>(row: &tiberius::Row) -> Result<T, sqlx::Error> {
    let json_val = row_to_json(row);
    serde_json::from_value::<T>(json_val)
        .map_err(|e| sqlx::Error::Decode(Box::new(e)))
}

/// Eksekusi SELECT query pada MSSQL dan petakan ke koleksi model `Vec<T>`
#[cfg(feature = "mssql")]
pub async fn query_models<T: DeserializeOwned>(
    pool: &DatabasePool,
    sql: &str,
) -> Result<Vec<T>, sqlx::Error> {
    let mssql_pool = pool.mssql().ok_or_else(|| {
        sqlx::Error::Configuration("MSSQL pool belum terhubung atau fitur mssql tidak aktif".into())
    })?;

    let mut conn = mssql_pool
        .get()
        .await
        .map_err(|e| sqlx::Error::Configuration(format!("Gagal acquire koneksi MSSQL: {}", e).into()))?;

    let stream = conn
        .simple_query(sql)
        .await
        .map_err(|e| sqlx::Error::Configuration(format!("MSSQL query error: {}", e).into()))?;

    let rows = stream
        .into_first_result()
        .await
        .map_err(|e| sqlx::Error::Configuration(format!("MSSQL stream error: {}", e).into()))?;

    let mut result = Vec::with_capacity(rows.len());
    for row in rows {
        let model: T = row_to_model(&row)?;
        result.push(model);
    }

    Ok(result)
}

/// Eksekusi query SELECT satu baris pada MSSQL
#[cfg(feature = "mssql")]
pub async fn query_first<T: DeserializeOwned>(
    pool: &DatabasePool,
    sql: &str,
) -> Result<Option<T>, sqlx::Error> {
    let models: Vec<T> = query_models(pool, sql).await?;
    Ok(models.into_iter().next())
}

/// Hitung jumlah baris (COUNT) pada MSSQL
#[cfg(feature = "mssql")]
pub async fn count(pool: &DatabasePool, sql: &str) -> Result<i64, sqlx::Error> {
    let mssql_pool = pool.mssql().ok_or_else(|| {
        sqlx::Error::Configuration("MSSQL pool belum terhubung atau fitur mssql tidak aktif".into())
    })?;

    let mut conn = mssql_pool
        .get()
        .await
        .map_err(|e| sqlx::Error::Configuration(format!("Gagal acquire koneksi MSSQL: {}", e).into()))?;

    let stream = conn
        .simple_query(sql)
        .await
        .map_err(|e| sqlx::Error::Configuration(format!("MSSQL count query error: {}", e).into()))?;

    let rows = stream
        .into_first_result()
        .await
        .map_err(|e| sqlx::Error::Configuration(format!("MSSQL count stream error: {}", e).into()))?;

    if let Some(row) = rows.first() {
        if let Some(val) = row.get::<i32, _>(0) {
            return Ok(val as i64);
        }
        if let Some(val) = row.get::<i64, _>(0) {
            return Ok(val);
        }
    }

    Ok(0)
}

/// Eksekusi perintah mutasi (UPDATE, DELETE) pada MSSQL
#[cfg(feature = "mssql")]
pub async fn execute(pool: &DatabasePool, sql: &str) -> Result<u64, sqlx::Error> {
    let mssql_pool = pool.mssql().ok_or_else(|| {
        sqlx::Error::Configuration("MSSQL pool belum terhubung atau fitur mssql tidak aktif".into())
    })?;

    let mut conn = mssql_pool
        .get()
        .await
        .map_err(|e| sqlx::Error::Configuration(format!("Gagal acquire koneksi MSSQL: {}", e).into()))?;

    let stream = conn
        .simple_query(sql)
        .await
        .map_err(|e| sqlx::Error::Configuration(format!("MSSQL execute query error: {}", e).into()))?;

    let results = stream
        .into_first_result()
        .await
        .map_err(|e| sqlx::Error::Configuration(format!("MSSQL execute stream error: {}", e).into()))?;

    Ok(results.len() as u64)
}

/// Eksekusi perintah INSERT dan kembalikan SCOPE_IDENTITY() pada MSSQL
#[cfg(feature = "mssql")]
pub async fn execute_insert_and_get_id(pool: &DatabasePool, sql: &str) -> Result<i64, sqlx::Error> {
    let mssql_pool = pool.mssql().ok_or_else(|| {
        sqlx::Error::Configuration("MSSQL pool belum terhubung atau fitur mssql tidak aktif".into())
    })?;

    let mut conn = mssql_pool
        .get()
        .await
        .map_err(|e| sqlx::Error::Configuration(format!("Gagal acquire koneksi MSSQL: {}", e).into()))?;

    let full_sql = format!("{}; SELECT CAST(SCOPE_IDENTITY() AS BIGINT) AS inserted_id;", sql);
    let stream = conn
        .simple_query(&full_sql)
        .await
        .map_err(|e| sqlx::Error::Configuration(format!("MSSQL insert query error: {}", e).into()))?;

    let rows = stream
        .into_first_result()
        .await
        .map_err(|e| sqlx::Error::Configuration(format!("MSSQL insert stream error: {}", e).into()))?;

    if let Some(row) = rows.first() {
        if let Some(id) = row.get::<i64, _>(0) {
            return Ok(id);
        }
        if let Some(id) = row.get::<i32, _>(0) {
            return Ok(id as i64);
        }
    }

    Ok(0)
}

// =========================================================================
// Fallback Stubs ketika fitur `mssql` tidak aktif di Cargo
// =========================================================================

#[cfg(not(feature = "mssql"))]
pub async fn query_models<T>(_pool: &DatabasePool, _sql: &str) -> Result<Vec<T>, sqlx::Error> {
    Err(sqlx::Error::Configuration("Fitur mssql tidak diaktifkan pada build ini".into()))
}

#[cfg(not(feature = "mssql"))]
pub async fn query_first<T>(_pool: &DatabasePool, _sql: &str) -> Result<Option<T>, sqlx::Error> {
    Err(sqlx::Error::Configuration("Fitur mssql tidak diaktifkan pada build ini".into()))
}

#[cfg(not(feature = "mssql"))]
pub async fn count(_pool: &DatabasePool, _sql: &str) -> Result<i64, sqlx::Error> {
    Err(sqlx::Error::Configuration("Fitur mssql tidak diaktifkan pada build ini".into()))
}

#[cfg(not(feature = "mssql"))]
pub async fn execute(_pool: &DatabasePool, _sql: &str) -> Result<u64, sqlx::Error> {
    Err(sqlx::Error::Configuration("Fitur mssql tidak diaktifkan pada build ini".into()))
}

#[cfg(not(feature = "mssql"))]
pub async fn execute_insert_and_get_id(_pool: &DatabasePool, _sql: &str) -> Result<i64, sqlx::Error> {
    Err(sqlx::Error::Configuration("Fitur mssql tidak diaktifkan pada build ini".into()))
}
