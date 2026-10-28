# 📝 Changelog

Semua perubahan penting pada proyek **Lumina Framework** didokumentasikan di dalam berkas ini.

Format pencatatan mengacu pada [Keep a Changelog](https://keepachangelog.com/en/1.0.0/) dan mengikuti standar [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.4.1] - 2026-10-29

### 🚀 Ditambahkan & Ditingkatkan (Added & Improved)
- **Ekstraksi Crate Mandiri `lumina-live`**:
  - Memisahkan runtime reaktif Lumina Live ke crate terpisah (`lumina-live`) dan repositori mandiri (`https://github.com/lumina-java/lumina-live`) agar inti framework tetap ramping dan modular.
  - Menambahkan trait ekstensi `LiveApplicationExt` (`with_live()`, `with_live_component()`).
- **Arsitektur Modular Extensibility pada Inti Framework**:
  - `Application::with_raw_router(router)`: Mendukung penggabungan sub-router Axum dari ekstensi/plugin modular pihak ketiga.
  - `blade::register_global_directive(name, handler)`: Registrasi direktif kustom Blade Engine secara dinamis tanpa mengotori inti engine.
- **Validasi Integrasi End-to-End**:
  - Uji coba aplikasi Todo List reaktif (`apptodolist`) membuktikan interaksi WebSocket & HTTP micro-RPC tanpa ketergantungan frontend eksternal.

---

## [0.4.0] - 2026-10-28

### 🚀 Ditambahkan (Added)
- **Lumina Live — Server-Driven Reactive UI Suite**:
  - Trait `LiveComponent` di Rust dengan lifecycle `mount()`, `view()`, `call_action()`, dan `render_html()`.
  - **Tri-Dialect Runtime Engine**: Pemrosesan simultan 22 direktif interaksi dalam 3 dialek bahasa:
    - 🇮🇩 Dialek Indonesia (`kilat:*`): `kilat:klik`, `kilat:kirim`, `kilat:model`, `kilat:tunggu`, `kilat:target`, `kilat:yakin`, `kilat:pantau`, `kilat:navigasi`, dsb.
    - 🌐 Dialek Universal (`live:*`): `live:click`, `live:submit`, `live:model`, `live:loading`, `live:confirm`, `live:poll`, dsb.
    - 🅰️ Dialek Laravel Legacy (`wire:*`): `wire:click`, `wire:submit`, `wire:model`, `wire:loading`, `wire:confirm`, `wire:poll`, dsb.
  - **Cryptographic Tamper-Proof Snapshot**: Pengamanan state komponen menggunakan signature HMAC-SHA256 (`APP_KEY`) dengan *constant-time comparison* (`subtle::ConstantTimeEq`) anti manipulasi browser DevTools.
  - **Embedded Micro-Runtime (`lumina-live.js`)**: Script vanilla JS mandiri (~6.5 KB) tanpa dependensi eksternal, disajikan langsung dari biner via route `GET /_lumina/live/live.js` dengan HTTP cache headers.
  - **Idiomorph DOM Morpher**: Algoritma diff DOM cerdas yang mempertahankan fokus input, posisi kursor pengetikan, dan scroll bar pengguna saat HTML diperbarui secara reaktif.
  - **Transport Endpoint**: Route HTTP Micro-RPC stateless `POST /_lumina/live/call` untuk eksekusi aksi dan re-rendering berlatensi sub-milidetik.
  - **Integrasi Blade Engine**: Direktif `@kilatScripts`, `@luminaLive`, `@livewireScripts`, serta render tag `@kilat('name')` dan `@live('name')`.
  - **CLI Scaffolding**: Perintah `lumina make:live <Name>` untuk membangkitkan komponen Rust dan template Blade reaktif dalam 1 detik.
  - **Showcase Demo**: Halaman `/live-demo` dengan komponen reaktif `Counter` yang memamerkan interaksi dua arah dan tombol aksi instan.

---

## [0.3.2] - 2026-10-27

### 🚀 Ditambahkan (Added)
- **Pilar 1: Testing DX & CLI View Generator**:
  - Cargo features default disederhanakan menjadi `default = ["postgres", "sqlite"]`, sehingga pengujian aplikasi pengguna berjalan instan tanpa wajib menginstal dependensi database ganda.
  - Perintah CLI baru `lumina make:view <nama>` (e.g. `lumina make:view products.index` atau `auth.profile`) yang otomatis membuat direktori hirarkis dan template `.blade.rs` standar dengan layout `@extends('layouts.app')`.
- **Pilar 2: Blade Real-World Form DX & Helpers**:
  - Direktif validasi form `@error('field') ... @enderror` yang otomatis menyuntikkan variabel lokal `$message`.
  - Helper fungsi Blade `old('field', 'default')` untuk mempertahankan input form saat validasi gagal (*old input repopulation*).
  - Helper fungsi URL: `asset('path')`, `url('path')`, dan `route('name')`.
  - Pemeriksaan error kondisional di Blade: `@if(error('field'))`, `@if(hasError('field'))`, dan `@if(anyErrors)`.
- **Pilar 3: Model Soft-Deletes & Entity Lifecycle**:
  - Penambahan method instance fluen pada seluruh model yang me-derive `LuminaModel`: `.soft_delete(db)`, `.restore(db)`, dan `.force_delete(db)`.
  - Static model helpers: `<Model>::restore_by_id(db, id)` dan `<Model>::force_delete_by_id(db, id)`.
  - Penambahan method `.soft_delete()` pada `QueryBuilder` di samping `.restore()`, `.with_trashed()`, dan `.only_trashed()`.
- **Pilar 4: Lumina Background Queue Suite**:
  - Trait `Job` dan `Dispatchable` untuk eksekusi tugas di latar belakang (background queue).
  - Dispatching fluen dari controller: `SendWelcomeEmail { email }.dispatch(&req).await?` atau `req.dispatch(name, payload)`.
  - `QueueManager::new_with_pool` dengan auto-translation placeholder SQL (`$1` PostgreSQL vs `?` MySQL/SQLite) saat menyimpan antrean ke tabel `jobs`.
  - CLI Generator `lumina make:job <Name>` untuk membangkitkan kelas job terstruktur di `src/app/jobs/` dan meregistrasikannya secara otomatis.
  - CLI Worker Daemon `lumina queue:work` yang memantau antrean database dan mengeksekusi tugas secara terus-menerus (*persistent queue daemon*).

---

## [0.3.1] - 2026-10-26

### 🚀 Ditambahkan (Added)
- **Native Transactional Migration & Seeder CLI Engine**:
  - `lumina migrate`: Runner migrasi SQL nyata multi-database (PostgreSQL, MySQL, SQLite) transaksional dengan batch tracking di tabel `_lumina_migrations`.
  - `lumina migrate:rollback`: Membatalkan batch migrasi terakhir secara transaksional dengan mengeksekusi berkas `*.down.sql` dan membersihkan entri dari database.
  - `lumina migrate:status`: Menampilkan tabel ringkasan status setiap berkas migrasi (`Ran` vs `Pending`, batch, dan tanggal eksekusi).
  - `lumina db:seed`: Eksekusi berkas seeder `database/seeders/*.sql` secara terurut dan transaksional.
  - Resolusi otomatis string koneksi database dari berkas `.env` (`DATABASE_URL` atau variabel granular `DB_CONNECTION`, `DB_HOST`, `DB_PORT`, `DB_DATABASE`, `DB_USERNAME`, `DB_PASSWORD`).
- **One-Command CLI & Project Upgrader (`lumina upgrade` / `lumina update`)**:
  - Perintah baru untuk meng-upgrade binary CLI secara mandiri langsung dari repositori resmi GitHub (`lumina upgrade`).
  - Mendeteksi dan menyinkronkan dependensi `lumina-framework` di `Cargo.toml` proyek aktif secara otomatis.
  - Opsi fleksibel: `--cli-only`, `--project-only`, dan `--local <path>`.
- **Ergonomi Middleware Router Tingkat Lanjut**:
  - `Router::guard(async_fn)` dan `Router::middleware_fn(async_fn)`: Menghubungkan fungsi middleware asinkron secara langsung tanpa kewajiban membungkus `axum::middleware::from_fn(...)`.
  - Re-export `from_fn`, `from_fn_with_state`, dan `Next` langsung di `lumina::prelude::*`.
- **Financial Types, UUID & Native Pool Access**:
  - Re-export `uuid::Uuid`, `rust_decimal::Decimal`, dan makro `dec!` langsung di `lumina::prelude::*`.
  - Dukungan decoding native `Uuid` dan `Decimal` di SQLx pada entitas `LuminaModel`.
  - Akses direct native pool: `req.pg_pool()`, `req.sqlite_pool()`, dan `req.mysql_pool()` untuk kebutuhan query spesifik vendor seperti row locks (`SELECT ... FOR UPDATE`) atau PostgreSQL `LISTEN/NOTIFY`.
- **Two-Layer Security Suite**:
  - `TokenBlacklist`: Engine pembatalan (revocation) token JWT instan berbasis Moka cache dengan auto-eviction TTL 7 hari untuk mengamankan alur logout REST API (`req.revoke_token().await`).
  - Native HMAC-SHA256 signature verification (`verify_hmac_sha256`) dengan perbandingan waktu konstan (*constant-time comparison*) menggunakan trait `subtle::ConstantTimeEq` anti timing-attacks.
  - Perlindungan anti replay attack dan toleransi clock-drift maksimal 300 detik pada middleware `verify_api_signature`.

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
