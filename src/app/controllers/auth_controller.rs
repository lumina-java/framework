use axum::response::{IntoResponse, Response};
use crate::core::view::View;
use crate::core::validation::ValidatedForm;
use crate::core::request::Request;
use crate::core::auth::Auth;
use crate::app::models::user::User;
use crate::database::model::Model;
use crate::app::requests::auth_request::{LoginRequest, RegisterRequest};

/// AuthController — Menangani pendaftaran, login, dan logout.
pub struct AuthController;

impl AuthController {
    /// Tampilkan halaman login.
    pub async fn show_login(req: Request) -> Response {
        if let Some(res) = req.redirect_if_authenticated("/dashboard").await { return res; }
        View::make("auth.login").render(&req).await.into_response()
    }

    /// Proses login.
    pub async fn login(req: Request, form: ValidatedForm<LoginRequest>) -> Response {
        let f = &form.0;
        if let Ok(Some(user)) = User::find_by_email(req.db(), &f.email).await {
            if Auth::verify(&f.password, &user.password) {
                Auth::login(&req, user).await;
                return req.redirect("/dashboard").with_success("Welcome back!").go(&req).await;
            }
        }
        req.back().with_error("Invalid email or password.").go(&req).await
    }

    /// Tampilkan halaman pendaftaran.
    pub async fn show_register(req: Request) -> Response {
        if let Some(res) = req.redirect_if_authenticated("/dashboard").await { return res; }
        View::make("auth.register").render(&req).await.into_response()
    }

    /// Proses pendaftaran user baru.
    pub async fn register(req: Request, form: ValidatedForm<RegisterRequest>) -> Response {
        let f = &form.0;
        if f.password != f.password_confirmation {
            return req.back().with_error("Passwords do not match.").go(&req).await;
        }

        if let Ok(Some(_)) = User::find_by_email(req.db(), &f.email).await {
            return req.back().with_error("Email already registered.").go(&req).await;
        }

        let user = User {
            name: f.name.clone(),
            email: f.email.clone(),
            password: Auth::make_hash(&f.password),
            role: "user".to_string(),
            ..Default::default()
        };

        if user.save(req.db()).await.is_ok() {
            return req.redirect("/auth/login").with_success("Success! Please login.").go(&req).await;
        }

        req.back().with_error("Registration failed.").go(&req).await
    }

    /// Proses logout.
    pub async fn logout(req: Request) -> Response {
        Auth::logout(&req).await;
        req.redirect("/auth/login").with_info("You have been logged out.").go(&req).await
    }
}
