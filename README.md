# ✨ Lumina Framework

A beautiful, fast, and elegant web framework for Rust.  
Inspired by Laravel's clean and readable syntax.

---

## ✨ Features

- 🚀 **Fast & Efficient** — Built on Tokio + Axum, fully async
- 🎨 **Clean Syntax** — Laravel-like fluent API
- 🏗️ **MVC Architecture** — Controllers, Models, Services
- 🛣️ **Elegant Routing** — GET, POST, PUT, PATCH, DELETE + route params
- 🔌 **Modular Design** — Web routes & API routes terpisah
- 📦 **Service Container** — Dependency injection
- 🗄️ **Database Layer** — SQLx dengan support SQLite, MySQL, PostgreSQL
- ✅ **Auto Validation** — `ValidatedJson` & `ValidatedForm` dengan pesan error otomatis
- 🔐 **JWT Auth** — Register, Login, Protected routes siap pakai
- 🎭 **Template Engine** — Tera (Jinja2-like) untuk HTML rendering
- 🛡️ **Middleware** — Logger & Auth middleware bawaan

---

## 🚀 Quick Start

### 1. Clone & Konfigurasi

```bash
git clone https://github.com/your-org/lumina.git
cd lumina
cp .env.example .env
```

### 2. Jalankan Server

```bash
cargo run
```

Server berjalan di **http://127.0.0.1:8000** 🎉

---

## 📁 Struktur Project

```
lumina/
├── src/
│   ├── core/          ← Router, Validation, Auth, View Engine
│   ├── http/          ← Server, Middleware
│   ├── app/
│   │   ├── controllers/
│   │   ├── models/
│   │   └── services/
│   └── database/      ← Connection, Migrations, Model trait
├── routes/
│   ├── web.rs         ← HTML routes
│   └── api.rs         ← API/JSON routes
└── resources/views/   ← Template HTML (Tera)
```

---

## ⚡ Contoh Penggunaan

### Routing

```rust
// routes/web.rs
Router::new()
    .get("/",          HomeController::index)
    .get("/users/:id", UserController::show)
    .post("/users",    UserController::store)
```

### Controller

```rust
pub async fn index(State(state): State<Arc<AppState>>) -> Html<String> {
    let mut ctx = Context::new();
    ctx.insert("title", "Selamat Datang");
    Html(state.view.render("home/index.html", &ctx))
}
```

### Validasi Otomatis

```rust
#[derive(Deserialize, Validate)]
pub struct CreateUserRequest {
    #[validate(length(min = 3))]
    pub name: String,
    #[validate(email)]
    pub email: String,
}

pub async fn store(
    ValidatedJson(payload): ValidatedJson<CreateUserRequest>
) -> Json<Value> {
    Json(json!({"status": "success", "data": payload}))
}
```

### API dengan JWT Auth

```bash
# Login
curl -X POST http://localhost:8000/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{"email":"user@lumina.rs","password":"secret123"}'

# Akses route terproteksi
curl http://localhost:8000/api/auth/me \
  -H "Authorization: Bearer <token>"
```

---

## 🛠️ CLI

```bash
lumina serve                          # Jalankan server
lumina make:controller ProductCtrl   # Buat controller baru
lumina make:model Product            # Buat model baru
lumina migrate                       # Jalankan migrasi
```

---

## 📚 Dokumentasi Lengkap

Untuk panduan detail, tutorial CRUD, konfigurasi middleware, dan referensi API lengkap:

**👉 [Baca DOCUMENTATION.md](./DOCUMENTATION.md)**

Topik yang tersedia di dokumentasi:
- Setup & konfigurasi `.env`
- Routing (web, API, parameter, middleware group)
- Controller (HTML & JSON response)
- Model & Database (CRUD, Soft Delete)
- Validasi Request (rules, error format)
- Template Engine Tera
- Autentikasi JWT end-to-end
- Service Layer pattern
- Membuat Middleware kustom
- Tutorial CRUD lengkap

---

## 🔧 Konfigurasi Database

```env
# SQLite (default, tidak perlu instalasi)
DATABASE_URL=sqlite:./lumina.db

# MySQL
DATABASE_URL=mysql://root:password@127.0.0.1/lumina

# PostgreSQL
DATABASE_URL=postgres://postgres:password@127.0.0.1/lumina
```

---

## 📦 Dependencies Utama

```toml
tokio      = "1"      # Async runtime
axum       = "0.7"    # HTTP framework
sqlx       = "0.8"    # Database (async)
tera       = "1.19"   # Template engine
validator  = "0.18"   # Input validation
jsonwebtoken = "9"    # JWT
bcrypt     = "0.15"   # Password hashing
```

---

## 📄 License

MIT — Slamet Sugandi <packercyber@gmail.com>
