# ✨ Lumina Framework

A beautiful, fast, and elegant web framework for Rust.  
Inspired by Laravel's clean and readable syntax.

---

## ✨ Features

- 🚀 **Fast & Efficient** — Built on Tokio + Axum, fully async
- 🎨 **Clean Syntax** — Laravel-like fluent API with `Request` bundle
- 🏗️ **MVC Architecture** — Controllers, Models, Services
- 🛣️ **Elegant Routing** — Route groups, Middleware, and Param binding
- 🪄 **Smart Scaffolding** — Full CRUD generation with fields in one command
- 🧪 **Testing Suite** — Built-in testing with fluent assertions
- 🗄️ **Database Layer** — SQLx, Migrations, Seeders, and Factories
- 🎭 **Template Engine** — Tera with `.blade.rs` preprocessor and HTMX support
- 🔐 **JWT Auth** — Full authentication scaffolding ready to use
- 📁 **File Upload** — Multipart handling & Image manipulation
- 🐳 **Deployment Tools** — Docker, Nginx, and Supervisor auto-scaffolding

---

## 📦 Installation

Anda dapat menginstall Lumina CLI secara global menggunakan cargo:

```bash
cargo install lumina-framework
```

Atau jika Anda ingin menginstall dari source:

```bash
git clone https://github.com/lumina-java/framework.git
cd lumina
cargo install --path .
```

---

## 🚀 Quick Start

### 1. Buat Project Baru
Gunakan CLI untuk membuat skeleton project baru:
```bash
lumina new my-app
cd my-app
cp .env.example .env
```

### 2. Jalankan Server
```bash
cargo run --bin lumina-server
```

Server berjalan di **http://127.0.0.1:8000** 🎉

---

## ⚡ Contoh Penggunaan

### Smart CRUD Scaffolding
Buat modul produk lengkap (Model, Migration, Controller, Views) dalam 1 detik:
```bash
./lumina make:crud Product name:string:required price:integer description:text
```

### Elegant Controller Logic
Gunakan `Request` bundle untuk akses cepat ke segalanya:
```rust
pub async fn store(req: Request, ValidatedForm(form): ValidatedForm<ProductRequest>) -> impl IntoResponse {
    let mut item = Product::new();
    item.name = form.name;
    item.price = form.price;

    let service = ProductService::new(req.db_arc());
    match service.create_model(item).await {
        Ok(_) => req.redirect("/products").with_success("Berhasil!").go(&req).await,
        Err(e) => req.back().with_error("Gagal!").go(&req).await
    }
}
```

### Fluent Testing
```rust
#[tokio::test]
async fn test_homepage_works() {
    let client = TestClient::new().await;
    client.get("/")
        .send()
        .await
        .assert_status(200)
        .assert_text("Selamat Datang");
}
```

---

## 🛠️ CLI Commands

| Command | Deskripsi |
| :--- | :--- |
| `serve` | Jalankan HTTP server |
| `make:crud` | Generate full CRUD scaffolding (Model, Controller, Migration, Views) |
| `make:auth` | Generate Authentication scaffolding (Register, Login, Views) |
| `migrate` | Jalankan database migrations |
| `db:seed` | Seed database dengan data dummy |
| `tinker` | Interactive REPL session |
| `make:docker` | Generate Dockerfile & docker-compose.yml |
| `make:nginx` | Generate Nginx configuration |
| `test:run` | Jalankan semua unit & integration tests |

---

## 📚 Dokumentasi Lengkap

Untuk panduan detail, tutorial CRUD, konfigurasi middleware, dan referensi API lengkap:

**👉 [Baca DOCUMENTATION.md](./DOCUMENTATION.md)**

---

## 📄 License

MIT — Lumina Java <admin@lumina-java.com>
