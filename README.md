# ✨ Lumina Framework

<p align="center">
  <img src="storage/app/public/assets/logo.png" alt="Lumina Framework Logo" width="180">
</p>

<p align="center">
  <strong>A beautiful, ultra-fast, and elegant web framework for Rust.</strong><br>
  <em>Menggabungkan performa dan memory-safety Rust dengan keanggunan serta keramahan sintaks Laravel MVC.</em>
</p>

<p align="center">
  <a href="#-fitur-utama"><img src="https://img.shields.io/badge/Architecture-MVC%20%2B%20Services-6366f1.svg" alt="MVC"></a>
  <a href="#-database--human-orm"><img src="https://img.shields.io/badge/Database-PostgreSQL%20|%20MySQL%20|%20SQLite-06b6d4.svg" alt="Databases"></a>
  <a href="#-cli--smart-scaffolding"><img src="https://img.shields.io/badge/CLI-Smart%20Scaffolding-ec4899.svg" alt="CLI"></a>
  <a href="#-real-time-broadcaster-lumina-echo"><img src="https://img.shields.io/badge/WebSocket-Lumina%20Echo-10b981.svg" alt="Echo"></a>
  <a href="#-license"><img src="https://img.shields.io/badge/License-MIT-amber.svg" alt="License"></a>
</p>

---

## 🌟 Mengapa Lumina?

Rust terkenal dengan kecepatan ekstrem dan keamanannya, namun sering kali memiliki kurva belajar yang curam dan membutuhkan banyak kode boilerplate untuk membuat aplikasi web sehari-hari.

**Lumina Framework** dirancang untuk mengubah hal tersebut. Terinspirasi oleh filosofi **Laravel**, Lumina menyediakan arsitektur MVC yang matang, *Dependency Injection* cerdas, *Human ORM*, sistem migrasi multi-database otomatis, serta CLI scaffolding instan tanpa mengorbankan performa async berbasis **Tokio** dan **Axum**.

---

## 🚀 Fitur Utama

- ⚡ **Supercharged Tokio + Axum Core** — Performa HTTP async tinggi dengan konsumsi resource yang sangat hemat.
- 🏗️ **Arsitektur MVC Lengkap** — Pembagian layer yang rapi: Controllers, Models, Services, Requests, Views, dan Providers.
- 🗄️ **Human ORM & Macro-based Models** — Gunakan `#[derive(LuminaModel)]` untuk mendapatkan fungsi CRUD otomatis, pagination (`LengthAwarePaginator`), eager loading, soft deletes, dan query builder lancar (*fluent*).
- 🔄 **Universal Multi-Database Layer** — Dukungan langsung untuk **PostgreSQL**, **MySQL**, dan **SQLite** dengan auto-migration runner.
- 🪄 **Lumina Smart CLI (`lumina`)** — Buat project baru, controller, model, migration, autentikasi instan (`make:auth`), dan modul CRUD lengkap (`make:crud`) hanya dengan satu perintah.
- 📦 **The Unified `Request` Extractor** — Cukup panggil `req: Request` untuk mengakses Database Pool, Session, CSRF Token, Auth User, Flash Messages, dan View Renderer.
- 🛡️ **Auto-Validation Engine** — `ValidatedJson<T>` & `ValidatedForm<T>` dengan pesan error validasi yang ramah developer.
- 🔐 **Dual Auth Suite** — Autentikasi berbasis **Session** untuk web serta **JWT Bearer Token** untuk API.
- 🎭 **Template Engine Blade + Tera** — Mendukung ekstensi `.blade.rs` dengan direktif familiar (`@extends`, `@section`, `@if`, `@foreach`, `@csrf`), serta compiler **Indonesian Script** (`.is`).
- 📡 **Lumina Echo (WebSocket Broadcaster)** — Broadcasting event real-time dengan channel authorization (Public, Private, Presence) dan verifikasi JWT.
- ⏳ **Background Queue & Jobs** — Eksekusi antrean tugas asinkron dengan persistensi database.
- ⏰ **Task Scheduler** — Penjadwalan cron-job bawaan yang mudah dikonfigurasi.
- 💾 **High-Speed Cache** — Cache in-memory berbasis Moka dengan dukungan tagging dan TTL.
- 📧 **Mail & Notification** — Fluent email builder dengan template rendering dan driver Log/SMTP.
- 📁 **File Storage & Image Manipulation** — Penanganan upload multipart dan manipulasi gambar otomatis (resize, thumbnail, grayscale).
- 🔭 **Telescope & Health Checks** — Endpoint pemantauan `/up` & `/health`, graceful shutdown, dan security headers bawaan.

