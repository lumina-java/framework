use axum::{
    response::{IntoResponse, Response},
    Json,
};
use serde_json::{json, Value};

/// Trait untuk mentransformasi data menjadi format JSON API yang terstandarisasi.
pub trait JsonResource {
    fn to_json(&self) -> Value;

    /// Membungkus hasil transformasi ke dalam object "data" (standar Laravel/JSON:API).
    fn to_response(&self) -> Response {
        Json(json!({
            "data": self.to_json()
        }))
        .into_response()
    }
}

/// Struktur untuk menangani koleksi data (list).
pub struct ResourceCollection<T: JsonResource> {
    pub data: Vec<T>,
}

impl<T: JsonResource> ResourceCollection<T> {
    pub fn new(data: Vec<T>) -> Self {
        Self { data }
    }
}

impl<T: JsonResource> IntoResponse for ResourceCollection<T> {
    fn into_response(self) -> Response {
        let transformed: Vec<Value> = self.data.iter().map(|item| item.to_json()).collect();
        Json(json!({
            "data": transformed
        }))
        .into_response()
    }
}

/// Macro untuk mempermudah implementasi JsonResource secara cepat.
#[macro_export]
macro_rules! resource {
    ($name:ident, $body:expr) => {
        impl JsonResource for $name {
            fn to_json(&self) -> serde_json::Value {
                $body
            }
        }
    };
}
