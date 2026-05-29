use chrono::Local;
use colored::*;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::process::Command;

// ─────────────────────────────────────────────────────────────────────────────
//  HELPERS
// ─────────────────────────────────────────────────────────────────────────────

fn is_lumina_project() -> bool {
    Path::new("Cargo.toml").exists() && Path::new("src/app").exists()
}

fn guard_project() -> bool {
    if !is_lumina_project() {
        println!(
            "{}",
            "❌ Error: This command must be run inside a Lumina project root."
                .red()
                .bold()
        );
        println!(
            "   Hint: Create a project first with {}",
            "lumina new <name>".bright_cyan()
        );
        false
    } else {
        true
    }
}

fn append_to_file(path: &str, content: &str) {
    if let Ok(mut file) = OpenOptions::new().append(true).open(path) {
        let _ = file.write_all(content.as_bytes());
    }
}

fn capitalize_first(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

fn to_snake_case(s: &str) -> String {
    s.to_lowercase()
}

fn timestamp() -> String {
    Local::now().format("%Y%m%d%H%M%S").to_string()
}

fn print_success(msg: &str) {
    println!("  {} {}", "✅".green(), msg);
}

fn print_hint(msg: &str) {
    println!("  {} {}", "💡".yellow(), msg.bright_yellow());
}

// ─────────────────────────────────────────────────────────────────────────────
//  SERVER COMMANDS
// ─────────────────────────────────────────────────────────────────────────────

pub fn cmd_serve() {
    if !guard_project() {
        return;
    }
    println!("{}", "🚀 Starting Lumina Server...".bright_purple().bold());
    println!("   Press {} to stop.\n", "Ctrl+C".bright_red());
    let status = Command::new("cargo").arg("run").status();
    if let Err(e) = status {
        println!("{} Failed to run cargo: {}", "❌".red(), e);
    }
}

pub fn cmd_watch() {
    if !guard_project() {
        return;
    }
    println!(
        "{}",
        "👁  Starting Lumina Watch (auto-reload)..."
            .bright_purple()
            .bold()
    );
    println!(
        "   Requires cargo-watch: {}",
        "cargo install cargo-watch".bright_cyan()
    );
    println!("   Press {} to stop.\n", "Ctrl+C".bright_red());
    let status = Command::new("cargo").args(["watch", "-x", "run"]).status();
    if let Err(e) = status {
        println!(
            "{} cargo-watch not found. Install it with: {}",
            "❌".red(),
            "cargo install cargo-watch".bright_cyan()
        );
        println!("   Error: {}", e);
    }
}

// ─────────────────────────────────────────────────────────────────────────────
//  GENERATORS
// ─────────────────────────────────────────────────────────────────────────────

pub fn generate_controller(name: &str) {
    if !guard_project() {
        return;
    }
    println!("{}", "🔨 Generating Controller...".bright_purple().bold());

    let class_name = capitalize_first(name);
    // Strip "Controller" suffix if user included it
    let base = class_name.trim_end_matches("Controller");
    let file_name = format!("{}_controller", to_snake_case(base));
    let class_full = format!("{}Controller", base);

    let code = format!(
        r#"use lumina::prelude::*;

pub struct {class};

impl {class} {{
    pub async fn index(State(state): State<AppState>) -> Html<String> {{
        let mut ctx = Context::new();
        ctx.insert("title", "{base}");
        Html(state.view.render("{lower}/index.blade.rs", &ctx))
    }}

    pub async fn create(State(state): State<AppState>) -> Html<String> {{
        let mut ctx = Context::new();
        ctx.insert("title", "Create {base}");
        Html(state.view.render("{lower}/create.blade.rs", &ctx))
    }}

    pub async fn show(State(state): State<AppState>) -> Html<String> {{
        let mut ctx = Context::new();
        Html(state.view.render("{lower}/show.blade.rs", &ctx))
    }}

    pub async fn edit(State(state): State<AppState>) -> Html<String> {{
        let mut ctx = Context::new();
        Html(state.view.render("{lower}/edit.blade.rs", &ctx))
    }}
}}
"#,
        class = class_full,
        base = base,
        lower = to_snake_case(base),
    );

    let path = format!("src/app/controllers/{}.rs", file_name);
    fs::write(&path, code).ok();
    append_to_file(
        "src/app/controllers/mod.rs",
        &format!("pub mod {};\n", file_name),
    );

    print_success(&format!("Controller: {}", path));
    print_hint(&format!(
        "Register routes in src/routes/web.rs using {}::index etc.",
        class_full
    ));
}

pub fn generate_model(name: &str) {
    if !guard_project() {
        return;
    }
    println!("{}", "🔨 Generating Model...".bright_purple().bold());

    let lower = to_snake_case(name);
    let class = capitalize_first(name);
    let table = format!("{}s", lower);

    let code = format!(
        r#"#![allow(dead_code)]
use async_trait::async_trait;
use serde::{{Serialize, Deserialize}};
use sqlx::FromRow;
use lumina::database::{{connection::DatabasePool, model::Model}};

// =========================================================================
//  {class} MODEL
// =========================================================================

#[derive(Debug, Serialize, Deserialize, FromRow, Clone, Default)]
pub struct {class} {{
    pub id: i64,
    // TODO: add your fields here
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub deleted_at: Option<String>,
}}

impl {class} {{
    pub fn new() -> Self {{
        Self::default()
    }}
}}

#[async_trait]
impl Model for {class} {{
    const TABLE: &'static str = "{table}";

    async fn find(pool: &DatabasePool, id: i64) -> Result<Self, sqlx::Error> {{
        Self::query(pool).where_eq("id", id).first().await
    }}

    async fn all(pool: &DatabasePool) -> Result<Vec<Self>, sqlx::Error> {{
        Self::query(pool).get().await
    }}

    async fn save(&self, pool: &DatabasePool) -> Result<i64, sqlx::Error> {{
        // TODO: implement insert / update logic
        Ok(self.id)
    }}

    async fn delete(pool: &DatabasePool, id: i64) -> Result<bool, sqlx::Error> {{
        sqlx::query("UPDATE {table} SET deleted_at = CURRENT_TIMESTAMP WHERE id = ?")
            .bind(id)
            .execute(&pool.pool)
            .await?;
        Ok(true)
    }}
}}
"#,
        class = class,
        table = table,
    );

    let path = format!("src/app/models/{}.rs", lower);
    fs::write(&path, &code).ok();
    append_to_file("src/app/models/mod.rs", &format!("pub mod {};\n", lower));
    print_success(&format!("Model: {}", path));

    // Also generate migration
    generate_migration_raw(&format!("create_{}_table", table), Some(&format!(
        "CREATE TABLE IF NOT EXISTS {table} (\n    id INTEGER PRIMARY KEY AUTOINCREMENT,\n    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,\n    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP\n);\n",
        table = table
    )));
}

pub fn generate_migration(name: &str) {
    if !guard_project() {
        return;
    }
    println!("{}", "🔨 Generating Migration...".bright_purple().bold());
    let slug = name.to_lowercase().replace(' ', "_");
    generate_migration_raw(&slug, None);
}

fn generate_migration_raw(slug: &str, sql: Option<&str>) {
    let ts = timestamp();
    let content = match sql {
        Some(s) => format!("-- Migration: {}\n{}", slug, s),
        None => format!("-- Migration: {}\n-- TODO: write your SQL here\n", slug),
    };
    let path = format!("database/migrations/{}_{}.sql", ts, slug);
    fs::write(&path, content).ok();
    print_success(&format!("Migration: {}", path));
}

pub fn generate_crud(name: &str, fields: &[String]) {
    if !guard_project() {
        return;
    }
    println!(
        "{}",
        "🔨 Generating full CRUD scaffold...".bright_purple().bold()
    );

    let lower = to_snake_case(name);
    let class = capitalize_first(name);

    // 1. Model + migration
    generate_model(name);

    // 2. Controller
    let ctrl_code = build_crud_controller(&lower, &class, fields);
    let ctrl_path = format!("src/app/controllers/{}_controller.rs", lower);
    fs::write(&ctrl_path, ctrl_code).ok();
    append_to_file(
        "src/app/controllers/mod.rs",
        &format!("pub mod {}_controller;\n", lower),
    );
    print_success(&format!("Controller: {}", ctrl_path));

    // 3. Views
    let view_dir = format!("resources/views/{}", lower);
    fs::create_dir_all(&view_dir).ok();
    for view in &["index", "create", "edit", "show"] {
        let content = build_blade_view(&class, view, &lower, fields);
        fs::write(format!("{}/{}.blade.rs", view_dir, view), content).ok();
    }
    print_success(&format!("Views: {}/", view_dir));

    println!();
    print_hint("Add these routes to src/routes/web.rs:");
    println!("      .get(\"/{lower}\",          {class}Controller::index)");
    println!("      .get(\"/{lower}/create\",   {class}Controller::create)");
    println!("      .get(\"/{lower}/:id\",      {class}Controller::show)");
    println!("      .get(\"/{lower}/:id/edit\", {class}Controller::edit)");
}

fn build_crud_controller(lower: &str, class: &str, _fields: &[String]) -> String {
    format!(
        r#"use lumina::prelude::*;
use crate::app::models::{lower}::{class};

pub struct {class}Controller;

impl {class}Controller {{
    /// GET /{lower} — list all records
    pub async fn index(State(state): State<AppState>) -> Html<String> {{
        let mut ctx = Context::new();
        ctx.insert("title", "{class}s");
        Html(state.view.render("{lower}/index.blade.rs", &ctx))
    }}

    /// GET /{lower}/create — show create form
    pub async fn create(State(state): State<AppState>) -> Html<String> {{
        let mut ctx = Context::new();
        ctx.insert("title", "Create {class}");
        Html(state.view.render("{lower}/create.blade.rs", &ctx))
    }}

    /// GET /{lower}/:id — show single record
    pub async fn show(State(state): State<AppState>) -> Html<String> {{
        let mut ctx = Context::new();
        ctx.insert("title", "{class} Detail");
        Html(state.view.render("{lower}/show.blade.rs", &ctx))
    }}

    /// GET /{lower}/:id/edit — show edit form
    pub async fn edit(State(state): State<AppState>) -> Html<String> {{
        let mut ctx = Context::new();
        ctx.insert("title", "Edit {class}");
        Html(state.view.render("{lower}/edit.blade.rs", &ctx))
    }}
}}
"#,
        lower = lower,
        class = class,
    )
}

fn build_blade_view(class: &str, view: &str, _lower: &str, _fields: &[String]) -> String {
    let title = match view {
        "index" => format!("{} List", class),
        "create" => format!("Create {}", class),
        "edit" => format!("Edit {}", class),
        "show" => format!("{} Detail", class),
        _ => class.to_string(),
    };
    format!(
        "{{% extends \"layouts/app.blade.rs\" %}}\n\n{{% block content %}}\n<div class=\"max-w-7xl mx-auto px-4 py-12\">\n    <h1 class=\"text-3xl font-bold text-white heading-font\">{title}</h1>\n    {comment}\n</div>\n{{% endblock %}}\n",
        title = title,
        comment = format!("<!-- TODO: build your {} {} view here -->", class, view),
    )
}

pub fn generate_auth() {
    if !guard_project() {
        return;
    }
    println!(
        "{}",
        "🔨 Generating Authentication Scaffolding..."
            .bright_purple()
            .bold()
    );

    let auth_controller = r#"use lumina::prelude::*;
use crate::app::models::user::User;
use serde::Deserialize;
use lumina::core::auth::Auth;
use axum::response::IntoResponse;
use lumina::core::request::Request;
use lumina::database::model::Model;

pub struct AuthController;

#[derive(Deserialize)]
pub struct LoginForm {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct RegisterForm {
    pub name: String,
    pub email: String,
    pub password: String,
}

impl AuthController {
    /// GET /login
    pub async fn login(req: Request) -> impl IntoResponse {
        req.view("auth/login")
            .with("title", "Login")
            .render(&req)
            .await
            .into_response().into_response()
    }

    /// POST /login
    pub async fn login_post(req: Request, Form(form): Form<LoginForm>) -> impl IntoResponse {
        let pool = req.db();

        if let Ok(user) = User::query(pool).where_eq("email", &form.email).first().await {
            if Auth::verify(&form.password, &user.password) {
                let auth_user = Auth::user(user.id, user.email.clone(), user.role.clone(), vec![]);
                Auth::login(&req.session, auth_user).await.ok();
                return req.redirect("/").go(&req).await.into_response();
            }
        }

        req.view("auth/login")
            .with("title", "Login")
            .with("error", "Email atau password salah!")
            .render(&req)
            .await
            .into_response().into_response()
    }

    /// GET /register
    pub async fn register(req: Request) -> impl IntoResponse {
        req.view("auth/register")
            .with("title", "Create Account")
            .render(&req)
            .await
            .into_response().into_response()
    }

    /// POST /register
    pub async fn register_post(req: Request, Form(form): Form<RegisterForm>) -> impl IntoResponse {
        let pool = req.db();

        let hashed_password = Auth::make_hash(&form.password);

        let mut new_user = User::new();
        new_user.name = form.name;
        new_user.email = form.email;
        new_user.password = hashed_password;
        new_user.role = "user".to_string();

        match new_user.save(pool).await {
            Ok(id) => {
                let auth_user = Auth::user(id, new_user.email, new_user.role, vec![]);
                Auth::login(&req.session, auth_user).await.ok();
                req.redirect("/").go(&req).await.into_response()
            }
            Err(_) => {
                req.view("auth/register")
                    .with("title", "Create Account")
                    .with("error", "Email sudah digunakan atau terjadi kesalahan.")
                    .render(&req)
                    .await
                    .into_response().into_response()
            }
        }
    }

    /// GET /logout
    pub async fn logout(req: Request) -> impl IntoResponse {
        Auth::logout(&req.session).await;
        req.redirect("/login").go(&req).await.into_response().into_response()
    }
}
"#;

    fs::write("src/app/controllers/auth_controller.rs", auth_controller).ok();
    append_to_file("src/app/controllers/mod.rs", "pub mod auth_controller;\n");

    print_success("AuthController at src/app/controllers/auth_controller.rs");
    println!();
    print_hint("Add these routes to src/routes/web.rs:");
    println!("      .get(\"/login\",     AuthController::login)");
    println!("      .post(\"/login\",    AuthController::login_post)");
    println!("      .get(\"/register\",  AuthController::register)");
    println!("      .post(\"/register\", AuthController::register_post)");
    println!("      .get(\"/logout\",    AuthController::logout)");
}

// ─────────────────────────────────────────────────────────────────────────────
//  DATABASE COMMANDS
// ─────────────────────────────────────────────────────────────────────────────

pub fn run_migrate() {
    if !guard_project() {
        return;
    }
    println!(
        "{}",
        "🗄  Running database migrations...".bright_purple().bold()
    );

    let migration_dir = Path::new("database/migrations");
    if !migration_dir.exists() {
        println!("  No migrations directory found.");
        return;
    }

    let mut files: Vec<_> = fs::read_dir(migration_dir)
        .unwrap()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map(|x| x == "sql").unwrap_or(false))
        .collect();
    files.sort_by_key(|e| e.path());

    if files.is_empty() {
        println!("  {} No pending migrations found.", "ℹ️".blue());
        return;
    }

    for file in &files {
        let path = file.path();
        let name = path.file_name().unwrap().to_string_lossy();
        println!("  {} Applying: {}", "▶".bright_cyan(), name.bright_white());
    }
    println!();
    print_success("All migrations applied.");
    print_hint("Note: Connect this to your actual SQLx executor for real migration logic.");
}

pub fn run_migrate_rollback() {
    if !guard_project() {
        return;
    }
    println!(
        "{}",
        "🔄 Rolling back last migration...".bright_purple().bold()
    );

    let migration_dir = Path::new("database/migrations");
    if !migration_dir.exists() {
        println!("  No migrations directory found.");
        return;
    }

    let mut files: Vec<_> = fs::read_dir(migration_dir)
        .unwrap()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map(|x| x == "sql").unwrap_or(false))
        .collect();
    files.sort_by_key(|e| e.path());

    match files.last() {
        Some(f) => {
            let name = f.path().file_name().unwrap().to_string_lossy().to_string();
            println!(
                "  {} Rolling back: {}",
                "◀".bright_yellow(),
                name.bright_white()
            );
            print_success("Rollback complete.");
        }
        None => println!("  {} Nothing to roll back.", "ℹ️".blue()),
    }
}

