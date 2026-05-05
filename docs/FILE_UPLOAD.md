# 📁 Advanced File Upload & Image Manipulation

Lumina menyediakan sistem penanganan file yang intuitif dan *fluent*, terinspirasi dari kemudahan Laravel namun dengan performa pemrosesan gambar native Rust yang sangat cepat.

---

## 📑 Daftar Isi
1. [Konfigurasi Dasar](#konfigurasi-dasar)
2. [Menangani Upload di Controller](#menangani-upload-di-controller)
3. [UploadedFile API](#uploadedfile-api)
4. [Manipulasi Gambar (Resize, Thumbnail, Grayscale)](#manipulasi-gambar-resize-thumbnail-grayscale)
5. [Contoh Implementasi Lengkap](#contoh-implementasi-lengkap)
6. [Tampilan Form (HTML)](#tampilan-form-html)

---

## Konfigurasi Dasar

Pastikan Anda memiliki folder `uploads` di root project atau sesuai konfigurasi `STORAGE_ROOT` di file `.env`. Secara default, Lumina menyimpan file di folder `uploads` dan dapat diakses publik via `/storage/`.

```env
# .env
STORAGE_ROOT=./uploads
STORAGE_URL=/storage
```

---

## Menangani Upload di Controller

Gunakan extractor `LuminaMultipart` untuk menangani `multipart/form-data`. Anda tidak perlu lagi melakukan looping manual pada field.

```rust
use crate::core::upload::LuminaMultipart;

pub async fn store(req: Request, multipart: LuminaMultipart) -> impl IntoResponse {
    // Ambil file berdasarkan nama field di HTML
    if let Some(file) = multipart.file("avatar") {
        // Simpan file
        let path = file.store(&req, "avatars").await?;
        println!("File disimpan di: {}", path);
    }
    
    // Ambil input text biasa
    let name = multipart.input("name").unwrap_or(&"Guest".to_string());
}
```

---

## UploadedFile API

Setiap file yang diupload dibungkus dalam struct `UploadedFile` yang memiliki method bantuan berikut:

| Method | Return | Deskripsi |
|---|---|---|
| `.original_name` | `String` | Nama asli file (misal: `foto_profil.jpg`) |
| `.extension()` | `String` | Ekstensi file dalam lowercase (misal: `jpg`) |
| `.size()` | `usize` | Ukuran file dalam bytes |
| `.content_type` | `String` | MIME type (misal: `image/png`) |
| `.is_image()` | `bool` | Mengecek apakah file adalah gambar valid |
| `.store(dir)` | `Result<String, String>` | Simpan dengan nama unik (UUID) ke direktori |
| `.store_as(dir, name)` | `Result<String, String>` | Simpan dengan nama spesifik ke direktori |

---

## Manipulasi Gambar (Resize, Thumbnail, Grayscale)

Lumina terintegrasi dengan engine pemrosesan gambar yang memungkinkan Anda memodifikasi gambar sebelum disimpan.

### 1. Resize & Thumbnail
```rust
// Resize ke ukuran spesifik (Lanczos3 Filter)
file.clone().resize(800, 600).store(&req, "gallery").await?;

// Buat thumbnail cepat (Maintain aspect ratio)
file.clone().thumbnail(150, 150).store(&req, "thumbs").await?;
```

### 2. Efek Grayscale (Hitam Putih)
```rust
file.clone().grayscale().store(&req, "retro").await?;
```

### 3. Chaining (Rantai Perintah)
Anda bisa menggabungkan beberapa perintah sekaligus:
```rust
file.clone()
    .grayscale()
    .resize(400, 400)
    .store_as(&req, "profiles", "avatar_gray.jpg")
    .await?;
```

---

## Contoh Implementasi Lengkap

Berikut adalah contoh controller untuk mengupload foto produk, membuat thumbnail, dan menyimpan path ke database.

```rust
pub async fn upload_product(req: Request, multipart: LuminaMultipart) -> impl IntoResponse {
    if let Some(file) = multipart.file("photo") {
        // 1. Simpan Original
        let original_path = file.store(&req, "products/original").await.unwrap();

        // 2. Buat Thumbnail 300x300
        let thumb_path = file.clone()
            .thumbnail(300, 300)
            .store(&req, "products/thumbs").await.unwrap();

        // 3. Simpan ke Database
        sqlx::query("INSERT INTO products (image, thumb) VALUES (?, ?)")
            .bind(original_path)
            .bind(thumb_path)
            .execute(req.db()).await?;

        return req.redirect("/products").with_success("Produk berhasil diupload!").go(&req).await;
    }

    req.back().with_error("Pilih file terlebih dahulu!").go(&req).await
}
```

---

## Tampilan Form (HTML)

Pastikan tag `<form>` memiliki atribut `enctype="multipart/form-data"`.

```html
<form action="/upload" method="POST" enctype="multipart/form-data">
    <input type="hidden" name="csrf_token" value="{{ csrf_token }}">
    
    <div class="mb-3">
        <label>Nama Produk</label>
        <input type="text" name="name" class="form-control">
    </div>

    <div class="mb-3">
        <label>Foto Produk</label>
        <input type="file" name="photo" class="form-control">
    </div>

    <button type="submit" class="btn btn-primary">Upload Sekarang</button>
</form>
```

---

> [!TIP]
> Gunakan `.clone()` jika Anda ingin memproses satu file yang sama menjadi beberapa versi (misal: original dan thumbnail) agar data asli tidak hilang saat proses manipulasi pertama.
