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
    fs::write(
        base.join("src/app/mod.rs"),
        "pub mod controllers;\npub mod models;\n",
    )
    .ok();

    // 6. src/app/controllers/mod.rs
    fs::write(
        base.join("src/app/controllers/mod.rs"),
        "pub mod welcome_controller;\n",
    )
    .ok();

    // 7. src/app/controllers/welcome_controller.rs
    let welcome_ctrl = r#"use lumina::prelude::*;

// =========================================================================
//                        WELCOME CONTROLLER
// =========================================================================
// Selamat datang di Controller! Controller berfungsi untuk menerima request
// dari user dan mengembalikan response (biasanya berupa halaman web / HTML).
//
// Di Lumina, controller dibuat semudah di Laravel. Setiap fungsi controller
// menerima `State<AppState>` yang berisi semua hal yang kamu butuhkan
// (seperti view engine `state.view` atau koneksi database `state.db`).

pub struct WelcomeController;

impl WelcomeController {
    /// Fungsi ini menangani rute halaman utama ("/")
    pub async fn index(State(state): State<AppState>) -> Html<String> {
        // `Context` digunakan untuk mengirim data (variabel) dari Controller ke View (HTML).
        // Mirip seperti `return view('welcome', ['title' => 'Lumina'])` di Laravel.
        let mut ctx = Context::new();
        ctx.insert("title", "Welcome to Lumina");
        ctx.insert("app_name", "Lumina Framework");
        ctx.insert("version", "0.1.0");

        // `state.view.render` akan mencari file di folder `resources/views/welcome.blade.rs`
        Html(state.view.render("welcome.blade.rs", &ctx))
    }

    /// Menampilkan Halaman Login ("/login")
    pub async fn login(State(state): State<AppState>) -> Html<String> {
        let mut ctx = Context::new();
        ctx.insert("title", "Login - Lumina");
        Html(state.view.render("auth/login.blade.rs", &ctx))
    }

    /// Menampilkan Halaman Register ("/register")
    pub async fn register(State(state): State<AppState>) -> Html<String> {
        let mut ctx = Context::new();
        ctx.insert("title", "Register - Lumina");
        Html(state.view.render("auth/register.blade.rs", &ctx))
    }
}
"#;
    fs::write(
        base.join("src/app/controllers/welcome_controller.rs"),
        welcome_ctrl,
    )
    .ok();

    // 8. routes/mod.rs & routes/web.rs
    fs::create_dir_all(base.join("src/routes")).ok();
    fs::write(base.join("src/routes/mod.rs"), "pub mod web;\n").ok();

    let web_routes = r#"use lumina::prelude::*;
use crate::app::controllers::welcome_controller::WelcomeController;

// =========================================================================
//                            WEB ROUTING
// =========================================================================
// Di sinilah kamu mendaftarkan semua URL (route) web untuk aplikasimu.
// Konsepnya sama persis seperti `Route::get()` di Laravel!
//
// Cukup panggil method `.get()`, `.post()`, `.put()`, atau `.delete()`
// dan hubungkan dengan fungsi yang ada di dalam Controller kamu.