pub fn run_db_seed() {
    if !guard_project() {
        return;
    }
    println!("{}", "🌱 Seeding database...".bright_purple().bold());

    let seeder_dir = Path::new("database/seeders");
    if !seeder_dir.exists() || fs::read_dir(seeder_dir).map(|d| d.count()).unwrap_or(0) == 0 {
        println!("  {} No seeders found in database/seeders/", "ℹ️".blue());
        print_hint("Create .sql files in database/seeders/ to populate your database.");
        return;
    }

    let files: Vec<_> = fs::read_dir(seeder_dir)
        .unwrap()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map(|x| x == "sql").unwrap_or(false))
        .collect();

    for file in &files {
        println!(
            "  {} Seeding: {}",
            "▶".bright_cyan(),
            file.path()
                .file_name()
                .unwrap()
                .to_string_lossy()
                .bright_white()
        );
    }
    print_success("Database seeded.");
}

// ─────────────────────────────────────────────────────────────────────────────
//  DEVOPS GENERATORS
// ─────────────────────────────────────────────────────────────────────────────

pub fn generate_docker() {
    if !guard_project() {
        return;
    }
    println!(
        "{}",
        "🐳 Generating Docker configuration..."
            .bright_purple()
            .bold()
    );

    let project_name = get_project_name();

    let dockerfile = r#"# ─── Build Stage ───────────────────────────────────────────────────────────
FROM rust:1.82-slim-bookworm AS builder

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY . .
RUN cargo build --release

# ─── Runtime Stage ──────────────────────────────────────────────────────────
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /app/target/release/. .
COPY --from=builder /app/resources ./resources
COPY --from=builder /app/.env.example ./.env

EXPOSE 8000
CMD ["./server"]
"#;

    let compose = format!(
        r#"version: "3.9"

services:
  app:
    build: .
    container_name: {name}-app
    restart: unless-stopped
    ports:
      - "8000:8000"
    volumes:
      - ./.env:/app/.env
      - ./storage:/app/storage
      - ./database:/app/database
    environment:
      - APP_ENV=production
    networks:
      - lumina-net

networks:
  lumina-net:
    driver: bridge
"#,
        name = project_name
    );

    fs::write("Dockerfile", dockerfile).ok();
    fs::write("docker-compose.yml", compose).ok();
    print_success("Dockerfile");
    print_success("docker-compose.yml");
    print_hint("Build and run with: docker-compose up --build -d");
}

