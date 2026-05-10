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
    #[sqlx(skip)]
    pub roles:    Option<Vec<Role>>,
}

use crate::app::models::role::Role;

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

    /// Relasi: User belongs to many Roles
    pub async fn roles(&self, pool: &DatabasePool) -> Result<Vec<Role>, sqlx::Error> {
        sqlx::query_as::<_, Role>(
            "SELECT r.id, r.name, r.slug 
             FROM roles r
             JOIN user_role ur ON r.id = ur.role_id
             WHERE ur.user_id = ? AND r.deleted_at IS NULL"
        )
        .bind(self.id)
        .fetch_all(&pool.pool)
        .await
    }

    /// Ambil semua slug permission dari semua role user
    pub async fn all_permissions(&self, pool: &DatabasePool) -> Result<Vec<String>, sqlx::Error> {
        let permissions = sqlx::query_scalar::<_, String>(
            "SELECT DISTINCT p.slug 
             FROM permissions p
             JOIN role_permission rp ON p.id = rp.permission_id
             JOIN user_role ur ON rp.role_id = ur.role_id
             WHERE ur.user_id = ? AND p.deleted_at IS NULL"
        )
        .bind(self.id)
        .fetch_all(&pool.pool)
        .await?;
        
        Ok(permissions)
    }

    pub async fn has_role(&self, pool: &DatabasePool, role_slug: &str) -> bool {
        let count: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM user_role ur 
             JOIN roles r ON ur.role_id = r.id 
             WHERE ur.user_id = ? AND r.slug = ?"
        )
        .bind(self.id)
        .bind(role_slug)
        .fetch_one(&pool.pool)
        .await
        .unwrap_or(0);
        
        count > 0
    }

    pub async fn has_permission(&self, pool: &DatabasePool, permission_slug: &str) -> bool {
        let count: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM permissions p
             JOIN role_permission rp ON p.id = rp.permission_id
             JOIN user_role ur ON rp.role_id = ur.role_id
             WHERE ur.user_id = ? AND p.slug = ?"
        )
        .bind(self.id)
        .bind(permission_slug)
        .fetch_one(&pool.pool)
        .await
        .unwrap_or(0);
        
        count > 0
    }
}
