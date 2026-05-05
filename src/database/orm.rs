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
            order_by: None,
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
        self.wheres.push(format!("{}{} {} ?", prefix, column, operator));
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
        self.wheres.push(format!("{}{} IS NOT NULL", prefix, column));
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
    pub async fn get(self) -> Result<Vec<T>, sqlx::Error> 
    where T: crate::database::model::Model
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

        if let Some(ref order) = self.order_by {
            sql.push_str(&format!(" ORDER BY {}", order));
        }

        if let Some(limit) = self.limit {
            sql.push_str(&format!(" LIMIT {}", limit));
        }

        sql
    }
}
