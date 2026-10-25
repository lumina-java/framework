# 📚 Dokumentasi Resmi Lumina Framework (v0.2.0)

Selamat datang di pusat dokumentasi resmi **Lumina Framework**! Dokumentasi ini disusun secara berseri (*per episode/modul*) agar mudah dipahami, terstruktur secara profesional, dan mencakup seluruh fitur framework dari instalasi hingga deployment produksi.

---

## 📑 Daftar Episode & Modul Pembelajaran

Klik tautan di bawah untuk mempelajari setiap modul secara mendalam:

| No | Modul Dokumentasi | Topik Pembahasan |
| :---: | :--- | :--- |
| **01** | [**Persyaratan Sistem & Instalasi**](./01_instalasi_dan_persyaratan.md) | Requirement Rust, instalasi CLI global, `lumina new --db`, struktur folder MVC, `.env`, dan `lumina serve`. |
| **02** | [**Routing & Middleware**](./02_routing_dan_middleware.md) | Definisi rute GET/POST/PUT/DELETE, parameter URL, group & prefix, custom middleware, security headers, rate limiting, dan CSRF. |
| **03** | [**Controller & Request Handling**](./03_controller_dan_request.md) | `lumina make:controller`, Dependency Injection, Form & JSON payload, `lumina make:request` (validasi), dan file upload. |
| **04** | [**Model & Eloquent ORM**](./04_model_dan_eloquent_orm.md) | `lumina make:model`, migrasi SQL, `#[derive(LuminaModel)]`, CRUD, Fluent Query Builder, pagination, relasi 1-baris, dan **Model Factory & Faker Engine (`make:factory`)**. |
| **05** | [**Blade Template Engine**](./05_blade_template_engine.md) | Variabel & escaping, layout inheritance (`@extends`, `@section`), perulangan `@foreach` & `$loop`, pipeline filter `{{ $val \| rupiah }}`, direktif `@wasm`, dan `@eval_is`. |
| **06** | [**WASM Sandbox & Plugin**](./06_wasm_sandbox_dan_plugin.md) | Arsitektur WASM Native Hybrid (`wasmi 2.0`), memory fault isolation, `lumina plugin:new`, `lumina plugin:build`, `lumina plugin:list`, dan ABI memory bridge. |
| **07** | [**Indonesian Script (.is)**](./07_indonesian_script.md) | Konsep skrip backend, sintaks bahasa natural (`fungsi`, `jika`, `selama`), built-in helpers (`rupiah`, `panjang`), eksekusi di Controller & Blade `@eval_is`. |
| **08** | [**Autentikasi & Keamanan**](./08_autentikasi_dan_keamanan.md) | `lumina make:auth`, hashing Bcrypt, Dual Auth (Session Web + JWT API), RBAC & perizinan (`can`), direktif Blade `@auth`/`@guest`, dan **Lumina Shield Security Scanner (`lumina audit`)**. |
| **09** | [**Broadcasting & WebSocket (Lumina Echo)**](./09_broadcasting_dan_websocket.md) | Server WebSocket, Public vs Private Channel, otorisasi token JWT, event `ShouldBroadcast`, dan integrasi frontend JS. |
| **10** | [**Queue, Jobs & Scheduler**](./10_queue_jobs_dan_scheduler.md) | Background Jobs (`lumina make:job`), dispatching asinkron, persistent queue database, dan cron-style task scheduler. |
| **11** | [**Observabilitas & Lumina Telescope**](./11_observabilitas_dan_telescope.md) | Dasbor pemantauan `/lumina/telescope`, request logger, query tracer, exception viewer, health checks `/up`, dan Tab Sandbox Metrics. |
| **12** | [**Deployment & DevOps**](./12_deployment_dan_devops.md) | Kompilasi produksi `cargo build --release`, Dockerfile multi-stage (`make:docker`), reverse proxy Nginx (`make:nginx`), Supervisor daemon (`make:supervisor`), dan backup otomatis. |

---

## 🌟 Filosofi Desain Lumina

1. **Rust Speed, Laravel Elegance**: Menghadirkan kenikmatan sintaks Laravel dengan jaminan performa tinggi dan keamanan memori dari bahasa Rust.
2. **Zero-Overhead Hybrid**: Fitur inti yang membutuhkan performa kritis ditulis dalam Rust native, sedangkan aturan bisnis dan plugin pihak ketiga dapat berjalan dalam sandbox WASM atau Indonesian Script.
3. **Batteries Included**: Framework sudah menyertakan ORM, template engine, autentikasi, WebSocket, queue, logger, dan CLI toolchain tanpa perlu merakit puluhan library terpisah.

---

👉 **Mulai petualangan Anda dengan membaca:** [**Modul 01: Persyaratan Sistem & Instalasi**](./01_instalasi_dan_persyaratan.md)
