# 📝 Changelog

Semua perubahan penting pada proyek **Lumina Framework** didokumentasikan di dalam berkas ini.

Format pencatatan mengacu pada [Keep a Changelog](https://keepachangelog.com/en/1.0.0/) dan mengikuti standar [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.1.0] - 2026-11-21

### 🤖 AI-First Architecture, Hardware Bridge & Desktop Compilation Engine
- **Universal AI Agent Context Scaffolding (`lumina new` & `skeleton.rs`)**:
  - Auto-generate master AI instructions pada instalasi proyek baru:
    - `AGENTS.md`: Panduan master universal (standar Antigravity, Cline, Windsurf).
    - `.cursorrules`: Instruksi prioritas tinggi untuk Cursor AI Composer.
    - `.github/copilot-instructions.md`: Format resmi workspace prompt untuk GitHub Copilot.
    - `CLAUDE.md`: Standar konvensi kode untuk Anthropic Claude Code CLI.
  - **The Standard CRUD Blueprint**: Resep baku 6 langkah lengkap pembuatan fitur CRUD (Model, Migration, Validator, Controller 7 RESTful actions, Routes, Blade/JSON responses) tanpa halusinasi.
- **CLI AI Context Sync (`lumina make:ai-context` / `lumina ai:init`)**:
  - Sinkronisasi atau update seluruh file instruksi AI pada proyek existing atau hasil `git clone` dengan sekali perintah.
- **Multi-Model AI Skills & Reverse-Engineer Stubs (`lumina make:ai-skill`)**:
  - Generator tool definition universal kompatibel dengan **OpenAI Function Calling**, **Google Gemini Function Declarations**, **Anthropic Claude Tools**, dan **Antigravity/Cursor SKILL.md**.
  - Handler Rust ber-tipe kuat (`src/app/services/ai/`) yang aman dipanggil oleh AI models.
  - **Specialized Reverse-Engineer Skill (`lumina make:ai-skill ReverseEngineer`)**: AI Agent merekayasa balik website/HTML target menjadi komponen Blade Tailwind, komponen reaktif Lumina Live, dan Eloquent models.
- **Lumina Hardware Bridge (`src/core/hardware/` & Client WASM)**:
  - Menghubungkan server web Linux di Cloud ke perangkat keras lokal client:
  - `EscPosBuilder`: Fluent builder struk belanja thermal printer USB/Bluetooth (WebUSB/WebSerial) dengan export Base64.
  - `BarcodeScanEvent`: Parser barcode serial dengan validasi check digit EAN-13.
  - `FingerprintTemplate`: Validasi dan penanganan template sidik jari biometrik via WebHID/WASM.
  - `CameraConfig`: Konfigurasi live camera frame & scanning QR/barcode via WebRTC MediaDevices.
  - `device_bridge_script()`: Pre-bundled client JavaScript (`window.LuminaDevice`) dengan WebSocket dispatcher.
- **Lumina Desktop Compilation Engine (`src/desktop.rs` & `lumina-cli`)**:
  - Mengompilasi aplikasi web Lumina menjadi **Single Standalone Executable Desktop App** (.exe di Windows, Linux binary, macOS .app).
  - Menggunakan engine WebView native sistem tanpa beban Chromium raksasa (< 60MB RAM).
  - CLI commands: `lumina desktop:init`, `lumina desktop:run`, dan `lumina desktop:build --release`.
- **Lumina Native AI Client Facade (`src/core/ai/`)**:
  - Pemanggilan model LLM secara ekspresif ala Laravel: `Ai::gemini()`, `Ai::openai()`, dan `Ai::claude()`.
- **Dokumentasi Resmi VitePress**:
  - Modul 22: *AI-Ready Architecture & Agent Skills*
  - Modul 23: *Lumina Hardware Bridge*
  - Modul 24: *Lumina Desktop Compilation Engine*

---

## [1.0.0] - 2026-11-19

### 👑 Enterprise Masterpiece (LTS Release)
- **Lumina Tinker — Interactive CLI REPL Shell (`src/core/tinker/`)**:
  - Perintah baru di Artisan CLI: `lumina tinker`.
  - Terminal REPL interaktif berbasis `rustyline` dengan persistent command history (`.lumina_tinker_history`).
  - Dynamic Model & Query Evaluator: `Model::all()`, `Model::find(id)`, `Model::count()`, `DB::table(...)`.
  - Eksekusi langsung query Raw SQL dengan kalkulasi waktu eksekusi presisi (*stopwatch timer*).
  - Evaluasi langsung utility & security facades: `Hash::make()`, `Hash::check()`, `AutoSlug::slugify()`, `Uuid::new()`.
  - Framework Introspection: perintah `routes`, `tables` / `models`, `env` (dengan perlindungan masking pada password dan secrets), `clear`, dan `help`.
  - Visualisasi terminal elegan: Tabel ANSI box-drawing (`┌─┬─┐`) dengan auto-alignment lebar kolom dinamis.
- **Dynamic WASM Plugin Registry & Zero-Downtime Hot-Reloading (`src/core/sandbox/`)**:
  - Pemuatan ulang plugin WebAssembly otomatis di memori (*background file watcher*) tanpa restart server host.
  - Swap instance atomic aman konkurensi (`Arc<RwLock<...>>`) tanpa request terputus (*zero-downtime*).
  - Lifecycle Hooks Pipeline: `PluginHook::OnRequest` (`lumina_on_request`), `PluginHook::BeforeSave` (`lumina_before_save`), `PluginHook::AfterResponse` (`lumina_after_response`).
  - Isolasi Memori & Trap Fault-Tolerance mutlak berstandar militer (error pada plugin tidak merusak stabilitas server).
  - Dashboard Metrik Performa Plugin (`lumina plugin:metrics`) dan reload manual (`lumina plugin:reload`).
- **Suite Pengujian Baru**:
  - `tests/tinker_test.rs`: 2 test suite komprehensif untuk evaluasi model, query builder, raw sql, facades, dan formatter.
  - `tests/wasm_hot_reload_test.rs`: 3 test suite komprehensif untuk zero-downtime hot-reloading, lifecycle hooks, dan trap isolation.
  - Total pengujian otomatis lulus mencapai **92 passed (100% success)**.
- **Dokumentasi Resmi**:
  - Modul 20: *Lumina Tinker (Interactive CLI REPL Shell)* di `docs/documentations/20_lumina_tinker.md`.
  - Modul 21: *Dynamic WASM Plugin Registry & Zero-Downtime Hot-Reloading* di `docs/documentations/21_wasm_plugins_hot_reload.md`.
  - Pembaruan seluruh indeks modul resmi dan navigasi sidebar VitePress portal.

---

## [0.9.0] - 2026-11-18

### 🚀 Ditambahkan & Ditingkatkan (Added & Improved)
- **Advanced Eloquent ORM & Model Lifecycle Observers (`src/database/observer.rs`)**:
  - Trait `ModelLifecycle` dengan hook asinkron: `saving`, `saved`, `creating`, `created`, `updating`, `updated`, `deleting`, dan `deleted`.
  - Trait `Observer<T>` untuk pemisahan logika lifecycle ke dedicated observer classes ala Laravel.
  - `AutoSlug`: Otomasi pembuatan slug URL SEO-friendly yang bersih dan unik dengan auto-collision resolver (`-1`, `-2`, dst.).
  - `AutoHash`: Otomasi hashing password Bcrypt idempotent (mencegah double-hash kata sandi).
- **Dedicated Database Seeder Class Runner (`src/database/seeder.rs`)**:
  - Trait `Seeder` untuk pembuatan kelas seeder Rust murni independen.
  - Helper `call_seeder::<S>(pool)` untuk chaining hierarki seeder di dalam `DatabaseSeeder`.
  - `SeederRegistry` untuk pemetaan dinamis dan eksekusi seeder berdasarkan nama class via CLI.
  - Perintah Artisan CLI baru: `lumina make:seeder <Name>` dan `lumina db:seed --class <Name>`.
- **Declarative Enterprise Audit Trail Logging (`src/database/audit.rs`)**:
  - Struct `AuditLog` dengan snapshot JSON `old_values` dan `new_values`.
  - Fluent builder `Audit::record(pool).on(table, id).action(...).diff(old, new).save()` untuk pelacakan mutasi kepatuhan SOC 2 & ISO 27001.
  - Method `Audit::history()` untuk membaca riwayat audit per entitas dan `Audit::latest()` untuk feed audit global.
  - Skema database auto-migration via `Audit::ensure_table(pool)`.
- **Advanced Eloquent Query Builder (`src/database/orm.rs`)**:
  - `.chunk(size, callback)`: streaming data ramah RAM untuk memproses jutaan baris data secara bertahap tanpa OOM.
  - `.update_or_create(values)`: pencarian dan update otomatis atau insert record baru.
  - Implementasi `primary_key(&self) -> i64` otomatis pada derive `LuminaModel`.
- **Suite Pengujian Baru**:
  - `tests/advanced_eloquent_test.rs`: 5 test suite komprehensif untuk lifecycle observers, auto slug, seeder class runner, audit trail, dan query builder chunk / update_or_create.
  - Total pengujian otomatis lulus meningkat menjadi 87 passed (100% success).
- **Dokumentasi Resmi**:
  - Modul 19: *Advanced Eloquent, Model Observers & Seeding* di `docs/documentations/19_advanced_eloquent_model_observers_dan_seeding.md` dan integrasi VitePress portal.

---

## [0.8.0] - 2026-11-18

### 🚀 Ditambahkan & Ditingkatkan (Added & Improved)
- **API-First Suite & Automated OpenAPI v3 Specification (`src/core/openapi/`)**:
  - Dukungan OpenAPI v3.0.3 lengkap dengan serialisasi JSON rapi dan YAML serializer.
  - Fluent Route Decorators pada Lumina `Router`: `.summary()`, `.description()`, `.tag()`, `.tags()`, `.deprecated()`, dan `.operation_id()`.
  - Normalisasi otomatis path parameter Axum (`:id` dan `*slug` dikonversi ke format standar `{id}` dengan inferensi skema tipe data).
  - Pewarisan metadata dan prefix otomatis pada route group (`router.group`, `router.nest`, `router.merge`).
  - Generator koleksi Postman v2.1 instan (`spec.to_postman_collection()`).
- **Built-in Interactive Swagger UI v5**:
  - Router method `.swagger_ui(mount_path, title, version)`.
  - Melayani UI interaktif modern di `GET /docs` dan `GET /docs/swagger`.
  - Melayani endpoint spesifikasi mentah di `GET /docs/openapi.json`.
- **Artisan CLI Toolchain**:
  - `lumina route:openapi`: Pindai route proyek dan ekspor spesifikasi ke `openapi.json` atau `openapi.yaml`.
  - `lumina export:postman`: Ekspor koleksi Postman v2.1 siap pakai untuk integrasi frontend/QA.
- **Fluent HTTP Testing Assertion Kit (`lumina::testing::*`)**:
  - Harness pengujian in-memory `TestApp` terisolasi dengan SQLite memory, Tokio, dan Tower `oneshot` tanpa port collision jaringan TCP.
  - Fluent request builder: `app.req()`, `app.get_req()`, `app.post_req()`, `app.put_req()`, `app.patch_req()`, `app.delete_req()`.
  - Request decorators: `.with_header()`, `.with_token()`, `.with_json()`, `.with_form()`, `.acting_as()`.
  - 20+ Assertion methods fluen ala Laravel:
    - Status: `assert_ok()`, `assert_created()`, `assert_status()`, `assert_unprocessable()`, `assert_unauthorized()`, `assert_forbidden()`, `assert_not_found()`, dll.
    - JSON Dot-Notation: `assert_json_path()`, `assert_json_has()`, `assert_json_missing()`, `assert_json_count()`.
    - Konten & Redireksi: `assert_see()`, `assert_dont_see()`, `assert_header()`, `assert_redirect()`.
  - Database Testing Assertions: `app.assert_database_has()`, `app.assert_database_missing()`, `app.assert_database_count()`.
- **Suite Pengujian Baru**:
  - `tests/openapi_test.rs`: 2 test suite komprehensif untuk OpenAPI spec & Swagger UI.
  - `tests/fluent_testing_kit_test.rs`: 2 test suite komprehensif untuk assertion status, json path, dan database assertions.
  - Total pengujian otomatis lulus meningkat menjadi 82 passed (100% success).
- **Dokumentasi Resmi**:
  - Modul 18: *API-First Suite, Swagger UI & Fluent Testing Kit* di `docs/documentations/18_api_first_openapi_swagger_dan_testing_kit.md` dan integrasi portal VitePress.

---

## [0.7.0] - 2026-11-17

### 🚀 Ditambahkan & Ditingkatkan (Added & Improved)
- **Universal Storage Facade (`Storage`) & Pure-Rust S3 Driver**:
  - Abstraksi multi-disk terpadu via trait `StorageDriver` yang mendukung disk `local`, `public`, dan `s3`.
  - Driver S3 mandiri performa tinggi berbasis REST API dengan implementasi penuh **AWS Signature Version 4 (SigV4)** murni (`hmac` + `sha2` + `hex`).
  - Kompatibel penuh dengan **AWS S3, Cloudflare R2, MinIO, dan RustFS** (mendukung virtual-host addressing dan path-style endpoint).
  - Generator **Presigned Temporary URL** berbatas waktu (*TTL*) instan untuk pengunduhan aman berkas privat.
  - Proteksi anti-*Directory Traversal Attack* (`../`) bawaan pada driver filesystem lokal (`LocalStorage`).
  - Global Facade `Storage::put_file()`, `Storage::get_file()`, `Storage::disk_named("s3")`, serta helper `req.storage().disk("s3")`.
- **Distributed Redis Cache Suite**:
  - Trait multi-driver `CacheDriver` yang mendukung `moka` (in-memory) dan `redis` (terdistribusi antar-pod).
  - Helper `Cache::remember("key", ttl, || async { ... })` dengan serialisasi JSON otomatis via `serde_json`.
  - Inisialisasi otomatis dari environment variable `CACHE_DRIVER=redis` dan `REDIS_URL`.
- **High-Speed Redis Queue Driver**:
  - Antrean tugas asinkron berbasis Redis Lists (`RPUSH` & `LPOP`) dengan persistensi data dan latensi pemrosesan sub-milidetik.
  - Dukungan multi-pod queue worker daemon (`lumina queue:work`).
- **Lumina Echo Redis Pub/Sub WebSocket Broadcaster**:
  - Adapter Redis Pub/Sub untuk clustering WebSocket multi-server di lingkungan Kubernetes atau Docker Swarm.
  - Event broadcast yang dikirim dari satu instance/pod otomatis diteruskan dan diterima oleh client browser di seluruh pod lainnya.
- **Suite Pengujian Live RustFS**:
  - Validasi integrasi live langsung terhadap server Object Storage Rust (`rustfs.exe`) pada port 9000 (`tests/rustfs_live_test.rs`).
- **Modul Dokumentasi Baru**:
  - Modul 17: *Universal Storage & Redis Multi-Driver Suite* di `docs/documentations/17_universal_storage_dan_redis_suite.md` dan integrasi VitePress portal.

---

## [0.6.1] - 2026-11-06

### 🚀 Ditambahkan & Ditingkatkan (Added & Improved)
- **Universal Smart MSSQL Connection Parser**:
  - Auto-normalisasi berbagai varian format string koneksi Microsoft SQL Server secara transparan:
    - Format URL standar (`sqlserver://user:pass@host:port/database` atau `mssql://...`).
    - Format Named Instance Enterprise (`sqlserver://user:pass@host/INSTANCE?database=...`).
    - Format JDBC langsung (`jdbc:sqlserver://host\instance;user=...;password=...;database=...`).
    - Format ADO connection string (`Server=host,port;Database=...;User Id=...;Password=...`).
  - Penanganan cerdas escaping tanda backslash ganda (`\\`) pada file konfigurasi `.env` untuk named instance (misal `192.168.10.29\\SQL2012`).
- **Pre-Warmed Connection Pool Optimization**:
  - Konfigurasi `min_idle(Some(2))` pada Tiberius/bb8 pool untuk mengeliminasi latensi *cold-start* TCP & TDS handshake.
  - Penyesuaian batas `connection_timeout` adaptif 10 detik dan `max_size(15)`.
- **Pengujian Komprehensif Driver MSSQL**:
  - Test suite unit terisolasi `tests/mssql_parser_test.rs` yang memvalidasi 6 varian format koneksi MSSQL.
- **Modernisasi Dokumentasi Model & Scaffolding**:
  - Pembaruan dokumen `04_model_dan_eloquent_orm.md` menyelaraskan seluruh contoh CRUD dan Relasi ke gaya Unified `Request` bundle.
  - Panduan performa lingkungan produksi enterprise (SQL Browser UDP 1434 overhead, optimasi Bcrypt release mode, dan indexing).

---

## [0.6.0] - 2026-11-03

### 🚀 Ditambahkan (Added)
- **Zero-Ampersand Fluent ACID Transactions**:
  - Fasade transaksi `DB::transaction(|tx| async move { ... })` dengan auto-commit saat sukses dan auto-rollback saat terjadi error atau panic.
  - Akses database terikat transaksi via `tx.db()` dan helper `req.tx()`.
- **Smart Declarative Soft Deletes**:
  - Makro derive `#[derive(LuminaModel)]` dengan atribut `#[soft_deletes]` untuk otomasi pengelolaan `deleted_at`.
  - Filter query builder otomatis: `.without_trashed()` (default), `.with_trashed()`, `.only_trashed()`, serta method `.restore()` dan `.force_delete()`.
- **Integrasi Transaksional Aurora Starter Kit**:
  - `AuthController` memanfaatkan transaksi ACID pada alur registrasi pengguna dan audit log.

---

## [0.5.1] - 2026-11-01

### 🚀 Ditambahkan (Added)
- **Enterprise Microsoft SQL Server (MSSQL / Tiberius) Support**:
  - Koneksi TDS native melalui Tiberius dan bb8 connection pool.
  - Auto-translation dialek SQL standar ke sintaks T-SQL (`TOP n`, `@P1`, `@P2`, `ROW_NUMBER() OVER (...)`).
  - Dukungan penuh QueryBuilder dan Model untuk MSSQL (`get`, `first`, `count`, `paginate`).
- **Audit Kompatibilitas Aurora Starter Kit**:
  - Penyesuaian seluruh alur autentikasi dan model Aurora agar sepenuhnya kompatibel dengan database MSSQL.

---

## [0.5.0] - 2026-10-31

### 🚀 Ditambahkan (Added)
- **Lumina Aurora Starter Kit**:
  - Paket lengkap autentikasi modern berbasis Blade UI: Login, Register, Forgot Password, Reset Password, Dasbor Profil, dan Navigasi Responsif.
- **Enterprise Security Hardening**:
  - Token Blacklist instan dengan auto-eviction berbasis cache Moka.
  - HMAC-SHA256 signature verification dengan constant-time comparison anti-timing attacks.
  - Middleware security headers bawaan (CSP, HSTS, X-Content-Type-Options, Referrer-Policy).
- **Portal Dokumentasi Interaktif & Benchmarking**:
  - Portal dokumentasi modern dengan pencarian instan dan grafik perbandingan performa engine.

---

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
