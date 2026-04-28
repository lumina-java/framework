use axum::{
    extract::State,
    response::{Html, IntoResponse},
};
use std::sync::Arc;
use crate::core::application::AppState;
use crate::core::auth::Claims;
use tera::Context;

pub struct DashboardController;

impl DashboardController {
    /// GET /dashboard
    /// Route ini dilindungi oleh middleware web_auth
    pub async fn index(
        State(state): State<Arc<AppState>>,
        // Claims akan disuntikkan oleh middleware web_auth melalui request extension
        axum::Extension(user): axum::Extension<Claims>,
    ) -> impl IntoResponse {
        let mut context = Context::new();
        context.insert("user", &user);
        context.insert("title", "Dashboard — Lumina");

        let rendered = state.view.render("dashboard/index.html", &context);
        Html(rendered)
    }
}
