use serde::{Serialize, Deserialize};
use async_trait::async_trait;
use axum::{
    extract::FromRequestParts,
    http::request::Parts,
};

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
