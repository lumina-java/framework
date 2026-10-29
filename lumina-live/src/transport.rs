use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        Json, State,
    },
    http::header,
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use futures::{sink::SinkExt, stream::StreamExt};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use lumina::prelude::AppState;
use lumina::core::error::AppError;
use crate::registry::LiveRegistry;
use crate::snapshot::{LiveSnapshot, SnapshotManager};

pub const LUMINA_LIVE_JS: &str = include_str!("assets/lumina-live.js");

#[derive(Debug, Deserialize, Serialize)]
pub struct LiveCallRequest {
    pub id: String,
    pub name: String,
    pub snapshot: String,
    pub action: String,
    #[serde(default)]
    pub params: Vec<Value>,
    #[serde(default)]
    pub updates: HashMap<String, Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LiveCallResponse {
    pub id: String,
    pub snapshot: String,
    pub html: String,
}

pub fn live_routes() -> Router<AppState> {
    Router::new()
        .route("/_lumina/live/live.js", get(serve_live_js))
        .route("/_lumina/live/call", post(call_action_handler))
        .route("/_lumina/live/ws", get(live_ws_handler))
}

pub async fn serve_live_js() -> impl IntoResponse {
    (
        [
            (header::CONTENT_TYPE, "application/javascript; charset=utf-8"),
            (header::CACHE_CONTROL, "public, max-age=86400, immutable"),
        ],
        LUMINA_LIVE_JS,
    )
}

pub async fn call_action_handler(
    State(state): State<AppState>,
    Json(req): Json<LiveCallRequest>,
) -> Result<Json<LiveCallResponse>, AppError> {
    let res = execute_live_call(&state, req).await?;
    Ok(Json(res))
}

pub async fn live_ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_live_socket(socket, state))
}

async fn handle_live_socket(socket: WebSocket, state: AppState) {
    let (mut sender, mut receiver) = socket.split();

    while let Some(Ok(msg)) = receiver.next().await {
        match msg {
            Message::Text(text) => {
                if let Ok(req) = serde_json::from_str::<LiveCallRequest>(&text) {
                    match execute_live_call(&state, req).await {
                        Ok(response) => {
                            if let Ok(json_str) = serde_json::to_string(&response) {
                                let _ = sender.send(Message::Text(json_str)).await;
                            }
                        }
                        Err(e) => {
                            let err_payload = serde_json::json!({
                                "error": e.to_string()
                            });
                            let _ = sender.send(Message::Text(err_payload.to_string())).await;
                        }
                    }
                }
            }
            Message::Ping(payload) => {
                let _ = sender.send(Message::Pong(payload)).await;
            }
            Message::Close(_) => {
                break;
            }
            _ => {}
        }
    }
}

pub async fn execute_live_call(
    state: &AppState,
    req: LiveCallRequest,
) -> Result<LiveCallResponse, AppError> {
    let snapshot = SnapshotManager::unseal(&req.snapshot)?;
    if snapshot.name != req.name {
        return Err(AppError::BadRequest(
            "Component name mismatch between request and snapshot".to_string(),
        ));
    }

    let handler = LiveRegistry::global().get(&req.name)
        .ok_or_else(|| {
            AppError::NotFound(format!("Live Component '{}' belum terdaftar", req.name))
        })?;

    let mut state_data = snapshot.data;

    // Terapkan update deferred model yang dikirim bersama aksi (Livewire v3 standard)
    if !req.updates.is_empty() {
        if let Some(obj) = state_data.as_object_mut() {
            for (k, v) in req.updates {
                obj.insert(k, v);
            }
        }
    }

    let (new_state, html) = if req.action == "$set" {
        if req.params.len() >= 2 {
            if let Some(prop_name) = req.params[0].as_str() {
                if let Some(obj) = state_data.as_object_mut() {
                    obj.insert(prop_name.to_string(), req.params[1].clone());
                }
            }
        }
        handler.render_initial(state_data, state.view.blade_engine())?
    } else if req.action == "$refresh" {
        handler.render_initial(state_data, state.view.blade_engine())?
    } else {
        handler
            .call(state_data, &req.action, req.params, state.view.blade_engine())
            .await?
    };

    let new_snapshot = LiveSnapshot::new(&req.id, &req.name, new_state);
    let sealed_snapshot = SnapshotManager::seal(&new_snapshot)?;

    let wrapped_html = format!(
        r#"<div data-kilat-id="{}" data-kilat-name="{}" data-kilat-snapshot="{}" data-live-id="{}" data-live-name="{}" data-live-snapshot="{}">{}</div>"#,
        req.id, req.name, sealed_snapshot, req.id, req.name, sealed_snapshot, html
    );

    Ok(LiveCallResponse {
        id: req.id,
        snapshot: sealed_snapshot,
        html: wrapped_html,
    })
}
