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
    pub headers: axum::http::HeaderMap,
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
            headers: parts.headers.clone(),
        })
    }
}

use axum::extract::FromRef;

impl Request {
    /// Cek apakah request berasal dari HTMX.
    pub fn is_htmx(&self) -> bool {
        self.headers.contains_key("HX-Request")
    }

    /// Shortcut untuk mengambil instance DatabasePool.
    pub fn db(&self) -> &crate::database::connection::DatabasePool {
        self.state.db()
    }

    /// Shortcut untuk mengambil user yang sedang login.
    pub fn user(&self) -> Option<crate::core::auth::AuthUser> {
        self.user.clone()
    }

    /// Shortcut untuk membuat instance ViewBuilder.
    pub fn view(&self, template: &str) -> crate::core::view::ViewBuilder {
        crate::core::view::View::make(template)
    }

    /// Shortcut untuk membuat instance Redirect.
    pub fn redirect(&self, path: &str) -> crate::core::response::Redirect {
        crate::core::response::Redirect::to(path)
    }

    /// Shortcut untuk redirect kembali ke halaman asal (Referer).
    pub fn back(&self) -> crate::core::response::Redirect {
        let referer = self.headers
            .get(axum::http::header::REFERER)
            .and_then(|h| h.to_str().ok())
            .unwrap_or("/");
        crate::core::response::Redirect::to(referer)
    }

    /// Shortcut untuk mengambil nilai dari session.
    pub async fn session_get<T: serde::de::DeserializeOwned>(&self, key: &str) -> Option<T> {
        self.session.get::<T>(key).await.unwrap_or_default()
    }

    /// Shortcut untuk menyimpan nilai ke dalam session.
    pub async fn session_set<T: serde::Serialize>(&self, key: &str, value: T) {
        let _ = self.session.insert(key, value).await;
    }

    /// Shortcut untuk mengembalikan JSON response secara instan.
    pub fn json<T: serde::Serialize>(&self, value: T) -> axum::response::Response {
        use axum::response::IntoResponse;
        axum::Json(value).into_response()
    }
}
