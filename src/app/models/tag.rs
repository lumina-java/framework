use async_trait::async_trait;
use serde::{Serialize, Deserialize};
use sqlx::FromRow;
use crate::database::{connection::DatabasePool, model::Model};
use crate::app::models::post::Post;

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Tag {
    pub id: i64,
    pub name: String,
}

#[async_trait]
impl Model for Tag {
    const TABLE: &'static str = "tags";

    async fn find(pool: &DatabasePool, id: i64) -> Result<Self, sqlx::Error> {
        sqlx::query_as::<_, Tag>("SELECT id, name FROM tags WHERE id = ? AND deleted_at IS NULL")
            .bind(id)
            .fetch_one(&pool.pool)
            .await
    }

    async fn all(pool: &DatabasePool) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Tag>("SELECT id, name FROM tags WHERE deleted_at IS NULL")
            .fetch_all(&pool.pool)
            .await
    }

    async fn save(&self, pool: &DatabasePool) -> Result<i64, sqlx::Error> {
        let result = sqlx::query("INSERT INTO tags (name) VALUES (?)")
            .bind(&self.name)
            .execute(&pool.pool)
            .await?;
        Ok(result.last_insert_id().unwrap_or(0))
    }

    async fn delete(pool: &DatabasePool, id: i64) -> Result<bool, sqlx::Error> {
        sqlx::query("UPDATE tags SET deleted_at = CURRENT_TIMESTAMP WHERE id = ?")
            .bind(id)
            .execute(&pool.pool)
            .await?;
        Ok(true)
    }

    async fn eager_load(_relation: &str, _items: &mut [Self], _pool: &DatabasePool) -> Result<(), sqlx::Error> {
        Ok(())
    }
}

impl Tag {
    pub async fn posts(&self, pool: &DatabasePool) -> Result<Vec<Post>, sqlx::Error> {
        Self::belongs_to_many::<Post>(pool, "post_tag", "tag_id", "post_id", self.id).await
    }
}
