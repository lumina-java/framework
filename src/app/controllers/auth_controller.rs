use axum::{
    extract::State,
    response::{Html, IntoResponse, Redirect},
    Json,
};
use std::sync::Arc;
use serde_json::{json, Value};
use crate::core::application::AppState;
use crate::core::validation::{ValidatedForm, ValidatedJson};
use crate::core::auth::{Claims, hash};
use crate::app::models::user::User;
use crate::database::model::Model;
use serde::Deserialize;
use validator::Validate;

pub struct AuthController;

impl AuthController {
    /// GET /auth/login — Tampilkan halaman login (HTML)
    pub async fn show_login(State(state): State<Arc<AppState>>) -> Html<String> {
        let rendered = state.view.render("auth/login.html", &tera::Context::new());
        Html(rendered)
    }

    /// GET /auth/register — Tampilkan halaman registrasi (HTML)
    pub async fn show_register(State(state): State<Arc<AppState>>) -> Html<String> {
        let rendered = state.view.render("auth/register.html", &tera::Context::new());
        Html(rendered)
    }

    /// POST /auth/register — Proses pendaftaran user baru (Web Form)
    pub async fn register(
        State(state): State<Arc<AppState>>,
        ValidatedForm(payload): ValidatedForm<RegisterRequest>
    ) -> impl IntoResponse {
        // 1. Hash password
        let hashed_password = hash::make(&payload.password);

        // 2. Simpan ke database
        let user = User {
            id: 0,
            name: payload.name,
            email: payload.email,
            password: hashed_password,
            role: "user".to_string(),
        };

        match user.save(&state.db).await {
            Ok(_) => Redirect::to("/auth/login").into_response(),
            Err(e) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, format!("Database error: {}", e)).into_response(),
        }
    }

    /// POST /auth/login — Proses login (Web Form)
    pub async fn login(
        State(state): State<Arc<AppState>>,
        ValidatedForm(payload): ValidatedForm<LoginRequest>
    ) -> impl IntoResponse {
        // 1. Cari user berdasarkan email
        let user = match User::find_by_email(&state.db, &payload.email).await {
            Ok(u) => u,
            Err(_) => return (axum::http::StatusCode::UNAUTHORIZED, "Email atau password salah").into_response(),
        };

        // 2. Verifikasi password
        if !hash::check(&payload.password, &user.password) {
            return (axum::http::StatusCode::UNAUTHORIZED, "Email atau password salah").into_response();
        }

        // 3. Generate JWT token
        let claims = Claims::new(user.id, user.email, user.role, 24);
        match crate::http::auth::generate_token(&claims) {
            Ok(token) => {
                // Di sini idealnya simpan di Cookie, tapi untuk demo kita kembalikan JSON
                Json(json!({
                    "success": true,
                    "token": token,
                    "message": "Login berhasil! Gunakan token ini di header Authorization: Bearer <token>"
                })).into_response()
            },
            Err(_) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Gagal generate token").into_response(),
        }
    }

    /// POST /api/auth/register — Proses pendaftaran via API (JSON)
    pub async fn api_register(
        State(state): State<Arc<AppState>>,
        ValidatedJson(payload): ValidatedJson<RegisterRequest>
    ) -> Json<Value> {
        let hashed_password = hash::make(&payload.password);
        let user = User {
            id: 0,
            name: payload.name,
            email: payload.email,
            password: hashed_password,
            role: "user".to_string(),
        };

        match user.save(&state.db).await {
            Ok(id) => Json(json!({"success": true, "message": "User registered", "id": id})),
            Err(e) => Json(json!({"success": false, "message": format!("Error: {}", e)})),
        }
    }

    /// POST /api/auth/login — Proses login via API (JSON)
    pub async fn api_login(
        State(state): State<Arc<AppState>>,
        ValidatedJson(payload): ValidatedJson<LoginRequest>
    ) -> Json<Value> {
        let user = match User::find_by_email(&state.db, &payload.email).await {
            Ok(u) => u,
            Err(_) => return Json(json!({"success": false, "message": "Unauthorized"})),
        };

        if !hash::check(&payload.password, &user.password) {
            return Json(json!({"success": false, "message": "Unauthorized"}));
        }

        let claims = Claims::new(user.id, user.email, user.role, 24);
        match crate::http::auth::generate_token(&claims) {
            Ok(token) => Json(json!({"success": true, "token": token})),
            Err(_) => Json(json!({"success": false, "message": "Token error"})),
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
