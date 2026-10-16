use std::fs;
use std::path::Path;

pub fn create_full_skeleton(base: &Path, name: &str) {
    // 1. Create Folder Structure
    let folders = [
        "src",
        "src/app",
        "src/app/controllers",
        "src/app/models",
        "src/app/services",
        "src/app/requests",
        "src/core",
        "src/routes",
        "resources",
        "resources/views",
        "resources/views/auth",
        "resources/views/layouts",
        "resources/js",
        "resources/css",
        "database",
        "database/migrations",
        "database/seeders",
        "storage",
        "storage/app/public",
        "storage/cache",
        "public",
        "tests",
    ];
    for folder in folders {
        fs::create_dir_all(base.join(folder)).ok();
    }

    // 2. Cargo.toml
    let lib_name = name.replace("-", "_");
    let cargo_toml = format!(
        r#"[package]
name = "{}"
version = "0.1.0"
edition = "2021"
default-run = "{}-server"

[lib]
name = "{}"
path = "src/lib.rs"

[[bin]]
name = "{}-server"
path = "src/main.rs"

[dependencies]
lumina = {{ package = "lumina-framework", path = "../framework" }}
tokio = {{ version = "1", features = ["full"] }}
axum = {{ version = "0.7", features = ["macros", "multipart"] }}
serde = {{ version = "1", features = ["derive"] }}
serde_json = "1"
dotenv = "0.15"
tera = "1.19"
async-trait = "0.1"
chrono = {{ version = "0.4", features = ["serde"] }}
sqlx = {{ version = "0.8", features = ["runtime-tokio", "postgres", "mysql", "sqlite", "any", "macros", "chrono"] }}
"#,
        name, name, lib_name, name
    );
    fs::write(base.join("Cargo.toml"), cargo_toml).ok();

    // 3. .env.example & .env
    let env_example = r#"# ─── LUMINA FRAMEWORK CONFIGURATION ──────────────────────────────────────────

# Application Settings
APP_NAME=Lumina
APP_ENV=local
APP_DEBUG=true
APP_URL=http://localhost:8000
APP_KEY=lumina-super-secret-key-change-me-in-production

# Logging
LOG_LEVEL=info
LOG_TO_FILE=true

# Database Configuration (Multi-Driver: sqlite / postgres / mysql)
DB_CONNECTION=sqlite
DB_HOST=127.0.0.1
DB_PORT=5432
DB_DATABASE=./database.sqlite
DB_USERNAME=root
DB_PASSWORD=

# Atau gunakan URL eksplisit:
# DATABASE_URL=sqlite:./database.sqlite
# DATABASE_URL=postgres://postgres:password@127.0.0.1:5432/my_db
# DATABASE_URL=mysql://root:password@127.0.0.1:3306/my_db

# Rate Limiting
RATE_LIMIT_MAX=60
RATE_LIMIT_WINDOW=60

# Storage Settings
UPLOAD_MAX_SIZE_MB=5

# Session Configuration
JWT_SECRET=lumina-jwt-secret-key-change-me-in-production

# ──────────────────────────────────────────────────────────────────────────────
"#;
    fs::write(base.join(".env.example"), env_example).ok();
    fs::write(base.join(".env"), env_example).ok();

    // 4. src/lib.rs (Enables modular code & integration tests in tests/)
    let lib_rs = r#"pub mod app;
pub mod routes;
"#;
    fs::write(base.join("src/lib.rs"), lib_rs).ok();

    // 5. src/main.rs
    let main_rs = r#"mod app;
mod routes;

use lumina::prelude::*;

//                  LUMINA FRAMEWORK APPLICATION ENTRYPOINT
// High-performance, elegant Rust web backend & API engine.

#[tokio::main]
async fn main() {
    // 1. Load configuration from .env file
    dotenv::dotenv().ok();

    // 2. Initialize the application engine with Web & API routes
    let app = Application::new()
        .with_web(routes::web::router())
        .with_api(routes::api::router());

    println!("⚡ Welcome to Lumina Framework");
    println!("🚀 Server running at http://127.0.0.1:8000");
    println!("📖 API Endpoints ready at http://127.0.0.1:8000/api");

    // 3. Launch HTTP server
    app.serve("127.0.0.1:8000").await;
}
"#;
    fs::write(base.join("src/main.rs"), main_rs).ok();

    // 6. src/app/mod.rs
    fs::write(
        base.join("src/app/mod.rs"),
        r#"pub mod controllers;
pub mod models;
pub mod services;
pub mod requests;

/// Application Prelude Hub — Satu baris import untuk seluruh controller & service:
/// `use crate::app::prelude::*;`
pub mod prelude {
    pub use lumina::prelude::*;
    pub use crate::app::models::*;
    pub use crate::app::requests::*;
}
"#,
    )
    .ok();

    // 7. src/app/models/mod.rs & requests/mod.rs & services/mod.rs
    fs::write(base.join("src/app/models/mod.rs"), "// Models module\n").ok();
    fs::write(base.join("src/app/requests/mod.rs"), "// Requests module\n").ok();
    fs::write(base.join("src/app/services/mod.rs"), "// Services module\n").ok();

    // 8. src/app/controllers/mod.rs
    fs::write(
        base.join("src/app/controllers/mod.rs"),
        "pub mod welcome_controller;\n",
    )
    .ok();

    // 9. src/app/controllers/welcome_controller.rs (Dual-Mode Response: HTML + Lumen JSON)
    let welcome_ctrl = r#"use lumina::prelude::*;
use serde_json::json;

//                        WELCOME CONTROLLER
// Controller halaman utama aplikasi Lumina.
// Mendukung Dual-Mode Response:
// 1. Browser Request (Accept: text/html) -> Mengembalikan Dashboard HTML modern ala Laravel.
// 2. API / cURL Request (Accept: application/json atau ?format=json) -> Mengembalikan JSON ala Lumen!

pub struct WelcomeController;

impl WelcomeController {
    /// Halaman Utama ("/") - Dual Mode Response
    pub async fn index(req: Request) -> impl IntoResponse {
        // Cek informasi koneksi database aktif
        let db_info = if let Some(pool) = &req.state.db {
            let driver = format!("{:?}", pool.kind);
            let is_ok = sqlx::query("SELECT 1").execute(&pool.pool).await.is_ok();
            (driver, if is_ok { "connected" } else { "disconnected" })
        } else {
            ("SQLite".to_string(), "not connected")
        };

        // 1. Jika request meminta JSON (via Header Accept: application/json atau ?format=json)
        // Mengembalikan spesifikasi framework ala Lumen microframework!
        let wants_json = req.wants_json() 
            || req.header("accept").map(|a| a.contains("json")).unwrap_or(false);

        if wants_json {
            return req.json(json!({
                "name": "Lumina Framework",
                "tagline": "The High-Performance Rust Backend for Modern APIs",
                "version": "0.1.5",
                "rust_edition": "2021",
                "engine": "Axum 0.7 + Tokio Async",
                "environment": "local",
                "database": {
                    "driver": db_info.0,
                    "status": db_info.1
                },
                "documentation": "https://github.com/lumina-java/framework",
                "endpoints": {
                    "home": "/",
                    "health": "/health",
                    "api_index": "/api",
                    "api_health": "/api/health"
                }
            }));
        }

        // 2. Jika diakses via Web Browser biasa, render Blade HTML Dashboard
        req.view("welcome")
            .with("title", "Lumina Framework - High Performance Rust Backend")
            .with("app_name", "Lumina Framework")
            .with("version", "0.1.5")
            .with("rust_edition", "2021")
            .with("db_driver", &db_info.0)
            .with("db_status", db_info.1)
            .with("docs_url", "https://github.com/lumina-java/framework")
            .render(&req)
            .await
            .into_response()
    }

    /// Menampilkan Halaman Login ("/login")
    pub async fn login(req: Request) -> impl IntoResponse {
        req.view("auth.login")
            .with("title", "Login - Lumina")
            .render(&req)
            .await
            .into_response()
    }

    /// Menampilkan Halaman Register ("/register")
    pub async fn register(req: Request) -> impl IntoResponse {
        req.view("auth.register")
            .with("title", "Register - Lumina")
            .render(&req)
            .await
            .into_response()
    }
}
"#;
    fs::write(
        base.join("src/app/controllers/welcome_controller.rs"),
        welcome_ctrl,
    )
    .ok();

    // 10. src/routes/mod.rs, web.rs & api.rs
    fs::create_dir_all(base.join("src/routes")).ok();
    fs::write(
        base.join("src/routes/mod.rs"),
        "pub mod api;\npub mod web;\n",
    )
    .ok();

    let web_routes = r#"use lumina::prelude::*;
use crate::app::controllers::welcome_controller::WelcomeController;

//                            WEB ROUTING
// Rute untuk web browser (HTML views, Blade templates, auth UI).

pub fn router() -> Router<AppState> {
    Router::new()
        // Halaman Utama Lumina (Dual Mode: HTML Dashboard di browser / JSON di API)
        .get("/", WelcomeController::index)
        
        // Rute autentikasi
        .get("/login", WelcomeController::login)
        .get("/register", WelcomeController::register)
}
"#;
    fs::write(base.join("src/routes/web.rs"), web_routes).ok();

    let api_routes = r#"use lumina::prelude::*;
use serde_json::json;

//                            API ROUTING
// Rute di sini otomatis diprefix dengan "/api" oleh Application engine.
// Sangat ideal untuk membangun RESTful API, Microservices, dan Webhooks.

pub fn router() -> Router<AppState> {
    Router::new()
        .get("/", api_index)
        .get("/health", api_health)
        .get("/v1", api_v1_index)
}

/// Endpoint: GET /api
async fn api_index() -> impl IntoResponse {
    Json(json!({
        "framework": "Lumina Framework",
        "tagline": "The High-Performance Rust Backend for Modern APIs",
        "version": "0.1.5",
        "status": "online",
        "endpoints": {
            "health": "/api/health",
            "v1": "/api/v1"
        }
    }))
}

/// Endpoint: GET /api/health
async fn api_health(State(state): State<AppState>) -> impl IntoResponse {
    let db_status = if let Some(pool) = &state.db {
        if sqlx::query("SELECT 1").execute(&pool.pool).await.is_ok() {
            "connected"
        } else {
            "error"
        }
    } else {
        "disabled"
    };

    Json(json!({
        "status": "ok",
        "database": db_status,
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))
}

/// Endpoint: GET /api/v1
async fn api_v1_index() -> impl IntoResponse {
    Json(json!({
        "version": "v1",
        "message": "Lumina API v1 is ready. Register your controllers in src/routes/api.rs!"
    }))
}
"#;
    fs::write(base.join("src/routes/api.rs"), api_routes).ok();

    // 11. resources/views/layouts/app.blade.rs
    let layout = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{{ title | default("Lumina Framework") }}</title>
    <script src="https://cdn.tailwindcss.com"></script>
    <link rel="preconnect" href="https://fonts.googleapis.com">
    <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
    <link href="https://fonts.googleapis.com/css2?family=JetBrains+Mono:wght@400;500;600&family=Plus+Jakarta+Sans:wght@300;400;500;600;700;800&family=Space+Grotesk:wght@500;600;700;800&display=swap" rel="stylesheet">
    <style>
        body {
            font-family: 'Plus Jakarta Sans', sans-serif;
            background-color: #030712;
        }
        .heading-font {
            font-family: 'Space Grotesk', sans-serif;
        }
        .mono-font {
            font-family: 'JetBrains Mono', monospace;
        }
        .glass-nav {
            background: rgba(10, 15, 30, 0.75);
            backdrop-filter: blur(16px);
            border-bottom: 1px solid rgba(255, 255, 255, 0.08);
        }
        .glass-card {
            background: rgba(17, 24, 39, 0.6);
            backdrop-filter: blur(12px);
            border: 1px solid rgba(255, 255, 255, 0.08);
        }
        .glass-card:hover {
            border-color: rgba(99, 102, 241, 0.4);
            box-shadow: 0 10px 30px -10px rgba(99, 102, 241, 0.2);
        }
    </style>
</head>
<body class="text-slate-100 min-h-screen flex flex-col justify-between selection:bg-indigo-500 selection:text-white">

    <!-- Top Navigation Header -->
    <nav class="glass-nav sticky top-0 z-50">
        <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
            <div class="flex justify-between h-16 items-center">
                <div class="flex items-center space-x-3">
                    <a href="/" class="flex items-center space-x-3">
                        <div class="w-9 h-9 bg-gradient-to-tr from-indigo-500 via-purple-500 to-pink-500 rounded-xl flex items-center justify-center shadow-lg shadow-indigo-500/25">
                            <span class="text-white font-black heading-font text-xl">⚡</span>
                        </div>
                        <div class="flex items-center gap-2">
                            <span class="text-xl font-bold tracking-tight text-white heading-font">Lumina</span>
                            <span class="text-[10px] uppercase font-bold tracking-widest px-2 py-0.5 rounded-full bg-indigo-500/20 text-indigo-300 border border-indigo-500/30">Backend</span>
                        </div>
                    </a>
                </div>
                <div class="flex items-center space-x-6">
                    <a href="/api" target="_blank" class="text-slate-300 hover:text-indigo-400 font-medium text-sm transition flex items-center gap-1.5">
                        <span class="w-2 h-2 rounded-full bg-emerald-400 animate-pulse"></span>
                        API Endpoints
                    </a>
                    <a href="https://github.com/lumina-java/framework" target="_blank" class="text-slate-300 hover:text-indigo-400 font-medium text-sm transition">Docs</a>
                    <a href="https://github.com/lumina-java/framework" target="_blank" class="bg-indigo-600 hover:bg-indigo-500 text-white px-4 py-2 rounded-xl text-sm font-semibold shadow-lg shadow-indigo-500/20 transition-all">GitHub Repo</a>
                </div>
            </div>
        </div>
    </nav>

    <!-- Main Content Slot -->
    <main class="flex-grow">
        @yield('content')
    </main>

    <!-- Footer Segment -->
    <footer class="border-t border-slate-900 bg-black/60 py-8">
        <div class="max-w-7xl mx-auto px-4 text-center text-slate-500 text-sm flex flex-col sm:flex-row justify-between items-center gap-4">
            <div class="flex items-center gap-2">
                <span>&copy; 2026 <strong>Lumina Framework</strong>. All rights reserved.</span>
            </div>
            <div class="flex items-center gap-1.5 text-slate-400">
                Engineered with <span class="text-red-500">❤️</span> in <strong>Rust</strong> (Axum 0.7 + Tokio) for ultra-fast APIs.
            </div>
        </div>
    </footer>

</body>
</html>
"#;
    fs::write(base.join("resources/views/layouts/app.blade.rs"), layout).ok();

    // 12. resources/views/welcome.blade.rs (Laravel / Lumen Backend Homepage)
    let welcome = r#"@extends('layouts.app')

@section('content')
<div class="relative overflow-hidden pt-12 pb-24">
    <!-- Ambient Glow Effects -->
    <div class="absolute top-[-10%] left-[-10%] w-[50vw] h-[50vw] bg-indigo-600/15 rounded-full blur-[140px] pointer-events-none"></div>
    <div class="absolute top-[20%] right-[-10%] w-[45vw] h-[45vw] bg-pink-600/10 rounded-full blur-[140px] pointer-events-none"></div>

    <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 relative z-10">
        <!-- Hero Section -->
        <div class="text-center max-w-4xl mx-auto">
            <div class="inline-flex items-center gap-2 py-1 px-3.5 rounded-full text-xs font-semibold bg-indigo-500/10 text-indigo-300 border border-indigo-500/25 mb-6 uppercase tracking-wider shadow-inner">
                <span class="w-2 h-2 rounded-full bg-emerald-400 animate-ping"></span>
                <span>Lumina v{{ version }} • High-Performance Rust Backend Engine</span>
            </div>
            <h1 class="text-5xl md:text-7xl font-extrabold tracking-tight text-white mb-6 heading-font leading-tight">
                The <span class="bg-clip-text text-transparent bg-gradient-to-r from-indigo-400 via-purple-300 to-pink-400">Elegance</span> of Laravel.<br>
                The <span class="text-white underline decoration-indigo-500 decoration-wavy decoration-2">Raw Speed</span> of Rust.
            </h1>
            <p class="text-lg md:text-xl text-slate-400 max-w-3xl mx-auto mb-10 leading-relaxed">
                Lumina is a developer-first Rust web framework specifically built for modern <strong>Backend APIs</strong>, microservices, and enterprise applications. Enjoy seamless routing, unified database pooling, and rapid scaffolding.
            </p>
            <div class="flex flex-wrap justify-center gap-4">
                <a href="/api" target="_blank" class="bg-gradient-to-r from-indigo-600 to-purple-600 text-white px-7 py-3.5 rounded-xl font-bold hover:shadow-lg hover:shadow-indigo-500/25 transition-all transform hover:-translate-y-0.5 flex items-center gap-2">
                    <span>🚀 Inspect API Spec</span>
                    <span class="text-xs bg-black/30 px-2 py-0.5 rounded-md mono-font">GET /api</span>
                </a>
                <a href="https://github.com/lumina-java/framework" target="_blank" class="bg-slate-900/80 text-slate-200 border border-slate-800 px-7 py-3.5 rounded-xl font-semibold hover:bg-slate-800 hover:text-white transition-all transform hover:-translate-y-0.5 flex items-center gap-2">
                    <span>📖 Documentation</span>
                </a>
            </div>
        </div>

        <!-- Telemetry Status Grid (Laravel / Lumen Health Badges) -->
        <div class="mt-14 grid grid-cols-2 md:grid-cols-4 gap-4">
            <div class="glass-card rounded-2xl p-5 text-left">
                <div class="text-slate-400 text-xs font-semibold uppercase tracking-wider mb-1">Framework Version</div>
                <div class="text-xl font-bold text-white heading-font flex items-center gap-2">
                    <span>Lumina {{ version }}</span>
                </div>
                <div class="text-xs text-indigo-400 mt-1">Rust {{ rust_edition }} Edition</div>
            </div>

            <div class="glass-card rounded-2xl p-5 text-left">
                <div class="text-slate-400 text-xs font-semibold uppercase tracking-wider mb-1">Runtime Engine</div>
                <div class="text-xl font-bold text-white heading-font">Axum 0.7</div>
                <div class="text-xs text-emerald-400 mt-1">Tokio Multi-Threaded Async</div>
            </div>

            <div class="glass-card rounded-2xl p-5 text-left">
                <div class="text-slate-400 text-xs font-semibold uppercase tracking-wider mb-1">Database Driver</div>
                <div class="text-xl font-bold text-white heading-font flex items-center gap-2">
                    <span>{{ db_driver }}</span>
                    <span class="w-2.5 h-2.5 rounded-full {% if db_status == 'connected' %}bg-emerald-400 shadow-sm shadow-emerald-400/50{% else %}bg-amber-400{% endif %}"></span>
                </div>
                <div class="text-xs {% if db_status == 'connected' %}text-emerald-400{% else %}text-amber-400{% endif %} mt-1 font-medium capitalize">{{ db_status }}</div>
            </div>

            <div class="glass-card rounded-2xl p-5 text-left">
                <div class="text-slate-400 text-xs font-semibold uppercase tracking-wider mb-1">Environment</div>
                <div class="text-xl font-bold text-white heading-font">Local</div>
                <div class="text-xs text-slate-400 mt-1">Debug Mode Active</div>
            </div>
        </div>

        <!-- Dual Mode Lumen API Showcase -->
        <div class="mt-10 bg-slate-950/80 border border-slate-800 rounded-3xl p-6 md:p-8 backdrop-blur-md shadow-2xl text-left">
            <div class="flex flex-col sm:flex-row justify-between items-start sm:items-center gap-4 border-b border-slate-800/80 pb-5 mb-6">
                <div>
                    <div class="flex items-center gap-2">
                        <span class="px-2.5 py-0.5 rounded-full text-xs font-bold bg-pink-500/20 text-pink-300 border border-pink-500/30">Dual-Mode API</span>
                        <h2 class="text-xl font-bold text-white heading-font">Lumen-Style Headless JSON Response</h2>
                    </div>
                    <p class="text-slate-400 text-sm mt-1">Kirim header <code class="mono-font bg-slate-900 text-indigo-300 px-1.5 py-0.5 rounded">Accept: application/json</code> ke URL root (<code class="mono-font text-indigo-300">/</code>) untuk langsung menerima JSON spec.</p>
                </div>
                <div class="flex items-center gap-2">
                    <a href="/?format=json" target="_blank" class="text-xs font-semibold bg-slate-800 hover:bg-slate-700 text-indigo-300 px-3 py-1.5 rounded-lg border border-slate-700 transition flex items-center gap-1.5">
                        <span>Lihat Raw JSON</span>
                        <span>↗</span>
                    </a>
                </div>
            </div>

            <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
                <!-- Terminal Command -->
                <div>
                    <div class="text-xs font-semibold text-slate-400 uppercase tracking-wider mb-2">1. Coba Request via Terminal / cURL:</div>
                    <div class="bg-black/90 rounded-2xl p-4 border border-slate-800 mono-font text-sm text-slate-300 relative group">
                        <div class="flex items-center justify-between text-xs text-slate-500 pb-2 border-b border-slate-800/60 mb-3">
                            <span>bash terminal</span>
                            <span class="text-emerald-400 font-semibold">GET /</span>
                        </div>
                        <div class="overflow-x-auto text-pink-400 select-all">
                            curl -H "Accept: application/json" http://127.0.0.1:8000/
                        </div>
                    </div>
                </div>

                <!-- JSON Response Preview -->
                <div>
                    <div class="text-xs font-semibold text-slate-400 uppercase tracking-wider mb-2">2. JSON Response yang Dihasilkan:</div>
                    <pre class="bg-black/90 rounded-2xl p-4 border border-slate-800 mono-font text-xs text-emerald-400 overflow-x-auto leading-relaxed">
{
  <span class="text-slate-400">"name"</span>: <span class="text-amber-300">"Lumina Framework"</span>,
  <span class="text-slate-400">"version"</span>: <span class="text-amber-300">"{{ version }}"</span>,
  <span class="text-slate-400">"engine"</span>: <span class="text-amber-300">"Axum 0.7 + Tokio Async"</span>,
  <span class="text-slate-400">"database"</span>: {
    <span class="text-slate-400">"driver"</span>: <span class="text-amber-300">"{{ db_driver }}"</span>,
    <span class="text-slate-400">"status"</span>: <span class="text-amber-300">"{{ db_status }}"</span>
  },
  <span class="text-slate-400">"documentation"</span>: <span class="text-amber-300">"https://github.com/lumina-java/framework"</span>
}</pre>
                </div>
            </div>
        </div>

        <!-- Backend Capabilities Feature Cards -->
        <div class="mt-14 grid grid-cols-1 md:grid-cols-3 gap-6 text-left">
            <div class="glass-card rounded-3xl p-7 transition duration-300">
                <div class="w-12 h-12 bg-indigo-500/10 border border-indigo-500/20 text-indigo-400 rounded-2xl flex items-center justify-center text-2xl mb-5">
                    🛣️
                </div>
                <h3 class="text-xl font-bold text-white heading-font mb-2">Dedicated API Routing</h3>
                <p class="text-slate-400 text-sm leading-relaxed mb-4">
                    Pisahkan rute API dan Web dengan mudah. File <code class="mono-font bg-slate-900 text-indigo-300 px-1 rounded">src/routes/api.rs</code> otomatis diprefix dengan <code class="mono-font text-indigo-300">/api</code> lengkap dengan middleware CORS & Rate Limiter.
                </p>
                <a href="/api/health" target="_blank" class="text-xs font-semibold text-indigo-400 hover:text-indigo-300 flex items-center gap-1">
                    Cek /api/health <span>&rarr;</span>
                </a>
            </div>

            <div class="glass-card rounded-3xl p-7 transition duration-300">
                <div class="w-12 h-12 bg-pink-500/10 border border-pink-500/20 text-pink-400 rounded-2xl flex items-center justify-center text-2xl mb-5">
                    🗄️
                </div>
                <h3 class="text-xl font-bold text-white heading-font mb-2">Unified Multi-DB Pool</h3>
                <p class="text-slate-400 text-sm leading-relaxed mb-4">
                    Gunakan SQLite untuk development lokal cepat tanpa setup, lalu beralih ke <strong>PostgreSQL</strong> atau <strong>MySQL</strong> di production cukup dengan mengubah file <code class="mono-font bg-slate-900 text-indigo-300 px-1 rounded">.env</code>.
                </p>
                <span class="text-xs font-semibold text-pink-400 flex items-center gap-1">
                    PostgreSQL • MySQL • SQLite
                </span>
            </div>

            <div class="glass-card rounded-3xl p-7 transition duration-300">
                <div class="w-12 h-12 bg-purple-500/10 border border-purple-500/20 text-purple-400 rounded-2xl flex items-center justify-center text-2xl mb-5">
                    🪄
                </div>
                <h3 class="text-xl font-bold text-white heading-font mb-2">Active Record Models</h3>
                <p class="text-slate-400 text-sm leading-relaxed mb-4">
                    Tulis model database semudah di Laravel Eloquent dengan makro sakti <code class="mono-font bg-slate-900 text-indigo-300 px-1 rounded">#[derive(LuminaModel)]</code>. Otomatis menyediakan <code class="mono-font text-indigo-300">all()</code>, <code class="mono-font text-indigo-300">find(id)</code>, dan CRUD.
                </p>
                <a href="https://github.com/lumina-java/framework#models" target="_blank" class="text-xs font-semibold text-purple-400 hover:text-purple-300 flex items-center gap-1">
                    Pelajari Makro Model <span>&rarr;</span>
                </a>
            </div>
        </div>

        <!-- Quick Start Guide -->
        <div class="mt-14 bg-slate-900/40 border border-slate-800 rounded-3xl p-8 md:p-10 text-left">
            <h3 class="text-2xl font-bold text-white heading-font mb-6 flex items-center gap-3">
                <span class="w-8 h-8 rounded-xl bg-indigo-600 text-white flex items-center justify-center text-sm font-black">⚡</span>
                <span>Cara Cepat Membuat API Endpoint Baru:</span>
            </h3>

            <div class="space-y-6">
                <div class="flex gap-4">
                    <div class="w-8 h-8 shrink-0 bg-indigo-500/20 text-indigo-300 rounded-full flex items-center justify-center font-bold text-sm border border-indigo-500/30">1</div>
                    <div class="flex-grow">
                        <div class="text-white font-semibold mb-1">Daftarkan rute di <code class="mono-font bg-slate-950 text-indigo-300 px-2 py-0.5 rounded text-sm">src/routes/api.rs</code></div>
                        <pre class="bg-black/80 p-3.5 rounded-xl text-xs mono-font text-slate-300 overflow-x-auto border border-slate-800/80 mt-2">
pub fn router() -> Router&lt;AppState&gt; {
    Router::new()
        .get("/products", get_products)
}</pre>
                    </div>
                </div>

                <div class="flex gap-4">
                    <div class="w-8 h-8 shrink-0 bg-indigo-500/20 text-indigo-300 rounded-full flex items-center justify-center font-bold text-sm border border-indigo-500/30">2</div>
                    <div class="flex-grow">
                        <div class="text-white font-semibold mb-1">Kembalikan respons JSON dengan mudah:</div>
                        <pre class="bg-black/80 p-3.5 rounded-xl text-xs mono-font text-slate-300 overflow-x-auto border border-slate-800/80 mt-2">
async fn get_products() -> impl IntoResponse {
    Json(serde_json::json!({
        "status": "success",
        "data": ["Rust", "Axum", "Lumina"]
    }))
}</pre>
                    </div>
                </div>
            </div>
        </div>

    </div>
</div>
@endsection
"#;
    fs::write(base.join("resources/views/welcome.blade.rs"), welcome).ok();

    // 13. resources/views/auth/login.blade.rs & register.blade.rs
    let login = r#"@extends('layouts.app')

@section('content')
<div class="min-h-[75vh] flex items-center justify-center px-4 relative">
    <div class="absolute w-[400px] h-[400px] bg-indigo-500/5 rounded-full blur-[100px] pointer-events-none"></div>

    <div class="max-w-md w-full bg-slate-900/60 border border-slate-800/80 rounded-3xl p-10 backdrop-filter backdrop-blur-md shadow-2xl relative z-10">
        <div class="text-center mb-8">
            <h2 class="text-3xl font-extrabold heading-font text-white">Welcome Back</h2>
            <p class="text-slate-400 mt-2 text-sm">Enter your credentials to manage your account</p>
        </div>
        
        <form action="/login" method="POST" class="space-y-6">
            @csrf
            <div>
                <label class="block text-sm font-semibold text-slate-300 mb-2">Email Address</label>
                <input type="email" name="email" class="w-full bg-slate-950/80 border border-slate-800 rounded-xl px-4 py-3 text-white focus:outline-none focus:border-indigo-500 focus:ring-1 focus:ring-indigo-500 transition" placeholder="you@example.com" required>
            </div>
            <div>
                <label class="block text-sm font-semibold text-slate-300 mb-2">Password</label>
                <input type="password" name="password" class="w-full bg-slate-950/80 border border-slate-800 rounded-xl px-4 py-3 text-white focus:outline-none focus:border-indigo-500 focus:ring-1 focus:ring-indigo-500 transition" placeholder="••••••••" required>
            </div>
            <button type="submit" class="w-full bg-indigo-600 text-white py-3.5 rounded-xl font-bold hover:bg-indigo-500 shadow-lg shadow-indigo-600/20 transition transform active:scale-98">
                Sign In
            </button>
        </form>
        
        <p class="text-center text-sm text-slate-400 mt-8">
            Don't have an account? <a href="/register" class="text-indigo-400 font-bold hover:underline">Register</a>
        </p>
    </div>
</div>
@endsection
"#;
    fs::write(base.join("resources/views/auth/login.blade.rs"), login).ok();

    let register = r#"@extends('layouts.app')

@section('content')
<div class="min-h-[75vh] flex items-center justify-center px-4 relative">
    <div class="absolute w-[400px] h-[400px] bg-pink-500/5 rounded-full blur-[100px] pointer-events-none"></div>

    <div class="max-w-md w-full bg-slate-900/60 border border-slate-800/80 rounded-3xl p-10 backdrop-filter backdrop-blur-md shadow-2xl relative z-10">
        <div class="text-center mb-8">
            <h2 class="text-3xl font-extrabold heading-font text-white">Create Account</h2>
            <p class="text-slate-400 mt-2 text-sm">Get started with the Lumina ecosystem</p>
        </div>
        
        <form action="/register" method="POST" class="space-y-6">
            @csrf
            <div>
                <label class="block text-sm font-semibold text-slate-300 mb-2">Full Name</label>
                <input type="text" name="name" class="w-full bg-slate-950/80 border border-slate-800 rounded-xl px-4 py-3 text-white focus:outline-none focus:border-indigo-500 focus:ring-1 focus:ring-indigo-500 transition" placeholder="John Doe" required>
            </div>
            <div>
                <label class="block text-sm font-semibold text-slate-300 mb-2">Email Address</label>
                <input type="email" name="email" class="w-full bg-slate-950/80 border border-slate-800 rounded-xl px-4 py-3 text-white focus:outline-none focus:border-indigo-500 focus:ring-1 focus:ring-indigo-500 transition" placeholder="you@example.com" required>
            </div>
            <div>
                <label class="block text-sm font-semibold text-slate-300 mb-2">Password</label>
                <input type="password" name="password" class="w-full bg-slate-950/80 border border-slate-800 rounded-xl px-4 py-3 text-white focus:outline-none focus:border-indigo-500 focus:ring-1 focus:ring-indigo-500 transition" placeholder="••••••••" required>
            </div>
            <button type="submit" class="w-full bg-indigo-600 text-white py-3.5 rounded-xl font-bold hover:bg-indigo-500 shadow-lg shadow-indigo-600/20 transition transform active:scale-98">
                Create Account
            </button>
        </form>
        
        <p class="text-center text-sm text-slate-400 mt-8">
            Already have an account? <a href="/login" class="text-indigo-400 font-bold hover:underline">Sign In</a>
        </p>
    </div>
</div>
@endsection
"#;
    fs::write(
        base.join("resources/views/auth/register.blade.rs"),
        register,
    )
    .ok();

    // 14. database/migrations/0001_create_users_table.sql
    let migration = r#"-- Users Table Migration
CREATE TABLE IF NOT EXISTS users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    email TEXT NOT NULL UNIQUE,
    password TEXT NOT NULL,
    role TEXT NOT NULL DEFAULT 'user',
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);
"#;
    fs::write(
        base.join("database/migrations/0001_create_users_table.sql"),
        migration,
    )
    .ok();

    // 15. src/app/models/mod.rs & user.rs
    fs::write(base.join("src/app/models/mod.rs"), "pub mod user;\n").ok();

    let user_model = r#"#![allow(dead_code)]
use async_trait::async_trait;
use serde::{Serialize, Deserialize};
use sqlx::FromRow;
use lumina::prelude::*;

//                            USER MODEL
// Di Laravel, ini adalah class Eloquent (`User extends Model`).
// Cukup tambahkan `#[derive(LuminaModel)]` untuk mendapatkan fungsi CRUD otomatis:
// `User::find(id)`, `User::all()`, dll.
#[derive(Debug, Serialize, Deserialize, FromRow, Clone, Default, LuminaModel)]
#[table("users")]
pub struct User {
    pub id: i64,
    pub name: String,
    pub email: String,
    pub password: String,
    pub role: String,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub deleted_at: Option<String>,
}

impl User {
    pub fn new() -> Self {
        Self::default()
    }
}
"#;
    fs::write(base.join("src/app/models/user.rs"), user_model).ok();

    // 16. Auto-generate comprehensive README.md
    let readme = format!(
        r#"# ⚡ Welcome to Your Lumina Backend Application: {}

Selamat! Anda telah berhasil menginisialisasi project baru dengan **Lumina Framework** — Web & RESTful API framework modern berbasis Rust (Axum 0.7 & Tokio Async) yang memadukan kecepatan komputasi sistem Rust dengan elegansi dan ergonomi sintaks **Laravel** dan **Lumen**.

---

## 🚀 Menjalankan Server Lokal (Quick Start)

Secara default, project ini menggunakan database **SQLite** (file lokal). Anda dapat langsung menjalankan server tanpa perlu setup database external:

```bash
cargo run
```

Akses di browser atau API client Anda:
- 🌐 **Web Dashboard (Laravel Style)**: [http://127.0.0.1:8000](http://127.0.0.1:8000)
- ⚡ **Lumen-Style Headless JSON Spec**:
  ```bash
  curl -H "Accept: application/json" http://127.0.0.1:8000/
  ```
- 🩺 **Health Check Endpoint**: [http://127.0.0.1:8000/health](http://127.0.0.1:8000/health)
- 🛣️ **REST API Base Path**: [http://127.0.0.1:8000/api](http://127.0.0.1:8000/api)

---

## 🗄️ Beralih ke PostgreSQL atau MySQL

Lumina mendukung multi-driver database secara native. Cukup sesuaikan variabel di `.env`:

### Contoh PostgreSQL:
```env
DB_CONNECTION=postgres
DB_HOST=127.0.0.1
DB_PORT=5432
DB_DATABASE=nama_db_kamu
DB_USERNAME=postgres
DB_PASSWORD=secret
DATABASE_URL=postgres://postgres:secret@127.0.0.1:5432/nama_db_kamu
```

### Jalankan Migrasi Database:
```bash
lumina migrate
```

---

## 📁 Struktur Folder Proyek

```text
├── Cargo.toml                  # Konfigurasi dependensi (Axum, Tokio, SQLx, Lumina)
├── .env                        # Variabel environment (Database, App Secret, Port)
├── src/
│   ├── main.rs                 # Application entrypoint & HTTP server bootstrap
│   ├── lib.rs                  # Module exports (app, routes) untuk modular testing
│   ├── routes/
│   │   ├── web.rs              # Route frontend / HTML views
│   │   └── api.rs              # Route RESTful API (prefix /api)
│   └── app/
│       ├── controllers/        # Request handling & responses
│       ├── models/             # Active Record models (#[derive(LuminaModel)])
│       └── services/           # Business logic layer
├── resources/views/            # Blade template views (.blade.rs)
├── database/migrations/        # Migrasi skema database SQL
└── tests/                      # Automated integration tests
```

---

## 🦀 Selamat Berkarya dengan Lumina Framework!
Dokumentasi lengkap dan update framework: [github.com/lumina-java/framework](https://github.com/lumina-java/framework)
"#,
        name
    );
    fs::write(base.join("README.md"), readme).ok();
}