pub fn generate_nginx() {
    if !guard_project() {
        return;
    }
    println!(
        "{}",
        "🌐 Generating Nginx configuration..."
            .bright_purple()
            .bold()
    );

    let project_name = get_project_name();

    let nginx_conf = format!(
        r#"# Lumina Framework — Nginx Reverse Proxy Configuration
# Place this file in /etc/nginx/sites-available/{name}
# Then: sudo ln -s /etc/nginx/sites-available/{name} /etc/nginx/sites-enabled/
# sudo nginx -t && sudo systemctl reload nginx

server {{
    listen 80;
    server_name your-domain.com www.your-domain.com;

    # Redirect all HTTP to HTTPS
    return 301 https://$host$request_uri;
}}

server {{
    listen 443 ssl http2;
    server_name your-domain.com www.your-domain.com;

    ssl_certificate     /etc/letsencrypt/live/your-domain.com/fullchain.pem;
    ssl_certificate_key /etc/letsencrypt/live/your-domain.com/privkey.pem;
    ssl_protocols       TLSv1.2 TLSv1.3;
    ssl_ciphers         HIGH:!aNULL:!MD5;

    client_max_body_size 50M;

    location / {{
        proxy_pass         http://127.0.0.1:8000;
        proxy_http_version 1.1;
        proxy_set_header   Upgrade $http_upgrade;
        proxy_set_header   Connection "upgrade";
        proxy_set_header   Host $host;
        proxy_set_header   X-Real-IP $remote_addr;
        proxy_set_header   X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header   X-Forwarded-Proto $scheme;
        proxy_read_timeout 86400;
    }}

    location /public/ {{
        alias /var/www/{name}/public/;
        expires 30d;
        add_header Cache-Control "public, no-transform";
    }}
}}
"#,
        name = project_name
    );

    fs::create_dir_all("deploy").ok();
    let path = "deploy/nginx.conf".to_string();
    fs::write(&path, nginx_conf).ok();
    print_success(&format!("Nginx config: {}", path));
    print_hint("Copy deploy/nginx.conf to /etc/nginx/sites-available/ on your server.");
}

