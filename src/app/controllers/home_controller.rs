use axum::{extract::State, response::Html};
use std::sync::Arc;
use crate::core::application::AppState;
use tera::Context;

pub struct HomeController;

impl HomeController {
    /// GET /
    pub async fn index(State(state): State<Arc<AppState>>) -> Html<String> {
        let context = Context::new();
        // context.insert("name", "User");
        
        let rendered = state.view.render("home/index.html", &context);
        Html(rendered)
    }

    /// GET /about
    pub async fn about(State(_state): State<Arc<AppState>>) -> Html<String> {
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
}
