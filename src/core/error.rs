use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Data tidak ditemukan: {0}")]
    NotFound(String),

    #[error("Terjadi kesalahan database: {0}")]
    DatabaseError(#[from] sqlx::Error),

    #[error("Validasi gagal: {0}")]
    ValidationError(String),

    #[error("Bad Request: {0}")]
    BadRequest(String),

    #[error("{0}")]
    Generic(String),

    #[error("Akses tidak diizinkan")]
    Unauthorized,

    #[error("Kesalahan internal server")]
    InternalServerError,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, code, message) = match self {
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, "404", msg),
            AppError::DatabaseError(e) => {
                // Sembunyikan detail teknis di log, tampilkan pesan umum di response
                tracing::error!("Database Error: {:?}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "500",
                    "Masalah koneksi database".to_string(),
                )
            }
            AppError::ValidationError(msg) => (StatusCode::BAD_REQUEST, "422", msg.clone()),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, "400", msg.clone()),
            AppError::Generic(msg) => (StatusCode::INTERNAL_SERVER_ERROR, "500", msg.clone()),
            AppError::Unauthorized => (
                StatusCode::UNAUTHORIZED,
                "401",
                "Anda harus login terlebih dahulu".to_string(),
            ),
            AppError::InternalServerError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "500",
                "Kesalahan internal server".to_string(),
            ),
        };

        let body = Json(json!({
            "metaData": {
                "code": code,
                "message": message
            },
            "response": null
        }));

        (status, body).into_response()
    }
}

impl From<String> for AppError {
    fn from(error: String) -> Self {
        AppError::Generic(error)
    }
}