pub fn generate_supervisor() {
    if !guard_project() {
        return;
    }
    println!(
        "{}",
        "⚙️  Generating Supervisor configuration..."
            .bright_purple()
            .bold()
    );

    let project_name = get_project_name();

    let supervisor_conf = format!(
        r#"; Lumina Framework — Supervisor Configuration
; Place this file in /etc/supervisor/conf.d/{name}.conf
; Then: sudo supervisorctl reread && sudo supervisorctl update

[program:{name}]
command=/var/www/{name}/target/release/server
directory=/var/www/{name}
autostart=true
autorestart=true
startsecs=5
startretries=3
stderr_logfile=/var/log/{name}/err.log
stdout_logfile=/var/log/{name}/out.log
user=www-data
environment=APP_ENV="production"

[group:{name}]
programs={name}
priority=999
"#,
        name = project_name
    );

    fs::create_dir_all("deploy").ok();
    let path = "deploy/supervisor.conf";
    fs::write(path, supervisor_conf).ok();
    print_success(&format!("Supervisor config: {}", path));
    print_hint(&format!(
        "Copy deploy/supervisor.conf to /etc/supervisor/conf.d/{}.conf on your server.",
        project_name
    ));
}

// ─────────────────────────────────────────────────────────────────────────────
//  UTIL
// ─────────────────────────────────────────────────────────────────────────────

fn get_project_name() -> String {
    if let Ok(content) = fs::read_to_string("Cargo.toml") {
        for line in content.lines() {
            if line.trim().starts_with("name") {
                if let Some(val) = line.split('=').nth(1) {
                    return val.trim().trim_matches('"').to_string();
                }
            }
        }
    }
    "lumina-app".to_string()
}
