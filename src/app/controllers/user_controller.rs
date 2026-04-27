use axum::{extract::{Path, State}, response::Html};
use std::sync::Arc;
use crate::core::application::AppState;
use tera::Context;
use serde_json::json;

pub struct UserController;

impl UserController {
    /// GET /users — Tampilkan daftar semua user (HTML)
    pub async fn index(State(state): State<Arc<AppState>>) -> Html<String> {
        let mut context = Context::new();
        
        // Data dummy untuk demo template loop
        let users = vec![
            json!({"id": 1, "name": "Slamet Sugandi", "email": "slamet@lumina.rs"}),
            json!({"id": 2, "name": "Alice Rust", "email": "alice@lumina.rs"}),
            json!({"id": 3, "name": "Bob Axum", "email": "bob@lumina.rs"}),
        ];
        
        context.insert("users", &users);
        
        let rendered = state.view.render("users/index.html", &context);
        Html(rendered)
    }

    /// GET /users/:id — Tampilkan detail user berdasarkan ID
    pub async fn show(State(state): State<Arc<AppState>>, Path(id): Path<u32>) -> Html<String> {
        let mut context = Context::new();
        
        let (name, email) = match id {
            1 => ("Slamet Sugandi", "slamet@lumina.rs"),
            2 => ("Alice Rust", "alice@lumina.rs"),
            3 => ("Bob Axum", "bob@lumina.rs"),
            _ => ("Unknown", "unknown@lumina.rs"),
        };

        context.insert("id", &id);
        context.insert("name", name);
        context.insert("email", email);
        
        let rendered = state.view.render("users/show.html", &context);
        Html(rendered)
    }
}
