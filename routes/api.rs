use axum::{Router as AxumRouter, routing, middleware::from_fn};
use crate::core::router::Router;
use crate::app::controllers::{
    api_controller::ApiController,
    auth_controller::AuthController,
};
use crate::http::middleware::auth_required;

/// Registrasi semua API Routes (JSON responses).
///
/// Dibagi dua group:
/// - **Public**   → tidak perlu token (`/auth/login`, `/auth/register`)
/// - **Protected** → wajib `Authorization: Bearer <token>` (`/auth/me`, `/users`, ...)
///
/// Semua route di sini akan diprefix `/api` oleh `Application::build_router()`.
pub fn register() -> Router {
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