---

## 🛠️ Instalasi Lumina CLI

Lumina CLI (`lumina`) adalah toolchain utama untuk membuat dan mengelola project Lumina.

### 1. Instalasi Secara Global (Lokal untuk Development)
Untuk menggunakan Lumina dari repositori lokal:
```bash
# Masuk ke direktori framework
cd "d:\FRAMEWORK RUST\framework"

# Install binary lumina secara global dari folder lumina-cli
cargo install --path lumina-cli --force
```

Verifikasi instalasi CLI:
```bash
lumina --help
```

---

## ⚡ Quick Start: Membuat Project Baru

### 1. Generate Project Skeleton
Jalankan perintah ini di direktori kerja Anda (misalnya di `D:\FRAMEWORK RUST`):
```bash
lumina new kontak_manager
```
Perintah ini akan menyiapkan seluruh struktur MVC, routing, dan file konfigurasi.

### 2. Konfigurasi Database (.env)
Buka file `.env` di dalam folder project baru Anda, lalu sesuaikan koneksi database.

#### Contoh PostgreSQL (Port Kustom, misal 54322):
```env
DB_CONNECTION=postgres
DB_HOST=127.0.0.1
DB_PORT=54322
DB_DATABASE=lumina_kontak
DB_USERNAME=postgres
DB_PASSWORD=postgress

DATABASE_URL=postgres://postgres:postgress@127.0.0.1:54322/lumina_kontak
```

#### Contoh SQLite (Default, Tanpa Setup):
```env
DB_CONNECTION=sqlite
DATABASE_URL=sqlite:./database.sqlite
```

#### Contoh MySQL:
```env
DB_CONNECTION=mysql
DB_HOST=127.0.0.1
DB_PORT=3306
DB_DATABASE=nama_db
DB_USERNAME=root
DB_PASSWORD=secret
DATABASE_URL=mysql://root:secret@127.0.0.1:3306/nama_db
```

### 3. Menjalankan Server
```bash
cd kontak_manager
lumina serve
# atau dengan fitur auto-reload:
lumina watch
```
Kunjungi aplikasi di 👉 **http://127.0.0.1:8000** 🎉

---

## 📖 Tutorial: Membuat CRUD API & Service Kontak Manager (PostgreSQL)

Berikut adalah panduan langkah demi langkah implementasi CRUD Kontak Manager menggunakan PostgreSQL di Lumina:

### 1. File Migrasi SQL (`database/migrations/20261016000002_create_contacts_table.sql`)
```sql
CREATE TABLE IF NOT EXISTS contacts (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    email VARCHAR(100) NOT NULL,
    phone VARCHAR(50) NOT NULL,
    address TEXT,
    company VARCHAR(100),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_contacts_deleted_at ON contacts(deleted_at);
```

### 2. Model Kontak (`src/app/models/contact.rs`)
```rust
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use lumina::prelude::*;

#[derive(Debug, Serialize, Deserialize, FromRow, Clone, Default, LuminaModel)]
#[table("contacts")]
pub struct Contact {
    pub id: i64,
    pub name: String,
    pub email: String,
    pub phone: String,
    pub address: Option<String>,
    pub company: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub deleted_at: Option<String>,
}
```

