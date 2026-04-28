use axum::extract::Path;
use serde_json::{json, Value};
use crate::core::response::ApiResponse;
use crate::core::error::AppError;

pub struct ApiController;

impl ApiController {
    /// GET /api/users — Kembalikan daftar semua user sebagai JSON
    pub async fn index() -> Result<ApiResponse<Value>, AppError> {
        let users = json!([
            { "id": 1, "name": "Slamet Sugandi", "email": "slamet@lumina.rs", "role": "admin" },
            { "id": 2, "name": "Alice Rust",     "email": "alice@lumina.rs",  "role": "developer" },
            { "id": 3, "name": "Bob Axum",       "email": "bob@lumina.rs",    "role": "designer" }
        ]);
        
        Ok(ApiResponse::with_message(users, "Users retrieved successfully"))
    }

    /// GET /api/users/:id — Kembalikan detail user berdasarkan path parameter
    pub async fn show(Path(id): Path<u32>) -> Result<ApiResponse<Value>, AppError> {
        let (name, email, role) = match id {
            1 => ("Slamet Sugandi", "slamet@lumina.rs", "admin"),
            2 => ("Alice Rust",    "alice@lumina.rs",   "developer"),
            3 => ("Bob Axum",      "bob@lumina.rs",     "designer"),
            _ => return Err(AppError::NotFound(format!("User dengan ID {} tidak ditemukan", id))),
        };

        let user = json!({
            "id":    id,
            "name":  name,
            "email": email,
            "role":  role
        });

        Ok(ApiResponse::with_message(user, &format!("User #{} retrieved", id)))
    }

    /// POST /api/users — Simulasi pembuatan user baru dengan validasi
    pub async fn store(
        crate::core::validation::ValidatedJson(payload): crate::core::validation::ValidatedJson<CreateUserRequest>
    ) -> Result<ApiResponse<Value>, AppError> {
        Ok(ApiResponse::with_message(json!(payload), "User created successfully"))
    }

    /// POST /api/invoices — Contoh Master-Detail request (BPJS Style)
    pub async fn store_invoice(
        crate::core::validation::ValidatedJson(payload): crate::core::validation::ValidatedJson<InvoiceRequest>
    ) -> Result<crate::core::response::ApiResponse<serde_json::Value>, crate::core::error::AppError> {
        Ok(crate::core::response::ApiResponse::with_message(json!(payload), "Invoice created successfully"))
    }
}

#[derive(Debug, serde::Deserialize, validator::Validate, serde::Serialize)]
pub struct InvoiceRequest {
    #[validate(length(min = 1, message = "Nomor invoice wajib diisi"))]
    pub no_invoice: String,
    
    #[validate(length(min = 1, message = "Harus ada minimal satu item"))]
    pub items: Vec<InvoiceDetail>,
}

#[derive(Debug, serde::Deserialize, validator::Validate, serde::Serialize)]
pub struct InvoiceDetail {
    #[validate(length(min = 1, message = "Kode obat wajib diisi"))]
    pub kode_obat: String,
    pub jumlah: u32,
    pub harga: f64,
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
