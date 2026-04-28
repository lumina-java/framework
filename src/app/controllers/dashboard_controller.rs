use axum::{
    extract::{State, Query},
    http::HeaderMap,
    response::{Html, IntoResponse, Redirect},
};
use serde::Deserialize;
use std::sync::Arc;
use crate::core::application::AppState;

#[derive(Deserialize)]
pub struct DashboardQuery {
    success: Option<String>,
}

pub struct DashboardController;

impl DashboardController {
    /// GET /dashboard — Tampilkan dashboard untuk user yang sudah login
    pub async fn index(
        Query(query): Query<DashboardQuery>,
        headers: HeaderMap,
        State(state): State<Arc<AppState>>,
    ) -> impl IntoResponse {
        // Ekstrak token dari header Cookie
        let mut token_opt = None;
        if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
            if let Ok(cookie_str) = cookie_header.to_str() {
                for cookie in cookie_str.split(';') {
                    let cookie = cookie.trim();
                    if cookie.starts_with("token=") {
                        token_opt = Some(cookie["token=".len()..].to_string());
                    }
                }
            }
        }

        let token = match token_opt {
            Some(t) => t,
            None => {
                return Redirect::to("/auth/login?error=Silakan login terlebih dahulu").into_response()
            }
        };

        // Validasi token
        let claims = match crate::http::auth::validate_token(&token) {
            Ok(c) => c,
            Err(_) => {
                return Redirect::to("/auth/login?error=Session expired. Silakan login kembali.")
                    .into_response()
            }
        };

        // Render dashboard
        let mut context = tera::Context::new();
        context.insert("email", &claims.email);
        context.insert("role", &claims.role);
        context.insert("user_id", &claims.sub);
        
        if let Some(msg) = query.success {
            context.insert("success_msg", &msg);
        }

        let rendered = state.view.render("dashboard.html", &context);
        Html(rendered).into_response()
    }
}
