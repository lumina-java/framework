use async_trait::async_trait;
use serde::{Serialize, Deserialize};
use sqlx::FromRow;
use crate::database::{connection::DatabasePool, model::Model};

#[derive(Debug, Serialize, Deserialize, FromRow, Clone, Default)]
pub struct Permission {
    pub id:   i64,
    pub name: String,
    pub slug: String,
}

#[async_trait]
impl Model for Permission {
    const TABLE: &'static str = "permissions";

    async fn find(pool: &DatabasePool, id: i64) -> Result<Self, sqlx::Error> {
        sqlx::query_as::<_, Permission>(
            "SELECT id, name, slug FROM permissions WHERE id = ? AND deleted_at IS NULL"
        )
        .bind(id)
        .fetch_one(&pool.pool)
        .await
    }

    async fn all(pool: &DatabasePool) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Permission>(
            "SELECT id, name, slug FROM permissions WHERE deleted_at IS NULL ORDER BY name ASC"
        )
        .fetch_all(&pool.pool)
        .await
    }

    async fn save(&self, pool: &DatabasePool) -> Result<i64, sqlx::Error> {
        let result = sqlx::query(
            "INSERT INTO permissions (name, slug) VALUES (?, ?)"
        )
        .bind(&self.name)
        .bind(&self.slug)
        .execute(&pool.pool)
        .await?;

        Ok(result.last_insert_id().unwrap_or(0))
    }

    async fn delete(pool: &DatabasePool, id: i64) -> Result<bool, sqlx::Error> {
        sqlx::query("UPDATE permissions SET deleted_at = CURRENT_TIMESTAMP WHERE id = ?")
            .bind(id)
            .execute(&pool.pool)
            .await?;
        Ok(true)
    }
}

impl Permission {
    pub async fn find_by_slug(pool: &DatabasePool, slug: &str) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as::<_, Permission>(
            "SELECT id, name, slug FROM permissions WHERE slug = ? AND deleted_at IS NULL"
        )
        .bind(slug)
        .fetch_optional(&pool.pool)
        .await
    }
}
