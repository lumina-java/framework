use axum::{extract::{Path, State}, response::{Html, IntoResponse}};
use crate::core::application::AppState;
use tera::Context;
use serde_json::json;

pub struct UserController;

impl UserController {
    /// GET /users — Tampilkan daftar semua user (HTML)
    pub async fn index(State(state): State<AppState>) -> Html<String> {
        let mut context = Context::new();
        
        // Data dummy untuk demo template loop
        let users = vec![
            json!({"id": 1, "name": "Slamet Sugandi", "email": "slamet@lumina.rs"}),
            json!({"id": 2, "name": "Alice Rust", "email": "alice@lumina.rs"}),
            json!({"id": 3, "name": "Bob Axum", "email": "bob@lumina.rs"}),
        ];
        
        context.insert("users", &users);
        
        let rendered = state.view.render("users/index.blade.rs", &context);
        Html(rendered)
    }

    /// GET /users/:id — Tampilkan detail user berdasarkan ID
    pub async fn show(State(state): State<AppState>, Path(id): Path<u32>) -> Html<String> {
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
        
        let rendered = state.view.render("users/show.blade.rs", &context);
        Html(rendered)
    }

    /// POST /api/users — Buat user baru dengan validasi
    pub async fn store(
        crate::core::validation::ValidatedJson(payload): crate::core::validation::ValidatedJson<CreateUserRequest>
    ) -> Result<crate::core::response::ApiResponse<serde_json::Value>, crate::core::error::AppError> {
        Ok(crate::core::response::ApiResponse::with_message(json!(payload), "User created successfully"))
    }

    /// GET /test-orm — Demonstrasi penggunaan QueryBuilder
    pub async fn test_orm(State(state): State<AppState>) -> Result<crate::core::response::ApiResponse<serde_json::Value>, crate::core::error::AppError> {
        use crate::database::model::Model;
        use crate::app::models::user::User;

        // Ambil user pertama dengan email tertentu menggunakan ORM
        let db = state.db.as_ref().ok_or_else(|| crate::core::error::AppError::InternalServerError)?;
        let user = User::query(db)
            .select("id, name, email, role, password")
            .filter("role", "=", "user")
            .order_by("id", "DESC")
            .limit(1)
            .get()
            .await?;

        Ok(crate::core::response::ApiResponse::with_message(json!(user), "ORM query executed successfully"))
    }

    /// GET /test-event — Demonstrasi penggunaan Event Dispatcher
    pub async fn test_event(State(state): State<AppState>) -> Result<crate::core::response::ApiResponse<serde_json::Value>, crate::core::error::AppError> {
        use crate::app::events::user_registered::UserRegistered;
        use std::sync::Arc;

        let event = UserRegistered {
            name: "Lumina User".to_string(),
            email: "test@lumina.rs".to_string(),
        };

        // Dispatch event secara asinkron
        state.events.dispatch(event, Arc::new(state.clone())).await;

        Ok(crate::core::response::ApiResponse::with_message(
            json!({"event": "UserRegistered", "status": "dispatched"}),
            "Event dispatched! Cek log server untuk melihat listener bekerja."
        ))
    }

    /// GET /test-i18n
    pub async fn test_i18n(
        req: crate::core::request::Request,
    ) -> impl IntoResponse {
        crate::core::view::View::make("test_i18n")
            .with("name", "Lumina Developer")
            .render(&req)
            .await
    }

    /// GET /test-echo
    pub async fn test_echo(
        req: crate::core::request::Request,
    ) -> impl IntoResponse {
        crate::core::view::View::make("test_echo")
            .render(&req)
            .await
    }

    /// POST /test-broadcast
    pub async fn test_broadcast(
        State(state): State<AppState>,
    ) -> impl IntoResponse {
        state.echo.broadcast(
            "notifications", 
            "NewNotification", 
            json!({"message": "Pesan real-time dari server! 🚀"})
        );
        axum::http::StatusCode::OK
    }
}

use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate, serde::Serialize)]
pub struct CreateUserRequest {
    #[validate(length(min = 3, message = "Nama minimal 3 karakter"))]
    pub name: String,

    #[validate(email(message = "Format email tidak valid"))]
    pub email: String,

    #[validate(length(min = 8, message = "Password minimal 8 karakter"))]
    pub password: String,
}