pub fn router() -> Router<AppState> {
    Router::new()
        // Saat user mengakses URL "/" (halaman depan), panggil WelcomeController::index
        .get("/", WelcomeController::index)
        
        // Rute untuk halaman autentikasi (Login & Register)
        .get("/login", WelcomeController::login)
        .get("/register", WelcomeController::register)

        // 👇 CONTOH MENAMBAH RUTE BARU:
        // .get("/halo", WelcomeController::halo)
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

        <!-- Step-by-Step Beginner Guide -->
        <div class="mt-28 bg-slate-900/50 border border-slate-800/80 rounded-3xl backdrop-filter backdrop-blur-sm p-8 md:p-12 text-left shadow-2xl">
            <h2 class="text-3xl font-bold text-white mb-8 heading-font border-b border-slate-800 pb-4">🚀 Mari Mulai: Buat Halaman Pertamamu!</h2>
            
            <div class="space-y-8">
                <!-- Step 1 -->
                <div class="flex gap-4">
                    <div class="w-10 h-10 shrink-0 bg-indigo-600 text-white rounded-full flex items-center justify-center font-bold text-lg">1</div>
                    <div>
                        <h3 class="text-xl font-bold text-indigo-300 mb-2">Buat View (HTML)</h3>
                        <p class="text-slate-400 mb-3 text-sm">Buka folder <code class="bg-slate-800 text-indigo-300 px-2 py-1 rounded">resources/views/</code> dan buat file baru bernama <code class="bg-slate-800 text-indigo-300 px-2 py-1 rounded">halo.blade.rs</code>. Isi dengan kode HTML biasa:</p>
                        <pre class="bg-black/60 p-4 rounded-xl text-sm text-emerald-300 overflow-x-auto border border-slate-800">
&lt;h1&gt;Halo, Selamat datang di Lumina!&lt;/h1&gt;</pre>
                    </div>
                </div>

                <!-- Step 2 -->
                <div class="flex gap-4">
                    <div class="w-10 h-10 shrink-0 bg-indigo-600 text-white rounded-full flex items-center justify-center font-bold text-lg">2</div>
                    <div>
                        <h3 class="text-xl font-bold text-indigo-300 mb-2">Buat Controller</h3>
                        <p class="text-slate-400 mb-3 text-sm">Buka file <code class="bg-slate-800 text-indigo-300 px-2 py-1 rounded">src/app/controllers/welcome_controller.rs</code> dan tambahkan fungsi baru ini di dalam <code class="bg-slate-800 text-indigo-300 px-2 py-1 rounded">impl WelcomeController</code>:</p>
                        <pre class="bg-black/60 p-4 rounded-xl text-sm text-emerald-300 overflow-x-auto border border-slate-800">
// Fungsi ini akan merender file halo.blade.rs yang baru kita buat
pub async fn halo(State(state): State&lt;AppState&gt;) -&gt; Html&lt;String&gt; {
    let ctx = Context::new();
    Html(state.view.render("halo.blade.rs", &ctx))
}</pre>
                    </div>
                </div>

                <!-- Step 3 -->
                <div class="flex gap-4">
                    <div class="w-10 h-10 shrink-0 bg-indigo-600 text-white rounded-full flex items-center justify-center font-bold text-lg">3</div>
                    <div>
                        <h3 class="text-xl font-bold text-indigo-300 mb-2">Daftarkan Route</h3>
                        <p class="text-slate-400 mb-3 text-sm">Buka file <code class="bg-slate-800 text-indigo-300 px-2 py-1 rounded">src/routes/web.rs</code> dan tambahkan URL baru ke router kamu:</p>
                        <pre class="bg-black/60 p-4 rounded-xl text-sm text-emerald-300 overflow-x-auto border border-slate-800">
pub fn router() -&gt; Router&lt;AppState&gt; {
    Router::new()
        .get("/", WelcomeController::index)
        // Tambahkan baris ini 👇
        .get("/halo", WelcomeController::halo)
}</pre>
                        <div class="mt-4 p-4 bg-indigo-500/10 border border-indigo-500/20 rounded-xl">
                            <p class="text-slate-300 text-sm">🎉 <strong>Selesai!</strong> Sekarang buka <a href="/halo" target="_blank" class="text-indigo-400 font-bold hover:underline">http://localhost:8000/halo</a> di browser kamu!</p>
                        </div>
                    </div>
                </div>
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
    fs::write(
        base.join("resources/views/auth/register.blade.rs"),
        register,
    )
    .ok();

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
    fs::write(
        base.join("database/migrations/0001_create_users_table.sql"),
        migration,
    )
    .ok();

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
// Selamat datang di Model! Di Laravel, ini adalah class Eloquent (seperti `User extends Model`).
// Di Rust/Lumina, Model adalah `struct` yang mendefinisikan kolom apa saja yang
// ada di tabel database, lalu kita beritahu Lumina nama tabelnya (di `const TABLE`).

/// Struct `User` ini mencerminkan struktur kolom di tabel `users`.
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
    /// Membuat object User baru yang kosong (seperti `$user = new User()`)
    pub fn new() -> Self {
        Self::default()
    }

    /// Contoh fungsi kustom: Mencari user berdasarkan Email (Mirip `User::where('email', $email)->first()`)
    pub async fn find_by_email(pool: &DatabasePool, email: &str) -> Result<Self, sqlx::Error> {
        // Query builder Lumina menyembunyikan SQL mentah dari kamu!
        Self::query(pool).where_eq("email", email).first().await
    }
}

/// Di sinilah keajaiban "Eloquent" Lumina terjadi. Kita memberitahu Lumina
/// cara melakukan aksi CRUD standard (Create, Read, Update, Delete) ke tabel ini.
#[async_trait]
impl Model for User {
    // Nama tabel di database (seperti `$table = 'users'` di Laravel)
    const TABLE: &'static str = "users";

    /// Mencari data berdasarkan ID (Mirip `User::find($id)`)
    async fn find(pool: &DatabasePool, id: i64) -> Result<Self, sqlx::Error> {
        Self::query(pool).where_eq("id", id).first().await
    }

    /// Mengambil semua data (Mirip `User::all()`)
    async fn all(pool: &DatabasePool) -> Result<Vec<Self>, sqlx::Error> {
        Self::query(pool).get().await
    }

