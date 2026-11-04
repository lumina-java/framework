use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tokio::sync::Mutex;
use sqlx::{Any, Column, Row, Transaction as SqlxTransaction};
use crate::database::connection::{DatabaseKind, DatabasePool};

/// Jenis koneksi aktif di dalam transaksi.
pub enum TxInner {
    Sqlx(Option<SqlxTransaction<'static, Any>>),
    #[cfg(feature = "mssql")]
    Mssql(Option<bb8::PooledConnection<'static, bb8_tiberius::ConnectionManager>>),
}

/// Helper untuk mengonversi AnyRow ke serde_json::Value secara transparan tanpa generic `<>`.
pub fn any_row_to_json(row: &sqlx::any::AnyRow) -> serde_json::Value {
    let mut map = serde_json::Map::new();
    for col in row.columns() {
        let name = col.name().to_string();
        let val = if let Ok(v) = row.try_get::<i64, _>(name.as_str()) {
            serde_json::Value::Number(v.into())
        } else if let Ok(v) = row.try_get::<f64, _>(name.as_str()) {
            serde_json::Number::from_f64(v)
                .map(serde_json::Value::Number)
                .unwrap_or(serde_json::Value::Null)
        } else if let Ok(v) = row.try_get::<bool, _>(name.as_str()) {
            serde_json::Value::Bool(v)
        } else if let Ok(v) = row.try_get::<String, _>(name.as_str()) {
            serde_json::Value::String(v)
        } else {
            serde_json::Value::Null
        };
        map.insert(name, val);
    }
    serde_json::Value::Object(map)
}

/// Lumina Transaction Handle — representasi transaksi database ACID yang ergonomis,
/// bebas anotasi seumur hidup (lifetimes), thread-safe, dan auto-rollback on drop.
#[derive(Clone)]
pub struct Transaction {
    pub(crate) kind: DatabaseKind,
    pub(crate) inner: Arc<Mutex<TxInner>>,
    pub(crate) committed: Arc<AtomicBool>,
}

impl Transaction {
    /// Membuat instance transaksi baru dari sqlx transaction.
    pub fn from_sqlx(kind: DatabaseKind, tx: SqlxTransaction<'static, Any>) -> Self {
        Self {
            kind,
            inner: Arc::new(Mutex::new(TxInner::Sqlx(Some(tx)))),
            committed: Arc::new(AtomicBool::new(false)),
        }
    }

    /// Membuat instance transaksi baru dari koneksi MSSQL Tiberius.
    #[cfg(feature = "mssql")]
    pub fn from_mssql(conn: bb8::PooledConnection<'static, bb8_tiberius::ConnectionManager>) -> Self {
        Self {
            kind: DatabaseKind::Mssql,
            inner: Arc::new(Mutex::new(TxInner::Mssql(Some(conn)))),
            committed: Arc::new(AtomicBool::new(false)),
        }
    }

    /// Jenis driver database transaksi ini.
    pub fn kind(&self) -> DatabaseKind {
        self.kind
    }

    /// Menyesuaikan placeholder SQL query (?, $1, @p1) sesuai driver database.
    pub fn prepare_sql(&self, sql: &str) -> String {
        match self.kind {
            DatabaseKind::Postgres => {
                let mut result = String::with_capacity(sql.len() + 16);
                let mut idx = 1;
                for ch in sql.chars() {
                    if ch == '?' {
                        result.push('$');
                        result.push_str(&idx.to_string());
                        idx += 1;
                    } else {
                        result.push(ch);
                    }
                }
                result
            }
            DatabaseKind::Mssql => {
                let mut result = String::with_capacity(sql.len() + 16);
                let mut idx = 1;
                for ch in sql.chars() {
                    if ch == '?' {
                        result.push_str("@p");
                        result.push_str(&idx.to_string());
                        idx += 1;
                    } else {
                        result.push(ch);
                    }
                }
                result
            }
            _ => sql.to_string(),
        }
    }

    /// Commit seluruh perubahan data dalam transaksi ini ke database secara permanen.
    pub async fn commit(&self) -> Result<(), sqlx::Error> {
        if self.committed.swap(true, Ordering::SeqCst) {
            return Ok(()); // Sudah pernah di-commit
        }

        let mut lock = self.inner.lock().await;
        match &mut *lock {
            TxInner::Sqlx(opt) => {
                if let Some(tx) = opt.take() {
                    tx.commit().await?;
                }
                Ok(())
            }
            #[cfg(feature = "mssql")]
            TxInner::Mssql(opt) => {
                if let Some(conn) = opt.as_mut() {
                    conn.simple_query("COMMIT TRANSACTION").await.map_err(|e| {
                        sqlx::Error::Configuration(format!("Gagal commit transaksi MSSQL: {}", e).into())
                    })?;
                }
                Ok(())
            }
        }
    }

    /// Batalkan seluruh perubahan data dalam transaksi ini (Rollback manual).
    pub async fn rollback(&self) -> Result<(), sqlx::Error> {
        if self.committed.swap(true, Ordering::SeqCst) {
            return Ok(());
        }

        let mut lock = self.inner.lock().await;
        match &mut *lock {
            TxInner::Sqlx(opt) => {
                if let Some(tx) = opt.take() {
                    tx.rollback().await?;
                }
                Ok(())
            }
            #[cfg(feature = "mssql")]
            TxInner::Mssql(opt) => {
                if let Some(conn) = opt.as_mut() {
                    let _ = conn.simple_query("ROLLBACK TRANSACTION").await;
                }
                Ok(())
            }
        }
    }

    /// Buat Savepoint bertingkat di dalam transaksi aktif.
    pub async fn savepoint(&self, name: &str) -> Result<(), sqlx::Error> {
        let clean_name = name.replace(|c: char| !c.is_alphanumeric() && c != '_', "");
        let sql = match self.kind {
            DatabaseKind::Mssql => format!("SAVE TRANSACTION {}", clean_name),
            _ => format!("SAVEPOINT {}", clean_name),
        };
        self.execute_raw(&sql).await?;
        Ok(())
    }

    /// Batalkan perubahan hanya sampai ke titik Savepoint tertentu.
    pub async fn rollback_to(&self, name: &str) -> Result<(), sqlx::Error> {
        let clean_name = name.replace(|c: char| !c.is_alphanumeric() && c != '_', "");
        let sql = match self.kind {
            DatabaseKind::Mssql => format!("ROLLBACK TRANSACTION {}", clean_name),
            _ => format!("ROLLBACK TO SAVEPOINT {}", clean_name),
        };
        self.execute_raw(&sql).await?;
        Ok(())
    }

    /// Eksekusi SQL raw sederhana di dalam transaksi.
    pub async fn execute_raw(&self, sql: &str) -> Result<u64, sqlx::Error> {
        let mut lock = self.inner.lock().await;
        match &mut *lock {
            TxInner::Sqlx(opt) => {
                if let Some(tx) = opt.as_mut() {
                    let res = sqlx::query(sql).execute(&mut **tx).await?;
                    Ok(res.rows_affected())
                } else {
                    Err(sqlx::Error::Configuration("Transaksi sudah ditutup".into()))
                }
            }
            #[cfg(feature = "mssql")]
            TxInner::Mssql(opt) => {
                if let Some(conn) = opt.as_mut() {
                    let stream = conn.simple_query(sql).await.map_err(|e| {
                        sqlx::Error::Configuration(format!("Error execute raw MSSQL: {}", e).into())
                    })?;
                    let results = stream.into_results().await.map_err(|e| {
                        sqlx::Error::Configuration(format!("Error stream raw MSSQL: {}", e).into())
                    })?;
                    let total = results.iter().map(|v| v.len() as u64).sum();
                    Ok(total)
                } else {
                    Err(sqlx::Error::Configuration("Transaksi sudah ditutup".into()))
                }
            }
        }
    }

    /// Inisialisasi query builder berbasis nama tabel langsung TANPA sintaks generic `<>`.
    /// Contoh: `tx.table("accounts").where_eq("id", 1).decrement("balance", 50000).await?;`
    pub fn table(&self, name: &'static str) -> TableQueryBuilder {
        TableQueryBuilder::new(self.clone(), name)
    }

    /// Cari satu baris data berdasarkan ID langsung dari nama tabel tanpa generic `<>`.
    /// Contoh: `let user = tx.find("users", 1).await?;`
    pub async fn find(&self, table: &'static str, id: i64) -> Result<Option<serde_json::Value>, sqlx::Error> {
        self.table(table).where_eq("id", id).first().await
    }

    /// Cari satu baris data atau return Error RowNotFound jika tidak ada.
    pub async fn find_or_fail(&self, table: &'static str, id: i64) -> Result<serde_json::Value, sqlx::Error> {
        self.find(table, id)
            .await?
            .ok_or_else(|| sqlx::Error::RowNotFound)
    }
}

impl Drop for Transaction {
    fn drop(&mut self) {
        // Rollback HANYA jika ini adalah referensi TERAKHIR yang tersisa (strong_count == 1)
        // dan transaksi belum pernah di-commit!
        if Arc::strong_count(&self.inner) == 1 && !self.committed.load(Ordering::SeqCst) {
            self.committed.store(true, Ordering::SeqCst);
            let inner = self.inner.clone();
            tokio::spawn(async move {
                let mut lock = inner.lock().await;
                match &mut *lock {
                    TxInner::Sqlx(opt) => {
                        if let Some(tx) = opt.take() {
                            let _ = tx.rollback().await;
                        }
                    }
                    #[cfg(feature = "mssql")]
                    TxInner::Mssql(opt) => {
                        if let Some(conn) = opt.as_mut() {
                            let _ = conn.simple_query("ROLLBACK TRANSACTION").await;
                        }
                    }
                }
            });
        }
    }
}

/// Query Builder khusus tabel yang sangat mudah diketik tanpa anotasi `<>`.
pub struct TableQueryBuilder {
    tx: Transaction,
    table: &'static str,
    select: String,
    wheres: Vec<String>,
    bind_values: Vec<String>,
    order_by: Option<String>,
    limit: Option<usize>,
    offset: Option<usize>,
    has_soft_deletes: bool,
    with_trashed: bool,
    only_trashed: bool,
}

impl TableQueryBuilder {
    pub fn new(tx: Transaction, table: &'static str) -> Self {
        Self {
            tx,
            table,
            select: "*".to_string(),
            wheres: Vec::new(),
            bind_values: Vec::new(),
            order_by: None,
            limit: None,
            offset: None,
            has_soft_deletes: false,
            with_trashed: false,
            only_trashed: false,
        }
    }

    /// Tentukan kolom yang akan diambil (default: "*")
    pub fn select(mut self, cols: &str) -> Self {
        self.select = cols.to_string();
        self
    }

    /// Aktifkan mode soft deletes pada query tabel ini
    pub fn soft_deletes(mut self) -> Self {
        self.has_soft_deletes = true;
        self
    }

    /// Sertakan data yang telah di-soft-delete dalam query
    pub fn with_trashed(mut self) -> Self {
        self.has_soft_deletes = true;
        self.with_trashed = true;
        self
    }

    /// Ambil HANYA data yang telah di-soft-delete (Recycle Bin)
    pub fn only_trashed(mut self) -> Self {
        self.has_soft_deletes = true;
        self.only_trashed = true;
        self
    }

    /// Urutkan hasil query
    pub fn order_by(mut self, column: &str, direction: &str) -> Self {
        self.order_by = Some(format!("{} {}", column, direction));
        self
    }

    /// Shortcut urutkan dari yang terbaru (created_at DESC)
    pub fn latest(self) -> Self {
        self.order_by("created_at", "DESC")
    }

    /// Shortcut urutkan dari yang terlama (created_at ASC)
    pub fn oldest(self) -> Self {
        self.order_by("created_at", "ASC")
    }

    /// Batasi jumlah baris hasil query
    pub fn limit(mut self, n: usize) -> Self {
        self.limit = Some(n);
        self
    }

    /// Lewati sejumlah baris
    pub fn offset(mut self, n: usize) -> Self {
        self.offset = Some(n);
        self
    }

    /// Filter WHERE column = value
    pub fn where_eq<V: ToString>(mut self, column: &str, value: V) -> Self {
        let prefix = if self.wheres.is_empty() { "" } else { "AND " };
        self.wheres.push(format!("{}{} = ?", prefix, column));
        self.bind_values.push(value.to_string());
        self
    }

    /// Filter WHERE column != value
    pub fn where_ne<V: ToString>(mut self, column: &str, value: V) -> Self {
        let prefix = if self.wheres.is_empty() { "" } else { "AND " };
        self.wheres.push(format!("{}{} != ?", prefix, column));
        self.bind_values.push(value.to_string());
        self
    }

    /// Filter WHERE column LIKE pattern
    pub fn where_like<V: ToString>(mut self, column: &str, pattern: V) -> Self {
        let prefix = if self.wheres.is_empty() { "" } else { "AND " };
        self.wheres.push(format!("{}{} LIKE ?", prefix, column));
        self.bind_values.push(pattern.to_string());
        self
    }

    /// Filter WHERE column IS NULL
    pub fn where_null(mut self, column: &str) -> Self {
        let prefix = if self.wheres.is_empty() { "" } else { "AND " };
        self.wheres.push(format!("{}{} IS NULL", prefix, column));
        self
    }

    /// Filter WHERE column IS NOT NULL
    pub fn where_not_null(mut self, column: &str) -> Self {
        let prefix = if self.wheres.is_empty() { "" } else { "AND " };
        self.wheres.push(format!("{}{} IS NOT NULL", prefix, column));
        self
    }

    /// Bangun klausa SQL WHERE dengan memperhitungkan Smart Soft Deletes
    fn build_where_clause(&self) -> String {
        let mut wheres = self.wheres.clone();
        if self.has_soft_deletes {
            if self.only_trashed {
                wheres.push("deleted_at IS NOT NULL".to_string());
            } else if !self.with_trashed {
                wheres.push("deleted_at IS NULL".to_string());
            }
        }

        if !wheres.is_empty() {
            format!(" WHERE {}", wheres.join(" AND "))
        } else {
            String::new()
        }
    }

    /// Bangun query SELECT lengkap
    fn build_select_sql(&self) -> String {
        let mut sql = format!("SELECT {} FROM {}{}", self.select, self.table, self.build_where_clause());
        if let Some(ref ord) = self.order_by {
            sql.push_str(&format!(" ORDER BY {}", ord));
        }
        if let Some(lim) = self.limit {
            sql.push_str(&format!(" LIMIT {}", lim));
        }
        if let Some(off) = self.offset {
            sql.push_str(&format!(" OFFSET {}", off));
        }
        self.tx.prepare_sql(&sql)
    }

    /// Ambil semua baris data sebagai Vec<serde_json::Value>
    pub async fn get(&self) -> Result<Vec<serde_json::Value>, sqlx::Error> {
        let sql = self.build_select_sql();
        let mut lock = self.tx.inner.lock().await;
        match &mut *lock {
            TxInner::Sqlx(opt) => {
                if let Some(tx) = opt.as_mut() {
                    let mut q = sqlx::query(&sql);
                    for b in &self.bind_values {
                        if let Ok(num) = b.parse::<i64>() {
                            q = q.bind(num);
                        } else if let Ok(flt) = b.parse::<f64>() {
                            q = q.bind(flt);
                        } else {
                            q = q.bind(b);
                        }
                    }
                    let rows = q.fetch_all(&mut **tx).await?;
                    Ok(rows.iter().map(any_row_to_json).collect())
                } else {
                    Err(sqlx::Error::Configuration("Transaksi sudah ditutup".into()))
                }
            }
            #[cfg(feature = "mssql")]
            TxInner::Mssql(opt) => {
                if let Some(conn) = opt.as_mut() {
                    let stream = conn.simple_query(&sql).await.map_err(|e| {
                        sqlx::Error::Configuration(format!("Error query MSSQL: {}", e).into())
                    })?;
                    let rows = stream.into_first_result().await.map_err(|e| {
                        sqlx::Error::Configuration(format!("Error row MSSQL: {}", e).into())
                    })?;
                    Ok(rows.iter().map(crate::database::mssql_adapter::row_to_json).collect())
                } else {
                    Err(sqlx::Error::Configuration("Transaksi sudah ditutup".into()))
                }
            }
        }
    }

    /// Ambil satu baris data pertama sebagai Option<serde_json::Value>
    pub async fn first(&self) -> Result<Option<serde_json::Value>, sqlx::Error> {
        let mut items = self.get().await?;
        if items.is_empty() {
            Ok(None)
        } else {
            Ok(Some(items.remove(0)))
        }
    }

    /// Hitung jumlah baris data yang cocok dengan kriteria filter
    pub async fn count(&self) -> Result<i64, sqlx::Error> {
        let sql = format!("SELECT COUNT(*) as count FROM {}{}", self.table, self.build_where_clause());
        let sql = self.tx.prepare_sql(&sql);
        let mut lock = self.tx.inner.lock().await;
        match &mut *lock {
            TxInner::Sqlx(opt) => {
                if let Some(tx) = opt.as_mut() {
                    let mut q = sqlx::query(&sql);
                    for b in &self.bind_values {
                        if let Ok(num) = b.parse::<i64>() {
                            q = q.bind(num);
                        } else if let Ok(flt) = b.parse::<f64>() {
                            q = q.bind(flt);
                        } else {
                            q = q.bind(b);
                        }
                    }
                    let row = q.fetch_one(&mut **tx).await?;
                    let count: i64 = row.try_get(0).unwrap_or(0);
                    Ok(count)
                } else {
                    Err(sqlx::Error::Configuration("Transaksi sudah ditutup".into()))
                }
            }
            #[cfg(feature = "mssql")]
            TxInner::Mssql(_) => {
                drop(lock);
                let rows = self.get().await?;
                Ok(rows.len() as i64)
            }
        }
    }

    /// Kurangkan nilai kolom secara atomik di database (Laravel: decrement).
    pub async fn decrement(&self, column: &str, amount: i64) -> Result<u64, sqlx::Error> {
        let where_clause = self.build_where_clause();
        let raw_sql = format!(
            "UPDATE {} SET {} = {} - ?{}",
            self.table, column, column, where_clause
        );
        let sql = self.tx.prepare_sql(&raw_sql);

        let mut lock = self.tx.inner.lock().await;
        match &mut *lock {
            TxInner::Sqlx(opt) => {
                if let Some(tx) = opt.as_mut() {
                    let mut q = sqlx::query(&sql).bind(amount);
                    for b in &self.bind_values {
                        if let Ok(num) = b.parse::<i64>() {
                            q = q.bind(num);
                        } else if let Ok(flt) = b.parse::<f64>() {
                            q = q.bind(flt);
                        } else {
                            q = q.bind(b);
                        }
                    }
                    let res = q.execute(&mut **tx).await?;
                    Ok(res.rows_affected())
                } else {
                    Err(sqlx::Error::Configuration("Transaksi sudah ditutup".into()))
                }
            }
            #[cfg(feature = "mssql")]
            TxInner::Mssql(_) => {
                let where_mssql = if !self.wheres.is_empty() {
                    let mut w = self.wheres.join(" ");
                    for val in &self.bind_values {
                        let safe_val = val.replace('\'', "''");
                        if let Some(idx) = w.find('?') {
                            w.replace_range(idx..idx + 1, &format!("'{}'", safe_val));
                        }
                    }
                    format!(" WHERE {}", w)
                } else {
                    String::new()
                };
                let sql = format!(
                    "UPDATE {} SET {} = {} - {}{}",
                    self.table, column, column, amount, where_mssql
                );
                drop(lock);
                self.tx.execute_raw(&sql).await
            }
        }
    }

    /// Tambahkan nilai kolom secara atomik di database (Laravel: increment).
    pub async fn increment(&self, column: &str, amount: i64) -> Result<u64, sqlx::Error> {
        let where_clause = self.build_where_clause();
        let raw_sql = format!(
            "UPDATE {} SET {} = {} + ?{}",
            self.table, column, column, where_clause
        );
        let sql = self.tx.prepare_sql(&raw_sql);

        let mut lock = self.tx.inner.lock().await;
        match &mut *lock {
            TxInner::Sqlx(opt) => {
                if let Some(tx) = opt.as_mut() {
                    let mut q = sqlx::query(&sql).bind(amount);
                    for b in &self.bind_values {
                        if let Ok(num) = b.parse::<i64>() {
                            q = q.bind(num);
                        } else if let Ok(flt) = b.parse::<f64>() {
                            q = q.bind(flt);
                        } else {
                            q = q.bind(b);
                        }
                    }
                    let res = q.execute(&mut **tx).await?;
                    Ok(res.rows_affected())
                } else {
                    Err(sqlx::Error::Configuration("Transaksi sudah ditutup".into()))
                }
            }
            #[cfg(feature = "mssql")]
            TxInner::Mssql(_) => {
                let where_mssql = if !self.wheres.is_empty() {
                    let mut w = self.wheres.join(" ");
                    for val in &self.bind_values {
                        let safe_val = val.replace('\'', "''");
                        if let Some(idx) = w.find('?') {
                            w.replace_range(idx..idx + 1, &format!("'{}'", safe_val));
                        }
                    }
                    format!(" WHERE {}", w)
                } else {
                    String::new()
                };
                let sql = format!(
                    "UPDATE {} SET {} = {} + {}{}",
                    self.table, column, column, amount, where_mssql
                );
                drop(lock);
                self.tx.execute_raw(&sql).await
            }
        }
    }

    /// Insert baris baru ke tabel (Laravel: insert).
    pub async fn insert<K, V>(&self, values: impl IntoIterator<Item = (K, V)>) -> Result<u64, sqlx::Error>
    where
        K: AsRef<str>,
        V: ToString,
    {
        let pairs: Vec<(String, String)> = values
            .into_iter()
            .map(|(k, v)| (k.as_ref().to_string(), v.to_string()))
            .collect();

        if pairs.is_empty() {
            return Ok(0);
        }

        let cols: Vec<String> = pairs.iter().map(|(k, _)| k.clone()).collect();
        let placeholders: Vec<&str> = pairs.iter().map(|_| "?").collect();

        let raw_sql = format!(
            "INSERT INTO {} ({}) VALUES ({})",
            self.table,
            cols.join(", "),
            placeholders.join(", ")
        );
        let sql = self.tx.prepare_sql(&raw_sql);

        let mut lock = self.tx.inner.lock().await;
        match &mut *lock {
            TxInner::Sqlx(opt) => {
                if let Some(tx) = opt.as_mut() {
                    let mut q = sqlx::query(&sql);
                    for (_, val) in &pairs {
                        if let Ok(num) = val.parse::<i64>() {
                            q = q.bind(num);
                        } else if let Ok(flt) = val.parse::<f64>() {
                            q = q.bind(flt);
                        } else {
                            q = q.bind(val);
                        }
                    }
                    let res = q.execute(&mut **tx).await?;
                    Ok(res.rows_affected())
                } else {
                    Err(sqlx::Error::Configuration("Transaksi sudah ditutup".into()))
                }
            }
            #[cfg(feature = "mssql")]
            TxInner::Mssql(_) => {
                let safe_vals: Vec<String> = pairs
                    .iter()
                    .map(|(_, v)| format!("'{}'", v.replace('\'', "''")))
                    .collect();
                let sql = format!(
                    "INSERT INTO {} ({}) VALUES ({})",
                    self.table,
                    cols.join(", "),
                    safe_vals.join(", ")
                );
                drop(lock);
                self.tx.execute_raw(&sql).await
            }
        }
    }

    /// Update baris pada tabel berdasarkan filter WHERE yang ditentukan.
    pub async fn update<K, V>(&self, values: impl IntoIterator<Item = (K, V)>) -> Result<u64, sqlx::Error>
    where
        K: AsRef<str>,
        V: ToString,
    {
        let pairs: Vec<(String, String)> = values
            .into_iter()
            .map(|(k, v)| (k.as_ref().to_string(), v.to_string()))
            .collect();

        if pairs.is_empty() {
            return Ok(0);
        }

        let set_clause: Vec<String> = pairs.iter().map(|(k, _)| format!("{} = ?", k)).collect();
        let where_clause = self.build_where_clause();

        let raw_sql = format!(
            "UPDATE {} SET {}{}",
            self.table,
            set_clause.join(", "),
            where_clause
        );
        let sql = self.tx.prepare_sql(&raw_sql);

        let mut lock = self.tx.inner.lock().await;
        match &mut *lock {
            TxInner::Sqlx(opt) => {
                if let Some(tx) = opt.as_mut() {
                    let mut q = sqlx::query(&sql);
                    for (_, val) in &pairs {
                        if let Ok(num) = val.parse::<i64>() {
                            q = q.bind(num);
                        } else if let Ok(flt) = val.parse::<f64>() {
                            q = q.bind(flt);
                        } else {
                            q = q.bind(val);
                        }
                    }
                    for b in &self.bind_values {
                        if let Ok(num) = b.parse::<i64>() {
                            q = q.bind(num);
                        } else if let Ok(flt) = b.parse::<f64>() {
                            q = q.bind(flt);
                        } else {
                            q = q.bind(b);
                        }
                    }
                    let res = q.execute(&mut **tx).await?;
                    Ok(res.rows_affected())
                } else {
                    Err(sqlx::Error::Configuration("Transaksi sudah ditutup".into()))
                }
            }
            #[cfg(feature = "mssql")]
            TxInner::Mssql(_) => {
                let safe_sets: Vec<String> = pairs
                    .iter()
                    .map(|(k, v)| format!("{} = '{}'", k, v.replace('\'', "''")))
                    .collect();
                let where_mssql = if !self.wheres.is_empty() {
                    let mut w = self.wheres.join(" ");
                    for val in &self.bind_values {
                        let safe_val = val.replace('\'', "''");
                        if let Some(idx) = w.find('?') {
                            w.replace_range(idx..idx + 1, &format!("'{}'", safe_val));
                        }
                    }
                    format!(" WHERE {}", w)
                } else {
                    String::new()
                };
                let sql = format!(
                    "UPDATE {} SET {}{}",
                    self.table,
                    safe_sets.join(", "),
                    where_mssql
                );
                drop(lock);
                self.tx.execute_raw(&sql).await
            }
        }
    }

    /// Hapus baris dari tabel. Jika `soft_deletes` aktif, set `deleted_at = CURRENT_TIMESTAMP`.
    /// Jika tidak aktif, lakukan penghapusan fisik permanen (`DELETE FROM`).
    pub async fn delete(&self) -> Result<u64, sqlx::Error> {
        if self.has_soft_deletes {
            let where_clause = self.build_where_clause();
            let raw_sql = format!("UPDATE {} SET deleted_at = CURRENT_TIMESTAMP{}", self.table, where_clause);
            let sql = self.tx.prepare_sql(&raw_sql);

            let mut lock = self.tx.inner.lock().await;
            match &mut *lock {
                TxInner::Sqlx(opt) => {
                    if let Some(tx) = opt.as_mut() {
                        let mut q = sqlx::query(&sql);
                        for b in &self.bind_values {
                            if let Ok(num) = b.parse::<i64>() {
                                q = q.bind(num);
                            } else if let Ok(flt) = b.parse::<f64>() {
                                q = q.bind(flt);
                            } else {
                                q = q.bind(b);
                            }
                        }
                        let res = q.execute(&mut **tx).await?;
                        Ok(res.rows_affected())
                    } else {
                        Err(sqlx::Error::Configuration("Transaksi sudah ditutup".into()))
                    }
                }
                #[cfg(feature = "mssql")]
                TxInner::Mssql(_) => {
                    drop(lock);
                    self.tx.execute_raw(&sql).await
                }
            }
        } else {
            self.force_delete().await
        }
    }

    /// Pulihkan data yang terhapus sementara (Soft Delete Restore: set `deleted_at = NULL`).
    pub async fn restore(&self) -> Result<u64, sqlx::Error> {
        let mut wheres = self.wheres.clone();
        wheres.push("deleted_at IS NOT NULL".to_string());
        let where_clause = format!(" WHERE {}", wheres.join(" AND "));

        let raw_sql = format!("UPDATE {} SET deleted_at = NULL{}", self.table, where_clause);
        let sql = self.tx.prepare_sql(&raw_sql);

        let mut lock = self.tx.inner.lock().await;
        match &mut *lock {
            TxInner::Sqlx(opt) => {
                if let Some(tx) = opt.as_mut() {
                    let mut q = sqlx::query(&sql);
                    for b in &self.bind_values {
                        if let Ok(num) = b.parse::<i64>() {
                            q = q.bind(num);
                        } else if let Ok(flt) = b.parse::<f64>() {
                            q = q.bind(flt);
                        } else {
                            q = q.bind(b);
                        }
                    }
                    let res = q.execute(&mut **tx).await?;
                    Ok(res.rows_affected())
                } else {
                    Err(sqlx::Error::Configuration("Transaksi sudah ditutup".into()))
                }
            }
            #[cfg(feature = "mssql")]
            TxInner::Mssql(_) => {
                drop(lock);
                self.tx.execute_raw(&sql).await
            }
        }
    }

    /// Hapus fisik permanen dari database (Hard Delete).
    pub async fn force_delete(&self) -> Result<u64, sqlx::Error> {
        let where_clause = if !self.wheres.is_empty() {
            format!(" WHERE {}", self.wheres.join(" AND "))
        } else {
            String::new()
        };

        let raw_sql = format!("DELETE FROM {}{}", self.table, where_clause);
        let sql = self.tx.prepare_sql(&raw_sql);

        let mut lock = self.tx.inner.lock().await;
        match &mut *lock {
            TxInner::Sqlx(opt) => {
                if let Some(tx) = opt.as_mut() {
                    let mut q = sqlx::query(&sql);
                    for b in &self.bind_values {
                        if let Ok(num) = b.parse::<i64>() {
                            q = q.bind(num);
                        } else if let Ok(flt) = b.parse::<f64>() {
                            q = q.bind(flt);
                        } else {
                            q = q.bind(b);
                        }
                    }
                    let res = q.execute(&mut **tx).await?;
                    Ok(res.rows_affected())
                } else {
                    Err(sqlx::Error::Configuration("Transaksi sudah ditutup".into()))
                }
            }
            #[cfg(feature = "mssql")]
            TxInner::Mssql(_) => {
                let where_mssql = if !self.wheres.is_empty() {
                    let mut w = self.wheres.join(" ");
                    for val in &self.bind_values {
                        let safe_val = val.replace('\'', "''");
                        if let Some(idx) = w.find('?') {
                            w.replace_range(idx..idx + 1, &format!("'{}'", safe_val));
                        }
                    }
                    format!(" WHERE {}", w)
                } else {
                    String::new()
                };
                let sql = format!("DELETE FROM {}{}", self.table, where_mssql);
                drop(lock);
                self.tx.execute_raw(&sql).await
            }
        }
    }
}

/// Sumber koneksi database: dapat berupa Pool koneksi atau Transaksi aktif.
#[derive(Clone)]
pub enum DbSource<'a> {
    Pool(&'a DatabasePool),
    Tx(&'a Transaction),
}

impl<'a> From<&'a DatabasePool> for DbSource<'a> {
    fn from(pool: &'a DatabasePool) -> Self {
        DbSource::Pool(pool)
    }
}

impl<'a> From<&'a Transaction> for DbSource<'a> {
    fn from(tx: &'a Transaction) -> Self {
        DbSource::Tx(tx)
    }
}

impl<'a> DbSource<'a> {
    pub fn kind(&self) -> DatabaseKind {
        match self {
            DbSource::Pool(p) => p.kind,
            DbSource::Tx(t) => t.kind(),
        }
    }

    pub fn prepare_sql(&self, sql: &str) -> String {
        match self {
            DbSource::Pool(p) => p.prepare_sql(sql),
            DbSource::Tx(t) => t.prepare_sql(sql),
        }
    }
}
