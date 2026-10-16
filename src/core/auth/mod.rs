// use crate::app::models::user::User;
use async_trait::async_trait;
use axum::{extract::FromRequestParts, http::request::Parts};
use serde::{Deserialize, Serialize};
use tower_sessions::Session;

pub mod hash;
pub mod socialite;

/// AuthUser — Representasi pengguna yang terautentikasi.
/// Berfungsi sebagai Model data JWT sekaligus Custom Extractor (Dependency Injection).
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AuthUser {
    pub sub: String, // user_id
    pub email: String,
    pub role: String,
    pub permissions: Vec<String>,
    pub exp: usize, // Unix timestamp — kapan token expire
    pub iat: usize, // Unix timestamp — kapan token dibuat
}

impl AuthUser {
    pub fn new(
        user_id: i64,
        email: String,
        role: String,
        permissions: Vec<String>,
        hours: usize,
    ) -> Self {
        use std::time::{SystemTime, UNIX_EPOCH};
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as usize;
        Self {
            sub: user_id.to_string(),
            email,
            role,
            permissions,
            iat: now,
            exp: now + (hours * 3600),
        }
    }

    pub fn can(&self, permission: &str) -> bool {
        self.permissions.iter().any(|p| p == permission)
    }

    pub fn has_role(&self, role: &str) -> bool {
        self.role == role || self.role == "admin"
    }
}

/// Implementasi Extractor agar AuthUser bisa langsung disuntikkan ke parameter Controller.
#[async_trait]
impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = crate::core::error::AppError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // Ambil dari extension yang sudah disuntikkan oleh middleware auth
        parts
            .extensions
            .get::<Self>()
            .cloned()
            .ok_or(crate::core::error::AppError::Unauthorized)
    }
}
/// Auth Facade — Memberikan API yang bersih untuk operasi autentikasi.
pub struct Auth;

impl Auth {
    /// Hash password.
    pub fn make_hash(plain: &str) -> String {
        self::hash::make(plain)
    }

    /// Verifikasi password.
    pub fn verify(plain: &str, hashed: &str) -> bool {
        self::hash::check(plain, hashed)
    }

    /// Verifikasi apakah password cocok dengan hash.
    pub fn check(plain: &str, hashed: &str) -> bool {
        self::hash::check(plain, hashed)
    }

    /// Hasilkan AuthUser baru.
    pub fn user(user_id: i64, email: String, role: String, permissions: Vec<String>) -> AuthUser {
        AuthUser::new(user_id, email, role, permissions, 24)
    }

    /// Simpan user ke session (Login).
    pub async fn login(session: &Session, auth_user: AuthUser) -> Result<(), tower_sessions::session::Error> {
        // Simpan User object untuk akses cepat via req.user
        let _ = session.insert("user", auth_user.clone()).await;

        // Simpan JWT untuk divalidasi oleh web_auth_required middleware
        if let Ok(token) = crate::http::auth::generate_token(&auth_user) {
            let _ = session.insert("jwt", token).await;
        }
        Ok(())
    }

    /// Hapus user dari session (Logout).
    pub async fn logout(session: &Session) {
        session.clear().await;
    }
}
