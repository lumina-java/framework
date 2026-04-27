use axum::{Router as AxumRouter, routing, middleware::from_fn};
use crate::core::router::Router;
use crate::app::controllers::{
    api_controller::ApiController,
    auth_controller::AuthController,
};
use crate::http::middleware::auth_required;
use crate::core::application::AppState;
use std::sync::Arc;

/// Registrasi semua API Routes (JSON responses).
pub fn register() -> Router<Arc<AppState>> {
    // ── Public routes — tidak perlu JWT ──────────────────────────────────────
    let public = AxumRouter::new()
        .route("/auth/login",    routing::post(AuthController::login))
        .route("/auth/register", routing::post(AuthController::register));

    // ── Protected routes — wajib Bearer token ───────────────────────────────
    // .route_layer() menerapkan middleware hanya ke route-route di dalam group ini
    let protected = AxumRouter::new()
        .route("/auth/me",   routing::get(AuthController::me))
        .route("/users",     routing::get(ApiController::index))
        .route("/users/:id", routing::get(ApiController::show))
        .route("/users",     routing::post(ApiController::store))
        .route_layer(from_fn(auth_required));

    // Gabungkan public + protected, bungkus kembali dalam Lumina Router
    Router::from_axum(
        AxumRouter::new()
            .merge(public)
            .merge(protected),
    )
}