### 3. Request Validation (`src/app/requests/contact_request.rs`)
```rust
use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate, Clone)]
pub struct CreateContactRequest {
    #[validate(length(min = 2, message = "Nama kontak minimal 2 karakter"))]
    pub name: String,

    #[validate(email(message = "Format email tidak valid"))]
    pub email: String,

    #[validate(length(min = 8, message = "Nomor telepon minimal 8 digit"))]
    pub phone: String,

    pub address: Option<String>,
    pub company: Option<String>,
}
```

### 4. Controller Layer yang Cantik & Ergonomis (`src/app/controllers/contact_controller.rs`)

Dengan fitur **Lumina Eloquent ORM** dan **Unified Request Response**, Controller Anda sangat elegan, ekspresif, dan bebas dari boilerplate:

```rust
use lumina::prelude::*;
use crate::app::models::contact::Contact;
use crate::app::requests::contact_request::CreateContactRequest;

pub struct ContactController;

impl ContactController {
    /// GET /api/contacts — Ambil seluruh daftar kontak
    /// GET /api/contacts — Ambil seluruh daftar kontak
    pub async fn api_index(req: Request) -> LuminaResult {
        let contacts = Contact::all(req.db()).await?;
        req.ok(contacts)
    }

    /// GET /api/contacts/paged — Paginasi instan (otomatis membaca ?page=X&per_page=Y)
    pub async fn api_paginate(req: Request) -> LuminaResult {
        let (page, per_page) = req.pagination();
        let paginator = Contact::paginate(req.db(), page, per_page).await?;
        req.paginate(paginator)
    }

    /// GET /api/contacts/:id — Ambil detail satu kontak
    pub async fn api_show(req: Request) -> LuminaResult {
        let id = req.id_or_fail()?;
        match Contact::find(req.db(), id).await? {
            Some(contact) => req.ok(contact),
            None => req.not_found("Kontak tidak ditemukan"),
        }
    }

    /// POST /api/contacts — Simpan kontak baru (Active Record)
    pub async fn api_store(
        req: Request,
        ValidatedJson(payload): ValidatedJson<CreateContactRequest>,
    ) -> LuminaResult {
        let mut contact = Contact {
            name: payload.name,
            email: payload.email,
            phone: payload.phone,
            address: payload.address,
            company: payload.company,
            ..Default::default()
        };
        let id = contact.save(req.db()).await?;
        contact.id = id;
        req.created_with(contact, "Kontak berhasil ditambahkan")
    }

    /// PUT /api/contacts/:id — Update kontak
    pub async fn api_update(
        req: Request,
        ValidatedJson(payload): ValidatedJson<CreateContactRequest>,
    ) -> LuminaResult {
        let id = req.id_or_fail()?;
        let mut contact = match Contact::find(req.db(), id).await? {
            Some(c) => c,
            None => return req.not_found("Kontak tidak ditemukan"),
        };
        contact.name = payload.name;
        contact.email = payload.email;
        contact.phone = payload.phone;
        contact.address = payload.address;
        contact.company = payload.company;
        contact.save(req.db()).await?;
        req.success(contact, "Kontak berhasil diperbarui")
    }

    /// DELETE /api/contacts/:id — Soft delete kontak
    pub async fn api_destroy(req: Request) -> LuminaResult {
        let id = req.id_or_fail()?;
        if Contact::destroy(req.db(), id).await? {
            req.success(serde_json::json!({ "id": id }), "Kontak berhasil dihapus")
        } else {
            req.not_found("Kontak tidak ditemukan")
        }
    }
}
```

### 6. Pendaftaran Routes (`src/routes/api.rs`)
```rust
use lumina::prelude::*;
use crate::app::controllers::contact_controller::ContactController;

pub fn router() -> Router<AppState> {
    Router::new()
        .get("/contacts", ContactController::api_index)
        .post("/contacts", ContactController::api_store)
        .get("/contacts/:id", ContactController::api_show)
        .put("/contacts/:id", ContactController::api_update)
        .delete("/contacts/:id", ContactController::api_destroy)
}
```

---

## 🛠️ Ringkasan Perintah CLI Lumina

