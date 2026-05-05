use axum::response::IntoResponse;
use crate::core::response::Redirect;
use crate::core::view::View;
use crate::core::request::Request;
use crate::app::models::upload_image::UploadImage;
use crate::database::model::Model;
use crate::core::upload::LuminaMultipart;

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
    pub async fn store(req: Request, multipart: LuminaMultipart) -> impl IntoResponse {
        if let Some(file) = multipart.file("file") {
            // Simpan file dengan nama unik otomatis
            let path = match file.store(&req, "uploads").await {
                Ok(p) => p,
                Err(e) => return Redirect::to("/upload_images/create")
                    .with_error(&format!("Gagal upload file: {}", e))
                    .go(&req).await,
            };

            let nama = file.original_name.clone();
            let file_url = format!("/storage/{}", path);

            let result = sqlx::query("INSERT INTO upload_images (nama, file, created_at, updated_at) VALUES (?, ?, NOW(), NOW())")
                .bind(&nama)
                .bind(&file_url)
                .execute(&req.state.db().pool).await;

            match result {
                Ok(_) => Redirect::to("/upload_images").with_success("Data berhasil disimpan kawan!").go(&req).await,
                Err(e) => Redirect::to("/upload_images/create")
                    .with_error(&format!("Gagal menyimpan data ke DB: {}", e))
                    .go(&req).await
            }
        } else {
            Redirect::to("/upload_images/create")
                .with_error("File gambar tidak boleh kosong kawan!")
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
        multipart: LuminaMultipart,
    ) -> impl IntoResponse {
        let result = if let Some(file) = multipart.file("file") {
            let path = match file.store(&req, "uploads").await {
                Ok(p) => p,
                Err(e) => return Redirect::to(&format!("/upload_images/{}/edit", id))
                    .with_error(&format!("Gagal upload file: {}", e))
                    .go(&req).await,
            };
            
            let nama = &file.original_name;
            let file_url = format!("/storage/{}", path);

            sqlx::query("UPDATE upload_images SET nama = ?, file = ?, updated_at = NOW() WHERE id = ?")
                .bind(nama)
                .bind(&file_url)
                .bind(id)
                .execute(&req.state.db().pool).await
        } else {
            // Jika tidak ada file baru, kita biarkan saja (hanya query dummy atau update nama jika ada)
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
