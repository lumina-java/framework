use async_trait::async_trait;
use serde::{Serialize, Deserialize};
use sqlx::FromRow;
use crate::database::{connection::DatabasePool, model::Model};

/// Model Product — merepresentasikan satu baris dari tabel `products`.
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Product {
    pub id:    i64,
    pub name:  String,
    pub price: f64,
}

#[async_trait]
impl Model for Product {
    const TABLE: &'static str = "products";

    async fn find(pool: &DatabasePool, id: i64) -> Result<Self, sqlx::Error> {
        sqlx::query_as::<_, Product>(
            "SELECT id, name, price FROM products WHERE id = ? AND deleted_at IS NULL"
        )
        .bind(id)
        .fetch_one(&pool.pool)
        .await
    }

    async fn all(pool: &DatabasePool) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Product>(
            "SELECT id, name, price FROM products WHERE deleted_at IS NULL ORDER BY id ASC"
        )
        .fetch_all(&pool.pool)
        .await
    }

    async fn save(&self, pool: &DatabasePool) -> Result<i64, sqlx::Error> {
        let result = sqlx::query(
            "INSERT INTO products (name, price) VALUES (?, ?)"
        )
        .bind(&self.name)
        .bind(self.price)
        .execute(&pool.pool)
        .await?;

        Ok(result.last_insert_id().unwrap_or(0))
    }

    async fn delete(pool: &DatabasePool, id: i64) -> Result<bool, sqlx::Error> {
        sqlx::query(
            "UPDATE products SET deleted_at = CURRENT_TIMESTAMP WHERE id = ?"
        )
        .bind(id)
        .execute(&pool.pool)
        .await?;

        Ok(true)
    }
}
