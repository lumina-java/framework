use async_trait::async_trait;
use serde::{Serialize, Deserialize};
use sqlx::FromRow;
use crate::database::{connection::DatabasePool, model::Model};

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct UploadImage {
    pub id: i64,
    #[sqlx(rename = "nama")]
    pub name: String,
    pub file: String,
}

#[async_trait]
impl Model for UploadImage {
    const TABLE: &'static str = "upload_images";

    async fn find(pool: &DatabasePool, id: i64) -> Result<Self, sqlx::Error> {
        sqlx::query_as::<_, Self>(
            "SELECT id, nama, file FROM upload_images WHERE id = ?"
        )
        .bind(id)
        .fetch_one(&pool.pool)
        .await
    }

    async fn all(pool: &DatabasePool) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>(
            "SELECT id, nama, file FROM upload_images ORDER BY id DESC"
        )
        .fetch_all(&pool.pool)
        .await
    }

    async fn save(&self, pool: &DatabasePool) -> Result<i64, sqlx::Error> {
        let result = sqlx::query(
            "INSERT INTO upload_images (nama, file) VALUES (?, ?)"
        )
        .bind(&self.name)
        .bind(&self.file)
        .execute(&pool.pool)
        .await?;

        Ok(result.last_insert_id().unwrap_or(0))
    }

    async fn delete(pool: &DatabasePool, id: i64) -> Result<bool, sqlx::Error> {
        sqlx::query("DELETE FROM upload_images WHERE id = ?")
            .bind(id)
            .execute(&pool.pool)
            .await?;

        Ok(true)
    }
}
