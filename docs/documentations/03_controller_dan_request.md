# 📘 Modul 03: Controller & Request

Controller bertanggung jawab menerima request dari pengguna, memanggil model atau service untuk memproses logika bisnis, dan mengembalikan hasil dalam bentuk halaman HTML (View) atau format JSON (API).

---

## 1. Membuat Controller (`lumina make:controller`)

Gunakan CLI Lumina untuk membuat controller baru:

```bash
lumina make:controller ProductController
```
Perintah ini akan membuat file `src/app/controllers/product_controller.rs` dan otomatis mendaftarkannya ke dalam `src/app/controllers/mod.rs`.

---

## 2. Struktur Dasar Controller Handler

Controller di Lumina menggunakan sistem **Dependency Injection** otomatis melalui *Extractors* Axum:

```rust
use axum::{
    extract::{State, Path, Query, Form, Json},
    response::Html,
};
use serde::{Deserialize, Serialize};
use lumina::core::application::AppState;
use lumina::core::blade::BladeContext;

pub struct ProductController;

impl ProductController {
    /// GET /products
    pub async fn index(State(state): State<AppState>) -> Html<String> {
        let mut ctx = BladeContext::new();
        ctx.insert("title", "Katalog Produk");
        ctx.insert("version", "v0.2.0");

        let html = state.view.render("products/index.blade.rs", &ctx);
        Html(html)
    }

    /// GET /products/:id
    pub async fn show(
        State(state): State<AppState>,
        Path(id): Path<i64>,
    ) -> Html<String> {
        let mut ctx = BladeContext::new();
        ctx.insert("product_id", id);

        let html = state.view.render("products/show.blade.rs", &ctx);
        Html(html)
    }
}
```

---

## 3. Menangani Input Form & JSON Payload

### A. Input Form URL-Encoded (`Form<T>`)
Untuk formulir HTML biasa:
```rust
#[derive(Deserialize)]
pub struct CreateProductForm {
    pub name: String,
    pub price: f64,
    pub description: Option<String>,
}

pub async fn store_form(
    State(state): State<AppState>,
    Form(payload): Form<CreateProductForm>,
) -> String {
    format!("Produk '{}' berhasil disimpan dengan harga Rp {}", payload.name, payload.price)
}
```

### B. Input JSON Payload (`Json<T>`)
Untuk RESTful API:
```rust
use axum::Json;

#[derive(Deserialize, Serialize)]
pub struct ProductDto {
    pub name: String,
    pub price: f64,
}

pub async fn store_api(
    State(state): State<AppState>,
    Json(payload): Json<ProductDto>,
) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "success": true,
        "message": "Data berhasil dibuat",
        "data": payload
    }))
}
```

---

## 4. Validasi Otomatis (`lumina make:request`)

Untuk menjaga kebersihan controller, pindahkan aturan validasi ke Request DTO menggunakan crate `validator`:

```bash
lumina make:request StoreProductRequest
```
File `src/app/requests/store_product_request.rs`:
```rust
use validator::Validate;
use serde::Deserialize;

#[derive(Debug, Deserialize, Validate)]
pub struct StoreProductRequest {
    #[validate(length(min = 3, message = "Nama produk minimal 3 karakter"))]
    pub name: String,

    #[validate(range(min = 1000.0, message = "Harga minimal Rp 1.000"))]
    pub price: f64,

    #[validate(email(message = "Format email supplier tidak valid"))]
    pub supplier_email: String,
}
```

Di Controller:
```rust
use validator::Validate;
use axum::http::StatusCode;

pub async fn store(
    Json(payload): Json<StoreProductRequest>
) -> Result<String, (StatusCode, String)> {
    if let Err(errors) = payload.validate() {
        return Err((StatusCode::UNPROCESSABLE_ENTITY, errors.to_string()));
    }

    Ok("Data tervalidasi dan siap disimpan!".to_string())
}
```

---

## 5. Upload File Multipart & Image Handling

Lumina menyediakan penanganan unggah berkas multipart langsung melalui modul Storage:

```rust
use axum::extract::Multipart;

pub async fn upload_avatar(
    State(state): State<AppState>,
    mut multipart: Multipart,
) -> Result<String, String> {
    while let Some(field) = multipart.next_field().await.map_err(|e| e.to_string())? {
        let name = field.name().unwrap_or("").to_string();
        if name == "avatar" {
            let file_name = field.file_name().unwrap_or("upload.jpg").to_string();
            let data = field.bytes().await.map_err(|e| e.to_string())?;

            // Simpan ke storage/app/public/avatars/
            let target_path = format!("storage/app/public/avatars/{}", file_name);
            std::fs::write(&target_path, &data).map_err(|e| e.to_string())?;

            return Ok(format!("File berhasil diunggah ke {}", target_path));
        }
    }

    Err("Tidak ada file ditemukan dalam request".to_string())
}
```

---

### Navigasi Modul:
- **Sebelumnya**: [Modul 02: Routing & Middleware](./02_routing_dan_middleware.md) 
- **Berikutnya**: [Modul 04: Model & Eloquent ORM](./04_model_dan_eloquent_orm.md) ➔
- **Kembali ke**: [Daftar Isi Dokumentasi](./README.md)
