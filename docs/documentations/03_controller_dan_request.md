# 📘 Modul 03: Controller & Request Handling

Controller bertanggung jawab menerima request dari client, memanggil model atau service untuk memproses logika bisnis, dan mengembalikan hasil dalam bentuk halaman HTML (View) atau format JSON (API).

Di Lumina Framework, Controller dirancang dengan filosofi **Zero Boilerplate**. Anda tidak perlu menuliskan puluhan extractor mentah Axum (`State`, `Path`, `Query`, `Session`, `CsrfToken`, dsb.). Sebagai gantinya, Lumina menyediakan **Unified `Request` Bundle** dan response return type **`impl IntoResponse`** atau **`LuminaResult`** yang sangat ekspresif, bersih, dan mudah dibaca!

---

## 1. Membuat Controller (`lumina make:controller`)

Gunakan CLI Lumina untuk membuat controller baru:

```bash
lumina make:controller DashboardController
```
Perintah ini akan membuat file `src/app/controllers/dashboard_controller.rs` dan otomatis mendaftarkannya ke dalam `src/app/controllers/mod.rs`.

---

## 2. Controller Halaman Web (View Rendering)

Berikut adalah contoh penulisan controller standar produksi di Lumina. Perhatikan betapa bersih dan elegannya kode tanpa boilerplate:

```rust
use lumina::prelude::*;
use axum::response::IntoResponse;
use crate::app::models::user::User;

pub struct DashboardController;

impl DashboardController {
    /// GET /dashboard
    /// Menggunakan Request bundle untuk akses instan ke State, DB, Session, dan User.
    pub async fn index(req: Request) -> impl IntoResponse {
        // Karena route ini dilindungi auth middleware, req.user dijamin berisi data user
        let user = req.user.clone().expect("Unauthorized access to dashboard");

        // 🔥 EAGER LOADING IN ACTION (N+1 Solution)
        // Mengambil semua user beserta relasi post mereka dalam 2 query SQL optimal
        let users = User::query(req.db())
            .with("posts")
            .get()
            .await
            .unwrap_or_default();

        // Merender template Blade — Bersih, Human-Readable, Tanpa Tanda '&'
        View::make("dashboard.index")
            .with("title", "Dashboard — Lumina")
            .with("user_id", user.sub.clone())
            .with("email", user.email.clone())
            .with("role", user.role.clone())
            .with("user", user)
            .with("users", users)
            .render(req)
            .await
    }
}
```

> 💡 **Tips:** Anda juga dapat menggunakan shortcut `req.view("dashboard.index")` yang ekuivalen dengan `View::make("dashboard.index")`:
> ```rust
> req.view("dashboard.index")
>     .with("title", "Dashboard — Lumina")
>     .render(&req)
>     .await
> ```

---

## 3. Anatomi Unified `Request` Bundle

Objek `Request` (`lumina::core::request::Request` atau via `lumina::prelude::*`) menyediakan akses langsung ke seluruh konteks aplikasi hanya dengan satu parameter:

| Method / Properti | Fungsi | Contoh Penggunaan |
| :--- | :--- | :--- |
| `req.db()` | Mengambil referensi pool database (`&DatabasePool`) | `User::all(req.db()).await?` |
| `req.user` / `req.user()` | Mengambil objek user yang sedang login (`Option<AuthUser>`) | `let uid = req.user.as_ref()?.sub;` |
| `req.is_authenticated()` | Mengecek apakah pengunjung sudah login | `if req.is_authenticated() { ... }` |
| `req.is_guest()` | Mengecek apakah pengunjung adalah tamu (belum login) | `if req.is_guest() { ... }` |
| `req.session` | Mengakses session storage secara langsung | `req.session.insert("cart", cart).await` |
| `req.session_get<T>(key)` | Mengambil nilai dari session | `let cart: Option<Cart> = req.session_get("cart").await;` |
| `req.token` | Mengambil token CSRF aktif | `let csrf = req.token.authenticity_token();` |
| `req.param("id")` | Mengambil path parameter rute (`:id`) | `let id_str = req.param("id");` |
| `req.id_or_fail()?` | Mengambil `:id` sebagai `i64` (otomatis 400 jika bukan angka) | `let id = req.id_or_fail()?;` |
| `req.query_val("search")` | Mengambil nilai query parameter URL (`?search=...`) | `let search = req.query_val("search");` |
| `req.pagination()` | Mengambil tuple `(page, per_page)` dari URL (`?page=1&per_page=10`) | `let (page, per_page) = req.pagination();` |
| `req.storage()` | Mengambil instance File Storage Manager | `req.storage().put("avatars/u1.jpg", &bytes)` |
| `req.redirect("/path")` | Redirect fluen ke URL lain | `req.redirect("/login").go(&req).await` |
| `req.back()` | Redirect kembali ke halaman asal (Referer) | `req.back().go(&req).await` |

---

## 4. Menangani Form HTML dengan Auto-Validation (`ValidatedForm<T>`)

Ketika menangani form web konvensional, Lumina menyediakan `ValidatedForm<T>` yang bekerja persis seperti validasi form di Laravel:

1. Jika validasi **berhasil**, payload langsung siap digunakan.
2. Jika validasi **gagal**, Lumina secara otomatis:
   - Me-redirect user kembali ke halaman form sebelumnya (`Referer`).
   - Menyimpan seluruh pesan error ke dalam Session Flash `_errors`.
   - Menyimpan input lama ke dalam Session Flash `_old` (sehingga form tidak kosong kembali).

