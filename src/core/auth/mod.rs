use serde::{Serialize, Deserialize};
use async_trait::async_trait;
use axum::{
    extract::FromRequestParts,
    http::request::Parts,
};
use crate::app::models::user::User;

pub mod hash;

/// AuthUser — Representasi pengguna yang terautentikasi.
/// Berfungsi sebagai Model data JWT sekaligus Custom Extractor (Dependency Injection).
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AuthUser {
    pub sub:   String,   // user_id
    pub email: String,
    pub role:  String,
    pub exp:   usize,    // Unix timestamp — kapan token expire
    pub iat:   usize,    // Unix timestamp — kapan token dibuat
}

impl AuthUser {
    pub fn new(user_id: i64, email: String, role: String, hours: usize) -> Self {
        use std::time::{SystemTime, UNIX_EPOCH};
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as usize;
        Self {
            sub:   user_id.to_string(),
            email,
            role,
            iat:   now,
            exp:   now + (hours * 3600),
        }
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
        parts.extensions
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

    /// Hasilkan AuthUser baru.
    pub fn user(user_id: i64, email: String, role: String) -> AuthUser {
        AuthUser::new(user_id, email, role, 24)
    }

    /// Simpan user ke session (Login) menggunakan Request.
    pub async fn login(req: &crate::core::request::Request, user_model: User) {
        let auth_user = Self::user(user_model.id, user_model.email, user_model.role);
        
        // Simpan User object untuk akses cepat via req.user
        let _ = req.session.insert("user", auth_user.clone()).await;
        
        // Simpan JWT untuk divalidasi oleh web_auth_required middleware
        if let Ok(token) = crate::http::auth::generate_token(&auth_user) {
            let _ = req.session.insert("jwt", token).await;
        }
    }

    /// Hapus user dari session (Logout).
    pub async fn logout(req: &crate::core::request::Request) {
        req.session.clear().await;
    }
}
