use axum::{Router as AxumRouter, routing, middleware::from_fn, extract::State};
use crate::core::router::Router;
use crate::app::controllers::api::{
    api_controller::ApiController,
    storage_controller::StorageController,
};
use crate::http::middleware::auth_required;
use crate::core::application::AppState;

/// Registrasi semua API Routes (JSON responses).
pub fn register(config: &crate::core::config::ConfigManager) -> Router<AppState> {
    // ── Public routes — tidak perlu JWT ──────────────────────────────────────
    let public = AxumRouter::new()
        .route("/storage/test-upload", routing::post(StorageController::test_upload))
        .route("/test-queue", routing::get(|State(state): State<AppState>| async move {
            let data = serde_json::json!({ "message": "Halo dari Antrean!" });
            match state.queue.dispatch("test_job", data).await {
                Ok(_) => crate::core::response::ApiResponse::success(serde_json::json!({
                    "message": "Job dikirim ke antrean"
                })),
                Err(e) => crate::core::response::ApiResponse::error(&e),
            }
        }))
        .route("/test-cache", routing::get(|State(state): State<AppState>| async move {
            let key = "test_key";
            if let Some(val) = state.cache.get::<String>(key).await {
                return crate::core::response::ApiResponse::success(serde_json::json!({
                    "status": "Cache Hit",
                    "data": val
                }));
            }
            let new_data = "Lumina Cache Berhasil! 🧊".to_string();
            state.cache.put(key, new_data.clone(), 10).await;
            crate::core::response::ApiResponse::success(serde_json::json!({
                "status": "Cache Miss (Data set for 10s)",
                "data": new_data
            }))
        }));

    // ── Protected routes — wajib Bearer token ───────────────────────────────
    let protected = AxumRouter::new()
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
