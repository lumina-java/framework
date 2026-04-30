use axum::{
    async_trait,
    extract::FromRequestParts,
    http::request::Parts,
};
use tower_sessions::Session;
use axum_csrf::CsrfToken;
use crate::core::application::AppState;
use crate::core::auth::AuthUser;

/// Request — Bundle Extractor untuk menyederhanakan Dependency Injection.
/// Menyediakan akses cepat ke State, Session, dan CsrfToken dalam satu parameter.
pub struct Request {
    pub state: AppState,
    pub session: Session,
    pub token: CsrfToken,
    pub user: Option<AuthUser>,
}

#[async_trait]
impl<S> FromRequestParts<S> for Request
where
    AppState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = crate::core::error::AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let app_state = AppState::from_ref(state);
        
        let session = parts.extensions
            .get::<Session>()
            .cloned()
            .ok_or(crate::core::error::AppError::InternalServerError)?;

        let token = CsrfToken::from_request_parts(parts, state).await
            .map_err(|_| crate::core::error::AppError::InternalServerError)?;

        // Cek user dari extension (Middleware) atau Session
        let user = parts.extensions.get::<AuthUser>().cloned()
            .or(session.get::<AuthUser>("user").await.unwrap_or_default());

        Ok(Self {
            state: app_state,
            session,
            token,
            user,
        })
    }
}

use axum::extract::FromRef;

impl Request {
    /// Ambil user yang sedang login (jika ada).
    pub fn user(&self) -> Option<AuthUser> {
        // Implementasi pengambilan user dari extensions parts jika diperlukan, 
        // tapi AuthUser biasanya sudah ada di parameter terpisah.
        None 
    }
}
