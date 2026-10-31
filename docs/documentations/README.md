# 📚 Dokumentasi Resmi Lumina Framework (v0.5.0)

> *"Look like Laravel, Run at Rust Speed. Menghadirkan Developer Experience termanis dengan performa raw speed, memory safety, dan keamanan tingkat enterprise."*

Selamat datang di pusat dokumentasi resmi **Lumina Framework**! Dokumentasi ini disusun secara berseri (*per episode/modul*) agar mudah dipahami, terstruktur secara profesional, dan mencakup seluruh fitur framework dari instalasi hingga deployment produksi berskala enterprise.

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
| **13** | [**Lumina Live (Reactive UI)**](./13_lumina_live_server_driven_ui.md) | Server-Driven Reactive UI ala Livewire/LiveView, Tri-Dialect (`kilat:*`, `live:*`, `wire:*`), Idiomorph DOM Morphing, Two-way Binding, dan `lumina make:live`. |
| **14** | [**Lumina Aurora Starter Kit**](./14_lumina_aurora_starter_kit.md) | Official starter kit autentikasi & dashboard elegan dengan Pure Native Blade + Tailwind CSS (`lumina aurora:install`), Zero-JS bloat, respons sub-0.5ms. |
| **15** | [**Enterprise Security & Zero-Panic Safety**](./15_enterprise_security_dan_zero_panic.md) | Global Zero-Panic Error Boundary (`catch_unwind`), Anti Brute-Force Lockout Cache (Moka), Granular RBAC (`@can`, `@role`), Constant-Time Eq, OWASP Headers, ISO 27001 Audit Trail. |
| **16** | [**Whitepaper Benchmark & Performa Resmi**](./16_benchmark_dan_performance_whitepaper.md) | Data pengujian riil Throughput 118.000+ RPS, latensi sub-milidetik, konsumsi RAM hemat 15-38MB vs Laravel Octane, Express.js, dan FastAPI. |

---

## 🌟 Tiga Pilar Utama Lumina (The 3 Pillars)

1. **⚡ Fast (Rust Raw Speed)**: Dibangun di atas Tokio Async Runtime dan Axum Engine. Zero-cost abstractions, latensi respons sub-milidetik (< 0.5ms), dan penggunaan memori minimal (~15MB RAM).
2. **🎨 Beauty (Laravel Elegance)**: Sintaksis deklaratif, ekspresif, dan human-readable. Unified `Request` bundle, Fluent Query Builder, Native Blade Template, dan Starter Kit Aurora yang menawan.
3. **🛡️ Safety (Enterprise Resilience)**: Jaminan keamanan memori dari Rust, Zero-Panic error boundary anti server crash, granular RBAC, serta sertifikasi arsitektur kepatuhan SOC 2 / ISO 27001.

---

👉 **Mulai petualangan Anda dengan membaca:** [**Modul 01: Persyaratan Sistem & Instalasi**](./01_instalasi_dan_persyaratan.md)
