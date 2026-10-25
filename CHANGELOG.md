# 📝 Changelog

Semua perubahan penting pada proyek **Lumina Framework** didokumentasikan di dalam berkas ini.

Format pencatatan mengacu pada [Keep a Changelog](https://keepachangelog.com/en/1.0.0/) dan mengikuti standar [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [0.3.0] - 2026-10-25

### 🚀 Ditambahkan (Added)
- **High-Speed Model Factory & Faker Engine**:
  - Trait `Factory<M>` dan struct `FactoryBuilder` dengan chaining fluen ala Laravel: `.count(n)`, `.state(|m| ...)`, `.make()`, `.make_one()`, dan `.create(db).await`.
  - Generator `Faker` bawaan berkecepatan tinggi tanpa dependensi eksternal (*zero external crates*), dioptimalkan untuk data realistis Indonesia (nama pria/wanita, telepon 08xx, alamat jalan, kota, perusahaan, rupiah, kalimat lorem, email unik).
  - Generator CLI: `lumina make:factory <Name>` yang menghasilkan scaffold factory lengkap dan mendaftarkannya otomatis ke `database/factories/mod.rs`.
- **Lumina Shield Security Audit (`lumina audit`)**:
  - Pemindai otomatis postur keamanan dan hardening konfigurasi aplikasi.
  - Memverifikasi entropi kriptografi `APP_KEY` dan `JWT_SECRET` (>= 32 karakter).
  - Mendeteksi bahaya `APP_DEBUG=true` di lingkungan produksi.
  - Memindai seluruh template Blade untuk memastikan keberadaan proteksi `@csrf` pada seluruh formulir POST.
  - Menghasilkan visual security scoreboard dengan skor (0–100), peringkat mutu (*Grade A+*), dan rekomendasi langkah perbaikan konkret (*Actionable Fixes*).

---

## [0.2.1] - 2026-10-25

### 🚀 Ditambahkan (Added)
- **Sintaks Render View Bebas Ampersand (`&`)**: `ViewBuilder::render` kini mengimplementasikan `std::borrow::Borrow<Request>`, memungkinkan penulisan `View::make("...").with("kunci", nilai).render(req).await` tanpa tanda ampersand (`&`) yang menyulitkan pengetikan dan autocompletion.
- **Pedoman Aturan Kode (`AGENTS.md`)**: Panduan baku kode *Laravel-like elegance* untuk seluruh Controller, Model, View, Service, dan Repository.

### 🔄 Diubah (Changed)
- **Pembaruan Dokumentasi Modular**: Seluruh tutorial controller dan view di `docs/documentations/` diselaraskan ke sintaks fluen terbaru.

---

## [0.2.0] - 2026-10-25

### 🚀 Ditambahkan (Added)
- **Pure Native Blade Engine**: Implementasi penuh engine Blade native Rust independen tanpa ketergantungan runtime Tera. Mendukung layout inheritance (`@extends`, `@section`, `@yield`), perulangan `@foreach` & `$loop`, attribute helpers (`@checked`, `@disabled`, `@selected`), serta custom filter pipelines (`{{ $val | rupiah | uppercase }}`).
- **WebAssembly Native Hybrid Plugin Engine**: Dukungan eksekusi plugin pihak ketiga terisolasi dalam sandbox WASM (`wasmi 2.0`). Termasuk proteksi memory trap, zero-leak ABI memory bridge (`alloc`, `dealloc`, `transform`), serta direktif template `@wasm`.
- **Indonesian Script (.is) Backend Interpreter**: AST interpreter bahasa alami Indonesia (`fungsi`, `jika`, `selama`, `rupiah`, `panjang`, `potong`) yang dieksekusi langsung oleh backend Lumina di Controller (`req.state.script(...)`) dan View (`@eval_is`).
- **CLI Database Generator Flag**: Perintah `lumina new <name> --db sqlite|postgres|mysql|all` untuk mengonfigurasikan dependensi `Cargo.toml` dan template `.env` secara otomatis sesuai database yang dipilih.
- **CLI Plugin Tooling**: Perintah `lumina plugin:new <name>`, `lumina plugin:build <name>`, dan `lumina plugin:list` untuk kemudahan manajemen dan kompilasi plugin WebAssembly target `wasm32-wasip1`.
- **Lumina Telescope Sandbox Metrics**: Tab metrik pemantauan eksekusi sandbox (WASM & Indonesian Script) di dasbor `/lumina/telescope` untuk memantau durasi eksekusi dan isolasi memori secara real-time.
- **Dokumentasi Modular 12 Episode**: Struktur dokumentasi resmi baru di `docs/documentations/` dari Episode 01 (Persyaratan Sistem & Instalasi) hingga Episode 12 (Deployment & DevOps).
- **Aturan Agent & Kode (`AGENTS.md`)**: Pedoman arsitektur dan penulisan kode "Look Like Laravel, Run at Rust Speed" untuk seluruh Controller, Model, View, Service, dan Repository.

### 🔄 Diubah (Changed)
- **Standardisasi Controller Ergonomis**: Seluruh controller distandardisasi menggunakan Unified `Request` Bundle (`req: Request`), `impl IntoResponse` untuk View rendering via `View::make(...)`, dan `LuminaResult` untuk response API JSON.
- **Pembersihan Root & Isolasi Riwayat**: Seluruh riwayat pengujian, tutorial lawas, dan draf catatan kerja dipindahkan ke `docs/history_pekerjaan/` dan disembunyikan dari git via `.gitignore`.

---

## [0.1.0] - 2026-10-18

### 🚀 Rilis Perdana (Initial Release)
- **Core HTTP Engine**: Arsitektur MVC berbasis Tokio dan Axum berkecepatan tinggi dengan konsumsi memori minimal.
- **Human ORM & Macro**: `#[derive(LuminaModel)]` dengan fungsi CRUD otomatis, pagination (`LengthAwarePaginator`), eager loading, query builder fluen, dan soft deletes.
- **Universal Multi-Database Layer**: Dukungan universal untuk PostgreSQL, MySQL, dan SQLite via SQLx dengan auto-migration runner.
- **Smart CLI Toolchain (`lumina`)**: Pembuatan project (`lumina new`), controller (`make:controller`), model (`make:model`), migrasi (`make:migration`), seeder (`db:seed`), dan scaffolding CRUD instan (`make:crud`).
- **Dual Auth Suite**: Autentikasi berbasis Session Web dan JWT API Bearer Token instan (`make:auth`).
- **Real-time Broadcaster (Lumina Echo)**: WebSocket broadcaster dengan otorisasi channel Public, Private, dan Presence.
- **Queue & Scheduler**: Eksekusi antrean tugas asinkron dengan persistensi database serta cron task scheduler.
- **High-Speed Cache & Mailer**: In-memory cache Moka dengan TTL/tagging dan fluent Mailer (SMTP & Log driver).
- **File Storage**: Penanganan upload berkas multipart dan manipulasi gambar otomatis (resize, thumbnail, grayscale).
- **Telescope & Health Checks**: Dasbor pemantauan `/lumina/telescope` dan health check endpoint `/up`.
