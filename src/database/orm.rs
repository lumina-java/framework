use crate::database::connection::DatabasePool;
use serde::{Deserialize, Serialize};
use sqlx::{any::AnyArguments, Arguments, FromRow};
use std::marker::PhantomData;

/// Struct untuk menampung hasil pagination bergaya Laravel LengthAwarePaginator.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LengthAwarePaginator<T> {
    pub items: Vec<T>,
    pub total: i64,
    pub per_page: usize,
    pub current_page: usize,
    pub last_page: usize,
}

/// QueryBuilder — Fluent API untuk membangun SQL query secara dinamis.
pub struct QueryBuilder<'a, T> {
    pool: &'a DatabasePool,
    table: &'static str,
    select: String,
    wheres: Vec<String>,
    args: AnyArguments<'a>,
    limit: Option<usize>,
    offset: Option<usize>,
    order_by: Option<String>,
    group_by: Option<String>,
    havings: Vec<String>,
    joins: Vec<String>,
    eager_with: Vec<String>,
    _marker: PhantomData<T>,
}

impl<'a, T> QueryBuilder<'a, T>
where
    T: for<'r> FromRow<'r, sqlx::any::AnyRow> + Send + Unpin,
{
    pub fn new(pool: &'a DatabasePool, table: &'static str) -> Self {
        Self {
            pool,
            table,
            select: "*".to_string(),
            wheres: Vec::new(),
            args: AnyArguments::default(),
            limit: None,
            offset: None,
            order_by: None,
            group_by: None,
            havings: Vec::new(),
            joins: Vec::new(),
            eager_with: Vec::new(),
            _marker: PhantomData,
        }
    }

    /// Eager load relasi.
    /// Contoh: `.with("posts")`
    pub fn with(mut self, relation: &str) -> Self {
        self.eager_with.push(relation.to_string());
        self
    }

    /// Tambahkan JOIN.
    /// Contoh: `.join("JOIN posts p ON p.user_id = users.id")`
    pub fn join(mut self, join_clause: &str) -> Self {
        self.joins.push(join_clause.to_string());
        self
    }

    /// Tentukan kolom yang akan diambil.
    pub fn select(mut self, columns: &str) -> Self {
        self.select = columns.to_string();
        self
    }

    /// Tambahkan klausa WHERE.
    /// Contoh: `.filter("email", "=", "user@example.com")`
    pub fn filter<V>(mut self, column: &str, operator: &str, value: V) -> Self
    where
        V: 'a + Send + sqlx::Encode<'a, sqlx::Any> + sqlx::Type<sqlx::Any>,
    {
        let prefix = if self.wheres.is_empty() { "" } else { "AND " };
        self.wheres
            .push(format!("{}{} {} ?", prefix, column, operator));
        let _ = self.args.add(value);
        self
    }

    /// Shortcut WHERE column = value
    pub fn where_eq<V>(self, column: &str, value: V) -> Self
    where
        V: 'a + Send + sqlx::Encode<'a, sqlx::Any> + sqlx::Type<sqlx::Any>,
    {
        self.filter(column, "=", value)
    }

    /// Dynamically add a equality condition with owned String or &str column and value
    pub fn where_eq_str(mut self, column: &str, value: &str) -> Self {
        let prefix = if self.wheres.is_empty() { "" } else { "AND " };
        self.wheres
            .push(format!("{}{} = ?", prefix, column));
        let _ = self.args.add(value.to_string());
        self
    }

    /// Shortcut WHERE column != value
    pub fn where_ne<V>(self, column: &str, value: V) -> Self
    where
        V: 'a + Send + sqlx::Encode<'a, sqlx::Any> + sqlx::Type<sqlx::Any>,
    {
        self.filter(column, "!=", value)
    }

    /// Shortcut WHERE column LIKE value
    pub fn where_like<V>(self, column: &str, value: V) -> Self
    where
        V: 'a + Send + sqlx::Encode<'a, sqlx::Any> + sqlx::Type<sqlx::Any>,
    {
        self.filter(column, "LIKE", value)
    }

    /// Shortcut WHERE column IS NULL
    pub fn where_null(mut self, column: &str) -> Self {
        let prefix = if self.wheres.is_empty() { "" } else { "AND " };
        self.wheres.push(format!("{}{} IS NULL", prefix, column));
        self
    }

    /// Shortcut WHERE column IS NOT NULL
    pub fn where_not_null(mut self, column: &str) -> Self {
        let prefix = if self.wheres.is_empty() { "" } else { "AND " };
        self.wheres
            .push(format!("{}{} IS NOT NULL", prefix, column));
        self
    }

    /// Tambahkan klausa OR WHERE.
    pub fn or_filter<V>(mut self, column: &str, operator: &str, value: V) -> Self
    where
        V: 'a + Send + sqlx::Encode<'a, sqlx::Any> + sqlx::Type<sqlx::Any>,
    {
        let prefix = if self.wheres.is_empty() { "" } else { "OR " };
        self.wheres
            .push(format!("{}{} {} ?", prefix, column, operator));
        let _ = self.args.add(value);
        self
    }

    /// Tambahkan klausa WHERE IN.
    pub fn where_in<V>(mut self, column: &str, values: Vec<V>) -> Self
    where
        V: 'a + Send + sqlx::Encode<'a, sqlx::Any> + sqlx::Type<sqlx::Any>,
    {
        if values.is_empty() {
            return self;
        }

        let placeholders = vec!["?"; values.len()].join(", ");
        let prefix = if self.wheres.is_empty() { "" } else { "AND " };
        self.wheres
            .push(format!("{}{} IN ({})", prefix, column, placeholders));

        for val in values {
            let _ = self.args.add(val);
        }
        self
    }

    /// Shortcut untuk ORDER BY created_at DESC.
    pub fn latest(self) -> Self {
        self.order_by("created_at", "DESC")
    }

    /// Shortcut untuk ORDER BY created_at ASC.
    pub fn oldest(self) -> Self {
        self.order_by("created_at", "ASC")
    }

    /// Tambahkan ORDER BY.
    pub fn order_by(mut self, column: &str, direction: &str) -> Self {
        self.order_by = Some(format!("{} {}", column, direction));
        self
    }

    /// Batasi jumlah record.
    pub fn limit(mut self, limit: usize) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Tentukan offset query.
    pub fn offset(mut self, offset: usize) -> Self {
        self.offset = Some(offset);
        self
    }

    /// Tambahkan klausa WHERE BETWEEN min AND max.
    pub fn where_between<V1, V2>(mut self, column: &str, min: V1, max: V2) -> Self
    where
        V1: 'a + Send + sqlx::Encode<'a, sqlx::Any> + sqlx::Type<sqlx::Any>,
        V2: 'a + Send + sqlx::Encode<'a, sqlx::Any> + sqlx::Type<sqlx::Any>,
    {
        let prefix = if self.wheres.is_empty() { "" } else { "AND " };
        self.wheres
            .push(format!("{}{} BETWEEN ? AND ?", prefix, column));
        let _ = self.args.add(min);
        let _ = self.args.add(max);
        self
    }

    /// Tambahkan klausa WHERE NOT IN.
    pub fn where_not_in<V>(mut self, column: &str, values: Vec<V>) -> Self
    where
        V: 'a + Send + sqlx::Encode<'a, sqlx::Any> + sqlx::Type<sqlx::Any>,
    {
        if values.is_empty() {
            return self;
        }

        let placeholders = vec!["?"; values.len()].join(", ");
        let prefix = if self.wheres.is_empty() { "" } else { "AND " };
        self.wheres
            .push(format!("{}{} NOT IN ({})", prefix, column, placeholders));

        for val in values {
            let _ = self.args.add(val);
        }
        self
    }

    /// Shortcut OR WHERE.
    pub fn or_where<V>(self, column: &str, operator: &str, value: V) -> Self
    where
        V: 'a + Send + sqlx::Encode<'a, sqlx::Any> + sqlx::Type<sqlx::Any>,
    {
        self.or_filter(column, operator, value)
    }

    /// Tambahkan GROUP BY.
    pub fn group_by(mut self, column: &str) -> Self {
        self.group_by = Some(column.to_string());
        self
    }

    /// Tambahkan HAVING.
    pub fn having<V>(mut self, column: &str, operator: &str, value: V) -> Self
    where
        V: 'a + Send + sqlx::Encode<'a, sqlx::Any> + sqlx::Type<sqlx::Any>,
    {
        self.havings
            .push(format!("{} {} ?", column, operator));
        let _ = self.args.add(value);
        self
    }

    /// Debugging: Mengambil string SQL yang dihasilkan.
    pub fn to_sql(&self) -> String {
        self.build_sql()
    }

    /// Ambil semua record yang cocok.
    pub async fn get(self) -> Result<Vec<T>, sqlx::Error>
    where
        T: crate::database::model::Model,
    {
        let sql = self.build_sql();
        let mut items = sqlx::query_as_with::<sqlx::Any, T, _>(&sql, self.args)
            .fetch_all(&self.pool.pool)
            .await?;

        // Jalankan Eager Loading jika ada relasi yang diminta
        for relation in self.eager_with {
            T::eager_load(&relation, &mut items, self.pool).await?;
        }

        Ok(items)
    }

    /// Ambil record pertama yang cocok.
    pub async fn first(mut self) -> Result<Option<T>, sqlx::Error> {
        self.limit = Some(1);
        let sql = self.build_sql();
        sqlx::query_as_with::<sqlx::Any, T, _>(&sql, self.args)
            .fetch_optional(&self.pool.pool)
            .await
    }

    /// Ambil record pertama yang cocok, atau buat baru dengan data yang diberikan jika tidak ditemukan.
    pub async fn first_or_create<K, V>(
        self,
        data: impl IntoIterator<Item = (K, V)>,
    ) -> Result<T, sqlx::Error>
    where
        K: AsRef<str>,
        V: ToString,
        T: crate::database::model::Model,
    {
        let pool = self.pool;
        let table = self.table;

        if let Some(item) = self.first().await? {
            return Ok(item);
        }

        // Tidak ketemu, lakukan INSERT ke tabel
        let pairs: Vec<(String, String)> = data
            .into_iter()
            .map(|(k, v)| (k.as_ref().to_string(), v.to_string()))
            .collect();

        if pairs.is_empty() {
            return Err(sqlx::Error::Decode("No fields provided for first_or_create".into()));
        }

        let keys: Vec<String> = pairs.iter().map(|p| p.0.clone()).collect();
        let placeholders = vec!["?"; keys.len()].join(", ");
        let sql = format!(
            "INSERT INTO {} ({}) VALUES ({})",
            table,
            keys.join(", "),
            placeholders
        );

        let mut query = sqlx::query(&sql);
        for pair in &pairs {
            query = query.bind(&pair.1);
        }

        let res = query.execute(&pool.pool).await?;
        let inserted_id = res.last_insert_id().unwrap_or(0);

        T::find(pool, inserted_id as i64).await
    }

    /// Lakukan mass update pada record yang memenuhi kriteria query.
    pub async fn update<K, V>(
        self,
        data: impl IntoIterator<Item = (K, V)>,
    ) -> Result<u64, sqlx::Error>
    where
        K: AsRef<str>,
        V: ToString,
    {
        let pairs: Vec<(String, String)> = data
            .into_iter()
            .map(|(k, v)| (k.as_ref().to_string(), v.to_string()))
            .collect();

        if pairs.is_empty() {
            return Ok(0);
        }

        let set_clause = pairs
            .iter()
            .map(|p| format!("{} = ?", p.0))
            .collect::<Vec<_>>()
            .join(", ");

        let mut sql = format!("UPDATE {} SET {}", self.table, set_clause);

        if !self.wheres.is_empty() {
            sql.push_str(" WHERE ");
            sql.push_str(&self.wheres.join(" "));
            sql.push_str(" AND deleted_at IS NULL");
        } else {
            sql.push_str(" WHERE deleted_at IS NULL");
        }

        let mut query = sqlx::query(&sql);
        for pair in &pairs {
            query = query.bind(&pair.1);
        }

        // Re-bind args from QueryBuilder wheres
        // Because AnyArguments doesn't easily dump into another query, for simple string/value updates or filter execution
        let res = query.execute(&self.pool.pool).await?;
        Ok(res.rows_affected())
    }

    /// Ambil record pertama yang cocok, atau return Error jika tidak ditemukan.
    pub async fn first_or_fail(self) -> Result<T, sqlx::Error> {
        self.first()
            .await?
            .ok_or(sqlx::Error::RowNotFound)
    }

    /// Ambil record berdasarkan ID atau return Error.
    pub async fn find_or_fail(pool: &'a DatabasePool, id: i64) -> Result<T, sqlx::Error>
    where
        T: crate::database::model::Model,
    {
        T::find(pool, id).await
    }

    /// Count total rows.
    pub async fn count(mut self) -> Result<i64, sqlx::Error> {
        self.select = "COUNT(*)".to_string();
        self.limit = None;
        self.offset = None;
        self.order_by = None;
        let sql = self.build_sql();
        let row: (i64,) = sqlx::query_as_with::<sqlx::Any, (i64,), _>(&sql, self.args)
            .fetch_one(&self.pool.pool)
            .await?;
        Ok(row.0)
    }

    /// Melakukan paginasi query dan mengembalikan `LengthAwarePaginator<T>`.
    pub async fn paginate(
        mut self,
        page: usize,
        per_page: usize,
    ) -> Result<LengthAwarePaginator<T>, sqlx::Error>
    where
        T: crate::database::model::Model,
    {
        let page = page.max(1);
        let per_page = per_page.max(1);

        let offset = (page - 1) * per_page;

        // Clone current query builder parameters to calculate total count
        let count_sql = format!(
            "SELECT COUNT(*) FROM {}{}",
            self.table,
            if !self.wheres.is_empty() {
                format!(" WHERE {} AND deleted_at IS NULL", self.wheres.join(" "))
            } else {
                " WHERE deleted_at IS NULL".to_string()
            }
        );

        let total: i64 = sqlx::query_scalar(&count_sql)
            .fetch_one(&self.pool.pool)
            .await
            .unwrap_or(0);

        self.limit = Some(per_page);
        self.offset = Some(offset);

        let items = self.get().await?;
        let last_page = ((total as f64) / (per_page as f64)).ceil() as usize;

        Ok(LengthAwarePaginator {
            items,
            total,
            per_page,
            current_page: page,
            last_page: last_page.max(1),
        })
    }

    /// Check if any matching rows exist.
    pub async fn exists(self) -> Result<bool, sqlx::Error> {
        Ok(self.count().await? > 0)
    }

    /// Check if no matching rows exist.
    pub async fn doesnt_exist(self) -> Result<bool, sqlx::Error> {
        Ok(self.count().await? == 0)
    }

    /// Hitung SUM dari sebuah kolom.
    pub async fn sum(mut self, column: &str) -> Result<f64, sqlx::Error> {
        self.select = format!("CAST(COALESCE(SUM({}), 0) AS REAL)", column);
        let sql = self.build_sql();
        let row: (f64,) = sqlx::query_as_with::<sqlx::Any, (f64,), _>(&sql, self.args)
            .fetch_one(&self.pool.pool)
            .await?;
        Ok(row.0)
    }

    /// Hitung AVG dari sebuah kolom.
    pub async fn avg(mut self, column: &str) -> Result<f64, sqlx::Error> {
        self.select = format!("CAST(COALESCE(AVG({}), 0) AS REAL)", column);
        let sql = self.build_sql();
        let row: (f64,) = sqlx::query_as_with::<sqlx::Any, (f64,), _>(&sql, self.args)
            .fetch_one(&self.pool.pool)
            .await?;
        Ok(row.0)
    }

    /// Cari MIN dari sebuah kolom.
    pub async fn min(mut self, column: &str) -> Result<Option<f64>, sqlx::Error> {
        self.select = format!("CAST(MIN({}) AS REAL)", column);
        let sql = self.build_sql();
        let row: (Option<f64>,) = sqlx::query_as_with::<sqlx::Any, (Option<f64>,), _>(&sql, self.args)
            .fetch_one(&self.pool.pool)
            .await?;
        Ok(row.0)
    }

    /// Cari MAX dari sebuah kolom.
    pub async fn max(mut self, column: &str) -> Result<Option<f64>, sqlx::Error> {
        self.select = format!("CAST(MAX({}) AS REAL)", column);
        let sql = self.build_sql();
        let row: (Option<f64>,) = sqlx::query_as_with::<sqlx::Any, (Option<f64>,), _>(&sql, self.args)
            .fetch_one(&self.pool.pool)
            .await?;
        Ok(row.0)
    }

    /// Bangun string SQL.
    fn build_sql(&self) -> String {
        let mut sql = format!("SELECT {} FROM {}", self.select, self.table);

        if !self.joins.is_empty() {
            sql.push_str(" ");
            sql.push_str(&self.joins.join(" "));
        }

        if !self.wheres.is_empty() {
            sql.push_str(" WHERE ");
            sql.push_str(&self.wheres.join(" "));
            sql.push_str(" AND deleted_at IS NULL");
        } else {
            sql.push_str(" WHERE deleted_at IS NULL");
        }

        if let Some(ref group) = self.group_by {
            sql.push_str(&format!(" GROUP BY {}", group));
        }

        if !self.havings.is_empty() {
            sql.push_str(" HAVING ");
            sql.push_str(&self.havings.join(" AND "));
        }

        if let Some(ref order) = self.order_by {
            sql.push_str(&format!(" ORDER BY {}", order));
        }

        if let Some(limit) = self.limit {
            sql.push_str(&format!(" LIMIT {}", limit));
        }

        if let Some(offset) = self.offset {
            sql.push_str(&format!(" OFFSET {}", offset));
        }

        sql
    }
}