| Perintah | Deskripsi |
| :--- | :--- |
| `lumina new <name>` | Membuat project aplikasi baru dari skeleton |
| `lumina serve` | Menjalankan server aplikasi HTTP |
| `lumina watch` | Menjalankan server dengan fitur hot auto-reload (`cargo-watch`) |
| `lumina make:crud <Name> [fields]` | Membuat CRUD lengkap instan (Model, Controller, Migrasi, View) |
| `lumina make:auth` | Menghasilkan modul autentikasi lengkap (Register, Login, Session, Blade) |
| `lumina make:controller <Name>` | Menghasilkan controller baru |
| `lumina make:model <Name>` | Menghasilkan model baru beserta file migrasi |
| `lumina make:migration <Name>` | Membuat file migrasi database baru |
| `lumina migrate` | Menjalankan seluruh file migrasi SQL yang belum diaplikasikan |
| `lumina migrate:rollback` | Membatalkan migrasi terakhir |
| `lumina db:seed` | Menjalankan database seeder |
| `lumina make:docker` | Menghasilkan `Dockerfile` & `docker-compose.yml` multi-stage produksi |
| `lumina make:nginx` | Menghasilkan konfigurasi Nginx reverse proxy produksi |
| `lumina make:supervisor` | Menghasilkan konfigurasi Supervisor daemon |

---

## 📁 Struktur Direktori Project Lumina

```
my_app/
├── Cargo.toml                  ← Konfigurasi dependensi project
├── .env                        ← Konfigurasi environment (DB, Port, JWT, dsb.)
├── database/
│   ├── migrations/             ← File migrasi SQL (auto-runner)
│   └── seeders/                ← File seeder data dummy
├── resources/
│   ├── views/                  ← Template tampilan (.blade.rs)
│   ├── js/                     ← Skrip client (Echo broadcaster, dsb.)
│   └── css/                    ← Lembar gaya styling
├── src/
│   ├── main.rs                 ← Entrypoint server & server runner
│   ├── routes/
│   │   ├── web.rs              ← Rute antarmuka Web HTML
│   │   └── api.rs              ← Rute JSON REST API
│   └── app/
│       ├── controllers/        ← Handler HTTP request
│       ├── models/             ← Definisi entitas & ORM database
│       ├── requests/           ← Skema validasi input form/JSON
│       └── services/           ← Layer logika bisnis aplikasi
└── storage/
    ├── app/public/             ← File upload publik
    └── cache/                  ← Cache file lokal
```

---

## 📚 Dokumentasi Lanjutan

Untuk panduan dan referensi teknis yang lebih mendalam:

- 📖 **[DOCUMENTATION.md](./DOCUMENTATION.md)** — Dokumentasi komprehensif seluruh modul core.
- 💎 **[docs/ORM.md](./docs/ORM.md)** — Panduan Lengkap Lumina Eloquent ORM & Relationships.
- 🚀 **[TUTORIAL_INSTALL.md](./TUTORIAL_INSTALL.md)** — Panduan instalasi dan persiapan sistem.
- 📡 **[TUTORIAL_BROADCASTER.md](./TUTORIAL_BROADCASTER.md)** — Panduan real-time WebSocket broadcasting dengan Lumina Echo.
- ⚡ **[CHEAT_SHEET.md](./CHEAT_SHEET.md)** — Snippet cepat sintaks controller, query ORM, dan helper.

---

## 🧪 Menjalankan Pengujian (Testing)

Lumina dilengkapi dengan suite testing integrasi lengkap untuk pengujian API CRUD, ORM, middleware, dan broadcasting:

```bash
# Menjalankan seluruh test suite framework
cargo test

# Menjalankan test integrasi CRUD API
cargo test --test integration_test

# Menjalankan test ORM
cargo test --test orm_human_features_test

# Menjalankan test Broadcaster Echo
cargo test --test production_features_test
```

---

## 📄 License

Lumina Framework dilisensikan di bawah lisensi [MIT](LICENSE).  
Dibuat dengan cinta untuk ekosistem Rust oleh **Slamet Sugandi** (<packercyber@gmail.com>).
