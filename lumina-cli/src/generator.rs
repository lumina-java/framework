use chrono::Local;
use colored::*;
use std::fs::{self, OpenOptions};
use std::io::{self, Write};
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
use crate::app::models::*;

// =========================================================================
//  {class} CONTROLLER
// =========================================================================

pub struct {class};

impl {class} {{
    /// GET /{lower} — Ambil semua data
    pub async fn index(req: Request) -> LuminaResult {{
        req.ok(serde_json::json!({{
            "message": "List of {base}",
            "status": "success"
        }}))
    }}

    /// GET /{lower}/paged — Paginasi data otomatis dari ?page=X&per_page=Y
    pub async fn paginate(req: Request) -> LuminaResult {{
        let (page, per_page) = req.pagination();
        req.ok(serde_json::json!({{
            "page": page,
            "per_page": per_page,
            "message": "Paged {base}"
        }}))
    }}

    /// POST /{lower} — Simpan data baru
    pub async fn store(req: Request) -> LuminaResult {{
        req.created(serde_json::json!({{
            "message": "{base} created successfully"
        }}))
    }}

    /// GET /{lower}/:id — Ambil satu detail data
    pub async fn show(req: Request) -> LuminaResult {{
        let id = req.id_or_fail()?;
        req.ok(serde_json::json!({{
            "id": id,
            "message": "{base} detail"
        }}))
    }}

    /// PUT /{lower}/:id — Update data
    pub async fn update(req: Request) -> LuminaResult {{
        let id = req.id_or_fail()?;
        req.ok(serde_json::json!({{
            "id": id,
            "message": "{base} updated successfully"
        }}))
    }}

