# ✨ Lumina Framework

<p align="center">
  <img src="storage/app/public/assets/logo.png" alt="Lumina Framework Logo" width="180">
</p>

<p align="center">
  <strong>A beautiful, ultra-fast, and elegant web framework for Rust.</strong><br>
  <em>Menggabungkan performa dan memory-safety Rust dengan keanggunan serta keramahan sintaks Laravel MVC.</em>
</p>

<p align="center">
  <a href="https://crates.io/crates/lumina-framework"><img src="https://img.shields.io/crates/v/lumina-framework.svg" alt="Crates.io Version"></a>
  <a href="https://crates.io/crates/cargo-lumina"><img src="https://img.shields.io/badge/CLI-cargo--lumina-ec4899.svg" alt="Cargo Lumina CLI"></a>
  <a href="https://github.com/lumina-java/framework"><img src="https://img.shields.io/badge/GitHub-lumina--java%2Fframework-blue.svg" alt="GitHub Repo"></a>
  <a href="https://github.com/lumina-java/lumina"><img src="https://img.shields.io/badge/Starter%20Kit-lumina--java%2Flumina-6366f1.svg" alt="Starter Kit"></a>
  <a href="#-fitur-utama"><img src="https://img.shields.io/badge/Database-PostgreSQL%20|%20MySQL%20|%20SQLite-06b6d4.svg" alt="Databases"></a>
  <a href="LICENSE"><img src="https://img.shields.io/badge/License-MIT-amber.svg" alt="License"></a>
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
- 🏭 **Model Factory & Faker Engine** — Generator data tiruan (*mock / seeding*) berkecepatan tinggi dengan lokalisasi Indonesia & internasional serta chaining fluen ala Laravel (`User::factory().count(100).create(db).await`).
- 🔄 **Universal Multi-Database Layer** — Dukungan langsung untuk **PostgreSQL**, **MySQL**, dan **SQLite** dengan auto-migration runner serta generator database flag di CLI (`--db`).
- 🪄 **Lumina Smart CLI (`lumina`)** — Buat project baru dengan preset database, controller, model, factory (`make:factory`), audit keamanan (`audit`), autentikasi instan (`make:auth`), plugin WASM, dan modul CRUD lengkap (`make:crud`).
- 📦 **The Unified `Request` Extractor** — Cukup panggil `req: Request` untuk mengakses Database Pool, Session, CSRF Token, Auth User, Flash Messages, dan View Renderer.
- 🛡️ **Lumina Shield Security Scanner (`lumina audit`)** — Audit otomatis postur keamanan aplikasi (kekuatan secret, mode debug di produksi, proteksi CSRF form, storage hardening) dengan security scoreboard dan saran perbaikan.
- 🔐 **Dual Auth Suite** — Autentikasi berbasis **Session** untuk web serta **JWT Bearer Token** untuk API.
- 🎭 **Pure Native Blade Engine** — Template engine independen berkecepatan tinggi tanpa runtime eksternal. Mendukung direktif familiar (`@extends`, `@section`, `@if`, `@foreach`, `@csrf`), pipeline filter (`| rupiah`, `| uppercase`), dan render bebas tanda `&`.
- 🧩 **WASM Native Hybrid Plugin Engine** — Eksekusi plugin pihak ketiga terisolasi dalam sandbox WebAssembly (`wasmi 2.0`) yang aman dari crash memori dan terpantau di Telescope.
- 🇮🇩 **Indonesian Script (.is) Runtime** — Bahasa skrip berorientasi logika bisnis berbasis bahasa Indonesia natural (`fungsi`, `jika`, `selama`) yang dieksekusi langsung oleh backend Lumina.
- 📡 **Lumina Echo (WebSocket Broadcaster)** — Broadcasting event real-time dengan channel authorization (Public, Private, Presence) dan verifikasi JWT.
- ⏳ **Background Queue & Jobs** — Eksekusi antrean tugas asinkron dengan persistensi database.
- ⏰ **Task Scheduler** — Penjadwalan cron-job bawaan yang mudah dikonfigurasi.
- 💾 **High-Speed Cache** — Cache in-memory berbasis Moka dengan dukungan tagging dan TTL.
- 📧 **Mail & Notification** — Fluent email builder dengan template rendering dan driver Log/SMTP.
- 📁 **File Storage & Image Manipulation** — Penanganan upload multipart dan manipulasi gambar otomatis (resize, thumbnail, grayscale).
- 🔭 **Telescope & Health Checks** — Dasbor `/lumina/telescope` (Requests, Queries, Logs, Sandbox Metrics), health check `/up` & `/health`, graceful shutdown, dan security headers bawaan.

---

## 🛠️ Instalasi Lumina CLI (Crates.io)

Lumina CLI (`lumina`) adalah toolchain utama untuk membuat dan mengelola project Lumina.

### Instalasi Resmi via Cargo (Crates.io):
```bash
cargo install cargo-lumina
```

> **Catatan:** Paket `cargo-lumina` akan memasang executable binary `lumina` secara global di sistem Anda.

Verifikasi instalasi CLI:
```bash
lumina --version
lumina --help
```

---

## ⚡ Quick Start: Membuat Project Baru

### 1. Cara 1: Menggunakan Lumina CLI (Rekomendasi)
```bash
# Membuat project baru (otomatis mengambil template starter terbaru dari GitHub)
lumina new kontak_manager

# Atau pilih driver database tertentu (--db sqlite / postgres / mysql / all):
lumina new kontak_manager --db postgres
```

### 2. Cara 2: Clone Starter Template Langsung dari GitHub
```bash
git clone https://github.com/lumina-java/lumina.git kontak_manager
cd kontak_manager
cargo run
```

