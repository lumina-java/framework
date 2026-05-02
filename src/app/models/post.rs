use async_trait::async_trait;
use serde::{Serialize, Deserialize};
use sqlx::FromRow;
use crate::database::{connection::DatabasePool, model::Model};
use crate::app::models::user::User;

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Post {
    pub id: i64,
    pub user_id: i64,
    pub title: String,
    pub content: String,
    #[sqlx(skip)]
    pub author: Option<User>,
}

#[async_trait]
impl Model for Post {
    const TABLE: &'static str = "posts";

    async fn find(pool: &DatabasePool, id: i64) -> Result<Self, sqlx::Error> {
        sqlx::query_as::<_, Post>(
            "SELECT id, user_id, title, content FROM posts WHERE id = ? AND deleted_at IS NULL"
        )
        .bind(id)
        .fetch_one(&pool.pool)
        .await
    }

    async fn all(pool: &DatabasePool) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Post>(
            "SELECT id, user_id, title, content FROM posts WHERE deleted_at IS NULL"
        )
        .fetch_all(&pool.pool)
        .await
    }

    async fn save(&self, pool: &DatabasePool) -> Result<i64, sqlx::Error> {
        let result = sqlx::query(
            "INSERT INTO posts (user_id, title, content) VALUES (?, ?, ?)"
        )
        .bind(self.user_id)
        .bind(&self.title)
        .bind(&self.content)
        .execute(&pool.pool)
        .await?;
        
        Ok(result.last_insert_id().unwrap_or(0))
    }

    async fn delete(pool: &DatabasePool, id: i64) -> Result<bool, sqlx::Error> {
        sqlx::query("UPDATE posts SET deleted_at = CURRENT_TIMESTAMP WHERE id = ?")
            .bind(id)
            .execute(&pool.pool)
            .await?;
        Ok(true)
    }

    async fn eager_load(relation: &str, items: &mut [Self], pool: &DatabasePool) -> Result<(), sqlx::Error> {
        if relation == "user" || relation == "author" {
            let ids: Vec<i64> = items.iter().map(|p| p.user_id).collect();
            if ids.is_empty() { return Ok(()); }

            let users = User::query(pool)
                .where_in("id", ids)
                .get()
                .await?;

            for post in items {
                post.author = users.iter()
                    .find(|u| u.id == post.user_id)
                    .cloned();
            }
        }
        Ok(())
    }
}

impl Post {
    /// Relasi: Post belongs to User
    pub async fn user(&self, pool: &DatabasePool) -> Result<User, sqlx::Error> {
        Self::belongs_to::<User>(pool, self.user_id).await
    }
}
