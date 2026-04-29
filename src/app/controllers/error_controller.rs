use axum::{
    extract::State,
    response::{Html, IntoResponse, Json},
    http::{StatusCode, header},
};
use std::sync::Arc;
use tera::Context;
use serde_json::json;
use crate::core::application::AppState;

/// ErrorController — menangani semua halaman error secara terpusat.
///
/// # Halaman yang ditangani
/// - `404` Not Found       → `errors/404.html` atau JSON
/// - `500` Internal Error  → `errors/500.html` atau JSON
/// - `503` DB Unavailable  → `errors/whoops.html` atau JSON
pub struct ErrorController;

impl ErrorController {
    /// Handler untuk 404 Not Found.
    /// Dipasang sebagai `fallback()` pada router utama.
    pub async fn not_found(
        State(state): State<AppState>,
        req: axum::extract::Request,
    ) -> impl IntoResponse {
        let path = req.uri().path().to_string();
        let method = req.method().to_string();

        if Self::is_api_request(&req) {
            return (
                StatusCode::NOT_FOUND,
                Json(json!({
                    "success": false,
                    "error": "NOT_FOUND",
                    "message": format!("Route {} [{}] tidak ditemukan", path, method),
                    "path": path
                }))
            ).into_response();
        }

        let mut ctx = Context::new();
        ctx.insert("path", &path);
        ctx.insert("method", &method);
        ctx.insert("title", "Halaman Tidak Ditemukan");

        let html = state.view.render("errors/404.html", &ctx);
        (StatusCode::NOT_FOUND, Html(html)).into_response()
    }

    /// Handler untuk 500 Internal Server Error.
    /// Dapat dipanggil dari handler manapun saat terjadi panic/error tak terduga.
    pub async fn server_error(
        state: Arc<AppState>,
        path: String,
        error_message: String,
        is_api: bool,
    ) -> impl IntoResponse {
        let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

        if is_api {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "success": false,
                    "error": "INTERNAL_SERVER_ERROR",
                    "message": error_message,
                    "path": path,
                    "timestamp": timestamp
                }))
            ).into_response();
        }

        let mut ctx = Context::new();
        ctx.insert("path", &path);
        ctx.insert("error_message", &error_message);
        ctx.insert("timestamp", &timestamp);
        ctx.insert("error_code", "INTERNAL_SERVER_ERROR");

        let html = state.view.render("errors/500.html", &ctx);
        (StatusCode::INTERNAL_SERVER_ERROR, Html(html)).into_response()
    }

    /// Helper untuk mengecek apakah request ditujukan ke API atau Web.
    pub fn is_api_request(req: &axum::extract::Request) -> bool {
        let path = req.uri().path();
        
        // Cek prefix path /api
        if path.starts_with("/api") {
            return true;
        }

        // Cek header Accept
        if let Some(accept) = req.headers().get(header::ACCEPT) {
            if let Ok(accept_str) = accept.to_str() {
                if accept_str.contains("application/json") {
                    return true;
                }
            }
        }

        false
    }

    /// Render halaman "Whoops" untuk database error (General).
    pub fn render_db_error(state: &AppState, error_msg: &str, is_api: bool) -> impl IntoResponse {
        if is_api {
            return (
                StatusCode::SERVICE_UNAVAILABLE,
                Json(json!({
                    "success": false,
                    "error": "DATABASE_ERROR",
                    "message": "Database tidak tersedia",
                    "detail": error_msg
                }))
            ).into_response();
        }

        let mut ctx = Context::new();
        ctx.insert("db_error", error_msg);

        let html = state.view.render("errors/whoops.html", &ctx);
        (StatusCode::SERVICE_UNAVAILABLE, Html(html)).into_response()
    }
}
