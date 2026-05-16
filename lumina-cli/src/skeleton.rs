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
        "routes",
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
lumina = {{ git = "https://github.com/lumina-java/framework.git" }}
tokio = {{ version = "1", features = ["full"] }}
axum = {{ version = "0.7", features = ["macros", "multipart"] }}
serde = {{ version = "1", features = ["derive"] }}
serde_json = "1"
dotenv = "0.15"

[[bin]]
name = "{}-server"
path = "src/main.rs"
"#,
        name, name, name
    );
    fs::write(base.join("Cargo.toml"), cargo_toml).ok();

    // 3. .env.example
    let env_example = r#"APP_NAME=Lumina
APP_ENV=local
APP_KEY=
APP_DEBUG=true
APP_URL=http://localhost:8000

DB_CONNECTION=sqlite
DATABASE_URL=sqlite:./database.sqlite

# Auth configuration
JWT_SECRET=your-secret-key-here
"#;
    fs::write(base.join(".env.example"), env_example).ok();
    fs::write(base.join(".env"), env_example).ok();

    // 4. src/main.rs
    let main_rs = r#"use lumina::prelude::*;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    // Load environment variables
    dotenv::dotenv().ok();

    // Initialize application
    let app = Application::new()
        .with_web(crate::routes::web_routes());

    println!("✨ Lumina Framework");
    println!("🚀 Server starting at http://127.0.0.1:8000");

    app.serve("127.0.0.1:8000").await;
}

mod routes {
    use lumina::prelude::*;
    use crate::app::controllers::welcome_controller::WelcomeController;

    pub fn web_routes() -> Router<AppState> {
        Router::new()
            .get("/", WelcomeController::index)
            .get("/login", WelcomeController::login)
            .get("/register", WelcomeController::register)
    }
}

mod app {
    pub mod controllers {
        pub mod mod_rs {
            pub mod welcome_controller;
        }
        pub use mod_rs::*;
    }
    pub mod models;
}
"#;
    // Actually we'll structure src/ better
    fs::write(base.join("src/main.rs"), r#"mod app;
mod routes;

use lumina::prelude::*;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let app = Application::new()
        .with_web(routes::web::router());

    app.serve("127.0.0.1:8000").await;
}
"#).ok();

    // 5. src/app/mod.rs
    fs::write(base.join("src/app/mod.rs"), "pub mod controllers;\npub mod models;\n").ok();

    // 6. src/app/controllers/mod.rs
    fs::write(base.join("src/app/controllers/mod.rs"), "pub mod welcome_controller;\n").ok();

    // 7. src/app/controllers/welcome_controller.rs
    let welcome_ctrl = r#"use lumina::prelude::*;
use axum::response::Html;
use tera::Context;

pub struct WelcomeController;

impl WelcomeController {
    pub async fn index(State(state): State<AppState>) -> Html<String> {
        let mut ctx = Context::new();
        ctx.insert("title", "Welcome to Lumina");
        Html(state.view.render("welcome.blade.rs", &ctx))
    }

    pub async fn login(State(state): State<AppState>) -> Html<String> {
        let mut ctx = Context::new();
        ctx.insert("title", "Login - Lumina");
        Html(state.view.render("auth/login.blade.rs", &ctx))
    }

