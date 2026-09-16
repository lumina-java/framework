use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde_json::json;
use crate::core::application::AppState;

/// Health check handler (`/up` or `/health`)
pub async fn health_check_handler(State(state): State<AppState>) -> impl IntoResponse {
    let db_healthy = if let Some(pool) = &state.db {
        sqlx::query("SELECT 1").execute(&pool.pool).await.is_ok()
    } else {
        false
    };

    let status_code = if db_healthy || state.db.is_none() {
        StatusCode::OK
    } else {
        StatusCode::SERVICE_UNAVAILABLE
    };

    let payload = json!({
        "status": if db_healthy || state.db.is_none() { "ok" } else { "unhealthy" },
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "database": if state.db.is_some() {
            if db_healthy { "connected" } else { "disconnected" }
        } else {
            "disabled"
        },
        "version": env!("CARGO_PKG_VERSION"),
    });

    (status_code, Json(payload))
}