    /// DELETE /{lower}/:id — Hapus data
    pub async fn destroy(req: Request) -> LuminaResult {{
        let id = req.id_or_fail()?;
        req.ok(serde_json::json!({{
            "id": id,
            "message": "{base} deleted successfully"
        }}))
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
        "Register routes in src/routes/api.rs or web.rs using {}::index etc.",
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
        r#"use lumina::prelude::*;
use serde::{{Serialize, Deserialize}};
use sqlx::FromRow;

// =========================================================================
//  {class} MODEL (Eloquent / Active Record)
// =========================================================================

#[derive(Debug, Serialize, Deserialize, FromRow, Clone, Default, LuminaModel)]
#[lumina(table = "{table}")]
pub struct {class} {{
    pub id: i64,
    // Add custom fields here, e.g.:
    // pub name: String,
    // pub description: Option<String>,

    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub deleted_at: Option<String>,
}}

impl {class} {{
    pub fn new() -> Self {{
        Self::default()
    }}
}}
"#,
        class = class,
        table = table,
    );

    let path = format!("src/app/models/{}.rs", lower);
    fs::write(&path, &code).ok();
    append_to_file(
        "src/app/models/mod.rs",
        &format!("pub mod {};\npub use {}::{};\n", lower, lower, class),
    );
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
use serde::Deserialize;

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
    pub async fn login(State(state): State<AppState>) -> Html<String> {
        let mut ctx = Context::new();
        ctx.insert("title", "Login");
        Html(state.view.render("auth/login.blade.rs", &ctx))
    }

    /// POST /login
    pub async fn login_post(
        State(state): State<AppState>,
        Form(form): Form<LoginForm>,
    ) -> Html<String> {
        // TODO: validate credentials, create session/JWT
        let mut ctx = Context::new();
        ctx.insert("title", "Login");
        ctx.insert("error", "Invalid email or password.");
        Html(state.view.render("auth/login.blade.rs", &ctx))
    }

    /// GET /register
    pub async fn register(State(state): State<AppState>) -> Html<String> {
        let mut ctx = Context::new();
        ctx.insert("title", "Create Account");
        Html(state.view.render("auth/register.blade.rs", &ctx))
    }

    /// POST /register
    pub async fn register_post(
        State(state): State<AppState>,
        Form(form): Form<RegisterForm>,
    ) -> Html<String> {
        // TODO: hash password, insert user, redirect
        let mut ctx = Context::new();
        ctx.insert("title", "Create Account");
        Html(state.view.render("auth/register.blade.rs", &ctx))
    }

    /// GET /logout
    pub async fn logout() -> Html<String> {
        // TODO: destroy session/token
        Html("<script>window.location='/'</script>".to_string())
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

pub fn generate_docker(name: Option<String>, port: Option<u16>) {
    if !guard_project() {
        return;
    }
    println!(
        "{}",
        "🐳 Generating Production Docker, Compose & Nginx Configuration..."
            .bright_purple()
            .bold()
    );

    let default_name = get_project_name();
    let target_name = match name {
        Some(n) if !n.trim().is_empty() => n.trim().to_string(),
        _ => {
            print!(
                "  {} Masukkan nama project / domain server (misal: pos.sugandi.my.id atau service_pos_event) [{}]: ",
                "▶".bright_cyan(),
                default_name.bright_yellow()
            );
            let _ = io::stdout().flush();
            let mut input = String::new();
            let _ = io::stdin().read_line(&mut input);
            let trimmed = input.trim();
            if trimmed.is_empty() {
                default_name.clone()
            } else {
                trimmed.to_string()
            }
        }
    };

    let exposed_port = port.unwrap_or(8000);
    let safe_name = target_name.replace('.', "-").replace('_', "-").to_lowercase();
    let container_name = format!("{}-app", safe_name);
    let pkg_name = get_project_name();

    // 1. Dockerfile
    let dockerfile = format!(
        r#"# ─── Stage 1: Build Stage ───────────────────────────────────────────────────────────
FROM rust:1.80-slim-bookworm AS builder

RUN apt-get update && apt-get install -y pkg-config libssl-dev build-essential && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY . .

# Build dengan Docker BuildKit Cache Mounts untuk kompilasi ulang super cepat
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    --mount=type=cache,target=/app/target \
    cargo build --release && \
    (cp /app/target/release/{pkg_name}-server /app/server 2>/dev/null || \
     cp /app/target/release/{pkg_name} /app/server 2>/dev/null || \
     find /app/target/release/ -maxdepth 1 -type f -executable ! -name "*.so" ! -name "*.d" -exec cp {{}} /app/server \; )

# ─── Stage 2: Runtime Stage ──────────────────────────────────────────────────────────
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates libssl3 curl && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /app/server ./server
COPY --from=builder /app/resources ./resources
COPY --from=builder /app/public ./public
COPY --from=builder /app/database ./database
COPY --from=builder /app/storage ./storage
COPY --from=builder /app/.env.example ./.env

ENV APP_ENV=production
ENV APP_PORT={port}

EXPOSE {port}
CMD ["./server"]
"#,
        pkg_name = pkg_name,
        port = exposed_port
    );

    // 2. .dockerignore
    let dockerignore = r#"target/
.git/
.github/
.env
storage/cache/*
storage/logs/*
*.sqlite
*.db
node_modules/
"#;

    // 3. docker-compose.yml
    let compose = format!(
        r#"version: "3.9"

services:
  {safe_name}:
    build:
      context: .
      dockerfile: Dockerfile
    container_name: {container_name}
    restart: unless-stopped
    ports:
      - "{port}:{port}"
    volumes:
      - ./.env:/app/.env
      - ./storage:/app/storage
      - ./database:/app/database
      - ./public:/app/public
    environment:
      - APP_ENV=production
      - APP_PORT={port}
      - APP_URL=http://{raw_name}
    networks:
      - lumina-net

networks:
  lumina-net:
    driver: bridge
"#,
        safe_name = safe_name,
        container_name = container_name,
        port = exposed_port,
        raw_name = target_name
    );

    // 4. deploy/nginx.conf (aaPanel / Nginx ready reverse proxy)
    let nginx_conf = format!(
        r#"# Lumina Framework — Nginx / aaPanel Reverse Proxy Configuration
# Domain: {raw_name} -> Container Port: {port}

server {{
    listen 80;
    server_name {raw_name};

    # Redirect HTTP to HTTPS
    return 301 https://$host$request_uri;
}}

server {{
    listen 443 ssl http2;
    server_name {raw_name};

    # SSL Configuration (sesuaikan path certs aaPanel / Let's Encrypt jika menggunakan SSL)
    # ssl_certificate     /www/server/panel/vhost/cert/{raw_name}/fullchain.pem;
    # ssl_certificate_key /www/server/panel/vhost/cert/{raw_name}/privkey.pem;

    client_max_body_size 50M;

    location / {{
        proxy_pass         http://127.0.0.1:{port};
        proxy_http_version 1.1;
        proxy_set_header   Upgrade $http_upgrade;
        proxy_set_header   Connection "upgrade";
        proxy_set_header   Host $host;
        proxy_set_header   X-Real-IP $remote_addr;
        proxy_set_header   X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header   X-Forwarded-Proto $scheme;
        proxy_read_timeout 86400;
    }}
}}
"#,
        raw_name = target_name,
        port = exposed_port
    );

    fs::write("Dockerfile", dockerfile).ok();
    fs::write(".dockerignore", dockerignore).ok();
    fs::write("docker-compose.yml", compose).ok();

    fs::create_dir_all("deploy").ok();
    let nginx_path = format!("deploy/nginx-{}.conf", safe_name);
    fs::write(&nginx_path, nginx_conf).ok();

    println!();
    print_success("Dockerfile (Multi-Stage with BuildKit Cache Mounts)");
    print_success(".dockerignore");
    print_success("docker-compose.yml");
    print_success(&format!("Nginx / aaPanel config: {}", nginx_path));
    println!();
    print_hint("Build & run: DOCKER_BUILDKIT=1 docker-compose up --build -d");
    print_hint(&format!(
        "Server Domain / Service: http://{} (Port: {})",
        target_name.bright_green(),
        exposed_port.to_string().bright_yellow()
    ));
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
