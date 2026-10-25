# 📘 Modul 02: Routing & Middleware

Modul ini membahas cara mendefinisikan rute web dan API serta menambahkan lapisan penyaring (*middleware*) untuk otentikasi, pembatasan kuota (*rate limiting*), dan keamanan HTTP.

---

## 1. Definisi Route Dasar

Semua rute web didaftarkan di file `src/routes/web.rs`, sedangkan endpoint API di `src/routes/api.rs`.

Lumina menyediakan metode HTTP standar yang intuitif:

```rust
use axum::{response::Html, routing::{get, post, put, delete}, Router};
use lumina::core::application::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        // HTTP GET
        .route("/", get(welcome_handler))
        // HTTP POST
        .route("/users", post(create_user_handler))
        // HTTP PUT (Update seluruh data)
        .route("/users/:id", put(update_user_handler))
        // HTTP DELETE
        .route("/users/:id", delete(delete_user_handler))
}

async fn welcome_handler() -> Html<&'static str> {
    Html("<h1>Selamat Datang di Lumina Framework!</h1>")
}
```

---

## 2. Route Parameters

Ekstraksi parameter URL dilakukan dengan sangat mudah menggunakan extractor `Path`:

```rust
use axum::extract::Path;

// Menangkap parameter ID angka
pub async fn get_post(Path(id): Path<i64>) -> String {
    format!("Menampilkan postingan dengan ID: {}", id)
}

// Menangkap multiple parameter (misal kategori & slug)
pub async fn get_article(Path((category, slug)): Path<(String, String)>) -> String {
    format!("Kategori: {}, Artikel: {}", category, slug)
}
```

---

## 3. Route Groups & Prefix

Kelompokkan rute yang memiliki awalan URL sama menggunakan fungsi `nest`:

```rust
pub fn router() -> Router<AppState> {
    let api_routes = Router::new()
        .route("/posts", get(list_posts))
        .route("/posts/:id", get(show_post));

    let admin_routes = Router::new()
        .route("/dashboard", get(admin_dashboard))
        .route("/settings", get(admin_settings));

    Router::new()
        .nest("/api/v1", api_routes)
        .nest("/admin", admin_routes)
}
```

---

## 4. Middleware di Lumina

Middleware bertindak sebagai lapisan filter sebelum request mencapai controller handler atau sesudah response dihasilkan.

### A. Middleware Bawaan Lumina
1. **Security Headers Middleware**: Otomatis menyuntikkan header keamanan modern:
   - `X-Frame-Options: SAMEORIGIN` (mencegah Clickjacking)
   - `X-Content-Type-Options: nosniff` (mencegah MIME confusion)
   - `X-XSS-Protection: 1; mode=block`
   - `Referrer-Policy: strict-origin-when-cross-origin`
2. **Rate Limiting Middleware**: Membatasi jumlah request per IP client untuk mencegah serangan brute force dan DDoS:
   ```env
   RATE_LIMIT_MAX=60      # Maksimal 60 request
   RATE_LIMIT_WINDOW=60   # Per 60 detik
   ```
3. **Session & Flash Messages**: Otomatis mengelola cookie session terenkripsi.

### B. Membuat Custom Middleware
Anda dapat membuat middleware sendiri menggunakan pola closure atau fungsi Axum:

```rust
use axum::{
    extract::Request,
    middleware::Next,
    response::Response,
    http::StatusCode,
};

pub async fn admin_guard(req: Request, next: Next) -> Result<Response, StatusCode> {
    // Periksa header atau session
    if let Some(role) = req.headers().get("X-User-Role") {
        if role == "admin" {
            return Ok(next.run(req).await);
        }
    }

    // Tolak jika bukan admin
    Err(StatusCode::FORBIDDEN)
}
```

### C. Menerapkan Middleware pada Rute Tertentu
```rust
use axum::middleware;

pub fn admin_routes() -> Router<AppState> {
    Router::new()
        .route("/admin/users", get(list_all_users))
        .route("/admin/settings", post(save_settings))
        // Terapkan middleware admin_guard hanya untuk grup ini:
        .layer(middleware::from_fn(admin_guard))
}
```

---

## 5. Proteksi CSRF (Cross-Site Request Forgery)

Untuk semua request formulir web (`POST`, `PUT`, `DELETE`), Lumina menyediakan helper direktif Blade `@csrf`:

```html
<form method="POST" action="/profile/update">
    @csrf
    @method('PUT')
    
    <input type="text" name="name" value="{{ $user.name }}">
    <button type="submit">Simpan Perubahan</button>
</form>
```

Directif `@csrf` akan otomatis merender input hidden berisi token CSRF unik yang akan divalidasi oleh framework saat request dikirimkan.

---

### Navigasi Modul:
- **Sebelumnya**: [Modul 01: Persyaratan Sistem & Instalasi](./01_instalasi_dan_persyaratan.md) 
- **Berikutnya**: [Modul 03: Controller & Request](./03_controller_dan_request.md) ➔
- **Kembali ke**: [Daftar Isi Dokumentasi](./README.md)
