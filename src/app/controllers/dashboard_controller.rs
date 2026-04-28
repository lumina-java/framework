use axum::{
    extract::State,
    response::{Html, IntoResponse},
};
use std::sync::Arc;
use crate::core::application::AppState;
use crate::core::auth::Claims;
use serde_json::json;
use tera::Context;

pub struct DashboardController;

impl DashboardController {
    /// GET /dashboard
    /// Route ini dilindungi oleh middleware web_auth
    pub async fn index(
        State(state): State<Arc<AppState>>,
        session: tower_sessions::Session,
        // Claims akan disuntikkan oleh middleware web_auth melalui request extension
        axum::Extension(user): axum::Extension<Claims>,
    ) -> impl IntoResponse {
        let mut context = Context::new();
        context.insert("user", &json!({
            "email": user.email,
            "role": user.role,
            "sub": user.sub
        }));
        context.insert("title", "Dashboard — Lumina");

        let rendered = state.view.render_with_session("dashboard/index.html", context, &session).await;
        Html(rendered)
    }
}