---

### 3. Konfigurasi Database (.env)
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
| `lumina new <name> [--db <preset>]` | Membuat project baru (`--db sqlite`, `postgres`, `mysql`, atau `all`) |
| `lumina serve` | Menjalankan server aplikasi HTTP |
| `lumina watch` | Menjalankan server dengan fitur hot auto-reload (`cargo-watch`) |
| `lumina make:crud <Name> [fields]` | Membuat CRUD lengkap instan (Model, Controller, Migrasi, View) |
| `lumina make:auth` | Menghasilkan modul autentikasi lengkap (Register, Login, Session, Blade) |
| `lumina make:controller <Name>` | Menghasilkan controller baru |
| `lumina make:model <Name>` | Menghasilkan model baru beserta file migrasi |
| `lumina make:factory <Name>` | Menghasilkan Model Factory baru untuk testing & seeding |
| `lumina make:migration <Name>` | Membuat file migrasi database baru |
| `lumina migrate` | Menjalankan seluruh file migrasi SQL yang belum diaplikasikan |
| `lumina migrate:rollback` | Membatalkan migrasi terakhir |
| `lumina db:seed` | Menjalankan database seeder |
| `lumina audit` | Menjalankan Lumina Shield untuk audit keamanan & konfigurasi project |
| `lumina plugin:new <name>` | Membuat scaffold plugin WebAssembly baru |
| `lumina plugin:build <name>` | Membangun binary WASM target `wasm32-wasip1` |
| `lumina plugin:list` | Mendaftar semua plugin WASM yang terpasang di `storage/plugins` |
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

## 📚 Dokumentasi Lengkap & Seri Tutorial

Dokumentasi resmi Lumina disusun secara berseri (*12 episode/modul*) yang terstruktur, komprehensif, dan mudah dipelajari:

👉 **[Buka Indeks Dokumentasi Utama (docs/documentations/README.md)](./docs/documentations/README.md)**

| Episode | Modul Dokumentasi | Topik Utama |
| :---: | :--- | :--- |
| **01** | [**Persyaratan Sistem & Instalasi**](./docs/documentations/01_instalasi_dan_persyaratan.md) | Requirement Rust, CLI global, `lumina new --db`, struktur folder MVC, `.env`, `lumina serve`. |
| **02** | [**Routing & Middleware**](./docs/documentations/02_routing_dan_middleware.md) | Rute HTTP, parameter URL, group & prefix, custom middleware, security headers, rate limiting, CSRF. |
| **03** | [**Controller & Request Handling**](./docs/documentations/03_controller_dan_request.md) | `make:controller`, Dependency Injection, Form/JSON payload, `make:request` (validasi), file upload. |
| **04** | [**Model & Eloquent ORM**](./docs/documentations/04_model_dan_eloquent_orm.md) | `make:model`, migrasi, `#[derive(LuminaModel)]`, CRUD, Query Builder, pagination, relasi 1-baris. |
| **05** | [**Blade Template Engine**](./docs/documentations/05_blade_template_engine.md) | Layout inheritance (`@extends`, `@section`), looping `@foreach`, pipeline filter `\| rupiah`, `@wasm`, `@eval_is`. |
| **06** | [**WASM Sandbox & Plugin**](./docs/documentations/06_wasm_sandbox_dan_plugin.md) | Arsitektur WASM Native Hybrid (`wasmi 2.0`), fault isolation, `plugin:new`, `plugin:build`, ABI bridge. |
| **07** | [**Indonesian Script (.is)**](./docs/documentations/07_indonesian_script.md) | Skrip backend natural (`fungsi`, `jika`, `selama`), built-in helpers, eksekusi di Controller & Blade. |
| **08** | [**Autentikasi & Keamanan**](./docs/documentations/08_autentikasi_dan_keamanan.md) | `make:auth`, hashing Bcrypt, Dual Auth (Session Web + JWT API), RBAC (`can`), Blade `@auth`/`@guest`. |
| **09** | [**Broadcasting & WebSocket (Echo)**](./docs/documentations/09_broadcasting_dan_websocket.md) | Server WebSocket, Public vs Private Channel, otorisasi JWT, event `ShouldBroadcast`, client JS. |
| **10** | [**Queue, Jobs & Scheduler**](./docs/documentations/10_queue_jobs_dan_scheduler.md) | Background Jobs (`make:job`), dispatching asinkron, persistent queue database, cron scheduler. |
| **11** | [**Observabilitas & Telescope**](./docs/documentations/11_observabilitas_dan_telescope.md) | Dasbor `/lumina/telescope`, request logger, query tracer, exception viewer, Tab Sandbox Metrics. |
| **12** | [**Deployment & DevOps**](./docs/documentations/12_deployment_dan_devops.md) | Release build, Docker multi-stage (`make:docker`), Nginx (`make:nginx`), Supervisor (`make:supervisor`), backup. |

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

## 📜 Dokumen Panduan & Riwayat Rilis

- 🤖 **[AGENTS.md](./AGENTS.md)** — Pedoman arsitektur dan aturan baku penulisan kode (*Laravel-like elegance, Request bundle, dan clean architecture*) untuk AI Agent & developer.
- 📝 **[CHANGELOG.md](./CHANGELOG.md)** — Catatan riwayat rilis dan pembaruan fitur per versi framework.

---

## 📄 License

Lumina Framework dilisensikan di bawah lisensi [MIT](LICENSE).  
Dibuat dengan cinta untuk ekosistem Rust oleh **Slamet Sugandi** (<packercyber@gmail.com>).
