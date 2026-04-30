# 📚 Lumina Framework — Dokumentasi Lengkap

> Dokumentasi detail penggunaan Lumina Framework.
> Untuk quick start, lihat [README.md](./README.md).

---

## 📑 Daftar Isi

1. [Arsitektur](#arsitektur)
2. [Instalasi & Setup](#instalasi--setup)
3. [Konfigurasi](#konfigurasi)
4. [Routing](#routing)
5. [Controller & View](#controller--view)
6. [Model & Database (ORM)](#model--database-orm)
7. [Validasi Request](#validasi-request)
8. [Autentikasi & Facade Auth](#autentikasi--facade-auth)
9. [Session & Persistence](#session--persistence)
10. [Debugging (dd!)](#debugging-dd)
11. [CLI Tools](#cli-tools)
12. [**CHEAT SHEET (Quick Reference)**](./CHEAT_SHEET.md)

---

## Arsitektur
... (keep existing content) ...

## Controller & View

### View Builder (Laravel Style)
Lumina menggunakan fluent API untuk merender template. Ini menghindari penggunaan `tera::Context` secara manual.

```rust
use crate::core::view::View;

pub async fn index(State(state): State<AppState>, session: Session) -> impl IntoResponse {
    View::make("home.index")
        .with("title", "Lumina Framework")
        .with("version", "v0.1.0")
        .render(&state, &session)
        .await
}
```

---

## Model & Database (ORM)

### Relasi (Relationships)
Lumina mendukung relasi dasar secara out-of-the-box.

#### Has Many
```rust
// Di dalam impl User
pub async fn posts(&self, db: &DatabasePool) -> Result<Vec<Post>, sqlx::Error> {
    Self::has_many::<Post>(db, "user_id", self.id).await
}
```

#### Belongs To
```rust
// Di dalam impl Post
pub async fn user(&self, db: &DatabasePool) -> Result<User, sqlx::Error> {
    Self::belongs_to::<User>(db, self.user_id).await
}
```

---

## Autentikasi & Facade Auth

Untuk mempermudah penggunaan, gunakan struct `Auth` yang menyediakan API statis untuk operasi autentikasi.

```rust
use crate::core::auth::Auth;

// Cek password
if Auth::check("plain_password", &hashed_password) { ... }

// Login User (Menyimpan JWT dan data user ke session)
Auth::login(&session, auth_user).await?;

// Logout
Auth::logout(&session).await;
```

---

## Session & Persistence

Secara default, Lumina menyimpan session di **Database (MySQL/PostgreSQL/SQLite)** sesuai konfigurasi `DATABASE_URL` Anda.

- **Persistence**: Session tetap ada meskipun server di-restart.
- **Auto Logout**: User akan otomatis logout jika tidak ada aktivitas selama **5 menit** (Inactivity Timeout).
- **Fallback**: Jika database tidak tersedia, Lumina akan otomatis menggunakan memori RAM sebagai penyimpanan sementara.

---

## Debugging (dd!)

Gunakan makro `dd!` (Dump & Die) untuk melihat isi variabel langsung di browser dengan tampilan yang cantik.

```rust
crate::dd!(user_data);
```

---

## CLI Tools

| Command | Deskripsi |
|---|---|
| `./lumina watch` | Menjalankan server dengan **Hot Reload**. |
| `./lumina serve` | Menjalankan server biasa. |
| `./lumina migrate` | Menjalankan migrasi database. |
| `./lumina make:controller` | Membuat file controller baru. |

---

> Lihat [CHEAT_SHEET.md](./CHEAT_SHEET.md) untuk referensi cepat penulisan kode "Beauty Code".

---

## Arsitektur

Lumina mengikuti pola **MVC** terinspirasi dari Laravel, dibangun di atas:

| Komponen | Library |
|---|---|
| Async Runtime | Tokio |
| HTTP Layer | Axum 0.7 |
| Database | SQLx (SQLite / MySQL / PostgreSQL) |
| Template Engine | Tera |
| Autentikasi | jsonwebtoken |
| Validasi | validator |
| Password Hashing | bcrypt |
| Debugging | Custom dd!() & Dump Tool |

```
lumina/
├── src/
│   ├── core/          ← Router, Validation, Auth, View Engine
│   ├── http/          ← Server, Middleware, Auth middleware
│   ├── app/
│   │   ├── controllers/   ← Request handlers
│   │   ├── models/        ← Database models
│   │   └── services/      ← Business logic
│   └── database/      ← Connection pool, Migrations, Model trait
├── routes/
│   ├── web.rs         ← HTML routes
│   └── api.rs         ← JSON/API routes
└── resources/views/   ← Tera HTML templates
```

---

## Instalasi & Setup

### Prasyarat
- Rust stable terbaru: `rustup update`
- Git

### Clone & Jalankan

```bash
git clone https://github.com/your-org/lumina.git
cd lumina

# Salin konfigurasi
cp .env.example .env

# Jalankan server (migrasi DB otomatis berjalan)
cargo run
```

Server berjalan di **http://127.0.0.1:8000**

---

## Konfigurasi

Edit file `.env` di root project:

```env
APP_NAME=Lumina
APP_ENV=development
APP_DEBUG=true
APP_URL=http://localhost:8000

# Pilih salah satu:
DATABASE_URL=sqlite:./lumina.db
# DATABASE_URL=mysql://root:password@127.0.0.1/lumina
# DATABASE_URL=postgres://postgres:password@127.0.0.1/lumina

JWT_SECRET=your-super-secret-key-min-32-chars
```

> **Catatan:** `DATABASE_URL` dengan prefix `sqlite:` akan otomatis memilih driver SQLite. MySQL dan PostgreSQL juga didukung.

---

## Routing

### Web Routes (`routes/web.rs`)

Daftarkan route HTML di `routes/web.rs` menggunakan fluent builder API:

```rust
use crate::core::router::Router;
use crate::app::controllers::home_controller::HomeController;
use crate::core::application::AppState;
use std::sync::Arc;

pub fn register() -> Router<Arc<AppState>> {
    Router::new()
        .get("/",           HomeController::index)
        .get("/about",      HomeController::about)
        .get("/users",      UserController::index)
        .get("/users/:id",  UserController::show)
        .post("/users",     UserController::store)
        .put("/users/:id",  UserController::update)
        .delete("/users/:id", UserController::destroy)
}
```

### API Routes (`routes/api.rs`)

API routes di-prefix `/api` secara otomatis oleh `Application::build_router`.

```rust
use axum::{Router as AxumRouter, routing, middleware::from_fn};
use crate::http::middleware::auth_required;

pub fn register() -> Router<Arc<AppState>> {
    // Public — tidak butuh token
    let public = AxumRouter::new()
        .route("/auth/login",    routing::post(AuthController::api_login))
        .route("/auth/register", routing::post(AuthController::api_register));

    // Protected — wajib Bearer token
    let protected = AxumRouter::new()
        .route("/auth/me",   routing::get(AuthController::me))
        .route("/users",     routing::get(ApiController::index))
        .route("/users/:id", routing::get(ApiController::show))
        .route_layer(from_fn(auth_required));

    Router::from_axum(
        AxumRouter::new().merge(public).merge(protected)
    )
}
```

### HTTP Methods yang Didukung

| Method | Builder |
|--------|---------|
| GET | `.get(path, handler)` |
| POST | `.post(path, handler)` |
| PUT | `.put(path, handler)` |
| PATCH | `.patch(path, handler)` |
| DELETE | `.delete(path, handler)` |

### Route Parameters

Gunakan `:param` pada path, lalu ekstrak dengan `Path<T>`:

```rust
use axum::extract::Path;

pub async fn show(
    State(state): State<Arc<AppState>>,
    Path(id): Path<u32>
) -> Html<String> {
    // id tersedia sebagai u32
}
```

---

## Controller

### Membuat Controller Baru

Buat file di `src/app/controllers/nama_controller.rs`:

```rust
use axum::{extract::State, response::Html};
use std::sync::Arc;
use crate::core::application::AppState;
use tera::Context;

pub struct ProductController;

impl ProductController {
    /// GET /products — Daftar semua produk
    pub async fn index(State(state): State<Arc<AppState>>) -> Html<String> {
        let mut ctx = Context::new();
        ctx.insert("title", "Daftar Produk");
        ctx.insert("products", &vec!["Produk A", "Produk B"]);

        let html = state.view.render("products/index.html", &ctx);
        Html(html)
    }
}
```

Lalu daftarkan di `src/app/controllers/mod.rs`:

```rust
pub mod product_controller;
```

Dan tambahkan ke route:

```rust
// routes/web.rs
use crate::app::controllers::product_controller::ProductController;

Router::new()
    .get("/products", ProductController::index)
```

### JSON Response (API Controller)

```rust
use axum::Json;
use serde_json::{json, Value};

pub struct ApiController;

impl ApiController {
    pub async fn index() -> Json<Value> {
        Json(json!({
            "status": "success",
            "data": [
                {"id": 1, "name": "Alice"},
                {"id": 2, "name": "Bob"}
            ]
        }))
    }
}
```

### Fluent Redirect API

Lumina menyediakan *builder* untuk redirect bergaya Laravel:

```rust
use crate::core::response::Redirect;

pub async fn store(session: Session) -> impl IntoResponse {
    Redirect::to("/dashboard")
        .with_success("Data berhasil disimpan!")
        .send(&session)
        .await
}
```

Tersedia metode:
- `.with_success(msg)`: Pesan sukses hijau.
- `.with_error(msg)`: Pesan error merah.
- `.with_errors(hashmap)`: Error validasi per field.
- `.with_input(struct/json)`: Menyimpan data form agar tidak hilang (*Old Input*).

### Extractor & Dependency Injection

Lumina menggunakan sistem *Extractor* milik Axum untuk menyuntikkan data secara otomatis ke dalam parameter fungsi Controller (Dependency Injection).

| Parameter | Nama Extractor | Fungsi |
|-----------|----------------|--------|
| `State(state)` | `State<AppState>` | Memberikan akses ke database, view engine, dan service global. |
| `session` | `Session` | Akses ke session user (Flash messages, data login, dll). |
| `token` | `CsrfToken` | Digunakan untuk validasi CSRF dan sinkronisasi token di form. |
| `user` | `AuthUser` | Mengambil data user yang sedang login secara otomatis. |
| `Path(id)` | `Path<T>` | Mengambil parameter ID dari URL (misal: `/users/:id`). |
| `ValidatedForm(f)`| `ValidatedForm<T>` | Mengambil data form dan memvalidasinya secara otomatis. |

#### Contoh Penggunaan Lengkap:

```rust
pub async fn profile(
    State(state): State<AppState>, // Inject State
    user: AuthUser,                // Inject User Login
    session: Session               // Inject Session
) -> impl IntoResponse {
    View::make("user.profile")
        .with("user", user)
        .render(&state, &session)
        .await
}
```

---

## Model & Database

### Model Trait

Setiap model mengimplementasikan trait `Model` dari `crate::database::model`:

```rust
use async_trait::async_trait;
use serde::{Serialize, Deserialize};
use sqlx::FromRow;
use crate::database::{connection::DatabasePool, model::Model};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Product {
    pub id:    i64,
    pub name:  String,
    pub price: f64,
}

#[async_trait]
impl Model for Product {
    const TABLE: &'static str = "products";

    async fn find(pool: &DatabasePool, id: i64) -> Result<Self, sqlx::Error> {
        sqlx::query_as::<_, Product>(
            "SELECT id, name, price FROM products WHERE id = ? AND deleted_at IS NULL"
        )
        .bind(id)
        .fetch_one(&pool.pool)
        .await
    }

    async fn all(pool: &DatabasePool) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Product>(
            "SELECT id, name, price FROM products WHERE deleted_at IS NULL ORDER BY id ASC"
        )
        .fetch_all(&pool.pool)
        .await
    }

    async fn save(&self, pool: &DatabasePool) -> Result<i64, sqlx::Error> {
        let result = sqlx::query(
            "INSERT INTO products (name, price) VALUES (?, ?)"
        )
        .bind(&self.name)
        .bind(self.price)
        .execute(&pool.pool)
        .await?;
        Ok(result.last_insert_id().unwrap_or(0))
    }

    async fn delete(pool: &DatabasePool, id: i64) -> Result<bool, sqlx::Error> {
        sqlx::query(
            "UPDATE products SET deleted_at = CURRENT_TIMESTAMP WHERE id = ?"
        )
        .bind(id)
        .execute(&pool.pool)
        .await?;
        Ok(true)
    }
}
```

### Menggunakan Model di Controller

```rust
use crate::database::model::Model;
use crate::app::models::product::Product;

pub async fn index(State(state): State<Arc<AppState>>) -> Json<Value> {
    match Product::all(&state.db).await {
        Ok(products) => Json(json!({"status": "success", "data": products})),
        Err(e)       => Json(json!({"status": "error", "message": e.to_string()})),
    }
}

pub async fn show(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>
) -> Json<Value> {
    match Product::find(&state.db, id).await {
        Ok(product) => Json(json!({"status": "success", "data": product})),
        Err(_)      => Json(json!({"status": "error", "message": "Not found"})),
    }
}
```

### Soft Delete

Lumina menggunakan **soft delete** secara default. Record tidak benar-benar dihapus,
melainkan kolom `deleted_at` diisi timestamp:

```sql
-- Di migrasi, tambahkan kolom ini:
deleted_at TIMESTAMP NULL DEFAULT NULL
```

---

## Validasi Request

### ValidatedJson (untuk API)

```rust
use serde::Deserialize;
use validator::Validate;
use crate::core::validation::ValidatedJson;

#[derive(Debug, Deserialize, Validate)]
pub struct CreateProductRequest {
    #[validate(length(min = 3, message = "Nama minimal 3 karakter"))]
    pub name: String,

    #[validate(range(min = 0.0, message = "Harga tidak boleh negatif"))]
    pub price: f64,
}

pub async fn store(
    ValidatedJson(payload): ValidatedJson<CreateProductRequest>
) -> Json<Value> {
    // Jika sampai sini, data sudah valid
    Json(json!({"status": "success", "name": payload.name}))
}
```

Jika validasi gagal, Lumina otomatis mengembalikan **HTTP 422**:

```json
{
  "message": "The given data was invalid.",
  "errors": {
    "name": ["Nama minimal 3 karakter"],
    "price": ["Harga tidak boleh negatif"]
  }
}
```

### ValidatedForm (Otomatis & Cerdas)

`ValidatedForm<T>` adalah cara paling elegan untuk menangani form HTML. Berbeda dengan `ValidatedJson`, extractor ini **otomatis** melakukan interupsi jika data tidak valid.

```rust
use crate::core::validation::ValidatedForm;

pub async fn store(
    // Jika validasi gagal, Lumina otomatis:
    // 1. Redirect balik ke halaman asal (Referer).
    // 2. Flash pesan error ke session.
    // 3. Simpan data input ke session (Old Input).
    ValidatedForm(payload): ValidatedForm<CreateProductRequest>
) -> impl IntoResponse {
    // Jika kode ini jalan, berarti data SUDAH PASTI VALID.
    Redirect::to("/products").with_success("Produk dibuat!").send(&session).await
}
```

### Aturan Validasi yang Tersedia

| Aturan | Contoh |
|--------|--------|
| Panjang string | `#[validate(length(min = 3, max = 100))]` |
| Format email | `#[validate(email)]` |
| Rentang angka | `#[validate(range(min = 1, max = 999))]` |
| Wajib diisi | Field non-Option otomatis wajib diisi |
| Regex | `#[validate(regex(path = *MY_REGEX))]` |

---

## Template Engine

Lumina menggunakan **Tera** — template engine mirip Jinja2/Blade.

Template disimpan di `resources/views/**/*.html`.

### Rendering View

```rust
use tera::Context;

pub async fn index(State(state): State<Arc<AppState>>) -> Html<String> {
    let mut ctx = Context::new();
    ctx.insert("title", "Halaman Produk");
    ctx.insert("user_name", "Budi");

    Html(state.view.render("products/index.html", &ctx))
}
```

### Contoh Template (`resources/views/products/index.html`)

```html
{% extends "layout.html" %}

{% block content %}
<h1>{{ title }}</h1>
<p>Selamat datang, {{ user_name }}!</p>

{% if products %}
<ul>
  {% for product in products %}
  <li>{{ product.name }} — Rp{{ product.price }}</li>
  {% endfor %}
</ul>
{% else %}
<p>Belum ada produk.</p>
{% endif %}
{% endblock %}
```

### Layout (`resources/views/layout.html`)

```html
<!DOCTYPE html>
<html lang="id">
<head>
    <meta charset="UTF-8">
    <title>{% block title %}Lumina{% endblock %}</title>
</head>
<body>
    {% block content %}{% endblock %}
</body>
</html>
```

---

## Autentikasi JWT

### Register via API

**Endpoint:** `POST /api/auth/register`

```json
{
  "name": "Slamet Sugandi",
  "email": "slamet@lumina.rs",
  "password": "rahasia123"
}
```

**Response sukses:**
```json
{"success": true, "message": "User registered", "id": 1}
```

### Login via API

**Endpoint:** `POST /api/auth/login`

```json
{
  "email": "slamet@lumina.rs",
  "password": "rahasia123"
}
```

**Response sukses:**
```json
{"success": true, "token": "eyJ0eXAiOiJKV1QiLCJhbGci..."}
```

### Mengakses Route Terproteksi

Sertakan token di header:

```
Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGci...
```

**Endpoint:** `GET /api/auth/me`

```json
{"success": true, "data": {"id": 1, "name": "Authenticated User"}}
```

### JWT Claims

Token JWT berisi:

```json
{
  "sub": "1",
  "email": "slamet@lumina.rs",
  "role": "user",
  "iat": 1714218000,
  "exp": 1714304400
}
```

Token expire dalam **24 jam** secara default.

---

## Service Layer

Service layer memisahkan business logic dari controller.

### Membuat Service

```rust
// src/app/services/product_service.rs
use std::sync::Arc;
use crate::database::connection::DatabasePool;
use crate::app::models::product::Product;
use crate::database::model::Model;

pub struct ProductService {
    db: Arc<DatabasePool>,
}

impl ProductService {
    pub fn new(db: Arc<DatabasePool>) -> Self {
        Self { db }
    }

    pub async fn create(&self, name: String, price: f64) -> Result<i64, String> {
        let product = Product { id: 0, name, price };
        product.save(&self.db)
            .await
            .map_err(|e| format!("DB error: {}", e))
    }

    pub async fn get_all(&self) -> Result<Vec<Product>, String> {
        Product::all(&self.db)
            .await
            .map_err(|e| format!("DB error: {}", e))
    }
}
```

### Mendaftarkan Service ke AppState

Tambahkan di `src/core/application.rs`:

```rust
pub struct AppState {
    pub db:              Arc<DatabasePool>,
    pub view:            ViewEngine,
    pub auth_service:    Arc<AuthService>,
    pub product_service: Arc<ProductService>,   // ← tambahkan
}
```

---

## Middleware

### Middleware yang Tersedia

| Middleware | Lokasi | Fungsi |
|------------|--------|--------|
| `logger` | `src/http/middleware.rs` | Log setiap request |
| `auth_required` | `src/http/middleware.rs` | Validasi JWT Bearer token |

### Menerapkan Middleware ke Route Group

```rust
use axum::middleware::from_fn;
use crate::http::middleware::auth_required;

let protected = AxumRouter::new()
    .route("/dashboard", routing::get(DashboardController::index))
    .route_layer(from_fn(auth_required));
```

### Membuat Middleware Kustom

```rust
// src/http/middleware.rs
use axum::{
    middleware::Next,
    response::Response,
    http::Request,
    body::Body,
};

pub async fn my_middleware(
    req: Request<Body>,
    next: Next,
) -> Response {
    println!("→ Before handler");
    let response = next.run(req).await;
    println!("← After handler");
    response
}
```

Terapkan ke router:

```rust
use axum::middleware::from_fn;

Router::new()
    .get("/path", handler)
    .layer(from_fn(my_middleware))
```

---

## CLI Tools

Lumina memiliki CLI bawaan untuk mempercepat development.

### Menjalankan CLI

```bash
# Windows
lumina.bat <command>

# Linux/macOS
./lumina <command>
```

### Perintah yang Tersedia

```bash
# Menjalankan server
lumina serve
lumina serve --port 3000

# Membuat file baru
lumina make:controller ProductController
lumina make:model Product
lumina make:service ProductService
lumina make:middleware RateLimiter

# Database
lumina migrate
lumina migrate:fresh
```

### Contoh Output

```
✨ Lumina Framework v0.1.0
🔄 Running migrations...
✅ Migrations done.
🌐 Listening on http://127.0.0.1:8000
```

---

## 🎓 Tutorial: Membangun CRUD Produk

Tutorial langkah demi langkah membangun fitur CRUD produk.

### Langkah 1 — Setup

```bash
cp .env.example .env
# Edit DATABASE_URL di .env
cargo run
```

### Langkah 2 — Tambah Migrasi

Buat file di `database/migrations/` (migrasi berjalan otomatis saat startup):

```sql
CREATE TABLE IF NOT EXISTS products (
    id         INTEGER PRIMARY KEY AUTOINCREMENT,
    name       TEXT    NOT NULL,
    price      REAL    NOT NULL DEFAULT 0,
    deleted_at TIMESTAMP NULL DEFAULT NULL
);
```

### Langkah 3 — Buat Model

`src/app/models/product.rs` — implementasikan trait `Model` (lihat bagian [Model & Database](#model--database)).

Daftarkan di `src/app/models/mod.rs`:
```rust
pub mod product;
```

### Langkah 4 — Buat Controller

`src/app/controllers/product_controller.rs`:

```rust
use axum::{extract::{State, Path}, Json};
use std::sync::Arc;
use serde_json::{json, Value};
use crate::core::application::AppState;
use crate::database::model::Model;
use crate::app::models::product::Product;

pub struct ProductController;

impl ProductController {
    pub async fn index(State(state): State<Arc<AppState>>) -> Json<Value> {
        match Product::all(&state.db).await {
            Ok(p)  => Json(json!({"status": "success", "data": p})),
            Err(e) => Json(json!({"status": "error", "message": e.to_string()})),
        }
    }

    pub async fn show(
        State(state): State<Arc<AppState>>,
        Path(id): Path<i64>
    ) -> Json<Value> {
        match Product::find(&state.db, id).await {
            Ok(p)  => Json(json!({"status": "success", "data": p})),
            Err(_) => Json(json!({"status": "error", "message": "Not found"})),
        }
    }
}
```

### Langkah 5 — Daftarkan Route

`routes/api.rs`:

```rust
.route("/products",     routing::get(ProductController::index))
.route("/products/:id", routing::get(ProductController::show))
```

### Langkah 6 — Test

```bash
# Daftar semua produk
curl http://localhost:8000/api/products

# Produk by ID
curl http://localhost:8000/api/products/1
```

---

## 🐛 Troubleshooting

| Masalah | Solusi |
|---------|--------|
| `❌ Gagal connect ke database` | Periksa `DATABASE_URL` di `.env` |
| `❌ Parsing error(s)` saat startup | Periksa sintaks template HTML di `resources/views/` |
| JWT token invalid | Pastikan `JWT_SECRET` sama di semua environment |
| `cargo run` gagal build | Jalankan `cargo check` untuk melihat error detail |

---

## 🛠️ Debugging Tools (Premium)

Lumina dilengkapi dengan alat bantu debugging premium:

### Dump & Die (`dd!`)

Gunakan makro `dd!()` di mana saja dalam kode Rust Anda untuk menghentikan eksekusi dan menampilkan data secara visual di browser dengan tampilan *Dark Mode* yang elegan.

```rust
let user = User::find(&db, 1).await?;
dd!(user); // Eksekusi berhenti di sini dan merender UI debugger
```

### Template Dump (`dump`)

Dalam template Tera, Anda bisa melihat isi variabel menggunakan filter `dump`:

```html
{{ dump(var=products) }}
```

---

> **Lumina Framework** — Built with ❤️ and 🦀 Rust
>
> Lisensi: MIT | Penulis: Slamet Sugandi <packercyber@gmail.com>
