use axum::{
    async_trait,
    extract::{FromRequest, Multipart},
    http::Request,
    body::Body,
};
use std::collections::HashMap;
use std::io::Cursor;
use image::ImageFormat;
use crate::core::application::AppState;

/// UploadedFile — Membungkus data file hasil upload dengan helper methods.
#[derive(Clone)]
pub struct UploadedFile {
    pub name: String,
    pub original_name: String,
    pub content_type: String,
    pub data: Vec<u8>,
}

impl UploadedFile {
    pub fn extension(&self) -> String {
        std::path::Path::new(&self.original_name)
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("")
            .to_lowercase()
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn is_image(&self) -> bool {
        self.content_type.starts_with("image/")
    }

    /// Simpan file ke storage dengan nama unik otomatis.
    pub async fn store(&self, req: &crate::core::request::Request, directory: &str) -> Result<String, String> {
        let hash = uuid::Uuid::new_v4().to_string(); // Menggunakan UUID agar unik
        let filename = format!("{}.{}", hash, self.extension());
        let path = format!("{}/{}", directory.trim_end_matches('/'), filename);
        
        req.state.storage.put(&path, &self.data).await?;
        Ok(path)
    }

    /// Simpan file dengan nama spesifik.
    pub async fn store_as(&self, req: &crate::core::request::Request, directory: &str, name: &str) -> Result<String, String> {
        let path = format!("{}/{}", directory.trim_end_matches('/'), name);
        req.state.storage.put(&path, &self.data).await?;
        Ok(path)
    }

    /// Resize gambar (mengubah ukuran data di memory).
    pub fn resize(mut self, width: u32, height: u32) -> Self {
        if let Ok(img) = image::load_from_memory(&self.data) {
            let resized = img.resize(width, height, image::imageops::FilterType::Lanczos3);
            let mut buf = Cursor::new(Vec::new());
            let format = ImageFormat::from_extension(self.extension()).unwrap_or(ImageFormat::Png);
            if resized.write_to(&mut buf, format).is_ok() {
                self.data = buf.into_inner();
            }
        }
        self
    }

    /// Buat thumbnail cepat (mengubah ukuran data di memory).
    pub fn thumbnail(mut self, width: u32, height: u32) -> Self {
        if let Ok(img) = image::load_from_memory(&self.data) {
            let thumb = img.thumbnail(width, height);
            let mut buf = Cursor::new(Vec::new());
            let format = ImageFormat::from_extension(self.extension()).unwrap_or(ImageFormat::Png);
            if thumb.write_to(&mut buf, format).is_ok() {
                self.data = buf.into_inner();
            }
        }
        self
    }

    /// Ubah gambar ke hitam putih.
    pub fn grayscale(mut self) -> Self {
        if let Ok(img) = image::load_from_memory(&self.data) {
            let gray = img.grayscale();
            let mut buf = Cursor::new(Vec::new());
            let format = ImageFormat::from_extension(self.extension()).unwrap_or(ImageFormat::Png);
            if gray.write_to(&mut buf, format).is_ok() {
                self.data = buf.into_inner();
            }
        }
        self
    }
}

/// LuminaMultipart — Extractor ramah Laravel untuk menangani multipart/form-data.
pub struct LuminaMultipart {
    pub fields: HashMap<String, String>,
    pub files: HashMap<String, Vec<UploadedFile>>,
}

#[async_trait]
impl<S> FromRequest<S> for LuminaMultipart
where
    S: Send + Sync,
    AppState: axum::extract::FromRef<S>,
{
    type Rejection = crate::core::error::AppError;

    async fn from_request(req: Request<Body>, state: &S) -> Result<Self, Self::Rejection> {
        let mut multipart = Multipart::from_request(req, state).await
            .map_err(|_| crate::core::error::AppError::BadRequest("Format multipart tidak valid".into()))?;

        let mut fields = HashMap::new();
        let mut files: HashMap<String, Vec<UploadedFile>> = HashMap::new();

        while let Some(field) = multipart.next_field().await.map_err(|e| {
            crate::core::error::AppError::BadRequest(format!("Gagal membaca field: {}", e))
        })? {
            let name = field.name().unwrap_or_default().to_string();
            let file_name = field.file_name().map(|s| s.to_string());
            let content_type = field.content_type().map(|s| s.to_string()).unwrap_or_default();

            if let Some(orig_name) = file_name {
                let data = field.bytes().await.map_err(|e| {
                    crate::core::error::AppError::BadRequest(format!("Gagal membaca data file: {}", e))
                })?.to_vec();

                if !data.is_empty() {
                    let uploaded = UploadedFile {
                        name: name.clone(),
                        original_name: orig_name,
                        content_type,
                        data,
                    };
                    files.entry(name).or_default().push(uploaded);
                }
            } else {
                let value = field.text().await.map_err(|e| {
                    crate::core::error::AppError::BadRequest(format!("Gagal membaca text field: {}", e))
                })?;
                fields.insert(name, value);
            }
        }

        Ok(Self { fields, files })
    }
}

impl LuminaMultipart {
    /// Ambil satu file berdasarkan nama field.
    pub fn file(&self, name: &str) -> Option<&UploadedFile> {
        self.files.get(name).and_then(|v| v.first())
    }

    /// Ambil banyak file (array) berdasarkan nama field.
    pub fn files(&self, name: &str) -> Vec<&UploadedFile> {
        self.files.get(name).map(|v| v.iter().collect()).unwrap_or_default()
    }

    /// Ambil nilai text field.
    pub fn input(&self, name: &str) -> Option<&String> {
        self.fields.get(name)
    }

    /// Cek apakah field file ada.
    pub fn has_file(&self, name: &str) -> bool {
        self.files.contains_key(name)
    }
}
