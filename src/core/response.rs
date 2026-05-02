pub use axum::{
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use serde_json::json;

/// Wrapper standar untuk respon API Lumina (BPJS VClaim Style)
#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub data: T,
    pub message: String,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            data,
            message: "Sukses".to_string(),
        }
    }

    pub fn with_message(data: T, message: &str) -> Self {
        Self {
            data,
            message: message.to_string(),
        }
    }
}

impl ApiResponse<serde_json::Value> {
    pub fn error(message: &str) -> Self {
        Self {
            data: serde_json::Value::Null,
            message: message.to_string(),
        }
    }
}

impl<T: Serialize> IntoResponse for ApiResponse<T> {
    fn into_response(self) -> Response {
        let body = Json(json!({
            "metaData": {
                "code": "200",
                "message": self.message
            },
            "response": self.data
        }));

        body.into_response()
    }
}

/// Helper untuk Redirect bergaya Laravel
pub struct Redirect {
    path: String,
    flash_success: Option<String>,
    flash_error: Option<String>,
    errors: Option<std::collections::HashMap<String, String>>,
    old_input: Option<serde_json::Value>,
}

impl Redirect {
    pub fn to(path: &str) -> Self {
        Self {
            path: path.to_string(),
            flash_success: None,
            flash_error: None,
            errors: None,
            old_input: None,
        }
    }

    /// Menambahkan pesan sukses
    pub fn with_success(mut self, message: &str) -> Self {
        self.flash_success = Some(message.to_string());
        self
    }

    /// Menambahkan pesan error
    pub fn with_error(mut self, message: &str) -> Self {
        self.flash_error = Some(message.to_string());
        self
    }

    /// Menambahkan error validasi spesifik
    pub fn with_errors(mut self, errors: std::collections::HashMap<String, String>) -> Self {
        self.errors = Some(errors);
        self
    }

    /// Menambahkan input lama (Old Input)
    pub fn with_input<S: Serialize>(mut self, input: S) -> Self {
        self.old_input = Some(serde_json::json!(input));
        self
    }

    /// Redirect kembali ke halaman sebelumnya (Referer)
    pub fn back(req: &axum::http::Request<axum::body::Body>) -> Self {
        let referer = req.headers()
            .get(axum::http::header::REFERER)
            .and_then(|h| h.to_str().ok())
            .unwrap_or("/");
        Self::to(referer)
    }

    /// Eksekusi semua perubahan ke session dan kembalikan axum Redirect
    pub async fn send(self, session: &tower_sessions::Session) -> axum::response::Redirect {
        let flash = crate::core::session::FlashManager::new(session);
        
        if let Some(msg) = self.flash_success {
            flash.success(&msg).await;
        }
        
        if let Some(msg) = self.flash_error {
            flash.error(&msg).await;
        }
        
        if let Some(errors) = self.errors {
            let _ = session.insert("_errors", errors).await;
        }
        
        if let Some(old) = self.old_input {
            let _ = session.insert("_old", old).await;
        }

        axum::response::Redirect::to(&self.path)
    }

    /// Helper paling sakti: Kirim redirect sekaligus sinkronisasi token CSRF
    pub async fn go(
        self, 
        token: axum_csrf::CsrfToken, 
        session: &tower_sessions::Session
    ) -> axum::response::Response {
        let redirect = self.send(session).await;
        (token, redirect).into_response()
    }
}

impl IntoResponse for Redirect {
    fn into_response(self) -> Response {
        axum::response::Redirect::to(&self.path).into_response()
    }
}
