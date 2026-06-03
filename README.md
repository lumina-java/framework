# ✨ Lumina Framework

A beautiful, fast, and elegant web framework for Rust.  
Inspired by Laravel's clean and readable syntax, Lumina is designed to be the absolute easiest way for beginners to dive into the Rust ecosystem.

---

## 🚀 Tutorial Memulai (Untuk Pemula)

Apakah kamu baru belajar Rust dan terbiasa dengan kemudahan Laravel atau PHP? Jangan khawatir! Lumina dirancang khusus agar mudah dipahami. Ikuti 3 langkah sederhana ini untuk menjalankan project pertamamu.

### 📦 1. Install Lumina CLI
Pastikan komputer kamu sudah terpasang **Rust** (jika belum, install via [rustup.rs](https://rustup.rs/)). Setelah itu, buka Terminal atau Command Prompt kamu dan ketik perintah berikut untuk menginstall Lumina CLI secara global:

```bash
cargo install cargo-lumina
```
*(Tunggu hingga proses instalasi selesai)*

### 🏗️ 2. Buat Project Baru
Sama halnya seperti `laravel new`, kamu bisa membuat project baru (misal dengan nama `toko_online`) menggunakan perintah ini:

```bash
cargo lumina new toko_online
```
Lumina akan otomatis membuatkan folder `toko_online` beserta seluruh kerangka MVC-nya (Models, Views, Controllers, Routes, dll).

### 🏃 3. Jalankan Server (Database SQLite Otomatis!)
Masuk ke folder project yang baru dibuat, lalu jalankan aplikasinya!

```bash
cd toko_online
cargo run
```
**Selesai! 🎉** Buka browser kamu dan kunjungi 👉 **http://localhost:8000**
Secara default, Lumina sudah menggunakan database **SQLite**. Database ini tersimpan dalam bentuk file (di `database.sqlite`), sehingga kamu **tidak perlu menginstall atau mengatur database apapun di komputer kamu** saat pertama kali mencoba. Sangat praktis!

---

## 🗄️ Mengubah Database dari SQLite ke MySQL

Setelah kamu nyaman dan ingin beralih ke MySQL, mengubahnya sangatlah mudah.
Buka file bernama `.env` di dalam folder project kamu.

Cari bagian konfigurasi database (biasanya di baris-baris tengah) dan ubah dari SQLite ke MySQL.

**Ubah dari:**
```env
DB_CONNECTION=sqlite
DB_DATABASE=./database.sqlite
DATABASE_URL=sqlite:./database.sqlite
```

**Menjadi (Sesuaikan dengan username & password MySQL kamu):**
```env
DB_CONNECTION=mysql
DB_HOST=127.0.0.1
DB_PORT=3306
DB_DATABASE=nama_database_kamu
DB_USERNAME=root
DB_PASSWORD=password_kamu
DATABASE_URL=mysql://root:password_kamu@127.0.0.1:3306/nama_database_kamu
```

*Catatan: Pastikan kamu sudah membuat database kosong bernama `nama_database_kamu` di aplikasi MySQL kamu (misal lewat XAMPP, phpMyAdmin, atau DBeaver). Variabel utama yang dibaca oleh Rust SQLx adalah `DATABASE_URL`.*

Setelah mengubah `.env`, jalankan migrasi agar Lumina membuat tabel di MySQL:
```bash
cargo lumina migrate
```

---

## ✨ Features

- 🚀 **Fast & Efficient** — Built on Tokio + Axum, fully async
- 🎨 **Clean Syntax** — Laravel-like fluent API
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
