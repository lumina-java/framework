use async_trait::async_trait;
use serde::{Serialize, Deserialize};
use sqlx::FromRow;
use crate::database::{connection::DatabasePool, model::Model};
use crate::app::models::permission::Permission;

#[derive(Debug, Serialize, Deserialize, FromRow, Clone, Default)]
pub struct Role {
    pub id:   i64,
    pub name: String,
    pub slug: String,
    #[sqlx(skip)]
    pub permissions: Option<Vec<Permission>>,
}

#[async_trait]
impl Model for Role {
    const TABLE: &'static str = "roles";

    async fn find(pool: &DatabasePool, id: i64) -> Result<Self, sqlx::Error> {
        sqlx::query_as::<_, Role>(
            "SELECT id, name, slug FROM roles WHERE id = ? AND deleted_at IS NULL"
        )
        .bind(id)
        .fetch_one(&pool.pool)
        .await
    }

    async fn all(pool: &DatabasePool) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Role>(
            "SELECT id, name, slug FROM roles WHERE deleted_at IS NULL ORDER BY name ASC"
        )
        .fetch_all(&pool.pool)
        .await
    }

    async fn save(&self, pool: &DatabasePool) -> Result<i64, sqlx::Error> {
        let result = sqlx::query(
            "INSERT INTO roles (name, slug) VALUES (?, ?)"
        )
        .bind(&self.name)
        .bind(&self.slug)
        .execute(&pool.pool)
        .await?;

        Ok(result.last_insert_id().unwrap_or(0))
    }

    async fn delete(pool: &DatabasePool, id: i64) -> Result<bool, sqlx::Error> {
        sqlx::query("UPDATE roles SET deleted_at = CURRENT_TIMESTAMP WHERE id = ?")
            .bind(id)
            .execute(&pool.pool)
            .await?;
        Ok(true)
    }
}

impl Role {
    pub async fn find_by_slug(pool: &DatabasePool, slug: &str) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as::<_, Role>(
            "SELECT id, name, slug FROM roles WHERE slug = ? AND deleted_at IS NULL"
        )
        .bind(slug)
        .fetch_optional(&pool.pool)
        .await
    }

    pub async fn permissions(&self, pool: &DatabasePool) -> Result<Vec<Permission>, sqlx::Error> {
        sqlx::query_as::<_, Permission>(
            "SELECT p.id, p.name, p.slug 
             FROM permissions p
             JOIN role_permission rp ON p.id = rp.permission_id
             WHERE rp.role_id = ? AND p.deleted_at IS NULL"
        )
        .bind(self.id)
        .fetch_all(&pool.pool)
        .await
    }
}
