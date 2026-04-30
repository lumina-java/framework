use axum::{
    extract::State,
    response::IntoResponse,
};
use crate::core::application::AppState;
use crate::core::auth::AuthUser;
use crate::core::view::View;

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
        
        View::make("dashboard.index")
            .with("user", user)
            .with("title", "Dashboard — Lumina")
            .render(&state, &session)
            .await
    }
}
