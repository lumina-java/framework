use async_trait::async_trait;
use serde::{Serialize, Deserialize};
use sqlx::{FromRow, Row};
use crate::database::{connection::DatabasePool, model::Model};
use crate::app::models::user::User;
use crate::app::models::tag::Tag;

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Post {
    pub id: i64,
    pub user_id: i64,
    pub title: String,
    pub content: String,
    #[sqlx(skip)]
    pub author: Option<User>,
    #[sqlx(skip)]
    pub tags: Option<Vec<Tag>>,
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
        } else if relation == "tags" {
            let ids: Vec<i64> = items.iter().map(|p| p.id).collect();
            if ids.is_empty() { return Ok(()); }

            // Eager load many-to-many using a single query
            // SELECT t.*, pt.post_id FROM tags t JOIN post_tag pt ON t.id = pt.tag_id WHERE pt.post_id IN (...)
            let sql = format!(
                "SELECT t.id, t.name, pt.post_id FROM tags t JOIN post_tag pt ON t.id = pt.tag_id WHERE pt.post_id IN ({}) AND t.deleted_at IS NULL",
                vec!["?"; ids.len()].join(",")
            );

            let mut query = sqlx::query(&sql);
            for id in ids { query = query.bind(id); }

            let rows = query.fetch_all(&pool.pool).await?;
            
            for post in items {
                let post_tags: Vec<Tag> = rows.iter()
                    .filter(|row| {
                        let p_id: i64 = row.get("post_id");
                        p_id == post.id
                    })
                    .map(|row| Tag {
                        id: row.get("id"),
                        name: row.get("name"),
                    })
                    .collect();
                post.tags = Some(post_tags);
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

    /// Relasi: Post has many Tags (Many-to-Many)
    pub async fn tags(&self, pool: &DatabasePool) -> Result<Vec<Tag>, sqlx::Error> {
        Self::belongs_to_many::<Tag>(pool, "post_tag", "post_id", "tag_id", self.id).await
    }
}
