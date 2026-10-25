# 📝 Changelog

Semua perubahan penting pada proyek **Lumina Framework** didokumentasikan di dalam berkas ini.

Format pencatatan mengacu pada [Keep a Changelog](https://keepachangelog.com/en/1.0.0/) dan mengikuti standar [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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
