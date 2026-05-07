use async_trait::async_trait;
use serde::{Serialize, Deserialize};
use sqlx::FromRow;
use crate::database::{connection::DatabasePool, model::Model};

/// Model User — merepresentasikan satu baris dari tabel `users`.
#[derive(Debug, Serialize, Deserialize, FromRow, Clone, Default)]
pub struct User {
    pub id:       i64,
    pub name:     String,
    pub email:    String,
    pub password: String,
    pub role:     String,
    #[sqlx(skip)]
    pub posts:    Option<Vec<Post>>,
}

#[async_trait]
impl Model for User {
    const TABLE: &'static str = "users";

    async fn find(pool: &DatabasePool, id: i64) -> Result<Self, sqlx::Error> {
        sqlx::query_as::<_, User>(
            "SELECT id, name, email, password, role
             FROM users
             WHERE id = ? AND deleted_at IS NULL"
        )
        .bind(id)
        .fetch_one(&pool.pool)
        .await
    }

    async fn all(pool: &DatabasePool) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, User>(
            "SELECT id, name, email, password, role
             FROM users
             WHERE deleted_at IS NULL
             ORDER BY id ASC"
        )
        .fetch_all(&pool.pool)
        .await
    }

    async fn save(&self, pool: &DatabasePool) -> Result<i64, sqlx::Error> {
        let result = sqlx::query(
            "INSERT INTO users (name, email, password, role)
             VALUES (?, ?, ?, ?)"
        )
        .bind(&self.name)
        .bind(&self.email)
        .bind(&self.password)
        .bind(&self.role)
        .execute(&pool.pool)
        .await?;

        Ok(result.last_insert_id().unwrap_or(0))
    }

    async fn delete(pool: &DatabasePool, id: i64) -> Result<bool, sqlx::Error> {
        // Soft delete — tidak benar-benar menghapus dari DB
        sqlx::query(
            "UPDATE users SET deleted_at = CURRENT_TIMESTAMP WHERE id = ?"
        )
        .bind(id)
        .execute(&pool.pool)
        .await?;

        Ok(true)
    }

    async fn eager_load(relation: &str, items: &mut [Self], pool: &DatabasePool) -> Result<(), sqlx::Error> {
        if relation == "posts" {
            let ids: Vec<i64> = items.iter().map(|u| u.id).collect();
            if ids.is_empty() { return Ok(()); }

            let posts = Post::query(pool)
                .where_in("user_id", ids)
                .get()
                .await?;

            for user in items {
                let user_posts: Vec<Post> = posts.iter()
                    .filter(|p| p.user_id == user.id)
                    .cloned()
                    .collect();
                user.posts = Some(user_posts);
            }
        }
        Ok(())
    }
}

use crate::app::models::post::Post;

impl User {
    pub async fn find_by_email(pool: &DatabasePool, email: &str) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as::<_, User>(
            "SELECT id, name, email, password, role
             FROM users
             WHERE email = ? AND deleted_at IS NULL"
        )
        .bind(email)
        .fetch_optional(&pool.pool)
        .await
    }

    /// Relasi: User has many Posts
    pub async fn posts(&self, pool: &DatabasePool) -> Result<Vec<Post>, sqlx::Error> {
        Self::has_many::<Post>(pool, "user_id", self.id).await
    }
}