    pub async fn register(State(state): State<AppState>) -> Html<String> {
        let mut ctx = Context::new();
        ctx.insert("title", "Register - Lumina");
        Html(state.view.render("auth/register.blade.rs", &ctx))
    }
}
"#;
    fs::write(base.join("src/app/controllers/welcome_controller.rs"), welcome_ctrl).ok();

    // 8. routes/mod.rs & routes/web.rs
    fs::write(base.join("src/routes/mod.rs"), "pub mod web;\n").ok();
    fs::write(base.join("src/routes/web.rs"), r#"use lumina::prelude::*;
use crate::app::controllers::welcome_controller::WelcomeController;

pub fn router() -> Router<AppState> {
    Router::new()
        .get("/", WelcomeController::index)
        .get("/login", WelcomeController::login)
        .get("/register", WelcomeController::register)
}
"#).ok();

    // 9. resources/views/layouts/app.blade.rs
    let layout = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{{ title | default("Lumina Framework") }}</title>
    <script src="https://cdn.tailwindcss.com"></script>
    <link href="https://fonts.googleapis.com/css2?family=Inter:wght@300;400;600;700&display=swap" rel="stylesheet">
    <style>
        body { font-family: 'Inter', sans-serif; }
        .glass { background: rgba(255, 255, 255, 0.7); backdrop-filter: blur(10px); }
    </style>
</head>
<body class="bg-slate-50 text-slate-900">
    <nav class="glass sticky top-0 z-50 border-b border-slate-200">
        <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
            <div class="flex justify-between h-16 items-center">
                <div class="flex items-center space-x-2">
                    <div class="w-8 h-8 bg-blue-600 rounded-lg flex items-center justify-center">
                        <span class="text-white font-bold">L</span>
                    </div>
                    <span class="text-xl font-bold tracking-tight">Lumina</span>
                </div>
                <div class="hidden md:flex items-center space-x-8">
                    <a href="/" class="text-slate-600 hover:text-blue-600 transition">Home</a>
                    <a href="/login" class="text-slate-600 hover:text-blue-600 transition">Login</a>
                    <a href="/register" class="bg-blue-600 text-white px-4 py-2 rounded-lg hover:bg-blue-700 transition">Get Started</a>
                </div>
            </div>
        </div>
    </nav>

    <main>
        {% block content %}{% endblock %}
    </main>

    <footer class="bg-white border-t border-slate-200 py-12 mt-20">
        <div class="max-w-7xl mx-auto px-4 text-center text-slate-500 text-sm">
            &copy; 2026 Lumina Framework. Built with Rust for elegance and speed.
        </div>
    </footer>
</body>
</html>
"#;
    fs::write(base.join("resources/views/layouts/app.blade.rs"), layout).ok();

    // 10. resources/views/welcome.blade.rs
    let welcome = r#"{% extends "layouts/app.blade.rs" %}

{% block content %}
<div class="relative overflow-hidden pt-16 pb-32">
    <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 relative z-10">
        <div class="text-center">
            <h1 class="text-5xl md:text-7xl font-extrabold tracking-tight text-slate-900 mb-6">
                The <span class="text-blue-600">Elegance</span> of Rust<br> meets Laravel syntax.
            </h1>
            <p class="text-xl text-slate-600 max-w-2xl mx-auto mb-10">
                Lumina is a web framework built for developers who love speed, safety, and beautiful code. 
                Focus on your business logic, we handle the rest.
            </p>
            <div class="flex justify-center space-x-4">
                <a href="/register" class="bg-blue-600 text-white px-8 py-4 rounded-xl font-semibold hover:bg-blue-700 shadow-lg shadow-blue-200 transition-all hover:-translate-y-1">
                    Create New Project
                </a>
                <a href="https://github.com/lumina-java/framework" class="bg-white text-slate-900 border border-slate-200 px-8 py-4 rounded-xl font-semibold hover:bg-slate-50 transition-all hover:-translate-y-1">
                    View Documentation
                </a>
            </div>
        </div>

        <div class="mt-20 grid grid-cols-1 md:grid-cols-3 gap-8">
            <div class="bg-white p-8 rounded-2xl border border-slate-100 shadow-sm">
                <div class="w-12 h-12 bg-blue-100 text-blue-600 rounded-xl flex items-center justify-center mb-6">
                    <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z"></path></svg>
                </div>
                <h3 class="text-xl font-bold mb-3">Blazing Fast</h3>
                <p class="text-slate-500">Built on top of Tokio and Axum for maximum performance and concurrency.</p>
            </div>
            <div class="bg-white p-8 rounded-2xl border border-slate-100 shadow-sm">
                <div class="w-12 h-12 bg-green-100 text-green-600 rounded-xl flex items-center justify-center mb-6">
                    <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m5.618-4.016A11.955 11.955 0 0112 2.944a11.955 11.955 0 01-8.618 3.04A12.02 12.02 0 003 9c0 5.591 3.824 10.29 9 11.622 5.176-1.332 9-6.03 9-11.622 0-1.042-.133-2.052-.382-3.016z"></path></svg>
                </div>
                <h3 class="text-xl font-bold mb-3">Type Safe</h3>
                <p class="text-slate-500">Rust's memory safety and type system prevent common web vulnerabilities by default.</p>
            </div>
            <div class="bg-white p-8 rounded-2xl border border-slate-100 shadow-sm">
                <div class="w-12 h-12 bg-purple-100 text-purple-600 rounded-xl flex items-center justify-center mb-6">
                    <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6.253v13m0-13C10.832 5.477 9.246 5 7.5 5S4.168 5.477 3 6.253v13C4.168 18.477 5.754 18 7.5 18s3.332.477 4.5 1.253m0-13C13.168 5.477 14.754 5 16.5 5c1.747 0 3.332.477 4.5 1.253v13C19.832 18.477 18.247 18 16.5 18c-1.746 0-3.332.477-4.5 1.253"></path></svg>
                </div>
                <h3 class="text-xl font-bold mb-3">Laravel Style</h3>
                <p class="text-slate-500">Familiar routing, controllers, and template syntax for a smooth developer experience.</p>
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
<div class="min-h-[70vh] flex items-center justify-center px-4">
    <div class="max-w-md w-full bg-white rounded-2xl shadow-xl shadow-slate-200/50 p-10 border border-slate-100">
        <div class="text-center mb-10">
            <h2 class="text-3xl font-bold">Welcome Back</h2>
            <p class="text-slate-500 mt-2">Please enter your details to sign in</p>
        </div>
        <form action="/login" method="POST" class="space-y-6">
            <div>
                <label class="block text-sm font-semibold mb-2">Email Address</label>
                <input type="email" name="email" class="w-full px-4 py-3 rounded-xl border border-slate-200 focus:ring-2 focus:ring-blue-600 focus:border-transparent outline-none transition" placeholder="you@example.com">
            </div>
            <div>
                <label class="block text-sm font-semibold mb-2">Password</label>
                <input type="password" name="password" class="w-full px-4 py-3 rounded-xl border border-slate-200 focus:ring-2 focus:ring-blue-600 focus:border-transparent outline-none transition" placeholder="••••••••">
            </div>
            <button type="submit" class="w-full bg-blue-600 text-white py-4 rounded-xl font-bold hover:bg-blue-700 transition transform active:scale-[0.98]">
                Sign In
            </button>
        </form>
        <p class="text-center text-sm text-slate-500 mt-8">
            Don't have an account? <a href="/register" class="text-blue-600 font-bold hover:underline">Sign up</a>
        </p>
    </div>
</div>
{% endblock %}
"#;
    fs::write(base.join("resources/views/auth/login.blade.rs"), login).ok();

    // 12. resources/views/auth/register.blade.rs
    let register = r#"{% extends "layouts/app.blade.rs" %}

{% block content %}
<div class="min-h-[70vh] flex items-center justify-center px-4">
    <div class="max-w-md w-full bg-white rounded-2xl shadow-xl shadow-slate-200/50 p-10 border border-slate-100">
        <div class="text-center mb-10">
            <h2 class="text-3xl font-bold">Create Account</h2>
            <p class="text-slate-500 mt-2">Join the Lumina community today</p>
        </div>
        <form action="/register" method="POST" class="space-y-6">
            <div>
                <label class="block text-sm font-semibold mb-2">Full Name</label>
                <input type="text" name="name" class="w-full px-4 py-3 rounded-xl border border-slate-200 focus:ring-2 focus:ring-blue-600 focus:border-transparent outline-none transition" placeholder="John Doe">
            </div>
            <div>
                <label class="block text-sm font-semibold mb-2">Email Address</label>
                <input type="email" name="email" class="w-full px-4 py-3 rounded-xl border border-slate-200 focus:ring-2 focus:ring-blue-600 focus:border-transparent outline-none transition" placeholder="you@example.com">
            </div>
            <div>
                <label class="block text-sm font-semibold mb-2">Password</label>
                <input type="password" name="password" class="w-full px-4 py-3 rounded-xl border border-slate-200 focus:ring-2 focus:ring-blue-600 focus:border-transparent outline-none transition" placeholder="••••••••">
            </div>
            <button type="submit" class="w-full bg-blue-600 text-white py-4 rounded-xl font-bold hover:bg-blue-700 transition transform active:scale-[0.98]">
                Create Account
            </button>
        </form>
        <p class="text-center text-sm text-slate-500 mt-8">
            Already have an account? <a href="/login" class="text-blue-600 font-bold hover:underline">Sign in</a>
        </p>
    </div>
</div>
{% endblock %}
"#;
    fs::write(base.join("resources/views/auth/register.blade.rs"), register).ok();

    // 13. database/migrations/0001_create_users_table.sql
    let migration = r#"-- Users Table
CREATE TABLE IF NOT EXISTS users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    email TEXT NOT NULL UNIQUE,
    password TEXT NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);
"#;
    fs::write(base.join("database/migrations/0001_create_users_table.sql"), migration).ok();

    // 14. src/app/models/mod.rs & user.rs
    fs::write(base.join("src/app/models/mod.rs"), "pub mod user;\n").ok();
    let user_model = r#"use lumina::database::model::Model;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub email: String,
    pub password: String,
}

impl Model for User {
    fn table_name() -> &'static str {
        "users"
    }
}
"#;
    fs::write(base.join("src/app/models/user.rs"), user_model).ok();
}
