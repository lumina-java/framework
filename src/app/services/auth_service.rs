use std::sync::Arc;
use crate::database::connection::DatabasePool;
use crate::app::models::user::User;
use crate::database::model::Model;
use crate::core::auth::{AuthUser, hash};

pub struct AuthService {
    db: Arc<DatabasePool>,
}

impl AuthService {
    pub fn new(db: Arc<DatabasePool>) -> Self {
        Self { db }
    }

    /// Logika pendaftaran user baru
    pub async fn register(&self, name: String, email: String, password: String) -> Result<i64, String> {
        let hashed_password = hash::make(&password);
        let user = User {
            name,
            email,
            password: hashed_password,
            role: "user".to_string(),
            ..Default::default()
        };

        user.save(&self.db).await
            .map_err(|e| format!("Database error: {}", e))
    }

    /// Logika verifikasi login dan generate token
    pub async fn login(&self, email: &str, password: &str) -> Result<String, String> {
        let user = User::find_by_email(&self.db, email).await
            .map_err(|_| "Email atau password salah".to_string())?
            .ok_or("Email atau password salah".to_string())?;

        if !hash::check(password, &user.password) {
            return Err("Email atau password salah".to_string());
        }

        let auth_user = AuthUser::new(user.id, user.email, user.role, 24);
        crate::http::auth::generate_token(&auth_user)
            .map_err(|_| "Gagal generate token".to_string())
    }
}
