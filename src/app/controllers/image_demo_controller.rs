use axum::response::IntoResponse;
use crate::core::request::Request;
use crate::core::upload::LuminaMultipart;

pub struct ImageDemoController;

impl ImageDemoController {
    /// POST /demo/upload
    pub async fn upload(req: Request, multipart: LuminaMultipart) -> impl IntoResponse {
        if let Some(file) = multipart.file("image") {
            // 1. Simpan Original
            let original_path = file.store(&req, "original").await.unwrap_or_default();

            // 2. Simpan Thumbnail (300x300)
            let thumb_path = file.clone()
                .thumbnail(300, 300)
                .store_as(&req, "thumbs", &format!("thumb_{}", file.original_name))
                .await.unwrap_or_default();

            // 3. Simpan Grayscale
            let gray_path = file.clone()
                .grayscale()
                .resize(800, 600)
                .store_as(&req, "grayscale", &format!("gray_{}", file.original_name))
                .await.unwrap_or_default();

            return req.json(serde_json::json!({
                "message": "Upload & Manipulasi Berhasil!",
                "original": format!("/storage/{}", original_path),
                "thumbnail": format!("/storage/{}", thumb_path),
                "grayscale": format!("/storage/{}", gray_path),
            }));
        }

        req.json(serde_json::json!({ "error": "No image uploaded" }))
    }
}
