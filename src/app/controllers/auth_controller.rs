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

#[derive(Deserialize)]
pub struct AuthQuery {
    success: Option<String>,
    error: Option<String>,
}

pub struct AuthController;

impl AuthController {
    /// GET /auth/login — Tampilkan halaman login (HTML)
    pub async fn show_login(
        Query(query): Query<AuthQuery>,
        State(state): State<Arc<AppState>>
    ) -> Html<String> {
        let mut context = tera::Context::new();
        if let Some(msg) = query.success { context.insert("success_msg", &msg); }
        if let Some(msg) = query.error { context.insert("error_msg", &msg); }
        let rendered = state.view.render("auth/login.html", &context);
        Html(rendered)
    }

    /// GET /auth/register — Tampilkan halaman registrasi (HTML)
    pub async fn show_register(
        Query(query): Query<AuthQuery>,
        State(state): State<Arc<AppState>>
    ) -> Html<String> {
        let mut context = tera::Context::new();
        if let Some(msg) = query.success { context.insert("success_msg", &msg); }
        if let Some(msg) = query.error { context.insert("error_msg", &msg); }
        let rendered = state.view.render("auth/register.html", &context);
        Html(rendered)
    }

    /// POST /auth/register — Proses pendaftaran user baru (Web Form)
    pub async fn register(
        State(state): State<Arc<AppState>>,
        ValidatedForm(payload): ValidatedForm<RegisterRequest>
    ) -> impl IntoResponse {
        match state.auth_service.register(payload.name, payload.email, payload.password).await {
            Ok(_) => Redirect::to("/auth/login?success=Registrasi Berhasil! Silakan Login.").into_response(),
            Err(e) => {
                let uri = format!("/auth/register?error={}", e);
                Redirect::to(&uri).into_response()
            }
        }
    }

    /// POST /auth/login — Proses login (Web Form)
    pub async fn login(
        State(state): State<Arc<AppState>>,
        ValidatedForm(payload): ValidatedForm<LoginRequest>
    ) -> impl IntoResponse {
        match state.auth_service.login(&payload.email, &payload.password).await {
            Ok(token) => {
                // Set token as a cookie
                let cookie = format!("token={}; Path=/; HttpOnly; SameSite=Lax", token);
                let mut response = Redirect::to("/dashboard?success=Login Berhasil! Selamat datang di Lumina.").into_response();
                response.headers_mut().insert(axum::http::header::SET_COOKIE, cookie.parse().unwrap());
                response
            },
            Err(e) => {
                let uri = format!("/auth/login?error={}", e);
                Redirect::to(&uri).into_response()
            }
        }
    }

    /// GET /auth/logout — Proses logout (Web)
    pub async fn logout() -> impl IntoResponse {
        let cookie = "token=; Path=/; HttpOnly; Expires=Thu, 01 Jan 1970 00:00:00 GMT";
        let mut response = Redirect::to("/auth/login?success=Berhasil logout.").into_response();
        response.headers_mut().insert(axum::http::header::SET_COOKIE, cookie.parse().unwrap());
        response
    }

    /// POST /api/auth/register — Proses pendaftaran via API (JSON)
    pub async fn api_register(
        State(state): State<Arc<AppState>>,
        ValidatedJson(payload): ValidatedJson<RegisterRequest>
    ) -> Json<Value> {
        match state.auth_service.register(payload.name, payload.email, payload.password).await {
            Ok(id) => Json(json!({"success": true, "message": "User registered", "id": id})),
            Err(e) => Json(json!({"success": false, "message": e})),
        }
    }

    /// POST /api/auth/login — Proses login via API (JSON)
    pub async fn api_login(
        State(state): State<Arc<AppState>>,
        ValidatedJson(payload): ValidatedJson<LoginRequest>
    ) -> Json<Value> {
        match state.auth_service.login(&payload.email, &payload.password).await {
            Ok(token) => Json(json!({"success": true, "token": token})),
            Err(e) => Json(json!({"success": false, "message": e})),
        }
    }

    /// GET /api/auth/me — Protected API route
    pub async fn me() -> Json<Value> {
        Json(json!({
            "success": true,
            "data": {
                "id": 1,
                "name": "Authenticated User",
                "message": "Fitur 'me' akan diekstrak dari request extensions pada tahap selanjutnya."
            }
        }))
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
