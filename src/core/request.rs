use crate::core::application::AppState;
use crate::core::auth::AuthUser;
use crate::core::error::AppError;
use axum::{async_trait, extract::FromRequestParts, http::request::Parts};
use axum_csrf::CsrfToken;
use std::collections::HashMap;
use tower_sessions::Session;

/// Type alias untuk controller handler return type yang cantik dan ekspresif.
/// Menggantikan `Result<impl IntoResponse, AppError>` menjadi hanya `LuminaResult`.
pub type LuminaResult<T = axum::response::Response> = std::result::Result<T, AppError>;

/// Alias alternatif untuk LuminaResult.
pub type HttpResult<T = axum::response::Response> = LuminaResult<T>;

/// Request — Bundle Extractor untuk menyederhanakan Dependency Injection.
/// Menyediakan akses cepat ke State, Session, CSRF, Path Params, Query Params, dan Helper Response dalam satu parameter.
pub struct Request {
    pub state: AppState,
    pub session: Session,
    pub token: CsrfToken,
    pub user: Option<AuthUser>,
    pub headers: axum::http::HeaderMap,
    pub extensions: axum::http::Extensions,
    pub uri: axum::http::Uri,
    pub params: HashMap<String, String>,
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

        let session = parts
            .extensions
            .get::<Session>()
            .cloned()
            .ok_or(crate::core::error::AppError::InternalServerError)?;

        let token = CsrfToken::from_request_parts(parts, state)
            .await
            .map_err(|_| crate::core::error::AppError::InternalServerError)?;

        // Cek user dari extension (Middleware) atau Session
        let user = parts
            .extensions
            .get::<AuthUser>()
            .cloned()
            .or(session.get::<AuthUser>("user").await.unwrap_or_default());

        let uri = parts.uri.clone();

        let params = match axum::extract::RawPathParams::from_request_parts(parts, state).await {
            Ok(raw) => raw
                .iter()
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect::<HashMap<String, String>>(),
            Err(_) => HashMap::new(),
        };

        Ok(Self {
            state: app_state,
            session,
            token,
            user,
            headers: parts.headers.clone(),
            extensions: parts.extensions.clone(),
            uri,
            params,
        })
    }
}

use axum::extract::FromRef;

impl Request {

    /// Shortcut untuk mengambil instance DatabasePool.
    pub fn db(&self) -> &crate::database::connection::DatabasePool {
        self.state.db()
    }

    /// Shortcut untuk mengambil Arc<DatabasePool>.
    pub fn db_arc(&self) -> std::sync::Arc<crate::database::connection::DatabasePool> {
        self.state
            .db
            .as_ref()
            .expect("Database connection is not available")
            .clone()
    }

    /// Akses direct native PgPool (PostgreSQL) jika fitur postgres aktif.
    #[cfg(feature = "postgres")]
    pub fn pg_pool(&self) -> Option<sqlx::PgPool> {
        self.db().pg().cloned()
    }

    /// Akses direct native SqlitePool jika fitur sqlite aktif.
    #[cfg(feature = "sqlite")]
    pub fn sqlite_pool(&self) -> Option<sqlx::SqlitePool> {
        self.db().sqlite().cloned()
    }

    /// Akses direct native MySqlPool jika fitur mysql aktif.
    #[cfg(feature = "mysql")]
    pub fn mysql_pool(&self) -> Option<sqlx::MySqlPool> {
        self.db().mysql().cloned()
    }

    /// Ambil token Bearer dari header Authorization jika tersedia.
    pub fn bearer_token(&self) -> Option<String> {
        self.headers
            .get(axum::http::header::AUTHORIZATION)
            .and_then(|h| h.to_str().ok())
            .and_then(|h| {
                if h.starts_with("Bearer ") {
                    Some(h[7..].trim().to_string())
                } else {
                    None
                }
            })
    }

    /// Cabut / batalkan (revoke) token JWT aktif saat ini (misal pada endpoint POST /api/logout).
    pub async fn revoke_token(&self) -> bool {
        if let Some(t) = self.bearer_token() {
            crate::core::auth::TokenBlacklist::revoke(&t).await;
            true
        } else {
            false
        }
    }

