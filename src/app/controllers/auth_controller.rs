use axum::{
    extract::State,
    response::IntoResponse,
};
use crate::core::response::Redirect;
use crate::core::view::View;
use serde_json::json;
use crate::core::application::AppState;
use crate::core::validation::ValidatedForm;
use serde::{Deserialize, Serialize};
use validator::Validate;
use axum_csrf::CsrfToken;

#[derive(Deserialize, Serialize, Validate)]
pub struct RegisterForm {
    #[validate(length(min = 3, message = "Nama minimal 3 karakter"))]
    pub name: String,
    
    #[validate(email(message = "Format email tidak valid"))]
    pub email: String,
    
    #[validate(length(min = 6, message = "Password minimal 6 karakter"))]
    pub password: String,

    pub password_confirmation: String,
    pub csrf_token: String,
}

#[derive(Deserialize, Serialize, Validate)]
pub struct LoginForm {
    #[validate(email(message = "Format email tidak valid"))]
    pub email: String,
    pub password: String,
    pub csrf_token: String,
}

pub struct AuthController;

impl AuthController {
    /// GET /auth/login — Tampilkan halaman login (HTML)
    pub async fn show_login(
        State(state): State<AppState>,
        token: CsrfToken,
        session: tower_sessions::Session,
    ) -> impl IntoResponse {
        // Redirect if already logged in via session
        if session.get::<crate::core::auth::AuthUser>("user").await.unwrap_or_default().is_some() {
            return (token, Redirect::to("/dashboard")).into_response();
        }

        View::make("auth.login")
            .with("csrf_token", token.authenticity_token().unwrap())
            .render(&state, &session)
            .await
            .into_response(token)
    }

    /// GET /auth/register — Tampilkan halaman registrasi (HTML)
    pub async fn show_register(
        State(state): State<AppState>,
        token: CsrfToken,
        session: tower_sessions::Session,
    ) -> impl IntoResponse {
        View::make("auth.register")
            .with("csrf_token", token.authenticity_token().unwrap())
            .render(&state, &session)
            .await
            .into_response(token)
    }

    /// POST /auth/register — Proses pendaftaran user baru
    pub async fn register(
        State(state): State<AppState>,
        token: CsrfToken,
        session: tower_sessions::Session,
        ValidatedForm(form): ValidatedForm<RegisterForm>,
    ) -> impl IntoResponse {
        // 1. Validasi CSRF
        if let Err(_) = token.verify(&form.csrf_token) {
             return (token, Redirect::to("/auth/register").with_error("Invalid CSRF Token").send(&session).await).into_response();
        }

        // 2. Validasi Password Confirmation (Custom validation)
        if form.password != form.password_confirmation {
            let mut errors = std::collections::HashMap::new();
            errors.insert("password".to_string(), "Konfirmasi password tidak cocok.".to_string());
            
            return (token, Redirect::to("/auth/register")
                .with_errors(errors)
                .with_input(json!({"name": form.name, "email": form.email}))
                .send(&session).await).into_response();
        }

        // 3. Proses Simpan
        let hashed_password = crate::core::auth::hash::make(&form.password);
        let db = state.db.as_ref().expect("Database terputus").pool.clone();
        
        let result = sqlx::query("INSERT INTO users (name, email, password, role, created_at, updated_at) VALUES (?, ?, ?, ?, NOW(), NOW())")
            .bind(&form.name).bind(&form.email).bind(&hashed_password).bind("user")
            .execute(&db).await;

        match result {
            Ok(_) => (token, Redirect::to("/auth/login").with_success("Registrasi Berhasil! Silakan Login.").send(&session).await).into_response(),
            Err(_) => (token, Redirect::to("/auth/register")
                .with_input(json!({"name": form.name, "email": form.email}))
                .with_error("Registrasi Gagal: Email mungkin sudah terdaftar.").send(&session).await).into_response()
        }
    }

    /// POST /auth/login — Proses login
    pub async fn login(
        State(state): State<AppState>,
        token: CsrfToken,
        session: tower_sessions::Session,
        ValidatedForm(form): ValidatedForm<LoginForm>,
    ) -> impl IntoResponse {
        // 1. Validasi CSRF
        if let Err(_) = token.verify(&form.csrf_token) {
             return (token, Redirect::to("/auth/login").with_error("Invalid CSRF Token").send(&session).await).into_response();
        }

        // 2. Autentikasi
        let db = state.db.as_ref().expect("Database terputus").pool.clone();
        let user: Option<crate::app::models::user::User> = sqlx::query_as("SELECT id, name, email, password, role FROM users WHERE email = ? AND deleted_at IS NULL")
            .bind(&form.email).fetch_optional(&db).await.unwrap_or_default();

        if let Some(u) = user {
            if crate::core::auth::hash::check(&form.password, &u.password) {
                let auth_user = crate::core::auth::AuthUser::new(u.id, u.email, u.role, 24);
                let _ = session.insert("jwt", crate::http::auth::generate_token(&auth_user).unwrap()).await;
                let _ = session.insert("user", auth_user).await;
                
                return (token, Redirect::to("/dashboard").with_success("Selamat Datang kembali!").send(&session).await).into_response();
            }
        }
        
        (token, Redirect::to("/auth/login").with_input(json!({"email": form.email})).with_error("Email atau Password salah.").send(&session).await).into_response()
    }

    /// GET /auth/logout — Hapus sesi login
    pub async fn logout(token: CsrfToken, session: tower_sessions::Session) -> impl IntoResponse {
        session.clear().await;
        (token, Redirect::to("/auth/login").with_success("Anda telah berhasil logout.").send(&session).await)
    }
}
