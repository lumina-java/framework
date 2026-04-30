use axum::{extract::State, response::Html};
use crate::core::application::AppState;
use tera::Context;

pub struct HomeController;

impl HomeController {
    /// GET /
    pub async fn index(
        State(state): State<AppState>,
        session: tower_sessions::Session,
    ) -> Html<String> {
        let context = Context::new();
        let rendered = state.view.render_with_session("home/index.html", context, &session).await;
        Html(rendered)
    }

    /// GET /about
    pub async fn about(State(_state): State<AppState>) -> Html<String> {
        let mut context = Context::new();
        context.insert("title", "About Lumina");
        
        // Kita belum buat about.html, jadi sementara bisa gunakan index atau buat baru
        // let rendered = state.view.render("home/about.html", &context);
        Html("<h1>About Page</h1><p>Rendering via Template Engine coming soon...</p>".to_string())
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