### A. Definisi Request DTO (`src/app/requests/create_post_request.rs`)
```rust
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Validate, Clone)]
pub struct CreatePostRequest {
    #[validate(length(min = 5, message = "Judul postingan minimal 5 karakter"))]
    pub title: String,

    #[validate(length(min = 20, message = "Konten artikel minimal 20 karakter"))]
    pub content: String,
}
```

### B. Controller Handler
```rust
use lumina::prelude::*;
use axum::response::IntoResponse;
use crate::app::requests::create_post_request::CreatePostRequest;
use crate::app::models::post::Post;

pub async fn store(
    req: Request,
    ValidatedForm(form): ValidatedForm<CreatePostRequest>,
) -> impl IntoResponse {
    let mut post = Post {
        title: form.title,
        content: form.content,
        user_id: req.user.as_ref().map(|u| u.id).unwrap_or(1),
        ..Default::default()
    };
    
    post.save(req.db()).await.expect("Gagal menyimpan postingan");

    req.redirect("/posts")
        .with_success("Postingan berhasil diterbitkan!")
        .go(&req)
        .await
}
```

---

## 5. Menangani RESTful API dengan `ValidatedJson<T>` & `LuminaResult`

Untuk rute API JSON, handler mengembalikan `LuminaResult` (alias untuk `Result<Response, AppError>`). Objek `Request` memiliki serangkaian response helper instan:

```rust
use lumina::prelude::*;
use crate::app::models::product::Product;
use crate::app::requests::product_request::CreateProductRequest;

pub struct ApiProductController;

impl ApiProductController {
    /// GET /api/products — Ambil daftar produk dengan pagination
    pub async fn index(req: Request) -> LuminaResult {
        let (page, per_page) = req.pagination();
        let paginator = Product::paginate(req.db(), page, per_page).await?;
        req.paginate(paginator)
    }

    /// GET /api/products/:id — Detail satu produk
    pub async fn show(req: Request) -> LuminaResult {
        let id = req.id_or_fail()?;
        match Product::find(req.db(), id).await? {
            Some(product) => req.ok(product),
            None => req.not_found("Produk tidak ditemukan"),
        }
    }

    /// POST /api/products — Validasi otomatis payload JSON & simpan
    pub async fn store(
        req: Request,
        ValidatedJson(payload): ValidatedJson<CreateProductRequest>,
    ) -> LuminaResult {
        let mut product = Product {
            name: payload.name,
            price: payload.price,
            ..Default::default()
        };
        let id = product.save(req.db()).await?;
        product.id = id;

        req.created_with(product, "Produk berhasil ditambahkan")
    }

    /// PUT /api/products/:id — Update produk
    pub async fn update(
        req: Request,
        ValidatedJson(payload): ValidatedJson<CreateProductRequest>,
    ) -> LuminaResult {
        let id = req.id_or_fail()?;
        let mut product = match Product::find(req.db(), id).await? {
            Some(p) => p,
            None => return req.not_found("Produk tidak ditemukan"),
        };

        product.name = payload.name;
        product.price = payload.price;
        product.save(req.db()).await?;

        req.success(product, "Produk berhasil diperbarui")
    }

    /// DELETE /api/products/:id — Hapus produk
    pub async fn destroy(req: Request) -> LuminaResult {
        let id = req.id_or_fail()?;
        if Product::destroy(req.db(), id).await? {
            req.success(serde_json::json!({ "id": id }), "Produk berhasil dihapus")
        } else {
            req.not_found("Produk tidak ditemukan")
        }
    }
}
```

### Ringkasan Helper Respon JSON di `Request`:
- `req.ok(data)` ➔ HTTP 200 `{ "success": true, "data": ... }`
- `req.success(data, "Pesan")` ➔ HTTP 200 `{ "success": true, "message": "...", "data": ... }`
- `req.created_with(data, "Pesan")` ➔ HTTP 201 `{ "success": true, "message": "...", "data": ... }`
- `req.paginate(paginator)` ➔ HTTP 200 dengan data pagination terstandar
- `req.not_found("Pesan")` ➔ HTTP 404 `{ "success": false, "error": "..." }`
- `req.bad_request("Pesan")` ➔ HTTP 400 `{ "success": false, "error": "..." }`
- `req.unauthorized("Pesan")` ➔ HTTP 401 `{ "success": false, "error": "..." }`

---

## 6. Upload Berkas Multipart (`req.storage()`)

Lumina mempermudah penanganan berkas unggahan melalui abstraksi storage:

```rust
use lumina::prelude::*;
use axum::extract::Multipart;

pub async fn upload_avatar(req: Request, mut multipart: Multipart) -> LuminaResult {
    while let Some(field) = multipart.next_field().await.map_err(|e| AppError::BadRequest(e.to_string()))? {
        let name = field.name().unwrap_or("").to_string();
        if name == "avatar" {
            let file_name = field.file_name().unwrap_or("avatar.jpg").to_string();
            let data = field.bytes().await.map_err(|e| AppError::BadRequest(e.to_string()))?;

            // Simpan berkas ke storage/app/public/avatars/
            let target_path = format!("avatars/{}", file_name);
            req.storage().put(&target_path, &data)
                .map_err(|e| AppError::InternalError(format!("Gagal upload: {}", e)))?;

            return req.success(
                serde_json::json!({ "path": target_path }),
                "Avatar berhasil diunggah"
            );
        }
    }

    req.bad_request("Field avatar tidak ditemukan dalam form")
}
```

---

### Navigasi Modul:
- **Sebelumnya**: [Modul 02: Routing & Middleware](./02_routing_dan_middleware.md) 
- **Berikutnya**: [Modul 04: Model & Eloquent ORM](./04_model_dan_eloquent_orm.md) ➔
- **Kembali ke**: [Daftar Isi Dokumentasi](./README.md)
