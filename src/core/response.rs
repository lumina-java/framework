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
