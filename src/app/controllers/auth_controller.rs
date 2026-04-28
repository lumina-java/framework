use axum::{
    extract::{State, Query},
    response::{Html, IntoResponse, Redirect},
    Json,
};
use std::sync::Arc;
use serde_json::{json, Value};
use crate::core::application::AppState;
use crate::core::validation::{ValidatedForm, ValidatedJson};
use serde::Deserialize;
use validator::Validate;
use axum_extra::extract::CookieJar;
use axum_extra::extract::cookie::Cookie;

#[derive(Deserialize)]
pub struct AuthQuery {
    success: Option<String>,
    error: Option<String>,
}

pub struct AuthController;

impl AuthController {
    /// GET /auth/login — Tampilkan halaman login (HTML)
    pub async fn show_login(
        State(state): State<Arc<AppState>>,
        session: tower_sessions::Session,
    ) -> impl IntoResponse {
        // Redirect if already logged in via session
        if session.get::<String>("jwt").await.unwrap_or_default().is_some() {
            return Redirect::to("/dashboard").into_response();
        }

        let rendered = state.view.render_with_session("auth/login.html", tera::Context::new(), &session).await;
        Html(rendered).into_response()
    }

    /// GET /auth/register — Tampilkan halaman registrasi (HTML)
    pub async fn show_register(
        State(state): State<Arc<AppState>>,
        session: tower_sessions::Session,
    ) -> impl IntoResponse {
        // Redirect if already logged in
        if session.get::<String>("jwt").await.unwrap_or_default().is_some() {
            return Redirect::to("/dashboard").into_response();
        }

        let rendered = state.view.render_with_session("auth/register.html", tera::Context::new(), &session).await;
        Html(rendered).into_response()
    }

    /// POST /auth/register — Proses pendaftaran user baru (Web Form)
    pub async fn register(
        State(state): State<Arc<AppState>>,
        session: tower_sessions::Session,
        ValidatedForm(payload): ValidatedForm<RegisterRequest>
    ) -> impl IntoResponse {
        let svc = state.auth_service.as_ref().expect("db_guard seharusnya mencegah ini");
        let flash = crate::core::session::FlashManager::new(&session);

        match svc.register(payload.name, payload.email, payload.password).await {
            Ok(_) => {
                flash.success("Registrasi Berhasil! Silakan Login.").await;
                Redirect::to("/auth/login").into_response()
            },
            Err(e) => {
                flash.error(&format!("Gagal daftar: {}", e)).await;
                Redirect::to("/auth/register").into_response()
            }
        }
    }

    /// POST /auth/login — Proses login (Web Form)
    pub async fn login(
        State(state): State<Arc<AppState>>,
        session: tower_sessions::Session,
        ValidatedForm(payload): ValidatedForm<LoginRequest>
    ) -> impl IntoResponse {
        let svc = state.auth_service.as_ref().expect("db_guard seharusnya mencegah ini");
        let flash = crate::core::session::FlashManager::new(&session);

        match svc.login(&payload.email, &payload.password).await {
            Ok(token) => {
                // Simpan JWT di session
                let _ = session.insert("jwt", token).await;
                flash.success("Login Berhasil! Selamat datang.").await;
                Redirect::to("/dashboard").into_response()
            },
            Err(e) => {
                flash.error(&format!("Login gagal: {}", e)).await;
                Redirect::to("/auth/login").into_response()
            }
        }
    }

    /// GET /auth/logout — Hapus sesi login
    pub async fn logout(session: tower_sessions::Session) -> impl IntoResponse {
        let flash = crate::core::session::FlashManager::new(&session);
        flash.success("Berhasil logout.").await;
        session.clear().await;
        Redirect::to("/auth/login")
    }

    /// POST /api/auth/register — Proses pendaftaran via API (JSON)
    pub async fn api_register(
        State(state): State<Arc<AppState>>,
        ValidatedJson(payload): ValidatedJson<RegisterRequest>
    ) -> Json<Value> {
        let svc = state.auth_service.as_ref().expect("db_guard seharusnya mencegah ini");
        match svc.register(payload.name, payload.email, payload.password).await {
            Ok(id) => Json(json!({"success": true, "message": "User registered", "id": id})),
            Err(e) => Json(json!({"success": false, "message": e})),
        }
    }

    /// POST /api/auth/login — Proses login via API (JSON)
    pub async fn api_login(
        State(state): State<Arc<AppState>>,
        ValidatedJson(payload): ValidatedJson<LoginRequest>
    ) -> Json<Value> {
        let svc = state.auth_service.as_ref().expect("db_guard seharusnya mencegah ini");
        match svc.login(&payload.email, &payload.password).await {
            Ok(token) => Json(json!({"success": true, "token": token})),
            Err(e) => Json(json!({"success": false, "message": e})),
        }
    }

    /// GET /api/auth/me — Protected API route
    pub async fn me() -> Result<crate::core::response::ApiResponse<serde_json::Value>, crate::core::error::AppError> {
        Ok(crate::core::response::ApiResponse::success(json!({
            "id": 1,
            "name": "Authenticated User",
            "message": "Fitur 'me' akan diekstrak dari request extensions pada tahap selanjutnya."
        })))
    }
}

#[derive(Debug, Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(email(message = "Format email tidak valid"))]
    pub email: String,
    #[validate(length(min = 1, message = "Password wajib diisi"))]
    pub password: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(length(min = 3, message = "Nama minimal 3 karakter"))]
    pub name: String,
    #[validate(email(message = "Format email tidak valid"))]
    pub email: String,
    #[validate(length(min = 8, message = "Password minimal 8 karakter"))]
    pub password: String,
}
