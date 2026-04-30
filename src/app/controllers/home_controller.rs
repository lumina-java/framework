use axum::{extract::State, response::{Html, IntoResponse}};
use crate::core::application::AppState;
use crate::core::view::View;

pub struct HomeController;

impl HomeController {
    /// GET /
    pub async fn index(
        State(state): State<AppState>,
        session: tower_sessions::Session,
    ) -> impl IntoResponse {
        View::make("home.index")
            .render(&state, &session)
            .await
    }

    /// GET /about
    pub async fn about(State(state): State<AppState>, session: tower_sessions::Session) -> impl IntoResponse {
        View::make("home.about")
            .with("title", "About Lumina")
            .render(&state, &session)
            .await
    }

    /// GET /debug/panic — Menghasilkan panic untuk testing Error 500
    pub async fn debug_panic() -> Html<String> {
        panic!("Sengaja dibuat panic untuk testing halaman error 500 Lumina!");
    }

    /// GET /debug/dd — Menghasilkan dump & die untuk testing debugger Lumina
    pub async fn debug_dd() -> Html<String> {
        let sample_data = serde_json::json!({
            "framework": "Lumina",
            "version": "0.1.0",
            "features": ["MVC", "Routing", "ORM", "Auth", "Debugging"],
            "server": {
                "address": "127.0.0.1",
                "port": 8000,
                "os": "Windows"
            }
        });

        crate::dd!(sample_data);
        
        #[allow(unreachable_code)]
        Html("This will never be reached".to_string())
    }
}