    /// Shortcut untuk mengambil user yang sedang login.
    pub fn user(&self) -> Option<crate::core::auth::AuthUser> {
        self.user.clone()
    }

    /// Shortcut untuk mengakses Queue Manager
    pub fn queue(&self) -> &crate::core::queue::QueueManager {
        &self.state.queue
    }

    /// Shortcut untuk dispatch background job
    pub async fn dispatch(&self, job_name: &str, payload: serde_json::Value) -> Result<(), String> {
        self.state.queue.dispatch(job_name, payload).await
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
        let referer = self
            .headers
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

    /// Shortcut untuk mengambil instance StorageManager.
    pub fn storage(&self) -> &crate::core::storage::Storage {
        &self.state.storage
    }

    /// Shortcut untuk mengambil instance Mailer.
    pub fn mail(&self) -> &crate::core::mail::Mail {
        &self.state.mail
    }

    /// Cek apakah user sudah login.
    pub fn is_authenticated(&self) -> bool {
        self.user.is_some()
    }

    /// Cek apakah user belum login (guest).
    pub fn is_guest(&self) -> bool {
        self.user.is_none()
    }

    /// Cek apakah user yang sedang login memiliki izin (Permission).
    pub fn can(&self, permission: &str) -> bool {
        self.user.as_ref().map(|u| u.can(permission)).unwrap_or(false)
    }

    /// Cek apakah user tidak memiliki izin (Permission).
    pub fn cannot(&self, permission: &str) -> bool {
        !self.can(permission)
    }

    /// Cek apakah user memiliki peran (Role).
    pub fn has_role(&self, role: &str) -> bool {
        self.user.as_ref().map(|u| u.has_role(role)).unwrap_or(false)
    }

    /// Otorisasi izin secara deklaratif di Controller.
    /// Mengembalikan Ok(()) jika diizinkan, atau Err(AppError::Forbidden) jika ditolak.
    pub fn authorize(&self, permission: &str) -> Result<(), AppError> {
        match &self.user {
            Some(u) if u.can(permission) => Ok(()),
            Some(_) => Err(AppError::Forbidden(format!("Anda tidak memiliki izin '{}'", permission))),
            None => Err(AppError::Unauthorized),
        }
    }

    /// Otorisasi peran (Role) secara deklaratif di Controller.
    pub fn authorize_role(&self, role: &str) -> Result<(), AppError> {
        match &self.user {
            Some(u) if u.has_role(role) => Ok(()),
            Some(_) => Err(AppError::Forbidden(format!("Role '{}' diperlukan untuk aksi ini", role))),
            None => Err(AppError::Unauthorized),
        }
    }

    /// Catat log audit secara asinkron (Enterprise Audit Trail).
    pub fn audit_log(
        &self,
        action: &str,
        resource: &str,
        resource_id: Option<String>,
        details: Option<serde_json::Value>,
        status: &str,
    ) {
        let user_id = self.user.as_ref().map(|u| u.sub.clone());
        let ip = self.header("x-forwarded-for")
            .or_else(|| self.header("x-real-ip"))
            .unwrap_or("127.0.0.1");
        let ua = self.header("user-agent").map(|s| s.to_string());

        crate::core::security::AuditTrail::record(
            user_id,
            ip,
            ua,
            action,
            resource,
            resource_id,
            details,
            status,
        );
    }

    /// Redirect jika user sudah login (misal: di halaman login/register).
    pub async fn redirect_if_authenticated(&self, path: &str) -> Option<axum::response::Response> {
        if self.is_authenticated() {
            use axum::response::IntoResponse;
            return Some(self.redirect(path).go(self).await.into_response());
        }
        None
    }

    /// Mengambil nilai header HTTP sebagai string slice.
    pub fn header(&self, name: &str) -> Option<&str> {
        self.headers.get(name).and_then(|v| v.to_str().ok())
    }

    /// Cek apakah client meminta response berformat JSON (ala Laravel $request->wantsJson()).
    /// Memeriksa header Accept untuk 'application/json' atau '*/*'.
    pub fn wants_json(&self) -> bool {
        if let Some(accept) = self.header("accept") {
            accept.contains("application/json")
        } else {
            false
        }
    }

    /// Cek apakah request payload bertipe JSON (Content-Type: application/json).
    pub fn is_json(&self) -> bool {
        if let Some(content_type) = self.header("content-type") {
            content_type.contains("application/json")
        } else {
            false
        }
    }

    // ─────────────────────────────────────────────────────────────────────────
    //  PARAMETER & QUERY HELPERS (ALA LARAVEL $request->...)
    // ─────────────────────────────────────────────────────────────────────────

    /// Ambil nilai path parameter (misal: :id pada route /api/contacts/:id).
    pub fn param(&self, key: &str) -> Option<&str> {
        self.params.get(key).map(|s| s.as_str())
    }

    /// Ambil nilai path parameter dan parse ke tipe data tujuan.
    pub fn param_as<T: std::str::FromStr>(&self, key: &str) -> Option<T> {
        self.param(key).and_then(|s| s.parse::<T>().ok())
    }

    /// Shortcut untuk mengambil ID dari route parameter (misal: :id).
    pub fn id(&self) -> Option<i64> {
        self.param_as::<i64>("id")
    }

    /// Ambil ID dari route parameter atau kembalikan AppError::BadRequest jika tidak ada atau tidak valid.
    pub fn id_or_fail(&self) -> Result<i64, AppError> {
        self.id().ok_or_else(|| AppError::BadRequest("Parameter ID wajib diberikan dan harus angka".to_string()))
    }

    /// Ambil nilai query string berdasarkan nama key (misal: ?search=budi).
    pub fn query_val(&self, key: &str) -> Option<String> {
        let q = self.uri.query()?;
        for pair in q.split('&') {
            let mut parts = pair.splitn(2, '=');
            if let Some(k) = parts.next() {
                if k == key {
                    let v = parts.next().unwrap_or("");
                    let decoded = v.replace('+', " ");
                    return Some(decoded);
                }
            }
        }
        None
    }

    /// Ambil nomor halaman dari query parameter `?page=X` (default: 1).
    pub fn page(&self) -> usize {
        self.query_val("page")
            .and_then(|v| v.parse::<usize>().ok())
            .unwrap_or(1)
            .max(1)
    }

    /// Ambil jumlah per halaman dari query parameter `?per_page=X` atau `?limit=X` (default: default_per_page).
    pub fn per_page(&self, default_per_page: usize) -> usize {
        self.query_val("per_page")
            .or_else(|| self.query_val("limit"))
            .and_then(|v| v.parse::<usize>().ok())
            .unwrap_or(default_per_page)
            .max(1)
    }

    /// Shortcut mengambil tuple (page, per_page) untuk paginasi instan ala Laravel.
    /// Default per_page adalah 10 jika tidak ditentukan di URL (?page=1&per_page=10).
    pub fn pagination(&self) -> (usize, usize) {
        (self.page(), self.per_page(10))
    }

    /// Deserialize seluruh query string ke dalam struct bertipe T.
    pub fn query<T: serde::de::DeserializeOwned>(&self) -> Result<T, AppError> {
        let q = self.uri.query().unwrap_or("");
        serde_urlencoded::from_str(q).map_err(|e| AppError::BadRequest(format!("Format query parameter tidak valid: {}", e)))
    }

    // ─────────────────────────────────────────────────────────────────────────
    //  RESPONSE HELPERS (MENGEMBALIKAN LuminaResult / Ok(Response))
    // ─────────────────────────────────────────────────────────────────────────

    /// Shortcut untuk mengembalikan JSON response secara instan.
    pub fn json<T: serde::Serialize>(&self, value: T) -> LuminaResult {
        use axum::response::IntoResponse;
        Ok(axum::Json(value).into_response())
    }

    /// Mengembalikan response JSON standar HTTP 200 OK:
    /// `{ "success": true, "data": ... }`
    pub fn ok<T: serde::Serialize>(&self, data: T) -> LuminaResult {
        use axum::response::IntoResponse;
        Ok(axum::Json(serde_json::json!({
            "success": true,
            "data": data
        })).into_response())
    }

    /// Mengembalikan response JSON HTTP 200 OK dengan pesan sukses kustom:
    /// `{ "success": true, "message": "...", "data": ... }`
    pub fn success<T: serde::Serialize>(&self, data: T, message: &str) -> LuminaResult {
        use axum::response::IntoResponse;
        Ok(axum::Json(serde_json::json!({
            "success": true,
            "message": message,
            "data": data
        })).into_response())
    }

    /// Mengembalikan response JSON HTTP 201 Created:
    /// `{ "success": true, "data": ... }`
    pub fn created<T: serde::Serialize>(&self, data: T) -> LuminaResult {
        use axum::response::IntoResponse;
        Ok((
            axum::http::StatusCode::CREATED,
            axum::Json(serde_json::json!({
                "success": true,
                "data": data
            }))
        ).into_response())
    }

    /// Mengembalikan response JSON HTTP 201 Created dengan pesan:
    pub fn created_with<T: serde::Serialize>(&self, data: T, message: &str) -> LuminaResult {
        use axum::response::IntoResponse;
        Ok((
            axum::http::StatusCode::CREATED,
            axum::Json(serde_json::json!({
                "success": true,
                "message": message,
                "data": data
            }))
        ).into_response())
    }

    /// Mengembalikan response HTTP 204 No Content
    pub fn no_content(&self) -> LuminaResult {
        use axum::response::IntoResponse;
        Ok(axum::http::StatusCode::NO_CONTENT.into_response())
    }

    /// Mengembalikan response JSON HTTP 404 Not Found:
    /// `{ "success": false, "error": "..." }`
    pub fn not_found(&self, message: &str) -> LuminaResult {
        use axum::response::IntoResponse;
        Ok((
            axum::http::StatusCode::NOT_FOUND,
            axum::Json(serde_json::json!({
                "success": false,
                "error": message
            }))
        ).into_response())
    }

    /// Mengembalikan response JSON HTTP 400 Bad Request:
    /// `{ "success": false, "error": "..." }`
    pub fn bad_request(&self, message: &str) -> LuminaResult {
        use axum::response::IntoResponse;
        Ok((
            axum::http::StatusCode::BAD_REQUEST,
            axum::Json(serde_json::json!({
                "success": false,
                "error": message
            }))
        ).into_response())
    }

    /// Mengembalikan response JSON HTTP 401 Unauthorized
    pub fn unauthorized(&self, message: &str) -> LuminaResult {
        use axum::response::IntoResponse;
        Ok((
            axum::http::StatusCode::UNAUTHORIZED,
            axum::Json(serde_json::json!({
                "success": false,
                "error": message
            }))
        ).into_response())
    }

    /// Mengembalikan response JSON HTTP 403 Forbidden
    pub fn forbidden(&self, message: &str) -> LuminaResult {
        use axum::response::IntoResponse;
        Ok((
            axum::http::StatusCode::FORBIDDEN,
            axum::Json(serde_json::json!({
                "success": false,
                "error": message
            }))
        ).into_response())
    }

    /// Mengembalikan response JSON HTTP 422 Unprocessable Entity (Validasi error)
    pub fn unprocessable<E: serde::Serialize>(&self, errors: E, message: &str) -> LuminaResult {
        use axum::response::IntoResponse;
        Ok((
            axum::http::StatusCode::UNPROCESSABLE_ENTITY,
            axum::Json(serde_json::json!({
                "success": false,
                "message": message,
                "errors": errors
            }))
        ).into_response())
    }

    /// Mengembalikan response JSON data paginasi bergaya Laravel:
    /// `{ "success": true, "data": [...], "meta": { "total": ..., "current_page": ..., "last_page": ..., "per_page": ... } }`
    pub fn paginate<T: serde::Serialize>(&self, paginator: crate::database::orm::LengthAwarePaginator<T>) -> LuminaResult {
        use axum::response::IntoResponse;
        Ok(axum::Json(serde_json::json!({
            "success": true,
            "data": paginator.items,
            "meta": {
                "total": paginator.total,
                "current_page": paginator.current_page,
                "last_page": paginator.last_page,
                "per_page": paginator.per_page
            }
        })).into_response())
    }
}
