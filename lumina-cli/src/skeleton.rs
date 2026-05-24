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
    ];
    for folder in folders {
        fs::create_dir_all(base.join(folder)).ok();
    }

    // 2. Cargo.toml
    let cargo_toml = format!(
        r#"[package]
name = "{}"
version = "0.1.0"
edition = "2021"
default-run = "{}-server"

[dependencies]
lumina = {{ package = "lumina-framework", path = "../framework" }}
tokio = {{ version = "1", features = ["full"] }}
axum = {{ version = "0.7", features = ["macros", "multipart"] }}
serde = {{ version = "1", features = ["derive"] }}
serde_json = "1"
dotenv = "0.15"
tera = "1.19"
async-trait = "0.1"
sqlx = {{ version = "0.8", features = ["runtime-tokio", "sqlite", "any", "macros"] }}

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
    fs::write(base.join("src/app/controllers/mod.rs"), "pub mod welcome_controller;\n").ok();

    // 7. src/app/controllers/welcome_controller.rs
    let welcome_ctrl = r#"use lumina::prelude::*;

// =========================================================================
//                        WELCOME CONTROLLER
// =========================================================================
// Controllers handle incoming HTTP requests and return responses.
// In Lumina, controllers are simple async functions. They receive AppState
// via State extractor, which is available through `lumina::prelude::*`.

pub struct WelcomeController;

impl WelcomeController {
    // Renders the main welcoming landing page
    pub async fn index(State(state): State<AppState>) -> Html<String> {
        let mut ctx = Context::new();
        ctx.insert("title", "Welcome to Lumina");
        ctx.insert("app_name", "Lumina Framework");
        ctx.insert("version", "0.1.0");
        Html(state.view.render("welcome.blade.rs", &ctx))
    }

    // Renders the Login page
    pub async fn login(State(state): State<AppState>) -> Html<String> {
        let mut ctx = Context::new();
        ctx.insert("title", "Login - Lumina");
        Html(state.view.render("auth/login.blade.rs", &ctx))
    }

    // Renders the Register page
    pub async fn register(State(state): State<AppState>) -> Html<String> {
        let mut ctx = Context::new();
        ctx.insert("title", "Register - Lumina");
        Html(state.view.render("auth/register.blade.rs", &ctx))
    }
}
"#;
    fs::write(base.join("src/app/controllers/welcome_controller.rs"), welcome_ctrl).ok();


    // 8. routes/mod.rs & routes/web.rs
    fs::create_dir_all(base.join("src/routes")).ok();
    fs::write(base.join("src/routes/mod.rs"), "pub mod web;\n").ok();
    
    let web_routes = r#"use lumina::prelude::*;
use crate::app::controllers::welcome_controller::WelcomeController;

// =========================================================================
//                            WEB ROUTING
// =========================================================================
// This is where you register all web routes for your application.
// Lumina uses an intuitive routing syntax similar to modern MVC frameworks.

pub fn router() -> Router<AppState> {
    Router::new()
        // Landing index page
        .get("/", WelcomeController::index)
        
        // Simple authentication views
        .get("/login", WelcomeController::login)
        .get("/register", WelcomeController::register)
}
"#;
    fs::write(base.join("src/routes/web.rs"), web_routes).ok();

    // 9. resources/views/layouts/app.blade.rs
    let layout = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{{ title | default("Lumina Framework") }}</title>
    <script src="https://cdn.tailwindcss.com"></script>
    <link rel="preconnect" href="https://fonts.googleapis.com">
    <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
    <link href="https://fonts.googleapis.com/css2?family=Plus+Jakarta+Sans:wght@300;400;500;600;700;800&family=Space+Grotesk:wght@400;500;600;700&display=swap" rel="stylesheet">
    <style>
        body {
            font-family: 'Plus Jakarta Sans', sans-serif;
            background-color: #030712;
        }
        .heading-font {
            font-family: 'Space Grotesk', sans-serif;
        }
        .glass-nav {
            background: rgba(17, 24, 39, 0.7);
            backdrop-filter: blur(12px);
            border-bottom: 1px solid rgba(255, 255, 255, 0.08);
        }
    </style>
</head>
<body class="text-slate-100 min-h-screen flex flex-col justify-between selection:bg-indigo-500 selection:text-white">

    <!-- Top Navigation Header -->
    <nav class="glass-nav sticky top-0 z-50">
        <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
            <div class="flex justify-between h-16 items-center">
                <div class="flex items-center space-x-3">
                    <div class="w-9 h-9 bg-gradient-to-tr from-indigo-500 to-pink-500 rounded-xl flex items-center justify-center shadow-lg shadow-indigo-500/25">
                        <span class="text-white font-bold heading-font text-lg">L</span>
                    </div>
                    <span class="text-xl font-bold tracking-tight text-white heading-font">Lumina</span>
                </div>
                <div class="flex items-center space-x-6">
                    <a href="/" class="text-slate-300 hover:text-indigo-400 font-medium transition">Home</a>
                    <a href="/login" class="text-slate-300 hover:text-indigo-400 font-medium transition">Login</a>
                    <a href="/register" class="bg-indigo-600 text-white px-4 py-2 rounded-xl font-semibold hover:bg-indigo-700 hover:shadow-lg hover:shadow-indigo-500/30 transition-all">Get Started</a>
                </div>
            </div>
        </div>
    </nav>

    <!-- Main Content Slot -->
    <main class="flex-grow">
        {% block content %}{% endblock %}
    </main>

    <!-- Footer Segment -->
    <footer class="border-t border-slate-900 bg-black/40 py-8">
        <div class="max-w-7xl mx-auto px-4 text-center text-slate-500 text-sm flex flex-col sm:flex-row justify-between items-center gap-4">
            <div>&copy; 2026 Lumina Framework. All rights reserved.</div>
            <div class="flex items-center gap-1 text-slate-400">
                Engineered with <span class="text-red-500">❤️</span> in Rust for elegant speed.
            </div>
        </div>
    </footer>

</body>
</html>
"#;
    fs::write(base.join("resources/views/layouts/app.blade.rs"), layout).ok();

    // 10. resources/views/welcome.blade.rs
    let welcome = r#"{% extends "layouts/app.blade.rs" %}

{% block content %}
<div class="relative overflow-hidden pt-20 pb-32">
    <!-- Ambient Glow Effects -->
    <div class="absolute top-[-10%] left-[-10%] w-[50vw] h-[50vw] bg-indigo-500/10 rounded-full blur-[120px] pointer-events-none"></div>
    <div class="absolute bottom-[-10%] right-[-10%] w-[50vw] h-[50vw] bg-pink-500/10 rounded-full blur-[120px] pointer-events-none"></div>

    <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 relative z-10">
        <div class="text-center max-w-4xl mx-auto">
            <span class="inline-flex items-center gap-1.5 py-1 px-3 rounded-full text-xs font-semibold bg-indigo-500/10 text-indigo-300 border border-indigo-500/20 mb-8 uppercase tracking-wider">
                ⚡ Premium Developer Experience
            </span>
            <h1 class="text-5xl md:text-7xl font-extrabold tracking-tight text-white mb-8 heading-font leading-tight">
                The <span class="bg-clip-text text-transparent bg-gradient-to-r from-indigo-400 via-purple-400 to-pink-400">Elegance</span> of Rust<br> meets Laravel ease.
            </h1>
            <p class="text-lg md:text-xl text-slate-400 max-w-3xl mx-auto mb-12 leading-relaxed">
                Lumina is a highly ergonomic web framework designed for developers who love lightning-fast performance, strict type safety, and beautiful expressive code. Focus on your business logic, let us handle the rest.
            </p>
            <div class="flex flex-wrap justify-center gap-4">
                <a href="/register" class="bg-indigo-600 text-white px-8 py-4 rounded-2xl font-semibold hover:bg-indigo-500 hover:shadow-lg hover:shadow-indigo-500/30 transition-all transform hover:-translate-y-0.5 active:scale-98">
                    Launch New Account
                </a>
                <a href="https://github.com/lumina-java/framework" target="_blank" class="bg-slate-900 text-slate-300 border border-slate-800 px-8 py-4 rounded-2xl font-semibold hover:bg-slate-800 hover:text-white transition-all transform hover:-translate-y-0.5">
                    Explore Documentation
                </a>
            </div>
        </div>

        <!-- Features Matrix -->
        <div class="mt-28 grid grid-cols-1 md:grid-cols-3 gap-8">
            <div class="bg-slate-900/50 border border-slate-800/80 p-8 rounded-3xl backdrop-filter backdrop-blur-sm hover:border-indigo-500/30 transition-all duration-300 hover:-translate-y-1">
                <div class="w-12 h-12 bg-indigo-500/10 text-indigo-400 rounded-2xl flex items-center justify-center mb-6">
                    <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z"></path>
                    </svg>
                </div>
                <h3 class="text-xl font-bold mb-3 heading-font text-white">Blazing Fast Speed</h3>
                <p class="text-slate-400 leading-relaxed text-sm">Powered by Tokio and Axum under the hood to achieve concurrent connections with zero garbage collection overhead.</p>
            </div>
            
            <div class="bg-slate-900/50 border border-slate-800/80 p-8 rounded-3xl backdrop-filter backdrop-blur-sm hover:border-green-500/30 transition-all duration-300 hover:-translate-y-1">
                <div class="w-12 h-12 bg-green-500/10 text-green-400 rounded-2xl flex items-center justify-center mb-6">
                    <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m5.618-4.016A11.955 11.955 0 0112 2.944a11.955 11.955 0 01-8.618 3.04A12.02 12.02 0 003 9c0 5.591 3.824 10.29 9 11.622 5.176-1.332 9-6.03 9-11.622 0-1.042-.133-2.052-.382-3.016z"></path>
                    </svg>
                </div>
                <h3 class="text-xl font-bold mb-3 heading-font text-white">Total Type Safety</h3>
                <p class="text-slate-400 leading-relaxed text-sm">Rust's elite compilation system stops runtime database crashes, memory leaks, and null pointers before deployment.</p>
            </div>

            <div class="bg-slate-900/50 border border-slate-800/80 p-8 rounded-3xl backdrop-filter backdrop-blur-sm hover:border-pink-500/30 transition-all duration-300 hover:-translate-y-1">
                <div class="w-12 h-12 bg-pink-500/10 text-pink-400 rounded-2xl flex items-center justify-center mb-6">
                    <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6.253v13m0-13C10.832 5.477 9.246 5 7.5 5S4.168 5.477 3 6.253v13C4.168 18.477 5.754 18 7.5 18s3.332.477 4.5 1.253m0-13C13.168 5.477 14.754 5 16.5 5c1.747 0 3.332.477 4.5 1.253v13C19.832 18.477 18.247 18 16.5 18c-1.746 0-3.332.477-4.5 1.253"></path>
                    </svg>
                </div>
                <h3 class="text-xl font-bold mb-3 heading-font text-white">Intuitive MVC Architecture</h3>
                <p class="text-slate-400 leading-relaxed text-sm">Designed with clean, simple controllers, easy routes, and elegant view renders that PHP/JS developers will master immediately.</p>
            </div>
        </div>
    </div>
</div>
{% endblock %}
"#;
    fs::write(base.join("resources/views/welcome.blade.rs"), welcome).ok();

    // 11. resources/views/auth/login.blade.rs
    let login = r#"{% extends "layouts/app.blade.rs" %}

{% block content %}
<div class="min-h-[75vh] flex items-center justify-center px-4 relative">
    <div class="absolute w-[400px] h-[400px] bg-indigo-500/5 rounded-full blur-[100px] pointer-events-none"></div>

    <div class="max-w-md w-full bg-slate-900/60 border border-slate-800/80 rounded-3xl p-10 backdrop-filter backdrop-blur-md shadow-2xl relative z-10">
        <div class="text-center mb-8">
            <h2 class="text-3xl font-extrabold heading-font text-white">Welcome Back</h2>
            <p class="text-slate-400 mt-2">Enter your credentials to manage your account</p>
        </div>
        
        <form action="/login" method="POST" class="space-y-6">
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
{% endblock %}
"#;
    fs::write(base.join("resources/views/auth/login.blade.rs"), login).ok();

    // 12. resources/views/auth/register.blade.rs
    let register = r#"{% extends "layouts/app.blade.rs" %}

{% block content %}
<div class="min-h-[75vh] flex items-center justify-center px-4 relative">
    <div class="absolute w-[400px] h-[400px] bg-pink-500/5 rounded-full blur-[100px] pointer-events-none"></div>

    <div class="max-w-md w-full bg-slate-900/60 border border-slate-800/80 rounded-3xl p-10 backdrop-filter backdrop-blur-md shadow-2xl relative z-10">
        <div class="text-center mb-8">
            <h2 class="text-3xl font-extrabold heading-font text-white">Create Account</h2>
            <p class="text-slate-400 mt-2">Get started with the Lumina ecosystem</p>
        </div>
        
        <form action="/register" method="POST" class="space-y-6">
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
{% endblock %}
"#;
    fs::write(base.join("resources/views/auth/register.blade.rs"), register).ok();

    // 13. database/migrations/0001_create_users_table.sql
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
{{% extends "layouts/app.blade.rs" %}}

{{% block content %}}
<div class="max-w-4xl mx-auto px-4 py-20 text-center">
    <h1 class="text-4xl font-extrabold heading-font text-white">About Lumina</h1>
    <p class="text-slate-400 mt-4 text-lg">This is my first Lumina custom view!</p>
</div>
{{% endblock %}}
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
