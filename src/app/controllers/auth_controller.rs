use axum::response::IntoResponse;
use crate::core::response::Redirect;
use crate::core::view::View;
use serde_json::json;
use crate::core::auth::Auth;
use crate::app::models::user::User;
use crate::core::validation::ValidatedForm;
use crate::core::request::Request;
use lumina_macros::lumina_form;

#[lumina_form]
pub struct RegisterForm {
    #[rule("required|min:3")]
    pub name: String,
    
    #[rule("required|email")]
    pub email: String,
    
    #[rule("required|min:6")]
    pub password: String,

    pub password_confirmation: String,
    pub csrf_token: String,
}

#[lumina_form]
pub struct LoginForm {
    #[rule("required|email")]
    pub email: String,
    
    #[rule("required")]
    pub password: String,
    
    pub csrf_token: String,
}

pub struct AuthController;

impl AuthController {
    /// GET /auth/login — Tampilkan halaman login (HTML)
    pub async fn show_login(req: Request) -> impl IntoResponse {
        // Redirect jika sudah login
        if req.user.is_some() {
            return Redirect::to("/dashboard").go(&req).await
        }

        View::make("auth.login")
            .with("csrf_token", req.token.authenticity_token().unwrap())
            .render(&req)
            .await
            .into_response(req.token)
    }

    /// GET /auth/register — Tampilkan halaman registrasi (HTML)
    pub async fn show_register(req: Request) -> impl IntoResponse {
        View::make("auth.register")
            .with("csrf_token", req.token.authenticity_token().unwrap())
            .render(&req)
            .await
            .into_response(req.token)
    }

    /// POST /auth/register — Proses pendaftaran user baru
    pub async fn register(
        req: Request,
        ValidatedForm(form): ValidatedForm<RegisterForm>,
    ) -> impl IntoResponse {
        // 1. Validasi CSRF
        if req.token.verify(&form.csrf_token).is_err() {
             return Redirect::to("/auth/register").with_error("Invalid CSRF Token").go(&req).await
        }

        // 2. Validasi Password Confirmation
        if form.password != form.password_confirmation {
            let mut errors = std::collections::HashMap::new();
            errors.insert("password".to_string(), "Konfirmasi password tidak cocok.".to_string());
            
            return Redirect::to("/auth/register")
                .with_errors(errors)
                .with_input(json!({"name": form.name, "email": form.email}))
                .go(&req).await
        }

        // 3. Proses Simpan
        let hashed_password = crate::core::auth::hash::make(&form.password);
        
        let result = sqlx::query("INSERT INTO users (name, email, password, role, created_at, updated_at) VALUES (?, ?, ?, ?, NOW(), NOW())")
            .bind(&form.name).bind(&form.email).bind(&hashed_password).bind("user")
            .execute(&req.state.db().pool).await;

        match result {
            Ok(_) => Redirect::to("/auth/login").with_success("Registrasi Berhasil! Silakan Login.").go(&req).await,
            Err(_) => Redirect::to("/auth/register")
                .with_input(json!({"name": form.name, "email": form.email}))
                .with_error("Registrasi Gagal: Email mungkin sudah terdaftar.")
                .go(&req).await
        }
    }

    /// POST /auth/login — Proses login
    pub async fn login(
        req: Request,
        ValidatedForm(form): ValidatedForm<LoginForm>,
    ) -> impl IntoResponse {
        // 1. Validasi CSRF
        if req.token.verify(&form.csrf_token).is_err() {
             return Redirect::to("/auth/login").with_error("Invalid CSRF Token").go(&req).await
        }

        // 2. Autentikasi menggunakan Facade Auth
        let user = User::find_by_email(req.state.db(), &form.email).await.ok();

        if let Some(u) = user {
            if Auth::check(&form.password, &u.password) {
                let auth_user = Auth::user(u.id, u.email, u.role);
                let _ = Auth::login(&req.session, auth_user).await;
                
                return Redirect::to("/dashboard").with_success("Selamat Datang kembali!").go(&req).await
            }
        }
        
        Redirect::to("/auth/login")
            .with_input(json!({"email": form.email}))
            .with_error("Email atau Password salah.")
            .go(&req).await
    }

    /// GET /auth/logout — Hapus sesi login
    pub async fn logout(req: Request) -> impl IntoResponse {
        Auth::logout(&req.session).await;
        Redirect::to("/auth/login").with_success("Anda telah berhasil logout.").go(&req).await
    }
}
