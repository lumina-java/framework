use axum::{
    extract::{Multipart, State},
    response::IntoResponse,
};
use crate::core::application::AppState;
use crate::core::response::ApiResponse;

pub struct StorageController;

impl StorageController {
    /// POST /api/storage/test-upload
    /// Endpoint untuk mendemonstrasikan upload file menggunakan Storage System.
    pub async fn test_upload(
        State(state): State<AppState>,
        mut multipart: Multipart,
    ) -> impl IntoResponse {
        let mut file_url = String::new();
        let mut file_name = String::new();
        let mut found_fields = Vec::new();

        loop {
            match multipart.next_field().await {
                Ok(Some(field)) => {
                    let name = match field.name() {
                        Some(n) => n.to_string(),
                        None => {
                            found_fields.push("<TANPA NAMA>".to_string());
                            continue;
                        }
                    };
                    found_fields.push(name.clone());
                    
                    if name == "file" {
                        file_name = field.file_name().unwrap_or("upload.bin").to_string();
                        
                        let data = match field.bytes().await {
                            Ok(b) => b,
                            Err(e) => return ApiResponse::error(&format!("Gagal membaca stream file: {}", e)).into_response(),
                        };

                        // Gunakan Storage System Lumina
                        let storage = &state.storage;
                        
                        // Simpan ke disk (default local)
                        let result = storage.put(&file_name, &data).await;
                        
                        if result.is_ok() {
                            file_url = storage.url(&file_name);
                        } else {
                            return ApiResponse::error(&format!("Gagal menyimpan file: {:?}", result.err())).into_response();
                        }
                    }
                },
                Ok(None) => break,
                Err(e) => return ApiResponse::error(&format!("Multipart error: {}", e)).into_response(),
            }
        }

        if file_url.is_empty() {
            let msg = if found_fields.is_empty() {
                "Tidak ada field yang ditemukan dalam multipart form".to_string()
            } else {
                format!("Field 'file' tidak ditemukan. Field yang diterima: {:?}", found_fields)
            };
            return ApiResponse::error(&msg).into_response();
        }

        ApiResponse::success(serde_json::json!({
            "message": "File berhasil diunggah",
            "file_name": file_name,
            "url": file_url
        })).into_response()
    }
}
