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

        // Helper untuk mempermudah pengisian sesi old dan error
        let save_old_data = || async {
            let _ = session.insert("_old", json!({
                "name": form.name,
                "email": form.email,
            })).await;
        };

        // 1. Validasi CSRF
        if let Err(_) = token.verify(&form.csrf_token) {
             flash.error("Invalid CSRF Token").await;
             return (token, Redirect::to("/auth/register")).into_response();
        }

        // 2. Validasi Form Standard
        if let Err(e) = form.validate() {
            let _ = session.insert("_errors", e.to_map()).await;
            save_old_data().await;
            flash.error("Validasi gagal. Mohon periksa kembali form Anda.").await;
            return (token, Redirect::to("/auth/register")).into_response();
        }

        // 3. Validasi Tambahan: Password Confirmation
        if form.password != form.password_confirmation {
            let mut errors = std::collections::HashMap::new();
            errors.insert("password".to_string(), "Konfirmasi password tidak cocok.".to_string());
            
            let _ = session.insert("_errors", errors).await;
            save_old_data().await;
            
            flash.error("Konfirmasi password tidak cocok.").await;
            return (token, Redirect::to("/auth/register")).into_response();
        }

        // 4. Proses Simpan ke Database
        let hashed_password = crate::core::auth::hash::make(&form.password);
        let db = state.db.as_ref().expect("Database terputus").pool.clone();
        
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
            Err(_) => {
                // Biasanya error duplicate entry (email sudah dipakai)
                flash.error("Registrasi Gagal: Email mungkin sudah terdaftar.").await;
                save_old_data().await;
                // dd!(e); // <- Contoh penggunaan dd!() saat error database!
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

        let save_old_email = || async {
            let _ = session.insert("_old", json!({"email": form.email})).await;
        };

        // 1. Validasi CSRF
        if let Err(_) = token.verify(&form.csrf_token) {
             flash.error("Invalid CSRF Token").await;
             return (token, Redirect::to("/auth/login")).into_response();
        }

        // 2. Validasi Struktur Form
        if let Err(e) = form.validate() {
            let _ = session.insert("_errors", e.to_map()).await;
            save_old_email().await;
            flash.error("Format email tidak valid.").await;
            return (token, Redirect::to("/auth/login")).into_response();
        }

        // 3. Autentikasi Pengguna
        let db = state.db.as_ref().expect("Database terputus").pool.clone();
        
        let user: Option<crate::app::models::user::User> = sqlx::query_as(
            "SELECT id, name, email, password, role FROM users WHERE email = ? AND deleted_at IS NULL"
        )
        .bind(&form.email)
        .fetch_optional(&db)
        .await
        .unwrap_or_default(); // Mengembalikan None jika error

        if let Some(u) = user {
            // 4. Verifikasi Password
            if crate::core::auth::hash::check(&form.password, &u.password) {
                // Berhasil Login -> Buat Sesi & JWT
                let auth_user = crate::core::auth::AuthUser::new(u.id, u.email, u.role, 24);
                let token_str = crate::http::auth::generate_token(&auth_user).unwrap();
                
                let _ = session.insert("jwt", token_str).await;
                let _ = session.insert("user", auth_user).await;
                
                flash.success("Selamat Datang kembali!").await;
                return (token, Redirect::to("/dashboard")).into_response();
            }
        }
        
        // Gagal Login (User tidak ada atau Password salah)
        flash.error("Email atau Password salah.").await;
        save_old_email().await;
        (token, Redirect::to("/auth/login")).into_response()
    }

    /// GET /auth/logout — Hapus sesi login
    pub async fn logout(token: CsrfToken, session: tower_sessions::Session) -> impl IntoResponse {
        let flash = crate::core::session::FlashManager::new(&session);
        flash.success("Anda telah berhasil logout.").await;
        session.clear().await;
        (token, Redirect::to("/auth/login"))
    }
}
