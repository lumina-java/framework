use axum::{Router as AxumRouter, routing, middleware::from_fn};
use crate::core::router::Router;
use crate::app::controllers::{
    api_controller::ApiController,
    product_controller::ProductController,
    storage_controller::StorageController,
};
use crate::http::middleware::auth_required;
use crate::core::application::AppState;

/// Registrasi semua API Routes (JSON responses).
pub fn register() -> Router<AppState> {
    // ── Public routes — tidak perlu JWT ──────────────────────────────────────
    let public = AxumRouter::new()
        // .route("/auth/login",    routing::post(AuthController::api_login))
        // .route("/auth/register", routing::post(AuthController::api_register))
        .route("/products",     routing::get(ProductController::index))
        .route("/products/:id", routing::get(ProductController::show))
        .route("/storage/test-upload", routing::post(StorageController::test_upload));

    // ── Protected routes — wajib Bearer token ───────────────────────────────
    // .route_layer() menerapkan middleware hanya ke route-route di dalam group ini
    let protected = AxumRouter::new()
        // .route("/auth/me",   routing::get(AuthController::me))
        .route("/users",     routing::get(ApiController::index))
        .route("/users/:id", routing::get(ApiController::show))
        .route("/users",     routing::post(ApiController::store))
        .route("/invoices",  routing::post(ApiController::store_invoice))
        .route_layer(from_fn(auth_required));

    // Gabungkan public + protected, bungkus kembali dalam Lumina Router
    Router::from_axum(
        AxumRouter::new()
            .merge(public)
            .merge(protected)
            .layer(axum::extract::DefaultBodyLimit::max(
                crate::support::env::env("UPLOAD_MAX_SIZE_MB", "2")
                    .parse::<usize>()
                    .unwrap_or(2) * 1024 * 1024
            )),
    )
}
