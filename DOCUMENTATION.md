# 📚 Lumina Framework — Dokumentasi Lengkap

> Dokumentasi detail penggunaan Lumina Framework.
> Untuk quick start, lihat [README.md](./README.md).

---

## 📑 Daftar Isi

1. [Arsitektur](#arsitektur)
2. [Instalasi & Setup](#instalasi--setup)
3. [Konfigurasi](#konfigurasi)
4. [Routing](#routing)
5. [Middleware & Custom Security Guards](#middleware--custom-security-guards)
6. [Controller & View](#controller--view)
7. [Model & Database (ORM)](#model--database-orm)
8. [Validasi Request](#validasi-request)
9. [Autentikasi & Facade Auth](#autentikasi--facade-auth)
10. [Session & Persistence](#session--persistence)
11. [Debugging (dd!)](#debugging-dd)
12. [CLI Tools](#cli-tools)
13. [Eager Loading (N+1 Solution)](#eager-loading-n1-solution)
14. [HTMX Integration (Zero-Mouse UI)](#htmx-integration-zero-mouse-ui)
15. [File Upload & Image Manipulation (Advanced)](#file-upload--image-manipulation-advanced)
16. [Persistent Queue System](#persistent-queue-system)
17. [Blade Template Syntax](#blade-template-syntax)
18. [Logging & Debugging](#logging--debugging)
19. [Rate Limiting](#rate-limiting)
20. [**CHEAT SHEET (Quick Reference)**](./CHEAT_SHEET.md)

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

### 1-Line Relationship Macros (Eloquent & Human-Friendly)
Lumina menyediakan makro relasi 1-baris yang ekspresif, otomatis menangani type safety dan query:

```rust
use lumina::prelude::*;

#[derive(LuminaModel, Clone, Debug)]
#[table("posts")]
pub struct Post {
    pub id: i64,
    pub user_id: i64,
    pub title: String,
}

impl Post {
    // Relasi Belongs To: posts.user_id -> users.id
    belongs_to!(user, User, "user_id");
}

#[derive(LuminaModel, Clone, Debug)]
#[table("users")]
pub struct User {
    pub id: i64,
    pub name: String,
}

impl User {
    // Relasi Has Many: users.id -> posts.user_id
    has_many!(posts, Post, "user_id");

    // Relasi Has One
    has_one!(profile, Profile, "user_id");

    // Fungsi custom concurrent dengan alias &Db dan OrmResult
    pub async fn active_users(db: &Db) -> OrmResult<Vec<Self>> {
        Self::query(db).where_eq("is_active", 1).get().await
    }
}
```

### Shortcut `or_404` untuk Not Found Exception
Mengubah `Option<T>` langsung menjadi respon `404 Not Found` JSON atau HTML secara elegan:
```rust
let post = Post::find(&db, id).await.or_404("Artikel tidak ditemukan")?;
```

> 📖 **Panduan Lengkap:** Untuk tutorial mendalam tentang ORM, relasi (HasOne, HasMany, ManyToMany, BelongsToMany), dan Eager Loading, silakan baca [**Tutorial Database ORM**](./docs/ORM.md) dan [**Tutorial Relationships**](./docs/RELATIONSHIPS.md).

---

---

## Eager Loading (N+1 Solution)

Eager Loading digunakan untuk mengambil data relasi secara efisien dalam satu query tambahan (menggunakan `WHERE IN`), menghindari masalah N+1 query yang lambat.

### Cara Penggunaan
Cukup tambahkan `.with("nama_relasi")` pada saat membangun query.

```rust
// Mengambil user beserta semua posts mereka (Hanya 2 query total)
let users = User::query(&db).with("posts").get().await?;

for u in users {
    // Data posts tersedia di Option<Vec<Post>>
    if let Some(posts) = u.posts {
        println!("User {} punya {} posts", u.name, posts.len());
    }
}
```

### Konfigurasi Model
Untuk mendukung Eager Loading, model harus mengimplementasikan metode `eager_load` pada trait `Model`.

```rust
#[async_trait]
impl Model for User {
    // ...
    async fn eager_load(models: &mut [Self], db: &DatabasePool, relation: &str) {
        if relation == "posts" {
            let ids: Vec<i64> = models.iter().map(|m| m.id).collect();
            let all_posts = Post::query(db).where_in("user_id", &ids).get().await.unwrap_or_default();

            for model in models {
                let user_posts = all_posts.iter()
                    .filter(|p| p.user_id == model.id)
                    .cloned()
                    .collect();
                model.posts = Some(user_posts);
            }
        }
    }
}
```

---

## HTMX Integration (Zero-Mouse UI)

Lumina terintegrasi dengan **HTMX** untuk memberikan pengalaman Single Page Application (SPA) tanpa perlu menulis JavaScript. Ini sangat mendukung workflow **Keyboardholic**.

### Fitur Utama
1.  **Partial Rendering**: Lumina otomatis mendeteksi request HTMX dan dapat merender hanya bagian konten tanpa layout penuh.
2.  **HX-Boost**: Semua link standar otomatis dikonversi menjadi request HTMX yang cepat.
3.  **is_htmx Detection**: Di dalam controller, Anda bisa mengecek `req.is_htmx()` untuk memberikan respon yang berbeda.

### Penggunaan di Controller
Gunakan `req.is_htmx()` untuk logika percabangan jika diperlukan. Secara default, `View::render(&req)` sudah menyuntikkan variabel `is_htmx` ke template.

```rust
pub async fn index(req: Request) -> impl IntoResponse {
    View::make("dashboard.index")
        .render(&req) // Otomatis mengirim variabel is_htmx ke template
        .await
}
```

### Penggunaan di Template
Lumina menggunakan pola kondisinal di `layout.blade.rs` untuk menangani partial render:

```html
{% if is_htmx %}
    {% block content %}{% endblock %}
{% else %}
    <!-- Full Layout with Scripts & Styles -->
    <body hx-boost="true">
        {% block content %}{% endblock %}
    </body>
{% endif %}
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
└── resources/views/   ← Tera .blade.rs templates
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

Lumina Router menyediakan fluent API modern yang membungkus Axum, memungkinkan pendaftaran route web dan API, grouping, serta penyematan middleware secara ringkas dan deklaratif.

### Web Routes (`src/routes/web.rs`)

Daftarkan route HTML di `src/routes/web.rs` menggunakan fluent builder:

```rust
use lumina::prelude::*;
use crate::app::controllers::welcome_controller::WelcomeController;

pub fn router() -> Router<AppState> {
    Router::new()
        .get("/",           WelcomeController::index)
        .get("/login",      WelcomeController::login)
        .get("/register",   WelcomeController::register)
}
```

### API Routes (`src/routes/api.rs`)

Semua route di `src/routes/api.rs` otomatis diprefix dengan `/api` oleh engine `Application`. Anda dapat mengelompokkan route dengan `.group()` serta menyematkan middleware secara langsung:

```rust
use lumina::prelude::*;
use crate::app::controllers::api::*;

pub fn router() -> Router<AppState> {
    Router::new()
        // Public API
        .get("/health", api_health)
        .post("/auth/login", AuthController::api_login)

        // Protected API Group (v1)
        .group("/v1", |r| {
            r.middleware(from_fn(middleware::api_auth_with_pairing))
             .get("/users", UserController::index)
             .get("/users/:id", UserController::show)
             .post("/users", UserController::store)
        })
}
```

### HTTP Methods yang Didukung

| Method | Builder Method |
|---|---|
| GET | `.get(path, handler)` |
| POST | `.post(path, handler)` |
| PUT | `.put(path, handler)` |
| PATCH | `.patch(path, handler)` |
| DELETE | `.delete(path, handler)` |

### Route Parameters

Gunakan `:param` pada path, lalu ambil melalui bundle extractor `Request` atau `Path<T>`:

```rust
// Cara 1: Menggunakan Request Bundle Extractor (Direkomendasikan)
pub async fn show(req: Request) -> impl IntoResponse {
    let id: i64 = req.param("id").unwrap_or_default();
    // ...
}

// Cara 2: Menggunakan Axum Path Extractor
pub async fn show_alt(Path(id): Path<i64>) -> impl IntoResponse {
    // ...
}
```

---

## Middleware & Custom Security Guards

Lumina Framework menyediakan pipeline middleware berkinerja tinggi berbasis Tower & Axum. Anda dapat membuat custom middleware dengan mudah untuk validasi JWT Bearer token, header khusus (seperti `X-Signature`, `X-Token-Pairing`, `X-Timestamp`), API Key, rate limiting, logging, hingga tenancy context.

Middleware dapat disematkan di 3 level:
1. **Global Application**: Dijalankan pada SEMUA request aplikasi (Web, API, health check, dll).
2. **Pipeline (Web vs API)**: Dijalankan khusus pada grup Web (`/`) atau API (`/api`).
3. **Route & Group Level**: Dijalankan pada route spesifik atau sub-group (misal `/api/v1/secure`).

### 1. Struktur Pembuatan Custom Middleware

Middleware di Lumina adalah fungsi async standar dengan parameter `(req: Request, next: Next) -> Response`:

```rust
use lumina::http::middleware::{Request, Next, Response, unauthorized_json, get_header};

pub async fn my_custom_api_guard(req: Request, next: Next) -> Response {
    // 1. Logika Pre-Request (Sebelum controller dipanggil)
    let api_key = match get_header(&req, "X-Api-Key") {
        Some(k) if k == "secret-key-123" => k,
        _ => return unauthorized_json("Akses ditolak: X-Api-Key tidak valid atau tidak ditemukan."),
    };

    // 2. Teruskan ke handler berikutnya
    let mut response = next.run(req).await;

    // 3. Logika Post-Request (Sesudah controller selesai)
    response.headers_mut().insert(
        "X-Served-By",
        "Lumina-Engine".parse().unwrap(),
    );

    response
}
```

### 2. Custom Auth API dengan JWT + `X-Signature` & `X-Token-Pairing`

Pada skenario perbankan, fintech, open banking (SNAP BI), atau client mobile, API membutuhkan proteksi ganda: validasi JWT Bearer ditambah header keamanan seperti `X-Signature` dan `X-Token-Pairing`.

Lumina telah menyertakan guard siap pakai di `lumina::http::middleware`:

#### A. Verifikasi JWT Bearer + `X-Token-Pairing` (`api_auth_with_pairing`)
Memastikan Bearer JWT valid DAN header `X-Token-Pairing` (device/session binding) disertakan:

```rust
use lumina::prelude::*;
use lumina::http::middleware::api_auth_with_pairing;

pub fn router() -> Router<AppState> {
    Router::new()
        .group("/mobile", |r| {
            r.middleware(from_fn(api_auth_with_pairing))
             .get("/profile", user_profile)
             .post("/transfer", do_transfer)
        })
}
```

#### B. Verifikasi API Signature & Timestamp (`verify_api_signature`)
Memvalidasi header `X-Signature` dan `X-Timestamp` (sangat cocok untuk Webhook atau Open API):

```rust
use lumina::prelude::*;
use lumina::http::middleware::verify_api_signature;

pub fn router() -> Router<AppState> {
    Router::new()
        .group("/webhooks", |r| {
            r.middleware(from_fn(verify_api_signature))
             .post("/bca-notify", bca_webhook_handler)
        })
}
```

#### C. Full Composite Security Guard (`api_secure_guard`)
Kombinasi lengkap: memvalidasi JWT Bearer + `X-Signature` + `X-Token-Pairing` sekaligus:

```rust
use lumina::prelude::*;
use lumina::http::middleware::api_secure_guard;

pub fn router() -> Router<AppState> {
    Router::new()
        .group("/vault", |r| {
            r.middleware(from_fn(api_secure_guard))
             .get("/secrets", secret_data_handler)
        })
}
```

### 3. Pendaftaran Middleware di `Application` (`src/main.rs`)

Anda dapat mendaftarkan middleware langsung pada builder `Application`:

```rust
use lumina::prelude::*;
use lumina::http::middleware::from_fn;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let app = Application::new()
        .with_web(routes::web::router())
        .with_api(routes::api::router())
        // 1. Global Middleware: Berjalan di seluruh request (Web, API, health check)
        .with_global_middleware(from_fn(custom_global_tracker))
        // 2. Khusus Web Pipeline: Berjalan hanya pada request Web (HTML/Blade)
        .with_web_middleware(from_fn(web_traffic_logger))
        // 3. Khusus API Pipeline: Berjalan hanya pada request API (/api/*)
        .with_api_middleware(from_fn(api_rate_limiter_or_cors));

    app.serve("127.0.0.1:8000").await;
}
```

### 4. Pendaftaran Middleware di Routing (`routes/web.rs` & `routes/api.rs`)

#### Route Group Middleware:
```rust
use lumina::prelude::*;

pub fn router() -> Router<AppState> {
    Router::new()
        .get("/open", public_ping)
        
        // Group route dengan middleware
        .group("/v1", |r| {
            r.middleware(from_fn(middleware::api_auth_with_pairing))
             .get("/orders", list_orders)
             .post("/orders", create_order)
        })
}
```

#### Route-Level Middleware (`route_middleware`):
```rust
Router::new()
    .get("/reports/export", export_handler)
    .route_middleware(from_fn(verify_export_permission))
```

### 5. Helper Response & Header Utility Bawaan Lumina

Lumina menyediakan kumpulan helper ringkas di `lumina::http::middleware` agar kode middleware Anda tetap rapi tanpa boilerplate manual:

| Helper Function | Deskripsi |
|---|---|
| `get_header(&req, "Header-Name")` | Mengambil nilai string dari header request (Option<&str>). |
| `get_bearer_token(&req)` | Mengambil token JWT dari header `Authorization: Bearer <token>`. |
| `validate_required_headers(&req, &["A", "B"])` | Memastikan semua header wajib tersedia dan tidak kosong. |
| `unauthorized_json("Pesan")` | Mengembalikan Response HTTP 401 Unauthorized dalam format JSON standar Lumina. |
| `forbidden_json("Pesan")` | Mengembalikan Response HTTP 403 Forbidden dalam format JSON standar Lumina. |
| `bad_request_json("Pesan")` | Mengembalikan Response HTTP 400 Bad Request dalam format JSON standar Lumina. |
| `error_json(StatusCode, "Pesan")` | Mengembalikan Response JSON kustom dengan status code yang ditentukan. |

Format JSON error standar Lumina:
```json
{
  "metaData": {
    "code": "401",
    "message": "Token autentikasi tidak ditemukan. Sertakan header: Authorization: Bearer <token>"
  },
  "response": null
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

        let html = state.view.render("products/index.blade.rs", &ctx);
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

### ⚡ Lumina Request & Fluent API (Laravel-Style)

Untuk mempermudah kurva belajar dan mengurangi import boilerplate, Lumina menyediakan bundle extractor `Request` yang dibekali dengan *shortcut methods* ekspresif bergaya Laravel.

| Shortcut Method | Return Type | Kegunaan |
|-----------------|-------------|----------|
| `req.db()` | `&DatabasePool` | Akses langsung ke database pool |
| `req.view(tpl)` | `ViewBuilder` | Membuat view builder baru |
| `req.redirect(p)`| `Redirect` | Membuat instance Laravel-style redirect |
| `req.back()` | `Redirect` | Membuat redirect kembali ke halaman asal |
| `req.session_get(k)`| `Option<T>` | Mengambil data session |
| `req.session_set(k, v)`| `()` | Menyimpan data ke session |
| `req.json(v)` | `Response` | Mengembalikan JSON response instan |

#### Contoh Penggunaan Praktis:

```rust
use crate::core::request::Request;

pub struct HomeController;

impl HomeController {
    pub async fn index(req: Request) -> impl IntoResponse {
        // req.view() otomatis menggunakan ViewEngine di state
        req.view("home.index")
            .with("title", "Lumina Framework")
            .render(&req)
            .await
    }

    pub async fn store(req: Request) -> impl IntoResponse {
        // req.redirect() mempermudah redirect response
        req.redirect("/dashboard")
            .with_success("Data berhasil disimpan!")
            .go(&req)
            .await
    }
}
```

---

## Model & Lumina Eloquent ORM

Lumina hadir dengan **ORM Mandiri (Eloquent / Active Record)** yang anggun dan ekspresif. Anda tidak perlu lagi menulis raw SQL query string atau melakukan konversi parameter manual (`?` vs `$1`). Lumina menangani cross-database query (PostgreSQL, SQLite, MySQL) secara transparan!

### Mendeklarasikan Model (`#[derive(LuminaModel)]`)

Cukup tambahkan derive `LuminaModel` pada struct model Anda:

```rust
use lumina::prelude::*;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow, Clone, Default, LuminaModel)]
#[lumina(table = "products")]
pub struct Product {
    pub id: i64,
    pub name: String,
    pub price: i64,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub deleted_at: Option<String>,
}

impl Product {
    pub fn new() -> Self {
        Self::default()
    }
}
```

Dengan satu derive di atas, Anda langsung mendapatkan fitur Eloquent lengkap:
- `Product::all(db).await?` — Ambil semua record aktif (ORDER BY id DESC).
- `Product::find(db, id).await?` — Mengembalikan `Ok(Some(item))` atau `Ok(None)`.
- `Product::find_or_fail(db, id).await?` — Mengembalikan record atau error jika tidak ada.
- `Product::where_eq(db, "name", "Sepatu").first().await?` — Fluent query builder.
- `Product::latest(db).paginate(page, per_page).await?` — Paginasi bawaan.
- `Product::count(db).await?` — Hitung total record aktif.
- `$model.save(db).await?` — Otomatis `INSERT` jika ID == 0, atau `UPDATE` jika ID > 0.
- `Product::destroy(db, id).await?` / `$model.delete(db).await?` — Soft delete instan.

---

### Controller yang Cantik & Ergonomis (Laravel Style)

Dengan `Request` extractor dan `LuminaResult`, controller Anda menjadi sangat ringkas, bersih, dan ekspresif. Tidak ada lagi signature yang panjang atau konversi status code manual:

```rust
use lumina::prelude::*;
use crate::app::models::product::Product;

pub struct ProductController;

impl ProductController {
    /// GET /api/products — Ambil seluruh data
    pub async fn index(req: Request) -> LuminaResult {
        let products = Product::all(req.db()).await?;
        req.ok(products)
    }

    /// GET /api/products/paged — Paginasi instan (otomatis membaca ?page=X&per_page=Y)
    pub async fn paginate(req: Request) -> LuminaResult {
        let (page, per_page) = req.pagination();
        let paginator = Product::paginate(req.db(), page, per_page).await?;
        req.paginate(paginator)
    }

    /// GET /api/products/:id — Detail satu produk (otomatis membaca :id dari route)
    pub async fn show(req: Request) -> LuminaResult {
        let id = req.id_or_fail()?;
        match Product::find(req.db(), id).await? {
            Some(product) => req.ok(product),
            None => req.not_found("Produk tidak ditemukan"),
        }
    }

    /// POST /api/products — Simpan data baru (Active Record)
    pub async fn store(req: Request, ValidatedJson(input): ValidatedJson<CreateProductInput>) -> LuminaResult {
        let mut product = Product {
            name: input.name,
            price: input.price,
            ..Default::default()
        };
        let id = product.save(req.db()).await?;
        product.id = id;
        req.created_with(product, "Produk berhasil ditambahkan")
    }

    /// PUT /api/products/:id — Update data
    pub async fn update(
        req: Request,
        ValidatedJson(input): ValidatedJson<UpdateProductInput>,
    ) -> LuminaResult {
        let id = req.id_or_fail()?;
        let mut product = match Product::find(req.db(), id).await? {
            Some(p) => p,
            None => return req.not_found("Produk tidak ditemukan"),
        };
        product.name = input.name;
        product.price = input.price;
        product.save(req.db()).await?;
        req.success(product, "Produk berhasil diperbarui")
    }

    /// DELETE /api/products/:id — Hapus data
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

Template disimpan di `resources/views/**/*.blade.rs`.

### Rendering View

```rust
use tera::Context;

pub async fn index(State(state): State<Arc<AppState>>) -> Html<String> {
    let mut ctx = Context::new();
    ctx.insert("title", "Halaman Produk");
    ctx.insert("user_name", "Budi");

    Html(state.view.render("products/index.blade.rs", &ctx))
}
```

### Contoh Template (`resources/views/products/index.blade.rs`)

```html
{% extends "layout.blade.rs" %}

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

### Layout (`resources/views/layout.blade.rs`)

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

## File Upload & Image Manipulation (Advanced)

Lumina menyediakan sistem penanganan file yang intuitif dan *fluent*, terinspirasi dari kemudahan Laravel namun dengan performa pemrosesan gambar native Rust yang sangat cepat.

### Ringkasan Penggunaan

Gunakan `LuminaMultipart` di controller untuk mengekstrak file dan data form secara otomatis.

```rust
pub async fn store(req: Request, multipart: LuminaMultipart) -> impl IntoResponse {
    if let Some(file) = multipart.file("image") {
        // Manipulasi & Simpan
        let path = file.clone()
            .thumbnail(300, 300)
            .grayscale()
            .store(&req, "avatars").await?;

        return req.redirect("/profile").with_success("Avatar diupdate!").go(&req).await;
    }
}
```

> 📖 **Panduan Lengkap:** Untuk tutorial mendalam tentang manipulasi gambar, silakan baca [**Tutorial File Upload & Image Manipulation**](./docs/FILE_UPLOAD.md).

---

## 🐛 Troubleshooting

| Masalah | Solusi |
|---------|--------|
| `❌ Gagal connect ke database` | Periksa `DATABASE_URL` di `.env` |
| `❌ Parsing error(s)` saat startup | Periksa sintaks template .blade.rs di `resources/views/` |
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

## Persistent Queue System

Lumina memiliki sistem antrean (Queue) berbasis database yang persisten. Pekerjaan (Job) disimpan di tabel `jobs` dan diproses di background oleh worker.

### Membuat Job Baru
Gunakan CLI untuk membuat class Job baru:
```bash
./lumina make:job SendEmailJob
```

### Menjalankan Job
```rust
use crate::app::jobs::SendEmailJob;

// Dispatch job ke antrean
state.queue.push(SendEmailJob::new(email, content)).await?;
```

### Queue Resilience
Worker Lumina memiliki mekanisme *retry* otomatis. Jika tabel `jobs` belum tersedia (misal saat startup awal), worker akan menunggu dan mencoba kembali setiap 30 detik tanpa mematikan seluruh aplikasi.

---

## Blade Template Syntax

Lumina menyediakan *preprocessor* yang memungkinkan Anda menulis template dengan sintaks bergaya Laravel Blade. Sintaks ini akan diterjemahkan secara otomatis ke engine `Tera`.

### Direktif yang Tersedia
- `@extends('layout')`: Mewarisi layout (otomatis mencari file `.blade.rs`).
- `@section('name')` / `@endsection`: Mendefinisikan blok konten.
- `@yield('name')`: Menampilkan isi dari sebuah section.
- `@if(condition)` / `@elseif(condition)` / `@else` / `@endif`.
- `@foreach(items as item)` / `@endforeach`.
- `@auth` / `@endauth`: Menampilkan konten hanya jika user sudah login.
- `@guest` / `@endguest`: Menampilkan konten hanya jika user belum login.
- `@csrf`: Men-generate input hidden untuk token CSRF.
- `@include('partial')`: Menyertakan file template lain.

---

## Logging & Debugging

Lumina menggunakan sistem logging terstruktur yang mencetak ke console (development) dan ke file (production).

### Konfigurasi Logging
Atur level log di file `.env`:
```env
LOG_LEVEL=info  # debug, info, warn, error
```

### File Logging
Semua log disimpan secara otomatis di folder `storage/logs/`. File log di-roll setiap hari (misal: `lumina.2026-05-10.log`).

### Debugging Tools
- `dd!($var)`: Dump and Die (merender UI debugger cantik di browser).
- `dump!($var)`: Mencetak variabel ke terminal secara terstruktur tanpa menghentikan eksekusi.

---

## Rate Limiting

Lumina menyertakan middleware Rate Limiter bawaan untuk melindungi aplikasi dari brute-force atau DDoS level aplikasi.

### Konfigurasi
Atur batas request di file `.env`:
```env
RATE_LIMIT_MAX=60     # Maksimal request per window
RATE_LIMIT_WINDOW=60  # Durasi window dalam detik
```

### Penggunaan Per-Route
Anda bisa menerapkan limit yang lebih ketat untuk route tertentu (misal login):
```rust
use crate::core::rate_limit::{RateLimitConfig, rate_limit_with_config};

// Di route builder
.route("/login", post(handler))
.layer(from_fn_with_state(state, |state, req, next| async move {
    rate_limit_with_config(state, RateLimitConfig::strict(), req, next).await
}))
```

---

## 🌍 Lumina Localization (I18n)
Lumina menyediakan sistem multi-bahasa yang terintegrasi dengan middleware dan view engine.
- **Deteksi:** Mendukung deteksi bahasa via query param `?lang=id` atau header `Accept-Language`.
- **Global Function:** Gunakan `{{ __("key") }}` di dalam template Tera/Blade.

## 📡 Lumina Echo (WebSockets)
Sistem real-time broadcasting untuk komunikasi dua arah.
- **Event Broadcasting:** Cukup implementasikan trait `ShouldBroadcast` pada struct event Anda.
- **Client Side:** Gunakan `resources/js/echo.js` untuk mendengarkan event di browser.

## 🔐 Lumina Socialite (OAuth)
Autentikasi modular untuk provider pihak ketiga.
- **Modular Driver:** Mendukung penambahan provider baru dengan mudah.
- **Auto Registration:** Secara otomatis mendaftarkan user baru yang login via sosial media.

## 💾 Lumina Backup System
Sistem perlindungan data otomatis.
- **Full Backup:** Mencakup ekspor database (SQL/SQLite) dan pengarsipan folder storage.
- **Automatic Schedule:** Berjalan otomatis setiap hari jam 02:00 pagi.

---

> **Lumina Framework** — Built with ❤️ and 🦀 Rust
>
> Lisensi: MIT | Penulis: Slamet Sugandi <packercyber@gmail.com>
