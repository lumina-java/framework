use axum::{extract::Path, Json};
use serde_json::{json, Value};

pub struct ApiController;

impl ApiController {
    /// GET /api/users — Kembalikan daftar semua user sebagai JSON
    pub async fn index() -> Json<Value> {
        Json(json!({
            "success": true,
            "message": "Users retrieved successfully",
            "data": [
                { "id": 1, "name": "Slamet Sugandi", "email": "slamet@lumina.rs", "role": "admin" },
                { "id": 2, "name": "Alice Rust",     "email": "alice@lumina.rs",  "role": "developer" },
                { "id": 3, "name": "Bob Axum",       "email": "bob@lumina.rs",    "role": "designer" }
            ],
            "meta": {
                "total": 3,
                "framework": "Lumina",
                "version": "0.1.0"
            }
        }))
    }

    /// GET /api/users/:id — Kembalikan detail user berdasarkan path parameter
    pub async fn show(Path(id): Path<u32>) -> Json<Value> {
        let (name, email, role) = match id {
            1 => ("Slamet Sugandi", "slamet@lumina.rs", "admin"),
            2 => ("Alice Rust",    "alice@lumina.rs",   "developer"),
            3 => ("Bob Axum",      "bob@lumina.rs",     "designer"),
            _ => ("Unknown",       "unknown@lumina.rs", "guest"),
        };

        Json(json!({
            "success": true,
            "message": format!("User #{id} retrieved"),
            "data": {
                "id":    id,
                "name":  name,
                "email": email,
                "role":  role
            }
        }))
    }

    /// POST /api/users — Simulasi pembuatan user baru dengan validasi
    pub async fn store(
        crate::core::validation::ValidatedJson(payload): crate::core::validation::ValidatedJson<CreateUserRequest>
    ) -> Json<Value> {
        Json(json!({
            "success": true,
            "message": "User created successfully",
            "data": payload
        }))
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