    /// Menyimpan data ke database (Mirip `$user->save()`).
    /// Lumina akan otomatis mengecek: jika ID > 0 berarti UPDATE, jika 0 berarti INSERT baru.
    async fn save(&self, pool: &DatabasePool) -> Result<i64, sqlx::Error> {
        if self.id > 0 {
            // Mode UPDATE (Edit data lama)
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
            // Mode INSERT (Buat data baru)
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

    /// Menghapus data secara "Soft Delete" (Mirip `$user->delete()`)
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
        r#"# ⚡ Selamat Datang di Project Lumina Baru Kamu: {}

Selamat! Kamu telah berhasil membuat project baru dengan **Lumina Framework** — framework web berbasis Rust yang super cepat, aman, namun memiliki sintaks elegan dan semudah **Laravel**.

---

## 🚀 Panduan Menjalankan Project (Sangat Mudah!)

Secara default, project ini menggunakan database **SQLite** (file lokal). Artinya, **kamu tidak perlu repot menginstall database apapun** di komputer kamu untuk langsung mencoba framework ini!

Cukup ketik perintah ini di terminal kamu:
```bash
cargo run
```

Tunggu proses kompilasi Rust (hanya lama di awal), lalu buka browser dan kunjungi:
👉 **[http://localhost:8000](http://localhost:8000)**

Boom! Website kamu sudah berjalan! 🎉

---

## 🗄️ Mengubah Database ke MySQL

Kalau kamu sudah siap mendeploy aplikasi atau ingin beralih ke MySQL, mengubahnya sangatlah gampang, persis seperti di Laravel.

**Langkah-langkah ganti ke MySQL:**
1. Buka file `.env` di folder project ini.
2. Cari bagian `# Database Configuration`.
3. Matikan (hapus atau beri komentar `#`) konfigurasi SQLite.
4. Ganti baris `DATABASE_URL` agar mengarah ke MySQL kamu.

**Contoh isi `.env` untuk MySQL:**
```env
DB_CONNECTION=mysql
DB_HOST=127.0.0.1
DB_PORT=3306
DB_DATABASE=nama_db_kamu
DB_USERNAME=root
DB_PASSWORD=password_rahasia

# Variabel ini yang paling penting dan dibaca langsung oleh sistem:
DATABASE_URL=mysql://root:password_rahasia@127.0.0.1:3306/nama_db_kamu
```

*Jangan lupa: Buat dulu database kosong bernama `nama_db_kamu` di phpMyAdmin / MySQL kamu!*

Setelah itu, jalankan migrasi agar Lumina membuat tabel di MySQL:
```bash
cargo lumina migrate
```

---

## 📁 Penjelasan Struktur Folder (MVC)

Struktur folder Lumina dibuat agar sangat familiar bagi programmer PHP/JS:

*   **`src/`** — Di sinilah semua kode bahasa Rust kamu berada.
    *   **`src/routes/web.rs`** — Tempat mendaftarkan URL (seperti `Route::get()`).
    *   **`src/app/controllers/`** — Tempat menaruh logika aplikasi.
    *   **`src/app/models/`** — File untuk mengakses database (CRUD).
*   **`resources/views/`** — Tempat file HTML (dengan sintaks mirip Blade).
*   **`database/migrations/`** — File SQL untuk membuat tabel database.

---

## 💡 Cara Menambah Halaman Baru (Step-by-Step)

Ingin membuat halaman `/tentang`? Cuma butuh 3 langkah!

### Langkah 1: Buat Tampilan HTML (View)
Buat file `resources/views/tentang.blade.rs`:
```html
{{% extends "layouts/app.blade.rs" %}}

{{% block content %}}
<div class="max-w-4xl mx-auto px-4 py-20 text-center">
    <h1 class="text-4xl font-bold text-white">Tentang Kami</h1>
    <p class="text-slate-400 mt-4">Ini adalah halaman baruku!</p>
</div>
{{% endblock %}}
```

### Langkah 2: Buat Logika (Controller)
Buka `src/app/controllers/welcome_controller.rs` dan tambahkan fungsi ini ke dalam `impl WelcomeController`:
```rust
pub async fn tentang(State(state): State<AppState>) -> Html<String> {{
    let ctx = Context::new();
    Html(state.view.render("tentang.blade.rs", &ctx))
}}
```

### Langkah 3: Daftarkan URL (Route)
Buka `src/routes/web.rs` dan tambahkan rutenya:
```rust
pub fn router() -> Router<AppState> {{
    Router::new()
        .get("/", WelcomeController::index)
        .get("/tentang", WelcomeController::tentang) // <-- Tambahkan baris ini!
}}
```
Jalankan ulang `cargo run` dan buka `http://localhost:8000/tentang`.

---

## 🦀 Selamat Belajar Rust dengan Menyenangkan!
Dengan Lumina, belajar bahasa sistem yang kompleks menjadi ramah dan asyik.
"#,
        name
    );
    fs::write(base.join("README.md"), readme).ok();
}
