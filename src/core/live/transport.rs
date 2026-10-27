use axum::{
    extract::{Json, State},
    http::header,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::core::application::AppState;
use crate::core::error::AppError;
use crate::core::live::registry::LiveRegistry;
use crate::core::live::snapshot::{LiveSnapshot, SnapshotManager};

/// Aset JavaScript mikro-runtime Lumina Live yang disematkan langsung di dalam biner.
pub const LUMINA_LIVE_JS: &str = include_str!("assets/lumina-live.js");

/// Payload permintaan pemanggilan aksi dari browser client.
#[derive(Debug, Deserialize, Serialize)]
pub struct LiveCallRequest {
    pub id: String,
    pub name: String,
    pub snapshot: String,
    pub action: String,
    #[serde(default)]
    pub params: Vec<Value>,
}

/// Payload respons yang mengembalikan snapshot termutakhir beserta patch HTML hasil render ulang.
#[derive(Debug, Serialize, Deserialize)]
pub struct LiveCallResponse {
    pub id: String,
    pub snapshot: String,
    pub html: String,
}

/// Handler HTTP untuk menyajikan berkas JavaScript runtime `/_lumina/live/live.js`.
pub async fn serve_live_js() -> impl IntoResponse {
    (
        [
            (header::CONTENT_TYPE, "application/javascript; charset=utf-8"),
            (header::CACHE_CONTROL, "public, max-age=86400, immutable"),
        ],
        LUMINA_LIVE_JS,
    )
}

/// Handler HTTP Micro-RPC `POST /_lumina/live/call`.
/// Menangani verifikasi snapshot, hidrasi state, eksekusi aksi, dan re-rendering template.
pub async fn call_action_handler(
    State(state): State<AppState>,
    Json(req): Json<LiveCallRequest>,
) -> Result<Json<LiveCallResponse>, AppError> {
    // 1. Verifikasi tanda tangan HMAC-SHA256 snapshot
    let snapshot = SnapshotManager::unseal(&req.snapshot)?;
    if snapshot.name != req.name {
        return Err(AppError::BadRequest(
            "Component name mismatch between request and snapshot".to_string(),
        ));
    }

    // 2. Ambil handler komponen dari registry
    let handler = state
        .live
        .get(&req.name)
        .or_else(|| LiveRegistry::global().get(&req.name))
        .ok_or_else(|| {
            AppError::NotFound(format!("Live Component '{}' belum terdaftar", req.name))
        })?;

    // 3. Eksekusi aksi atau pembaruan model ($set)
    let (new_state, html) = if req.action == "$set" && req.params.len() >= 2 {
        let mut state_data = snapshot.data;
        if let Some(prop_name) = req.params[0].as_str() {
            if let Some(obj) = state_data.as_object_mut() {
                obj.insert(prop_name.to_string(), req.params[1].clone());
            }
        }
        // Render ulang dengan state yang diperbarui
        handler.render_initial(state_data, state.view.blade_engine())?
    } else if req.action == "$refresh" {
        handler.render_initial(snapshot.data, state.view.blade_engine())?
    } else {
        handler
            .call(snapshot.data, &req.action, req.params, state.view.blade_engine())
            .await?
    };

    // 4. Buat snapshot baru yang ditandatangani secara kriptografis
    let new_snapshot = LiveSnapshot::new(&req.id, &req.name, new_state);
    let sealed_snapshot = SnapshotManager::seal(&new_snapshot)?;

    // 5. Bungkus dengan container data atribut Lumina Live
    let wrapped_html = format!(
        r#"<div data-kilat-id="{}" data-kilat-name="{}" data-kilat-snapshot="{}" data-live-id="{}" data-live-name="{}" data-live-snapshot="{}">{}</div>"#,
        req.id, req.name, sealed_snapshot, req.id, req.name, sealed_snapshot, html
    );

    Ok(Json(LiveCallResponse {
        id: req.id,
        snapshot: sealed_snapshot,
        html: wrapped_html,
    }))
}
