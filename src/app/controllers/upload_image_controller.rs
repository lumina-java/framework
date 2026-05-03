use axum::response::IntoResponse;
use crate::core::response::Redirect;
use crate::core::view::View;
use crate::core::request::Request;
use crate::app::models::upload_image::UploadImage;
use crate::database::model::Model;
use axum::extract::Multipart;

pub struct UploadImageController;

impl UploadImageController {
    /// GET /upload_images
    pub async fn index(req: Request) -> impl IntoResponse {
        let items = UploadImage::all(req.state.db()).await.unwrap_or_default();

        View::make("upload_image.index")
            .with("items", &items)
            .render(&req)
            .await
    }

    /// GET /upload_images/create
    pub async fn create(req: Request) -> impl IntoResponse {
        View::make("upload_image.create")
            .with("csrf_token", req.token.authenticity_token().unwrap())
            .render(&req)
            .await
    }

    /// POST /upload_images
    pub async fn store(req: Request, mut multipart: Multipart) -> impl IntoResponse {
        let mut nama = String::new();
        let mut file_path = String::new();

        while let Some(field) = multipart.next_field().await.unwrap_or(None) {
            let field_name = field.name().unwrap_or_default().to_string();
            if field_name == "file" {
                let original_file_name = field.file_name().unwrap_or("upload").to_string();
                let data = field.bytes().await.unwrap_or_default();
                if !data.is_empty() {
                    let timestamp = std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_secs();
                    let new_filename = format!("{}_{}", timestamp, original_file_name);
                    let path = format!("uploads/{}", new_filename);
                    let _ = req.state.storage.disk.put(&path, &data).await;
                    
                    nama = original_file_name;
                    file_path = format!("/storage/uploads/{}", new_filename);
                }
            }
        }

        if file_path.is_empty() {
            return Redirect::to("/upload_images/create")
                .with_error("File gambar tidak boleh kosong kawan!")
                .go(&req).await;
        }

        let result = sqlx::query("INSERT INTO upload_images (nama, file, created_at, updated_at) VALUES (?, ?, NOW(), NOW())")
            .bind(&nama)
            .bind(&file_path)
            .execute(&req.state.db().pool).await;

        match result {
            Ok(_) => Redirect::to("/upload_images").with_success("Data berhasil disimpan kawan!").go(&req).await,
            Err(e) => Redirect::to("/upload_images/create")
                .with_error(&format!("Gagal menyimpan data: {}", e))
                .go(&req).await
        }
    }

    /// GET /upload_images/:id/edit
    pub async fn edit(req: Request, axum::extract::Path(id): axum::extract::Path<i64>) -> impl IntoResponse {
        let item = UploadImage::find(req.state.db(), id).await.expect("Data tidak ditemukan");

        View::make("upload_image.edit")
            .with("item", &item)
            .with("csrf_token", req.token.authenticity_token().unwrap())
            .render(&req)
            .await
    }

    /// POST /upload_images/:id/update
    pub async fn update(
        req: Request,
        axum::extract::Path(id): axum::extract::Path<i64>,
        mut multipart: Multipart,
    ) -> impl IntoResponse {
        let mut nama = String::new();
        let mut file_path = String::new();

        while let Some(field) = multipart.next_field().await.unwrap_or(None) {
            let field_name = field.name().unwrap_or_default().to_string();
            if field_name == "file" {
                let original_file_name = field.file_name().unwrap_or("upload").to_string();
                let data = field.bytes().await.unwrap_or_default();
                if !data.is_empty() {
                    let timestamp = std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_secs();
                    let new_filename = format!("{}_{}", timestamp, original_file_name);
                    let path = format!("uploads/{}", new_filename);
                    let _ = req.state.storage.disk.put(&path, &data).await;
                    
                    nama = original_file_name;
                    file_path = format!("/storage/uploads/{}", new_filename);
                }
            }
        }

        let result = if !file_path.is_empty() {
            sqlx::query("UPDATE upload_images SET nama = ?, file = ?, updated_at = NOW() WHERE id = ?")
                .bind(&nama)
                .bind(&file_path)
                .bind(id)
                .execute(&req.state.db().pool).await
        } else {
            sqlx::query("SELECT 1").execute(&req.state.db().pool).await
        };

        match result {
            Ok(_) => Redirect::to("/upload_images").with_success("Data berhasil diupdate kawan!").go(&req).await,
            Err(e) => Redirect::to(&format!("/upload_images/{}/edit", id))
                .with_error(&format!("Gagal update data: {}", e))
                .go(&req).await
        }
    }

    /// POST /upload_images/:id/delete
    pub async fn delete(req: Request, axum::extract::Path(id): axum::extract::Path<i64>) -> impl IntoResponse {
        if let Ok(item) = UploadImage::find(req.state.db(), id).await {
            let relative_path = item.file.trim_start_matches("/storage/");
            let _ = req.state.storage.disk.delete(relative_path).await;
        }

        let _ = sqlx::query("DELETE FROM upload_images WHERE id = ?")
            .bind(id)
            .execute(&req.state.db().pool).await;

        Redirect::to("/upload_images").with_success("Data berhasil dihapus kawan!").go(&req).await
    }
}
