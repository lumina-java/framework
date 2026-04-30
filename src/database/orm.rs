use sqlx::{Arguments, any::AnyArguments, FromRow};
use std::marker::PhantomData;
use crate::database::connection::DatabasePool;

/// QueryBuilder — Fluent API untuk membangun SQL query secara dinamis.
pub struct QueryBuilder<'a, T> {
    pool: &'a DatabasePool,
    table: &'static str,
    select: String,
    wheres: Vec<String>,
    args: AnyArguments<'a>,
    limit: Option<usize>,
    order_by: Option<String>,
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
            order_by: None,
            _marker: PhantomData,
        }
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
        self.wheres.push(format!("{}{} {} ?", prefix, column, operator));
        let _ = self.args.add(value);
        self
    }

    /// Tambahkan klausa OR WHERE.
    pub fn or_filter<V>(mut self, column: &str, operator: &str, value: V) -> Self
    where
        V: 'a + Send + sqlx::Encode<'a, sqlx::Any> + sqlx::Type<sqlx::Any>,
    {
        let prefix = if self.wheres.is_empty() { "" } else { "OR " };
        self.wheres.push(format!("{}{} {} ?", prefix, column, operator));
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
        self.wheres.push(format!("{}{} IN ({})", prefix, column, placeholders));
        
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

    /// Debugging: Mengambil string SQL yang dihasilkan.
    pub fn to_sql(&self) -> String {
        self.build_sql()
    }

    /// Ambil semua record yang cocok.
    pub async fn get(self) -> Result<Vec<T>, sqlx::Error> {
        let sql = self.build_sql();
        sqlx::query_as_with::<sqlx::Any, T, _>(&sql, self.args)
            .fetch_all(&self.pool.pool)
            .await
    }

    /// Ambil record pertama yang cocok.
    pub async fn first(mut self) -> Result<T, sqlx::Error> {
        self.limit = Some(1);
        let sql = self.build_sql();
        sqlx::query_as_with::<sqlx::Any, T, _>(&sql, self.args)
            .fetch_one(&self.pool.pool)
            .await
    }

    /// Bangun string SQL.
    fn build_sql(&self) -> String {
        let mut sql = format!("SELECT {} FROM {}", self.select, self.table);

        if !self.wheres.is_empty() {
            sql.push_str(" WHERE ");
            sql.push_str(&self.wheres.join(" "));
            sql.push_str(" AND deleted_at IS NULL");
        } else {
            sql.push_str(" WHERE deleted_at IS NULL");
        }

        if let Some(ref order) = self.order_by {
            sql.push_str(&format!(" ORDER BY {}", order));
        }

        if let Some(limit) = self.limit {
            sql.push_str(&format!(" LIMIT {}", limit));
        }

        sql
    }
}
