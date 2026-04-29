use axum::{Router as AxumRouter, routing, middleware::from_fn, extract::State};
use crate::core::router::Router;
use crate::app::controllers::{
    api_controller::ApiController,
    product_controller::ProductController,
    storage_controller::StorageController,
};
use crate::http::middleware::auth_required;
use crate::core::application::AppState;

/// Registrasi semua API Routes (JSON responses).
pub fn register(config: &crate::core::config::ConfigManager) -> Router<AppState> {
    // ── Public routes — tidak perlu JWT ──────────────────────────────────────
    let public = AxumRouter::new()
        // .route("/auth/login",    routing::post(AuthController::api_login))
        // .route("/auth/register", routing::post(AuthController::api_register))
        .route("/products",     routing::get(ProductController::index))
        .route("/products/:id", routing::get(ProductController::show))
        .route("/storage/test-upload", routing::post(StorageController::test_upload))
        .route("/test-queue", routing::get(|State(state): State<AppState>| async move {
            let job = crate::app::jobs::test_job::TestJob::new("Halo dari Antrean!");
            match state.queue.dispatch(job).await {
                Ok(_) => crate::core::response::ApiResponse::success(serde_json::json!({
                    "message": "Job dikirim ke antrean"
                })),
                Err(e) => crate::core::response::ApiResponse::error(&e),
            }
        }));

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
                config.get_int("UPLOAD_MAX_SIZE_MB", 2) as usize * 1024 * 1024
            )),
    )
}
