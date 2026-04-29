use axum::{
    extract::State,
    response::{Html, IntoResponse},
};
use crate::core::application::AppState;
use crate::core::auth::AuthUser;
use serde_json::json;
use tera::Context;

pub struct DashboardController;

impl DashboardController {
    /// GET /dashboard
    /// Route ini dilindungi oleh middleware web_auth.
    /// AuthUser disuntikkan secara otomatis (Dependency Injection) melalui Extractor.
    pub async fn index(
        State(state): State<AppState>,
        session: tower_sessions::Session,
        user: AuthUser,
    ) -> impl IntoResponse {
        let mut context = Context::new();
        context.insert("user", &json!({
            "email": user.email,
            "role": user.role,
            "sub": user.sub
        }));
        context.insert("user_id", &user.sub);
        context.insert("email", &user.email);
        context.insert("role", &user.role);
        context.insert("title", "Dashboard — Lumina");

        let rendered = state.view.render_with_session("dashboard/index.html", context, &session).await;
        Html(rendered)
    }
}
