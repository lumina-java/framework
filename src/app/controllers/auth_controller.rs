use axum::{
    extract::State,
    response::{Html, IntoResponse, Redirect},
};
use serde_json::json;
use crate::core::application::AppState;
use crate::core::validation::Validatable;
use serde::Deserialize;
use validator::Validate;
use axum_csrf::CsrfToken;

#[derive(Deserialize, Validate)]
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

#[derive(Deserialize, Validate)]
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

        let mut context = tera::Context::new();
        context.insert("csrf_token", &token.authenticity_token().unwrap());

        let rendered = state.view.render_with_session("auth/login.html", context, &session).await;
        (token, Html(rendered)).into_response()
    }

    /// GET /auth/register — Tampilkan halaman registrasi (HTML)
    pub async fn show_register(
        State(state): State<AppState>,
        token: CsrfToken,
        session: tower_sessions::Session,
    ) -> impl IntoResponse {
        let mut context = tera::Context::new();
        context.insert("csrf_token", &token.authenticity_token().unwrap());

        let rendered = state.view.render_with_session("auth/register.html", context, &session).await;
        (token, Html(rendered)).into_response()
    }

    /// POST /auth/register — Proses pendaftaran user baru
    pub async fn register(
        State(state): State<AppState>,
        token: CsrfToken,
        session: tower_sessions::Session,
        axum::Form(form): axum::Form<RegisterForm>,
    ) -> impl IntoResponse {
        let flash = crate::core::session::FlashManager::new(&session);

        // 1. Validasi CSRF
        if let Err(_) = token.verify(&form.csrf_token) {
             flash.error("Invalid CSRF Token").await;
             return (token, Redirect::to("/auth/register")).into_response();
        }

        // 2. Validasi Form
        if let Err(e) = form.validate() {
            let errors = e.to_map();
            session.insert("_errors", errors).await.unwrap();
            session.insert("_old", json!({
                "name": form.name,
                "email": form.email,
            })).await.unwrap();
            
            flash.error("Validasi gagal. Mohon periksa kembali form Anda.").await;
            return (token, Redirect::to("/auth/login")).into_response();
        }

        // 2.1 Validasi Password Confirmation
        if form.password != form.password_confirmation {
            let mut errors = std::collections::HashMap::new();
            errors.insert("password".to_string(), "Konfirmasi password tidak cocok.".to_string());
            session.insert("_errors", errors).await.unwrap();
            session.insert("_old", json!({
                "name": form.name,
                "email": form.email,
            })).await.unwrap();
            flash.error("Konfirmasi password tidak cocok.").await;
            return (token, Redirect::to("/auth/register")).into_response();
        }

        // 3. Proses Simpan
        let hashed_password = crate::core::auth::hash::make(&form.password);
        let db = state.db.as_ref().expect("db_guard should prevent this").pool.clone();
        
        let result = sqlx::query(
            "INSERT INTO users (name, email, password, role, created_at, updated_at) VALUES (?, ?, ?, ?, NOW(), NOW())"
        )
        .bind(&form.name)
        .bind(&form.email)
        .bind(&hashed_password)
        .bind("user")
        .execute(&db)
        .await;

        match result {
            Ok(_) => {
                flash.success("Registrasi Berhasil! Silakan Login.").await;
                (token, Redirect::to("/auth/login")).into_response()
            },
            Err(e) => {
                flash.error(&format!("Registrasi Gagal: {}", e)).await;
                (token, Redirect::to("/auth/register")).into_response()
            }
        }
    }

    /// POST /auth/login — Proses login
    pub async fn login(
        State(state): State<AppState>,
        token: CsrfToken,
        session: tower_sessions::Session,
        axum::Form(form): axum::Form<LoginForm>,
    ) -> impl IntoResponse {
        let flash = crate::core::session::FlashManager::new(&session);

        // 1. Validasi CSRF
        if let Err(_) = token.verify(&form.csrf_token) {
             flash.error("Invalid CSRF Token").await;
             return (token, Redirect::to("/auth/login")).into_response();
        }

        // 2. Validasi Form
        if let Err(e) = form.validate() {
            session.insert("_errors", e.to_map()).await.unwrap();
            session.insert("_old", json!({"email": form.email})).await.unwrap();
            flash.error("Format input tidak valid.").await;
            return (token, Redirect::to("/auth/login")).into_response();
        }

        // 3. Cari User
        let db = state.db.as_ref().expect("db_guard should prevent this").pool.clone();
        tracing::info!("Login attempt for email: [{}]", form.email);
        
        let user: Option<crate::app::models::user::User> = sqlx::query_as(
            "SELECT id, name, email, password, role FROM users WHERE email = ? AND deleted_at IS NULL"
        )
            .bind(&form.email)
            .fetch_optional(&db)
            .await
            .unwrap_or_else(|e| {
                tracing::error!("Database error during login: {:?}", e);
                None
            });

        tracing::info!("User found: {:?}", user.is_some());

        if let Some(u) = user {
            if crate::core::auth::hash::check(&form.password, &u.password) {
                // Generate JWT
                let auth_user = crate::core::auth::AuthUser::new(u.id, u.email, u.role, 24);
                let token_str = crate::http::auth::generate_token(&auth_user).unwrap();
                
                // Simpan ke Session
                session.insert("jwt", token_str).await.unwrap();
                session.insert("user", auth_user).await.unwrap();
                
                flash.success("Selamat Datang!").await;
                (token, Redirect::to("/dashboard")).into_response()
            } else {
                flash.error("Email atau Password salah.").await;
                (token, Redirect::to("/auth/login")).into_response()
            }
        } else {
            flash.error("User tidak ditemukan.").await;
            (token, Redirect::to("/auth/login")).into_response()
        }
    }

    /// GET /auth/logout — Hapus sesi login
    pub async fn logout(token: CsrfToken, session: tower_sessions::Session) -> impl IntoResponse {
        let flash = crate::core::session::FlashManager::new(&session);
        flash.success("Berhasil logout.").await;
        session.clear().await;
        (token, Redirect::to("/auth/login"))
    }
}
