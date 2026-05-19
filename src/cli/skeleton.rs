use std::fs;
use std::path::Path;

pub fn create_full_skeleton(base: &Path, name: &str) {
    // 1. Create Folder Structure
    let folders = [
        "src",
        "src/app",
        "src/app/controllers",
        "src/app/models",
        "src/routes",
        "resources",
        "resources/views",
        "resources/views/auth",
        "resources/views/layouts",
        "resources/views/dashboard",
        "resources/js",
        "resources/css",
        "database",
        "database/migrations",
        "storage",
        "storage/app/public",
        "storage/cache",
        "public",
    ];
    for folder in folders {
        fs::create_dir_all(base.join(folder)).ok();
    }

    // 2. Cargo.toml
    let cargo_toml = format!(
        r#"[workspace]

[package]
name = "{}"
version = "0.1.0"
edition = "2021"
default-run = "{}-server"

[dependencies]
lumina = {{ package = "lumina-framework", git = "https://github.com/lumina-java/framework.git" }}
tokio = {{ version = "1", features = ["full"] }}
axum = {{ version = "0.7", features = ["macros", "multipart"] }}
serde = {{ version = "1", features = ["derive"] }}
serde_json = "1"
dotenv = "0.15"
async-trait = "0.1"
sqlx = {{ version = "0.8", features = ["runtime-tokio", "sqlite", "postgres", "mysql", "macros", "any", "chrono"] }}
validator = {{ version = "0.18", features = ["derive"] }}

[[bin]]
name = "{}-server"
path = "src/main.rs"
"#,
        name, name, name
    );
    fs::write(base.join("Cargo.toml"), cargo_toml).ok();

    // 3. .env.example
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

# OAuth2 (Socialite)
GOOGLE_CLIENT_ID=
GOOGLE_CLIENT_SECRET=
GOOGLE_REDIRECT_URL=http://127.0.0.1:8000/auth/google/callback

GITHUB_CLIENT_ID=
GITHUB_CLIENT_SECRET=
GITHUB_REDIRECT_URL=http://127.0.0.1:8000/auth/github/callback

# Database Configuration (Laravel Style)
DB_CONNECTION=sqlite
DB_HOST=127.0.0.1
DB_PORT=3306
DB_DATABASE=./database.sqlite
DB_USERNAME=root
DB_PASSWORD=

# Atau gunakan URL eksplisit (jika DATABASE_URL ada, maka variabel DB_* diabaikan)
# DATABASE_URL=sqlite:./database.sqlite

# Rate Limiting
RATE_LIMIT_MAX=60
RATE_LIMIT_WINDOW=60

# Storage Settings
# Batas maksimal upload file dalam satuan Megabyte
UPLOAD_MAX_SIZE_MB=2

# Session Configuration
JWT_SECRET=lumina-jwt-secret-key-change-me-in-production

# ──────────────────────────────────────────────────────────────────────────────
"#;
    fs::write(base.join(".env.example"), env_example).ok();
    fs::write(base.join(".env"), env_example).ok();
    fs::write(base.join("database.sqlite"), "").ok();

    // 4. src/main.rs
    let main_rs = r#"mod app;
mod routes;

use lumina::prelude::*;

// =========================================================================
//                  LUMINA FRAMEWORK APPLICATION ENTRYPOINT
// =========================================================================
// This is the core entrypoint of your Lumina application.
// Here, we load environment variables, register routes, and launch the server.

#[tokio::main]
async fn main() {
    // 1. Load configuration from the .env file
    dotenv::dotenv().ok();

    // 2. Initialize the application engine and web router
    let app = Application::new()
        .with_web(routes::web::router());

    println!("✨ Welcome to the Lumina Framework");
    println!("🚀 Starting server at http://127.0.0.1:8000");

    // 3. Serve the application
    app.serve("127.0.0.1:8000").await;
}
"#;
    fs::write(base.join("src/main.rs"), main_rs).ok();

    // 5. src/app/mod.rs
    fs::write(base.join("src/app/mod.rs"), "pub mod controllers;\npub mod models;\n").ok();

    // 6. src/app/controllers/mod.rs
    fs::write(
        base.join("src/app/controllers/mod.rs"),
        "pub mod welcome_controller;\npub mod dashboard_controller;\n",
    ).ok();

    // 7. src/app/controllers/welcome_controller.rs
    let welcome_ctrl = r#"use axum::response::IntoResponse;
use lumina::core::request::Request;

// =========================================================================
//                        WELCOME CONTROLLER
// =========================================================================
// Controllers handle incoming HTTP requests and return responses.
// In Lumina, controllers are simple async functions. They can receive Request
// automatically which provides access to views, database, and session.

pub struct WelcomeController;

impl WelcomeController {
    // Renders the main welcoming landing page
    pub async fn index(req: Request) -> impl IntoResponse {
        req.view("welcome")
            .with("title", "Welcome to Lumina")
            .with("app_name", "Lumina Framework")
            .with("version", "0.1.0")
            .render(&req)
            .await
    }

    // Renders the Login page
    pub async fn login(req: Request) -> impl IntoResponse {
        req.view("auth.login")
            .with("title", "Login - Lumina")
            .render(&req)
            .await
    }

    // Renders the Register page
    pub async fn register(req: Request) -> impl IntoResponse {
        req.view("auth.register")
            .with("title", "Register - Lumina")
            .render(&req)
            .await
    }
}
"#;
    fs::write(base.join("src/app/controllers/welcome_controller.rs"), welcome_ctrl).ok();

    // 7b. src/app/controllers/dashboard_controller.rs
    let dashboard_ctrl = r#"use axum::response::IntoResponse;
use lumina::core::request::Request;

pub struct DashboardController;

impl DashboardController {
    /// GET /dashboard — Main dashboard page (requires auth)
    pub async fn index(req: Request) -> impl IntoResponse {
        req.view("dashboard.index")
            .with("title", "Dashboard")
            .render(&req)
            .await
    }
}
"#;
    fs::write(base.join("src/app/controllers/dashboard_controller.rs"), dashboard_ctrl).ok();

    // 8. routes/mod.rs & routes/web.rs
    fs::write(base.join("src/routes/mod.rs"), "pub mod web;\n").ok();

    let web_routes = r#"use lumina::prelude::*;
use crate::app::controllers::welcome_controller::WelcomeController;
use crate::app::controllers::dashboard_controller::DashboardController;

// =========================================================================
//                            WEB ROUTING
// =========================================================================

pub fn router() -> Router<AppState> {
    Router::new()
        .get("/",            WelcomeController::index)
        .get("/login",       WelcomeController::login)
        .get("/register",    WelcomeController::register)
        .get("/dashboard",   DashboardController::index)
}
"#;
    fs::write(base.join("src/routes/web.rs"), web_routes).ok();

    // 9. resources/views/layouts/app.blade.rs
    let layout = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{{ title | default(value="Lumina Framework") }}</title>
    <script src="https://cdn.tailwindcss.com"></script>
    <link rel="preconnect" href="https://fonts.googleapis.com">
    <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
    <link href="https://fonts.googleapis.com/css2?family=Plus+Jakarta+Sans:wght@300;400;500;600;700;800&family=JetBrains+Mono:wght@400;500;600;700&display=swap" rel="stylesheet">
    <style>
        /* =============================================
           LUMINA DESIGN TOKENS - CSS Custom Properties
           ============================================= */
        :root {
            --color-primary:   #448aff;
            --color-success:   #2ed8b6;
            --color-danger:    #ff5370;
            --color-warning:   #ffb64d;
            --color-dark-bg:   #0b0f19;
            --color-panel-bg:  #111827;
            --color-card-bg:   #151c2c;
            --color-border-bg: #1f293d;
        }
        *, *::before, *::after { box-sizing: border-box; }
        html { background-color: var(--color-dark-bg); }
        body {
            font-family: 'Plus Jakarta Sans', sans-serif;
            background-color: var(--color-dark-bg);
            color: #e2e8f0;
            min-height: 100vh;
            display: flex;
            flex-direction: column;
        }
        .code-font { font-family: 'JetBrains Mono', monospace; }
        /* Bg utilities */
        .bg-dark-bg   { background-color: var(--color-dark-bg); }
        .bg-panel-bg  { background-color: var(--color-panel-bg); }
        .bg-card-bg   { background-color: var(--color-card-bg); }
        .bg-primary   { background-color: var(--color-primary); }
        .bg-success   { background-color: var(--color-success); }
        .bg-danger    { background-color: var(--color-danger); }
        .bg-warning   { background-color: var(--color-warning); }
        /* Text utilities */
        .text-primary  { color: var(--color-primary); }
        .text-success  { color: var(--color-success); }
        .text-danger   { color: var(--color-danger); }
        .text-warning  { color: var(--color-warning); }
        .text-dark-bg  { color: var(--color-dark-bg); }
        /* Border utilities */
        .border-card    { border-color: var(--color-border-bg); }
        .border-primary { border-color: var(--color-primary); }
        /* Navbar */
        .navbar {
            background-color: rgba(17,24,39,0.85);
            backdrop-filter: blur(12px);
            border-bottom: 1px solid var(--color-border-bg);
            position: sticky; top: 0; z-index: 50;
        }
        .nav-link {
            color: #cbd5e1; font-weight: 500; font-size: 0.75rem;
            padding: 4px 8px; border-radius: 6px;
            transition: color 0.15s, background-color 0.15s; text-decoration: none;
        }
        .nav-link:hover { color: var(--color-primary); background-color: #1e293b; }
        .btn-primary {
            background-color: var(--color-primary); color: #fff;
            font-weight: 700; font-size: 0.75rem; padding: 5px 12px;
            border-radius: 6px; transition: background-color 0.15s;
            text-decoration: none; display: inline-block;
        }
        .btn-primary:hover { background-color: #2563eb; }
        .btn-success {
            background-color: var(--color-success); color: var(--color-dark-bg);
            font-weight: 700; font-size: 0.75rem; padding: 5px 12px;
            border-radius: 6px; transition: background-color 0.15s;
            text-decoration: none; display: inline-block;
        }
        .btn-success:hover { background-color: #10b981; }
        /* Card */
        .card { background-color: var(--color-card-bg); border: 1px solid var(--color-border-bg); border-radius: 8px; }
        .card-header { border-bottom: 1px solid var(--color-border-bg); padding: 10px 16px; }
        /* Table */
        .table-lumina { width: 100%; border-collapse: collapse; font-size: 0.75rem; }
        .table-lumina thead tr { background-color: rgba(11,15,25,0.60); color: #94a3b8; text-transform: uppercase; font-size: 0.625rem; font-weight: 700; border-bottom: 1px solid var(--color-border-bg); }
        .table-lumina th, .table-lumina td { padding: 6px 12px; text-align: left; }
        .table-lumina tbody tr { border-bottom: 1px solid var(--color-border-bg); color: #cbd5e1; }
        .table-lumina tbody tr:hover { background-color: rgba(11,15,25,0.30); }
        /* Form */
        .form-input { width: 100%; background-color: var(--color-dark-bg); border: 1px solid var(--color-border-bg); border-radius: 5px; padding: 6px 10px; color: #fff; font-size: 0.6875rem; outline: none; transition: border-color 0.15s; font-family: 'Plus Jakarta Sans', sans-serif; }
        .form-input:focus { border-color: var(--color-primary); }
        .form-label { display: block; font-weight: 600; color: #cbd5e1; margin-bottom: 4px; font-size: 0.75rem; }
        /* Code block */
        .code-block { display: flex; align-items: center; justify-content: space-between; background-color: var(--color-dark-bg); border: 1px solid var(--color-border-bg); padding: 8px 10px; border-radius: 6px; font-family: 'JetBrains Mono', monospace; font-size: 0.75rem; }
        .code-block button { color: var(--color-primary); font-weight: 700; font-size: 0.6875rem; background: none; border: none; cursor: pointer; }
        .code-block button:hover { text-decoration: underline; }
        /* Badge */
        .badge { display: inline-block; font-weight: 700; font-size: 0.625rem; padding: 2px 8px; border-radius: 999px; }
        .badge-primary { background: rgba(68,138,255,0.15); color: var(--color-primary); }
        .badge-success { background: rgba(46,216,182,0.15); color: var(--color-success); }
        .badge-warning { background: rgba(255,182,77,0.15);  color: var(--color-warning); }
        .badge-danger  { background: rgba(255,83,112,0.15);  color: var(--color-danger); }
        /* Scrollbar */
        ::-webkit-scrollbar { width: 6px; height: 6px; }
        ::-webkit-scrollbar-track { background: var(--color-dark-bg); }
        ::-webkit-scrollbar-thumb { background: var(--color-border-bg); border-radius: 3px; }
        ::-webkit-scrollbar-thumb:hover { background: var(--color-primary); }
        @keyframes pulse { 0%, 100% { opacity: 1; } 50% { opacity: 0.4; } }
        .pulse { animation: pulse 2s cubic-bezier(0.4, 0, 0.6, 1) infinite; }
        main { flex-grow: 1; }
    </style>
</head>
<body>
    <!-- Top Navigation Header -->
    <nav class="navbar">
        <div style="max-width:1280px;margin:0 auto;padding:0 16px;display:flex;justify-content:space-between;align-items:center;height:48px;">
            <div style="display:flex;align-items:center;gap:8px;">
                <div style="width:28px;height:28px;background:linear-gradient(135deg,#448aff,#6366f1);border-radius:8px;display:flex;align-items:center;justify-content:center;box-shadow:0 4px 12px rgba(68,138,255,0.25);">
                    <span style="color:white;font-weight:800;font-size:14px;">L</span>
                </div>
                <span class="code-font" style="font-size:15px;font-weight:800;letter-spacing:-0.5px;color:#fff;">lumina</span>
                <span style="background:rgba(68,138,255,0.12);border:1px solid rgba(68,138,255,0.2);color:var(--color-primary);font-size:9px;font-weight:700;padding:1px 6px;border-radius:4px;letter-spacing:1px;">v0.1</span>
            </div>
            <div style="display:flex;align-items:center;gap:8px;">
                <a href="/" class="nav-link">Home</a>
                <a href="/login" class="nav-link">Login</a>
                <a href="/register" class="btn-primary">Get Started</a>
            </div>
        </div>
    </nav>

    <!-- Main Content Slot -->
    <main>
        @yield('content')
    </main>

    <!-- Footer Segment -->
    <footer style="border-top:1px solid var(--color-border-bg);background:rgba(17,24,39,0.40);padding:10px 0;">
        <div style="max-width:1280px;margin:0 auto;padding:0 16px;display:flex;justify-content:space-between;align-items:center;flex-wrap:wrap;gap:8px;">
            <div style="color:#64748b;font-size:0.7rem;">&copy; 2026 Lumina. Premium Desktop Webbase.</div>
            <div style="color:#94a3b8;font-size:0.7rem;display:flex;align-items:center;gap:4px;">Engineered with <span style="color:var(--color-danger);">❤️</span> in Rust for elegant speed.</div>
        </div>
    </footer>
</body>
</html>
"#;
    fs::write(base.join("resources/views/layouts/app.blade.rs"), layout).ok();

    // 10. resources/views/welcome.blade.rs
    let welcome = r#"@extends('layouts.app')

@section('content')
<div style="max-width:1280px;margin:0 auto;padding:24px 16px;">

    <!-- Top System Greeting & Status Badges -->
    <div class="card" style="padding:16px;margin-bottom:20px;display:flex;flex-wrap:wrap;justify-content:space-between;align-items:flex-start;gap:12px;">
        <div>
            <h1 class="code-font" style="font-size:1rem;font-weight:800;color:#fff;display:flex;align-items:center;gap:8px;margin:0 0 4px 0;">
                <span class="pulse" style="width:10px;height:10px;border-radius:50%;background:var(--color-success);display:inline-block;"></span>
                Lumina Framework Console
            </h1>
            <p style="color:#94a3b8;font-size:0.7rem;margin:0;">Welcome to your enterprise-ready desktop environment.</p>
        </div>
        <div style="display:flex;flex-wrap:wrap;gap:6px;">
            <span class="badge badge-primary" style="border:1px solid rgba(68,138,255,0.2);">ENV: LOCAL</span>
            <span class="badge badge-success" style="border:1px solid rgba(46,216,182,0.2);">DB: SQLITE CONNECTED</span>
            <span class="badge badge-warning" style="border:1px solid rgba(255,182,77,0.2);">PORT: 8000</span>
        </div>
    </div>

    <!-- Main 2-Column Split Dashboard -->
    <div style="display:grid;grid-template-columns:2fr 1fr;gap:20px;margin-bottom:20px;">

        <!-- Left: Quick Commands Console -->
        <div class="card" style="padding:16px;">
            <div class="card-header" style="margin:-16px -16px 12px -16px;display:flex;align-items:center;gap:8px;">
                <svg style="width:16px;height:16px;color:var(--color-primary);" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" d="M8 9l3 3-3 3m5 0h3M5 20h14a2 2 0 002-2V6a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z"/>
                </svg>
                <h2 class="code-font" style="font-size:0.8rem;font-weight:700;color:#fff;margin:0;">Developer Quick-start Command Line</h2>
            </div>
            <p style="color:#94a3b8;font-size:0.7rem;margin:0 0 10px 0;">Copy and run these commands in your project terminal to instantly generate assets.</p>
            <div style="display:flex;flex-direction:column;gap:6px;">
                <div class="code-block"><span style="color:#cbd5e1;">lumina make:controller ProductController</span><button onclick="navigator.clipboard.writeText('lumina make:controller ProductController')">Copy</button></div>
                <div class="code-block"><span style="color:#cbd5e1;">lumina make:model Product</span><button onclick="navigator.clipboard.writeText('lumina make:model Product')">Copy</button></div>
                <div class="code-block"><span style="color:#cbd5e1;">lumina make:crud Patient name:string age:integer</span><button onclick="navigator.clipboard.writeText('lumina make:crud Patient name:string age:integer')">Copy</button></div>
                <div class="code-block"><span style="color:#cbd5e1;">lumina make:auth</span><button onclick="navigator.clipboard.writeText('lumina make:auth')">Copy</button></div>
                <div class="code-block"><span style="color:#cbd5e1;">lumina migrate</span><button onclick="navigator.clipboard.writeText('lumina migrate')">Copy</button></div>
            </div>
        </div>

        <!-- Right: System Info Panel -->
        <div class="card" style="padding:16px;display:flex;flex-direction:column;justify-content:space-between;">
            <div>
                <div class="card-header" style="margin:-16px -16px 12px -16px;display:flex;align-items:center;gap:8px;">
                    <svg style="width:16px;height:16px;color:var(--color-warning);" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"/>
                    </svg>
                    <h2 class="code-font" style="font-size:0.8rem;font-weight:700;color:#fff;margin:0;">System Information</h2>
                </div>
                <div style="font-size:0.7rem;display:flex;flex-direction:column;gap:8px;">
                    <div style="display:flex;justify-content:space-between;border-bottom:1px solid var(--color-border-bg);padding-bottom:6px;"><span style="color:#94a3b8;">Framework:</span><span style="color:#fff;font-weight:700;">Lumina Rust Core</span></div>
                    <div style="display:flex;justify-content:space-between;border-bottom:1px solid var(--color-border-bg);padding-bottom:6px;"><span style="color:#94a3b8;">Database:</span><span style="color:#fff;font-weight:700;">SQLite 3.x</span></div>
                    <div style="display:flex;justify-content:space-between;border-bottom:1px solid var(--color-border-bg);padding-bottom:6px;"><span style="color:#94a3b8;">Templates:</span><span style="color:#fff;font-weight:700;">Tera Engine</span></div>
                    <div style="display:flex;justify-content:space-between;"><span style="color:#94a3b8;">Sessions:</span><span style="color:#fff;font-weight:700;">SQLite Pool</span></div>
                </div>
            </div>
            <div style="margin-top:16px;">
                <a href="/login" class="btn-success" style="display:block;text-align:center;">Access Admin Login &rarr;</a>
            </div>
        </div>
    </div>

    <!-- Bottom: Activity Log Table -->
    <div class="card" style="padding:16px;">
        <div style="display:flex;justify-content:space-between;align-items:center;margin-bottom:12px;">
            <h2 class="code-font" style="font-size:0.8rem;font-weight:700;color:#fff;margin:0;">Recent Application Activity Logs</h2>
            <span class="badge" style="background:#1e293b;color:#94a3b8;">Read-Only View</span>
        </div>
        <div style="overflow-x:auto;">
            <table class="table-lumina">
                <thead><tr><th>Timestamp</th><th>Module</th><th>Action</th><th>Status</th></tr></thead>
                <tbody>
                    <tr><td class="code-font" style="color:#64748b;">2026-05-19 08:34:12</td><td style="font-weight:600;">User Authentication</td><td style="color:#94a3b8;">Scaffolded registration views.</td><td><span class="badge badge-success">SUCCESS</span></td></tr>
                    <tr><td class="code-font" style="color:#64748b;">2026-05-19 08:32:05</td><td style="font-weight:600;">Database Schema</td><td style="color:#94a3b8;">Ran all outstanding migrations.</td><td><span class="badge badge-success">SUCCESS</span></td></tr>
                    <tr><td class="code-font" style="color:#64748b;">2026-05-19 08:30:00</td><td style="font-weight:600;">Lumina Engine</td><td style="color:#94a3b8;">Bootstrapped new project instance.</td><td><span class="badge badge-primary">INITIALIZED</span></td></tr>
                </tbody>
            </table>
        </div>
    </div>
</div>
@endsection
"#;
    fs::write(base.join("resources/views/welcome.blade.rs"), welcome).ok();

    // 11. resources/views/auth/login.blade.rs
    let login = r#"@extends('layouts.app')

@section('content')
<div style="min-height:75vh;display:flex;align-items:center;justify-content:center;padding:24px 16px;position:relative;">
    <div style="position:absolute;top:30%;left:50%;transform:translate(-50%,-50%);width:400px;height:400px;background:radial-gradient(circle,rgba(68,138,255,0.06) 0%,transparent 70%);pointer-events:none;"></div>
    <div class="card" style="width:100%;max-width:320px;padding:24px;position:relative;z-index:1;box-shadow:0 20px 60px rgba(0,0,0,0.5);">
        <div style="text-align:center;margin-bottom:20px;">
            <div style="width:40px;height:40px;background:linear-gradient(135deg,#448aff,#6366f1);border-radius:10px;display:flex;align-items:center;justify-content:center;margin:0 auto 10px auto;box-shadow:0 4px 16px rgba(68,138,255,0.3);">
                <span style="color:white;font-weight:900;font-size:18px;font-family:'JetBrains Mono',monospace;">L</span>
            </div>
            <h2 class="code-font" style="font-size:0.875rem;font-weight:800;color:#fff;margin:0 0 4px 0;">Welcome Back</h2>
            <p style="color:#94a3b8;font-size:0.6875rem;margin:0;">Enter credentials to access account</p>
        </div>
        <form action="/login" method="POST" style="display:flex;flex-direction:column;gap:12px;">
            <div>
                <label class="form-label">Email Address</label>
                <input type="email" name="email" class="form-input" placeholder="you@example.com" required>
            </div>
            <div>
                <label class="form-label">Password</label>
                <input type="password" name="password" class="form-input" placeholder="••••••••" required>
            </div>
            <button type="submit" style="width:100%;background:var(--color-primary);color:#fff;border:none;padding:8px 12px;border-radius:6px;font-weight:700;font-size:0.8rem;cursor:pointer;font-family:'Plus Jakarta Sans',sans-serif;">Sign In</button>
        </form>
        <div style="display:flex;align-items:center;gap:10px;margin:16px 0;">
            <div style="flex:1;height:1px;background:var(--color-border-bg);"></div>
            <span style="color:#475569;font-size:0.6rem;font-weight:600;letter-spacing:1px;">OR</span>
            <div style="flex:1;height:1px;background:var(--color-border-bg);"></div>
        </div>
        <p style="text-align:center;font-size:0.6875rem;color:#94a3b8;margin:0;">
            Don't have an account? <a href="/register" style="color:var(--color-primary);font-weight:700;text-decoration:none;">Register</a>
        </p>
    </div>
</div>
@endsection
"#;
    fs::write(base.join("resources/views/auth/login.blade.rs"), login).ok();

    // 12. resources/views/auth/register.blade.rs
    let register = r#"@extends('layouts.app')

@section('content')
<div style="min-height:75vh;display:flex;align-items:center;justify-content:center;padding:24px 16px;position:relative;">
    <div style="position:absolute;top:30%;left:50%;transform:translate(-50%,-50%);width:400px;height:400px;background:radial-gradient(circle,rgba(46,216,182,0.05) 0%,transparent 70%);pointer-events:none;"></div>
    <div class="card" style="width:100%;max-width:340px;padding:24px;position:relative;z-index:1;box-shadow:0 20px 60px rgba(0,0,0,0.5);">
        <div style="text-align:center;margin-bottom:20px;">
            <div style="width:40px;height:40px;background:linear-gradient(135deg,#2ed8b6,#448aff);border-radius:10px;display:flex;align-items:center;justify-content:center;margin:0 auto 10px auto;box-shadow:0 4px 16px rgba(46,216,182,0.25);">
                <span style="color:#0b0f19;font-weight:900;font-size:18px;font-family:'JetBrains Mono',monospace;">L</span>
            </div>
            <h2 class="code-font" style="font-size:0.875rem;font-weight:800;color:#fff;margin:0 0 4px 0;">Create Account</h2>
            <p style="color:#94a3b8;font-size:0.6875rem;margin:0;">Get started with the Lumina ecosystem</p>
        </div>
        <form action="/register" method="POST" style="display:flex;flex-direction:column;gap:12px;">
            <div>
                <label class="form-label">Full Name</label>
                <input type="text" name="name" class="form-input" placeholder="John Doe" required>
            </div>
            <div>
                <label class="form-label">Email Address</label>
                <input type="email" name="email" class="form-input" placeholder="you@example.com" required>
            </div>
            <div>
                <label class="form-label">Password</label>
                <input type="password" name="password" class="form-input" placeholder="Minimum 8 characters" required>
            </div>
            <div>
                <label class="form-label">Confirm Password</label>
                <input type="password" name="password_confirmation" class="form-input" placeholder="Repeat password" required>
            </div>
            <button type="submit" style="width:100%;background:var(--color-success);color:var(--color-dark-bg);border:none;padding:8px 12px;border-radius:6px;font-weight:700;font-size:0.8rem;cursor:pointer;font-family:'Plus Jakarta Sans',sans-serif;">Create Account</button>
        </form>
        <div style="display:flex;align-items:center;gap:10px;margin:16px 0;">
            <div style="flex:1;height:1px;background:var(--color-border-bg);"></div>
            <span style="color:#475569;font-size:0.6rem;font-weight:600;letter-spacing:1px;">OR</span>
            <div style="flex:1;height:1px;background:var(--color-border-bg);"></div>
        </div>
        <p style="text-align:center;font-size:0.6875rem;color:#94a3b8;margin:0;">
            Already have an account? <a href="/login" style="color:var(--color-primary);font-weight:700;text-decoration:none;">Sign In</a>
        </p>
    </div>
</div>
@endsection
"#;
    fs::write(base.join("resources/views/auth/register.blade.rs"), register).ok();

    // 13b. resources/views/dashboard/index.blade.rs
    let dashboard = r#"@extends('layouts.app')

@section('content')
<div style="max-width:1280px;margin:0 auto;padding:24px 16px;">

    {{-- Page Header --}}
    <div style="display:flex;justify-content:space-between;align-items:center;margin-bottom:20px;">
        <div>
            <h1 class="code-font" style="font-size:1.1rem;font-weight:800;color:#fff;margin:0 0 4px 0;">Dashboard</h1>
            <p style="color:#94a3b8;font-size:0.7rem;margin:0;">Overview &amp; application statistics</p>
        </div>
        <div style="display:flex;gap:8px;align-items:center;">
            <span class="badge badge-success" style="border:1px solid rgba(46,216,182,0.2);">
                <span class="pulse" style="width:6px;height:6px;border-radius:50%;background:var(--color-success);display:inline-block;margin-right:4px;"></span>
                System Online
            </span>
            <a href="/logout" style="color:#94a3b8;font-size:0.7rem;text-decoration:none;padding:4px 8px;border:1px solid var(--color-border-bg);border-radius:5px;">Logout</a>
        </div>
    </div>

    {{-- Stats Row --}}
    <div style="display:grid;grid-template-columns:repeat(4,1fr);gap:16px;margin-bottom:20px;">
        <div class="card" style="padding:16px;">
            <p style="color:#64748b;font-size:0.65rem;font-weight:700;text-transform:uppercase;letter-spacing:1px;margin:0 0 8px 0;">Total Users</p>
            <p style="color:#fff;font-size:1.5rem;font-weight:800;margin:0 0 4px 0;">{{ total_users | default(value=0) }}</p>
            <p style="color:var(--color-success);font-size:0.65rem;margin:0;">&#x2191; Active accounts</p>
        </div>
        <div class="card" style="padding:16px;">
            <p style="color:#64748b;font-size:0.65rem;font-weight:700;text-transform:uppercase;letter-spacing:1px;margin:0 0 8px 0;">Sessions Today</p>
            <p style="color:#fff;font-size:1.5rem;font-weight:800;margin:0 0 4px 0;">{{ sessions_today | default(value=0) }}</p>
            <p style="color:var(--color-primary);font-size:0.65rem;margin:0;">&#x2191; Active sessions</p>
        </div>
        <div class="card" style="padding:16px;">
            <p style="color:#64748b;font-size:0.65rem;font-weight:700;text-transform:uppercase;letter-spacing:1px;margin:0 0 8px 0;">Database Size</p>
            <p style="color:#fff;font-size:1.5rem;font-weight:800;margin:0 0 4px 0;">{{ db_size | default(value="0 KB") }}</p>
            <p style="color:var(--color-warning);font-size:0.65rem;margin:0;">SQLite storage</p>
        </div>
        <div class="card" style="padding:16px;">
            <p style="color:#64748b;font-size:0.65rem;font-weight:700;text-transform:uppercase;letter-spacing:1px;margin:0 0 8px 0;">Uptime</p>
            <p style="color:#fff;font-size:1.5rem;font-weight:800;margin:0 0 4px 0;">{{ uptime | default(value="--") }}</p>
            <p style="color:var(--color-success);font-size:0.65rem;margin:0;">Server running</p>
        </div>
    </div>

    {{-- Main Content Grid --}}
    <div style="display:grid;grid-template-columns:2fr 1fr;gap:16px;">

        {{-- Quick Actions --}}
        <div class="card" style="padding:16px;">
            <div class="card-header" style="margin:-16px -16px 14px -16px;display:flex;align-items:center;gap:8px;">
                <svg style="width:14px;height:14px;color:var(--color-primary);" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" d="M13 10V3L4 14h7v7l9-11h-7z"/></svg>
                <h2 class="code-font" style="font-size:0.78rem;font-weight:700;color:#fff;margin:0;">Quick Actions</h2>
            </div>
            <div style="display:grid;grid-template-columns:1fr 1fr;gap:8px;">
                <a href="/users" class="card" style="padding:12px;text-decoration:none;display:flex;align-items:center;gap:10px;transition:border-color 0.15s;" onmouseover="this.style.borderColor='var(--color-primary)'" onmouseout="this.style.borderColor='var(--color-border-bg)'">
                    <div style="width:32px;height:32px;background:rgba(68,138,255,0.12);border-radius:8px;display:flex;align-items:center;justify-content:center;flex-shrink:0;">
                        <svg style="width:14px;height:14px;color:var(--color-primary);" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0z"/></svg>
                    </div>
                    <div><p style="color:#fff;font-weight:700;font-size:0.75rem;margin:0;">Manage Users</p><p style="color:#64748b;font-size:0.65rem;margin:0;">View all users</p></div>
                </a>
                <a href="/settings" class="card" style="padding:12px;text-decoration:none;display:flex;align-items:center;gap:10px;transition:border-color 0.15s;" onmouseover="this.style.borderColor='var(--color-warning)'" onmouseout="this.style.borderColor='var(--color-border-bg)'">
                    <div style="width:32px;height:32px;background:rgba(255,182,77,0.12);border-radius:8px;display:flex;align-items:center;justify-content:center;flex-shrink:0;">
                        <svg style="width:14px;height:14px;color:var(--color-warning);" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z"/><path stroke-linecap="round" stroke-linejoin="round" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"/></svg>
                    </div>
                    <div><p style="color:#fff;font-weight:700;font-size:0.75rem;margin:0;">Settings</p><p style="color:#64748b;font-size:0.65rem;margin:0;">App configuration</p></div>
                </a>
            </div>
        </div>

        {{-- System Info --}}
        <div class="card" style="padding:16px;">
            <div class="card-header" style="margin:-16px -16px 14px -16px;display:flex;align-items:center;gap:8px;">
                <svg style="width:14px;height:14px;color:var(--color-success);" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"/></svg>
                <h2 class="code-font" style="font-size:0.78rem;font-weight:700;color:#fff;margin:0;">System Status</h2>
            </div>
            <div style="display:flex;flex-direction:column;gap:8px;font-size:0.7rem;">
                <div style="display:flex;justify-content:space-between;border-bottom:1px solid var(--color-border-bg);padding-bottom:6px;">
                    <span style="color:#94a3b8;">Framework</span>
                    <span style="color:var(--color-success);font-weight:700;">Lumina v0.1</span>
                </div>
                <div style="display:flex;justify-content:space-between;border-bottom:1px solid var(--color-border-bg);padding-bottom:6px;">
                    <span style="color:#94a3b8;">Database</span>
                    <span style="color:var(--color-success);font-weight:700;">Connected</span>
                </div>
                <div style="display:flex;justify-content:space-between;border-bottom:1px solid var(--color-border-bg);padding-bottom:6px;">
                    <span style="color:#94a3b8;">Sessions</span>
                    <span style="color:var(--color-success);font-weight:700;">Active</span>
                </div>
                <div style="display:flex;justify-content:space-between;">
                    <span style="color:#94a3b8;">Migrations</span>
                    <span style="color:var(--color-success);font-weight:700;">Up to date</span>
                </div>
            </div>
        </div>
    </div>
</div>
@endsection
"#;
    fs::write(base.join("resources/views/dashboard/index.blade.rs"), dashboard).ok();

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
    fs::write(base.join("database/migrations/0001_create_users_table.sql"), migration).ok();

    // 14. src/app/models/mod.rs & user.rs
    fs::write(base.join("src/app/models/mod.rs"), "pub mod user;\n").ok();
    
    let user_model = r#"#![allow(dead_code)]
use async_trait::async_trait;
use serde::{Serialize, Deserialize};
use sqlx::FromRow;
use lumina::database::{connection::DatabasePool, model::Model};

// =========================================================================
//                            USER MODEL
// =========================================================================
// Models map structural database records into Rust types.
// The `Model` trait tells Lumina what SQLite database table to fetch from.

#[derive(Debug, Serialize, Deserialize, FromRow, Clone, Default)]
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

    pub async fn find_by_email(pool: &DatabasePool, email: &str) -> Result<Self, sqlx::Error> {
        Self::query(pool).where_eq("email", email).first().await
    }
}

#[async_trait]
impl Model for User {
    const TABLE: &'static str = "users";

    async fn find(pool: &DatabasePool, id: i64) -> Result<Self, sqlx::Error> {
        Self::query(pool).where_eq("id", id).first().await
    }

    async fn all(pool: &DatabasePool) -> Result<Vec<Self>, sqlx::Error> {
        Self::query(pool).get().await
    }

    async fn save(&self, pool: &DatabasePool) -> Result<i64, sqlx::Error> {
        if self.id > 0 {
            sqlx::query("UPDATE users SET name = ?, email = ?, password = ?, role = ? WHERE id = ?")
                .bind(&self.name)
                .bind(&self.email)
                .bind(&self.password)
                .bind(&self.role)
                .bind(self.id)
                .execute(&pool.pool)
                .await?;
            Ok(self.id)
        } else {
            let result = sqlx::query(
                "INSERT INTO users (name, email, password, role) VALUES (?, ?, ?, ?)"
            )
            .bind(&self.name)
            .bind(&self.email)
            .bind(&self.password)
            .bind(&self.role)
            .execute(&pool.pool)
            .await?;

            Ok(result.last_insert_id().unwrap_or(0))
        }
    }

    async fn delete(pool: &DatabasePool, id: i64) -> Result<bool, sqlx::Error> {
        sqlx::query("UPDATE users SET deleted_at = CURRENT_TIMESTAMP WHERE id = ?")
            .bind(id)
            .execute(&pool.pool)
            .await?;

        Ok(true)
    }
}
"#;
    fs::write(base.join("src/app/models/user.rs"), user_model).ok();

    // 15. Auto-generate comprehensive, educational README.md
    let readme = format!(
        r#"# ⚡ Welcome to Your New Lumina Project: {}

Congratulations! You have successfully scaffolded a fresh project with the **Lumina Framework** — the highly ergonomic, high-performance web framework designed for developers who love the speed & memory-safety of **Rust** mixed with the clean, elegant MVC patterns of **Laravel**.

---

## 🚀 Quick Start Guide

Ready to get running? Just follow these simple steps:

### 1. Copy local configuration
We already copied `.env.example` to `.env` for you! Open `.env` to check your settings:
```bash
# Look inside your .env configuration
APP_URL=http://localhost:8000
DB_CONNECTION=sqlite
DATABASE_URL=sqlite:./database.sqlite
```

### 2. Launch the Application Server
Run the cargo command in your terminal:
```bash
cargo run
```
Your server is now active! Open your browser and navigate to:
👉 **[http://localhost:8000](http://localhost:8000)**

---

## 📁 Understanding the Folder Structure

Lumina follows a clean, intuitive MVC layout to make it incredibly easy for beginners and laypeople to explore:

*   **`src/`** — Houses all Rust source logic.
    *   **`src/main.rs`** — The entrypoint of your server where everything is loaded.
    *   **`src/routes/web.rs`** — Register all of your web endpoints here.
    *   **`src/app/controllers/`** — Write your controller handlers to handle requests.
    *   **`src/app/models/`** — Structural database models (e.g., `User` model).
*   **`resources/views/`** — HTML frontend templates using the Blade-style format.
*   **`database/migrations/`** — Standard SQL files to structure your database schemas.
*   **`storage/`** — Internal caching and public asset uploads.

---

## 💡 How to Add a New Route & View (Step-by-Step)

Want to add a custom `/about` page? It takes just 3 simple steps:

### Step A: Create the HTML View
Create a file at `resources/views/about.blade.rs` and write standard HTML:
```html
@extends('layouts.app')

@section('content')
<div class="max-w-4xl mx-auto px-4 py-20 text-center">
    <h1 class="text-4xl font-extrabold heading-font text-white">About Lumina</h1>
    <p class="text-slate-400 mt-4 text-lg">This is my first Lumina custom view!</p>
</div>
@endsection
```

### Step B: Create a Controller Method
Open `src/app/controllers/welcome_controller.rs` and add a new method:
```rust
pub async fn about(State(state): State<AppState>) -> Html<String> {{
    let mut ctx = Context::new();
    ctx.insert("title", "About Us");
    Html(state.view.render("about.blade.rs", &ctx))
}}
```

### Step C: Register the Route
Open `src/routes/web.rs` and link your URL to your new controller method:
```rust
pub fn router() -> Router<AppState> {{
    Router::new()
        .get("/", WelcomeController::index)
        .get("/about", WelcomeController::about) // <-- Just add this line!
}}
```
Compile and run again! Your new page will be live at `http://localhost:8000/about`.

---

## 🦀 Built with Pride in Rust
Enjoy building something incredibly fast, secure, and beautiful with **Lumina**!
"#,
        name
    );
    fs::write(base.join("README.md"), readme).ok();
}
