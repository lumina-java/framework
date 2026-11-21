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

pub fn cmd_clean(purge: bool) {
    if !guard_project() {
        return;
    }

    if purge {
        println!("{}", "🧹 Menjalankan pembersihan total (cargo clean)...".bright_yellow().bold());
        let _ = Command::new("cargo").arg("clean").status();
        println!("{} Seluruh target cache berhasil dibersihkan.", "✅".green());
        return;
    }

    println!("{}", "✨ Menjalankan Lumina Smart Clean (Optimasi Cache)...".bright_purple().bold());

    // 1. Jalankan cargo sweep jika terinstall
    let sweep_status = Command::new("cargo")
        .args(["sweep", "-m", "8GB"])
        .status();

    if let Err(_) = sweep_status {
        println!(
            "   {} cargo-sweep tidak terdeteksi. Silakan pasang: {}",
            "💡".yellow(),
            "cargo install cargo-sweep".bright_cyan()
        );
    }

    // 2. Bersihkan sesi incremental usang
    let inc_path = Path::new("target/debug/incremental");
    if inc_path.exists() {
        if fs::remove_dir_all(inc_path).is_ok() {
            println!("   {} Sesi kompilasi inkremental usang dibersihkan.", "✅".green());
        }
    }

    // 3. Bersihkan file symbol .pdb lama di target/debug/deps (Windows debug symbols)
    let deps_path = Path::new("target/debug/deps");
    if deps_path.exists() {
        if let Ok(entries) = fs::read_dir(deps_path) {
            let mut pdb_count = 0;
            let mut pdb_bytes = 0u64;
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() {
                    if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                        if ext.eq_ignore_ascii_case("pdb") {
                            if let Ok(meta) = path.metadata() {
                                pdb_bytes += meta.len();
                            }
                            let _ = fs::remove_file(path);
                            pdb_count += 1;
                        }
                    }
                }
            }
            if pdb_count > 0 {
                let mb = pdb_bytes as f64 / (1024.0 * 1024.0);
                println!(
                    "   {} Berhasil memangkas {} file Windows .pdb debug symbols ({:.1} MB).",
                    "✅".green(),
                    pdb_count,
                    mb
                );
            }
        }
    }

    // 4. Bersihkan log lama di storage/logs
    let logs_path = Path::new("storage/logs");
    if logs_path.exists() {
        if let Ok(entries) = fs::read_dir(logs_path) {
            let mut count = 0;
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() {
                    if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                        if name.starts_with("lumina.log.") {
                            count += 1;
                        }
                    }
                }
            }
            if count > 0 {
                println!("   {} Terdeteksi {} file log riwayat di storage/logs.", "ℹ️".blue(), count);
            }
        }
    }

    println!("\n{} Cache berhasil dioptimasi! Build aktif tetap hangat dan cepat.", "🎉".green().bold());
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
        r#"use pakem::prelude::*;
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
        r#"use pakem::prelude::*;
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
        r#"use pakem::prelude::*;
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

    let auth_controller = r#"use pakem::prelude::*;
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
// ─────────────────────────────────────────────────────────────────────────────
// (Database commands are now natively handled by migrator.rs)
// ─────────────────────────────────────────────────────────────────────────────

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

// ─────────────────────────────────────────────────────────────────────────────
//  WASM PLUGIN GENERATOR & COMPILER
// ─────────────────────────────────────────────────────────────────────────────

pub fn generate_plugin_new(name: &str) {
    let clean_name = name.trim().to_lowercase().replace("-", "_");
    if clean_name.is_empty() {
        println!("{}", "❌ Error: Plugin name cannot be empty!".red().bold());
        return;
    }

    let plugin_dir = Path::new("plugins").join(&clean_name);
    if plugin_dir.exists() {
        println!(
            "{}",
            format!("❌ Error: Plugin '{}' already exists in folder {:?}!", clean_name, plugin_dir).red().bold()
        );
        return;
    }

    let src_dir = plugin_dir.join("src");
    if let Err(e) = fs::create_dir_all(&src_dir) {
        println!("{}", format!("❌ Error: Could not create plugin directory: {}", e).red().bold());
        return;
    }

    // 1. Cargo.toml
    let cargo_toml = format!(
        r#"[package]
name = "{}"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
"#,
        clean_name
    );
    let _ = fs::write(plugin_dir.join("Cargo.toml"), cargo_toml);

    // 2. src/lib.rs
    let lib_rs = format!(
        r#"use std::mem;

/// Alokasi buffer memori heap guest untuk transfer data dari host Lumina.
#[no_mangle]
pub extern "C" fn alloc(len: i32) -> i32 {{
    let mut buf = Vec::with_capacity(len as usize);
    let ptr = buf.as_mut_ptr();
    mem::forget(buf);
    ptr as i32
}}

/// Dealokasi buffer memori heap guest (opsional).
#[no_mangle]
pub extern "C" fn dealloc(ptr: i32, len: i32) {{
    unsafe {{
        let _ = Vec::from_raw_parts(ptr as *mut u8, len as usize, len as usize);
    }}
}}

/// Fungsi filter transformasi untuk dipanggil dari Blade:
/// {{{{ $data | {} }}}} atau @wasm('{}', 'transform', $data)
/// Return: (out_ptr << 32) | (out_len & 0xFFFF_FFFF)
#[no_mangle]
pub extern "C" fn transform(ptr: i32, len: i32) -> i64 {{
    let input = unsafe {{
        let slice = std::slice::from_raw_parts(ptr as *const u8, len as usize);
        std::str::from_utf8(slice).unwrap_or("")
    }};

    // Logika transformasi default: bungkus dengan tanda kurung siku (silakan modifikasi sesuai kebutuhan)
    let output = format!("[{{}}]", input);
    let out_bytes = output.into_bytes();
    let out_len = out_bytes.len() as i64;
    let out_ptr = out_bytes.as_ptr() as i64;

    mem::forget(out_bytes);

    (out_ptr << 32) | (out_len & 0xFFFF_FFFF)
}}
"#,
        clean_name, clean_name
    );
    let _ = fs::write(src_dir.join("lib.rs"), lib_rs);

    // 3. README.md
    let readme = format!(
        r#"# Lumina WASM Plugin: {}

Plugin WebAssembly untuk Lumina Framework.

## Cara Kompilasi:
```bash
lumina plugin:build {}
```
Binary output `.wasm` akan otomatis disalin ke `plugins/{}.wasm`.

## Penggunaan di Blade View:
```html
{{{{ $data | {} }}}}
@wasm('{}', 'transform', $data)
```
"#,
        clean_name, clean_name, clean_name, clean_name, clean_name
    );
    let _ = fs::write(plugin_dir.join("README.md"), readme);

    println!(
        "✨ {}",
        format!("WASM Plugin '{}' successfully created in plugins/{}/", clean_name, clean_name)
            .bright_purple()
            .bold()
    );
    println!("👉 Next step: Compile the plugin with:");
    println!("   {}\n", format!("lumina plugin:build {}", clean_name).bright_cyan());
}

pub fn cmd_plugin_build(name: Option<&str>) {
    println!("{}", "🔨 Compiling Lumina WebAssembly Plugin(s)...".bright_blue().bold());

    let plugins_to_build: Vec<String> = if let Some(n) = name {
        vec![n.trim().to_lowercase().replace("-", "_")]
    } else {
        let mut list = Vec::new();
        if let Ok(entries) = fs::read_dir("plugins") {
            for entry in entries.flatten() {
                let p = entry.path();
                if p.is_dir() && p.join("Cargo.toml").exists() {
                    if let Some(folder_name) = p.file_name().and_then(|s| s.to_str()) {
                        list.push(folder_name.to_string());
                    }
                }
            }
        }
        list
    };

    if plugins_to_build.is_empty() {
        println!("{}", "⚠️  No plugin crates found to compile.".yellow());
        println!("💡 Tip: Create a new plugin with: {}", "lumina plugin:new <name>".bright_cyan());
        return;
    }

    for plugin in plugins_to_build {
        let plugin_dir = Path::new("plugins").join(&plugin);
        if !plugin_dir.join("Cargo.toml").exists() {
            println!(
                "{}",
                format!("❌ Error: Plugin folder 'plugins/{}' has no Cargo.toml!", plugin).red()
            );
            continue;
        }

        println!(
            "📦 Compiling plugin '{}' to target wasm32-unknown-unknown...",
            plugin.bright_cyan()
        );

        let status = Command::new("cargo")
            .arg("build")
            .arg("--target")
            .arg("wasm32-unknown-unknown")
            .arg("--release")
            .current_dir(&plugin_dir)
            .status();

        match status {
            Ok(s) if s.success() => {
                let wasm_src = plugin_dir
                    .join("target")
                    .join("wasm32-unknown-unknown")
                    .join("release")
                    .join(format!("{}.wasm", plugin));

                let wasm_dest = Path::new("plugins").join(format!("{}.wasm", plugin));

                if wasm_src.exists() {
                    if let Ok(_) = fs::copy(&wasm_src, &wasm_dest) {
                        let size_bytes = fs::metadata(&wasm_dest).map(|m| m.len()).unwrap_or(0);
                        let size_kb = (size_bytes as f64) / 1024.0;
                        println!(
                            "  {} {}",
                            "✅".green(),
                            format!("Output binary saved to: {:?} ({:.2} KB)", wasm_dest, size_kb).green()
                        );
                    } else {
                        println!(
                            "{}",
                            format!("❌ Failed to copy WASM binary from {:?} to {:?}", wasm_src, wasm_dest).red()
                        );
                    }
                } else {
                    println!(
                        "{}",
                        format!("❌ Binary output {:?} not found after build!", wasm_src).red()
                    );
                }
            }
            Ok(s) => {
                println!("{}", format!("❌ Compilation failed with exit code: {:?}", s.code()).red());
                println!("💡 Tip: Ensure WASM target is installed: rustup target add wasm32-unknown-unknown");
            }
            Err(e) => {
                println!("{}", format!("❌ Failed to execute cargo: {}", e).red());
                println!("💡 Tip: Ensure WASM target is installed: rustup target add wasm32-unknown-unknown");
            }
        }
    }
}

pub fn cmd_plugin_list() {
    println!("{}", "🔍 Scanning for Lumina Plugins & Indonesian Scripts...\n".bright_blue().bold());

    struct Item {
        name: String,
        kind: String,
        size_str: String,
        status: String,
    }

    let mut items = Vec::new();

    // 1. Scan plugins/ folder
    if let Ok(entries) = fs::read_dir("plugins") {
        for entry in entries.flatten() {
            let p = entry.path();
            if p.is_file() {
                if let Some(ext) = p.extension().and_then(|s| s.to_str()) {
                    let file_name = p.file_name().and_then(|s| s.to_str()).unwrap_or("?").to_string();
                    let size = fs::metadata(&p).map(|m| m.len()).unwrap_or(0);
                    let size_kb = format!("{:.2} KB", (size as f64) / 1024.0);

                    if ext == "wasm" {
                        items.push(Item {
                            name: file_name,
                            kind: "WASM Binary".to_string(),
                            size_str: size_kb,
                            status: "Ready to Execute ✅".to_string(),
                        });
                    } else if ext == "is" {
                        let content = fs::read_to_string(&p).unwrap_or_default();
                        let func_count = content.lines().filter(|l| l.trim().starts_with("fungsi ")).count();
                        items.push(Item {
                            name: file_name,
                            kind: "Script (.is)".to_string(),
                            size_str: size_kb,
                            status: format!("{} functions", func_count),
                        });
                    }
                }
            } else if p.is_dir() && p.join("Cargo.toml").exists() {
                let folder_name = p.file_name().and_then(|s| s.to_str()).unwrap_or("?").to_string();
                let wasm_exists = Path::new("plugins").join(format!("{}.wasm", folder_name)).exists();
                let status = if wasm_exists {
                    "Configured & Compiled".to_string()
                } else {
                    "Not built (run lumina plugin:build)".to_string()
                };
                items.push(Item {
                    name: format!("{}/ (Crate)", folder_name),
                    kind: "WASM Source".to_string(),
                    size_str: "-".to_string(),
                    status,
                });
            }
        }
    }

    // 2. Scan app/scripts/ and resources/scripts/ folder
    for script_dir in &["app/scripts", "resources/scripts"] {
        if let Ok(entries) = fs::read_dir(script_dir) {
            for entry in entries.flatten() {
                let p = entry.path();
                if p.is_file() && p.extension().and_then(|s| s.to_str()) == Some("is") {
                    let file_name = p.file_name().and_then(|s| s.to_str()).unwrap_or("?").to_string();
                    let size = fs::metadata(&p).map(|m| m.len()).unwrap_or(0);
                    let size_kb = format!("{:.2} KB", (size as f64) / 1024.0);
                    let content = fs::read_to_string(&p).unwrap_or_default();
                    let func_count = content.lines().filter(|l| l.trim().starts_with("fungsi ")).count();
                    items.push(Item {
                        name: format!("{}/{}", script_dir, file_name),
                        kind: "Script (.is)".to_string(),
                        size_str: size_kb,
                        status: format!("{} functions", func_count),
                    });
                }
            }
        }
    }

    if items.is_empty() {
        println!("{}", "ℹ️  No WASM plugins or Indonesian Scripts found.".yellow());
        println!("💡 Tip: Create a new plugin with: {}", "lumina plugin:new <name>".bright_cyan());
        println!("💡 Tip: Place .is scripts in folder {} or {}\n", "app/scripts/".bright_cyan(), "plugins/".bright_cyan());
        return;
    }

    println!("{:<32} {:<15} {:<12} {:<35}", "NAME", "TYPE", "SIZE", "STATUS / DETAIL");
    println!("{:-<32} {:-<15} {:-<12} {:-<35}", "", "", "", "");
    for item in &items {
        println!("{:<32} {:<15} {:<12} {:<35}", item.name.bright_green(), item.kind.bright_yellow(), item.size_str, item.status);
    }
    println!("{:-<32} {:-<15} {:-<12} {:-<35}", "", "", "", "");
    println!("Total detected items: {}\n", items.len());
}

pub fn cmd_plugin_metrics() {
    println!("{}", "📊 Lumina WASM Sandbox & Plugin Metrics Dashboard\n".bright_cyan().bold());

    let manager = pakem::core::sandbox::LuminaPluginManager::new();
    let loaded = manager.load_directory("plugins").unwrap_or(0);

    let metrics = manager.get_metrics();
    let stats = manager.plugin_stats();

    println!("Total WASM Modules Loaded : {}", loaded.to_string().bright_green().bold());
    println!("Script Modules Cached     : {}\n", metrics.is_scripts_count.to_string().bright_yellow().bold());

    if loaded == 0 {
        println!("{}", "ℹ️  Tidak ada biner .wasm aktif di direktori plugins/.".yellow());
        println!("💡 Tip: Buat plugin baru dengan: {}", "lumina plugin:new <name>".bright_cyan());
        println!("💡 Tip: Kompilasi plugin dengan: {}\n", "lumina plugin:build".bright_cyan());
        return;
    }

    println!("┌───────────────────────┬──────────────┬──────────────┬──────────────┬──────────────┐");
    println!("│ {:<21} │ {:<12} │ {:<12} │ {:<12} │ {:<12} │", "PLUGIN NAME", "CALLS COUNT", "ERRORS COUNT", "AVG DURATION", "STATUS");
    println!("├───────────────────────┼──────────────┼──────────────┼──────────────┼──────────────┤");

    for name in &metrics.wasm_plugins {
        let stat = stats.iter().find(|s| &s.name == name);
        let calls = stat.map(|s| s.calls_count).unwrap_or(0);
        let errors = stat.map(|s| s.errors_count).unwrap_or(0);
        let avg_dur = stat.map(|s| format!("{:.2}µs", s.avg_duration_us)).unwrap_or_else(|| "0.00µs".to_string());
        let status = if errors > 0 { "Warning ⚠️" } else { "Healthy ✅" };

        println!("│ {:<21} │ {:<12} │ {:<12} │ {:<12} │ {:<12} │", name.bright_green(), calls, errors, avg_dur, status);
    }

    println!("└───────────────────────┴──────────────┴──────────────┴──────────────┴──────────────┘\n");
}

pub fn cmd_plugin_reload() {
    println!("{}", "🔄 Reloading Lumina WASM Plugins (Zero-Downtime Hot-Reload)...".bright_purple().bold());
    let start = std::time::Instant::now();

    let manager = pakem::core::sandbox::LuminaPluginManager::new();
    match manager.reload("plugins") {
        Ok(count) => {
            let dur = start.elapsed().as_secs_f64() * 1000.0;
            println!("{} Berhasil memuat ulang {} plugin dalam {:.2}ms tanpa downtime! ⚡\n", "✅ [Reload Success]".green().bold(), count.to_string().bright_yellow().bold(), dur);
        }
        Err(e) => {
            println!("{} Gagal memuat ulang plugin: {}\n", "❌ [Reload Failed]".red().bold(), e);
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
//  MODEL FACTORY GENERATOR
// ─────────────────────────────────────────────────────────────────────────────

pub fn generate_factory(name: &str) {
    if !guard_project() {
        return;
    }
    println!("{}", "🏭 Generating Model Factory...".bright_purple().bold());

    let class_name = capitalize_first(name);
    let base = class_name.trim_end_matches("Factory");
    let model_name = base.to_string();
    let factory_name = format!("{}Factory", base);
    let file_base = format!("{}_factory", to_snake_case(base));
    let file_path = format!("database/factories/{}.rs", file_base);

    if Path::new(&file_path).exists() {
        println!("{} Factory already exists: {}", "⚠️".yellow(), file_path.yellow());
        return;
    }

    let _ = fs::create_dir_all("database/factories");

    let code = format!(
        r#"use pakem::prelude::*;
use crate::app::models::{snake_model}::{model_name};

// =========================================================================
//  {factory_name} (Lumina Model Factory)
// =========================================================================

pub struct {factory_name};

impl {factory_name} {{
    pub fn new() -> Self {{
        Self
    }}
}}

impl Factory<{model_name}> for {factory_name} {{
    /// Definisikan nilai dummy default untuk model {model_name}
    fn definition(&self) -> {model_name} {{
        {model_name} {{
            name: Faker::name(),
            email: Faker::unique_email(),
            phone: Faker::phone(),
            created_at: Some(Faker::datetime_recent()),
            updated_at: Some(Faker::datetime_recent()),
            ..Default::default()
        }}
    }}
}}

impl HasFactory<{factory_name}> for {model_name} {{
    fn factory() -> {factory_name} {{
        {factory_name}::new()
    }}
}}
"#,
        factory_name = factory_name,
        model_name = model_name,
        snake_model = to_snake_case(&model_name),
    );

    if let Err(e) = fs::write(&file_path, code) {
        println!("{} Failed to write factory file: {}", "❌".red(), e);
        return;
    }

    // Register into database/factories/mod.rs
    let mod_file = "database/factories/mod.rs";
    let mod_line = format!("pub mod {};\n", file_base);
    if Path::new(mod_file).exists() {
        let existing = fs::read_to_string(mod_file).unwrap_or_default();
        if !existing.contains(&mod_line) {
            append_to_file(mod_file, &mod_line);
        }
    } else {
        let _ = fs::write(mod_file, format!("// Lumina Database Factories\n{}", mod_line));
    }

    print_success(&format!("Factory created: {}", file_path.bright_green()));
    print_hint(&format!("Use in seeder/test: {}::new().count(10).create(req.db()).await", factory_name));
    print_hint(&format!("Or shortcut on model: {}::factory().count(10).create(req.db()).await\n", model_name));
}

pub fn generate_seeder(name: &str) {
    if !guard_project() {
        return;
    }
    println!("{}", "🌱 Generating Database Seeder Class...".bright_purple().bold());

    let clean = name.trim_end_matches(".rs").trim();
    let seeder_name = if clean.ends_with("Seeder") {
        clean.to_string()
    } else {
        format!("{}Seeder", capitalize_first(clean))
    };

    let file_base = to_snake_case(&seeder_name);
    let seeder_dir = "src/database/seeders";
    let _ = fs::create_dir_all(seeder_dir);

    let file_path = format!("{}/{}.rs", seeder_dir, file_base);
    if Path::new(&file_path).exists() {
        println!("{} Seeder {} already exists!", "❌".red(), file_path);
        return;
    }

    let code = format!(
        r#"use pakem::prelude::*;

// =========================================================================
//  {seeder_name} (Lumina Database Seeder Class)
// =========================================================================

#[derive(Default)]
pub struct {seeder_name};

#[async_trait::async_trait]
impl Seeder for {seeder_name} {{
    /// Eksekusi pengisian data awal atau data pengujian
    async fn run(&self, db: &DatabasePool) -> Result<(), sqlx::Error> {{
        println!("🌱 Menjalankan Seeder: {seeder_name}");

        // Contoh: Membuat 25 data realistis menggunakan Model Factory & Faker
        // User::factory()
        //     .count(25)
        //     .state(|u| u.is_active = true)
        //     .create(db)
        //     .await?;

        Ok(())
    }}
}}
"#,
        seeder_name = seeder_name
    );

    if let Err(e) = fs::write(&file_path, code) {
        println!("{} Failed to write seeder file: {}", "❌".red(), e);
        return;
    }

    let mod_file = "src/database/seeders/mod.rs";
    let mod_line = format!("pub mod {};\npub use {}::{};\n", file_base, file_base, seeder_name);
    if Path::new(mod_file).exists() {
        let existing = fs::read_to_string(mod_file).unwrap_or_default();
        if !existing.contains(&format!("pub mod {};", file_base)) {
            append_to_file(mod_file, &mod_line);
        }
    } else {
        let _ = fs::write(mod_file, format!("// Lumina Database Seeders\n{}", mod_line));
    }

    let sql_dir = "database/seeders";
    let _ = fs::create_dir_all(sql_dir);
    let sql_path = format!("{}/{}_{}.sql", sql_dir, timestamp(), file_base);
    let sql_content = format!("-- Seeder SQL template untuk {}\n-- INSERT INTO table (col) VALUES ('val');\n", seeder_name);
    let _ = fs::write(&sql_path, sql_content);

    print_success(&format!("Seeder created: {}", file_path.bright_green()));
    print_hint(&format!("Run this seeder with: lumina db:seed --class {}\n", seeder_name));
}

pub fn generate_view(name: &str) {
    if !guard_project() {
        return;
    }

    // Normalisasi nama view: "products.index" -> "products/index"
    let clean_name = name
        .trim()
        .trim_end_matches(".blade.rs")
        .trim_end_matches(".html")
        .replace('.', "/");

    let relative_path = if clean_name.starts_with("resources/views/") {
        clean_name.clone()
    } else {
        format!("resources/views/{}", clean_name)
    };

    let file_path = format!("{}.blade.rs", relative_path);
    let path = Path::new(&file_path);

    if path.exists() {
        println!("  {} View file already exists: {}", "⚠".yellow(), file_path.yellow());
        return;
    }

    if let Some(parent) = path.parent() {
        if let Err(e) = fs::create_dir_all(parent) {
            println!("{} Failed to create directory {:?}: {}", "❌".red(), parent, e);
            return;
        }
    }

    let view_dot_name = clean_name.trim_start_matches("resources/views/").replace('/', ".");
    let title_name = clean_name
        .split('/')
        .last()
        .unwrap_or("View")
        .split('_')
        .map(|w| capitalize_first(w))
        .collect::<Vec<_>>()
        .join(" ");

    let content = format!(
r#"@extends('layouts.app')

@section('title', '{title}')

@section('content')
<div class="container mx-auto px-4 py-8">
    <div class="bg-white dark:bg-gray-800 shadow rounded-lg p-6">
        <h1 class="text-2xl font-bold text-gray-800 dark:text-white mb-4">{title}</h1>
        <p class="text-gray-600 dark:text-gray-300">
            Halaman view baru Lumina: <code class="bg-gray-100 dark:bg-gray-700 px-2 py-1 rounded text-purple-600 dark:text-purple-400 font-mono text-sm">{view_dot}</code>
        </p>
    </div>
</div>
@endsection
"#,
        title = title_name,
        view_dot = view_dot_name,
    );

    if let Err(e) = fs::write(&file_path, content) {
        println!("{} Failed to write view file: {}", "❌".red(), e);
        return;
    }

    print_success(&format!("View template created: {}", file_path.bright_green()));
    print_hint(&format!("Render in controller: View::make(\"{}\").render(req).await\n", view_dot_name));
}

pub fn generate_job(name: &str) {
    if !guard_project() {
        return;
    }

    let job_name = capitalize_first(name.trim());
    let mut snake = to_snake_case(&job_name);
    let mut converted_snake = String::new();
    for (i, c) in job_name.chars().enumerate() {
        if c.is_uppercase() && i > 0 {
            converted_snake.push('_');
        }
        converted_snake.push(c.to_ascii_lowercase());
    }
    if !converted_snake.is_empty() {
        snake = converted_snake;
    }

    let jobs_dir = Path::new("src/app/jobs");
    if let Err(e) = fs::create_dir_all(jobs_dir) {
        println!("{} Failed to create directory {:?}: {}", "❌".red(), jobs_dir, e);
        return;
    }

    let file_path = format!("src/app/jobs/{}.rs", snake);
    if Path::new(&file_path).exists() {
        println!("  {} Job file already exists: {}", "⚠".yellow(), file_path.yellow());
        return;
    }

    let content = format!(
r#"use async_trait::async_trait;
use pakem::prelude::*;
use serde::{{Deserialize, Serialize}};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct {job_name} {{
    pub user_id: i64,
}}

impl Dispatchable for {job_name} {{
    fn job_name() -> &'static str {{
        "{job_name}"
    }}
}}

#[async_trait]
impl Job for {job_name} {{
    fn name(&self) -> &'static str {{
        Self::job_name()
    }}

    async fn handle(&self, state: std::sync::Arc<AppState>, payload: Value) -> Result<(), String> {{
        let job: Self = serde_json::from_value(payload).map_err(|e| e.to_string())?;
        tracing::info!("Executing {job_name} for user_id: {{}}", job.user_id);
        
        // Letakkan logika asynchronous background task Anda di sini:
        // misal: kirim email, kirim webhook, proses transaksi/laporan, dsb.

        Ok(())
    }}
}}
"#,
        job_name = job_name,
    );

    if let Err(e) = fs::write(&file_path, content) {
        println!("{} Failed to write job file: {}", "❌".red(), e);
        return;
    }

    // Register into src/app/jobs/mod.rs
    let mod_file = "src/app/jobs/mod.rs";
    let mod_line = format!("pub mod {};\npub use {}::{};\n", snake, snake, job_name);
    if Path::new(mod_file).exists() {
        let existing = fs::read_to_string(mod_file).unwrap_or_default();
        if !existing.contains(&format!("pub mod {};", snake)) {
            append_to_file(mod_file, &mod_line);
        }
    } else {
        let _ = fs::write(mod_file, format!("// Lumina Background Jobs Registry\n{}", mod_line));
    }

    print_success(&format!("Job class created: {}", file_path.bright_green()));
    print_hint(&format!("Dispatch in controller: {} {{ user_id: 123 }}.dispatch(&req).await?\n", job_name));
}

pub fn generate_live(name: &str) {
    if !guard_project() {
        return;
    }
    println!("{}", "⚡ Generating Lumina Live Component...".bright_purple().bold());

    let comp_name = capitalize_first(name);
    let snake = to_snake_case(&comp_name);

    let comp_dir = "src/app/live";
    let view_dir = "resources/views/live";
    let _ = fs::create_dir_all(comp_dir);
    let _ = fs::create_dir_all(view_dir);

    let comp_path = format!("{}/{}.rs", comp_dir, snake);
    let view_path = format!("{}/{}.blade.rs", view_dir, snake);

    if Path::new(&comp_path).exists() {
        println!("  {} Live component already exists: {}", "⚠".yellow(), comp_path.yellow());
        return;
    }

    let comp_content = format!(
r#"use async_trait::async_trait;
use pakem::prelude::*;
use serde::{{Deserialize, Serialize}};
use serde_json::Value;

/// Komponen Reaktif Lumina Live: {comp_name}
/// Mendukung Tri-Dialect: `kilat:*` (Indonesia), `live:*` (Universal), `wire:*` (Laravel Legacy).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct {comp_name} {{
    pub count: i32,
    pub title: String,
}}

impl Default for {comp_name} {{
    fn default() -> Self {{
        Self {{
            count: 0,
            title: "{comp_name}".to_string(),
        }}
    }}
}}

#[async_trait]
impl LiveComponent for {comp_name} {{
    fn name(&self) -> &'static str {{
        "{snake}"
    }}

    fn view(&self) -> &'static str {{
        "live.{snake}"
    }}

    async fn call_action(&mut self, action: &str, _params: Vec<Value>) -> Result<(), AppError> {{
        match action {{
            "tambah" | "increment" => self.count += 1,
            "kurang" | "decrement" => if self.count > 0 {{ self.count -= 1; }},
            "reset" => self.count = 0,
            _ => return Err(AppError::BadRequest(format!("Aksi tidak dikenal: {{}}", action))),
        }}
        Ok(())
    }}
}}
"#,
        comp_name = comp_name,
        snake = snake,
    );

    let view_content = format!(
r#"<div class="p-6 bg-white rounded-2xl shadow-xl border border-slate-100 max-w-md mx-auto my-6 text-center">
    <div class="mb-4">
        <span class="inline-flex items-center px-3 py-1 rounded-full text-xs font-semibold bg-emerald-100 text-emerald-800">
            ⚡ Lumina Live ({comp_name})
        </span>
    </div>

    <h2 class="text-xl font-bold text-slate-700">{{{{ $title }}}}</h2>
    <div class="my-4">
        <span class="text-5xl font-black text-slate-800">{{{{ $count }}}}</span>
    </div>

    <!-- Tombol Interaksi (Mendukung kilat:klik, live:click, wire:click) -->
    <div class="flex items-center justify-center space-x-3">
        <button kilat:klik="kurang" class="w-10 h-10 flex items-center justify-center bg-rose-500 hover:bg-rose-600 text-white font-bold rounded-lg transition">-</button>
        <button live:click="reset" kilat:yakin="Yakin ingin mereset?" class="px-3 h-10 bg-slate-100 hover:bg-slate-200 text-slate-700 text-sm font-semibold rounded-lg transition">Reset</button>
        <button kilat:klik="tambah" class="w-10 h-10 flex items-center justify-center bg-emerald-500 hover:bg-emerald-600 text-white font-bold rounded-lg transition">+</button>
    </div>

    <div class="mt-4">
        <span kilat:tunggu class="text-xs text-slate-400 animate-pulse">Menyinkronkan...</span>
    </div>
</div>
"#,
        comp_name = comp_name,
    );

    if let Err(e) = fs::write(&comp_path, comp_content) {
        println!("{} Failed to write component file: {}", "❌".red(), e);
        return;
    }
    if let Err(e) = fs::write(&view_path, view_content) {
        println!("{} Failed to write view file: {}", "❌".red(), e);
        return;
    }

    // Register into src/app/live/mod.rs
    let mod_file = "src/app/live/mod.rs";
    let mod_line = format!("pub mod {};\npub use {}::{};\n", snake, snake, comp_name);
    if Path::new(mod_file).exists() {
        let existing = fs::read_to_string(mod_file).unwrap_or_default();
        if !existing.contains(&format!("pub mod {};", snake)) {
            append_to_file(mod_file, &mod_line);
        }
    } else {
        let _ = fs::write(mod_file, format!("// Lumina Live Components Registry\n{}", mod_line));
    }

    print_success(&format!("Live component created: {}", comp_path.bright_green()));
    print_success(&format!("Live view template created: {}", view_path.bright_green()));
    print_hint(&format!("Embed in any Blade template: @kilat('{}') or @live('{}')\n", snake, snake));
}


/// Generate complete Lumina Aurora starter kit
pub fn generate_aurora() {
    if !guard_project() {
        return;
    }
    println!(
        "{}",
        "⚡ Installing Lumina Aurora Starter Kit..."
            .bright_purple()
            .bold()
    );

    let view_dirs = [
        "resources/views/aurora/layouts",
        "resources/views/aurora/auth",
        "resources/views/aurora/dashboard",
        "resources/views/aurora/profile",
        "src/app/controllers/aurora",
        "database/migrations",
        "database/seeders",
    ];

    for dir in view_dirs {
        let _ = fs::create_dir_all(dir);
    }

    let guest_layout = include_str!("../templates/views/aurora/layouts/guest.blade.rs");
    let app_layout = include_str!("../templates/views/aurora/layouts/app.blade.rs");
    let login_view = include_str!("../templates/views/aurora/auth/login.blade.rs");
    let register_view = include_str!("../templates/views/aurora/auth/register.blade.rs");
    let dashboard_view = include_str!("../templates/views/aurora/dashboard/index.blade.rs");
    let profile_view = include_str!("../templates/views/aurora/profile/edit.blade.rs");

    let auth_ctrl = include_str!("../templates/controllers/aurora/auth_controller.rs");
    let dash_ctrl = include_str!("../templates/controllers/aurora/dashboard_controller.rs");
    let prof_ctrl = include_str!("../templates/controllers/aurora/profile_controller.rs");
    let mod_ctrl = include_str!("../templates/controllers/aurora/mod.rs");

    let aurora_migration = r#"-- Users Table Migration for Lumina Aurora (RBAC Ready)
CREATE TABLE IF NOT EXISTS users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL UNIQUE,
    password VARCHAR(255) NOT NULL,
    role VARCHAR(50) NOT NULL DEFAULT 'member',
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);
"#;

    let aurora_seeder = r#"-- Lumina Aurora: Default RBAC Users Seeder
-- Akun Bawaan untuk pengujian hak akses peran:
-- 1. admin@lumina.dev   (Administrator) | Password: password123
-- 2. editor@lumina.dev  (Editor)        | Password: password123
-- 3. member@lumina.dev  (Member)        | Password: password123

DELETE FROM users WHERE email IN ('admin@lumina.dev', 'editor@lumina.dev', 'member@lumina.dev');

INSERT INTO users (name, email, password, role) VALUES 
('Administrator Lumina', 'admin@lumina.dev', '$2b$12$9PoqqsF9tgdGa57MX.JIA.ejM3WzhUR.2qslio9xbRzxVWnB9rhSu', 'admin'),
('Editor Konten', 'editor@lumina.dev', '$2b$12$9PoqqsF9tgdGa57MX.JIA.ejM3WzhUR.2qslio9xbRzxVWnB9rhSu', 'editor'),
('Member Komunitas', 'member@lumina.dev', '$2b$12$9PoqqsF9tgdGa57MX.JIA.ejM3WzhUR.2qslio9xbRzxVWnB9rhSu', 'member');
"#;

    let files = [
        ("resources/views/aurora/layouts/guest.blade.rs", guest_layout),
        ("resources/views/aurora/layouts/app.blade.rs", app_layout),
        ("resources/views/aurora/auth/login.blade.rs", login_view),
        ("resources/views/aurora/auth/register.blade.rs", register_view),
        ("resources/views/aurora/dashboard/index.blade.rs", dashboard_view),
        ("resources/views/aurora/profile/edit.blade.rs", profile_view),
        ("src/app/controllers/aurora/auth_controller.rs", auth_ctrl),
        ("src/app/controllers/aurora/dashboard_controller.rs", dash_ctrl),
        ("src/app/controllers/aurora/profile_controller.rs", prof_ctrl),
        ("src/app/controllers/aurora/mod.rs", mod_ctrl),
        ("database/seeders/0001_aurora_users_seeder.sql", aurora_seeder),
        ("database/migrations/0001_create_users_table.sql", aurora_migration),
    ];

    for (path, content) in files {
        if let Err(e) = fs::write(path, content) {
            println!("  ❌ Failed to write {}: {}", path, e);
        } else {
            print_success(&format!("Generated {}", path));
        }
    }

    println!("\n{}", "🎉 Lumina Aurora Starter Kit installed successfully!".bright_green().bold());
    println!("{}", "Langkah selanjutnya untuk migrasi & seeding akun bawaan:".bright_cyan());
    println!("  1. Jalankan Migrasi : {}", "lumina migrate".yellow());
    println!("  2. Jalankan Seeder  : {}", "lumina db:seed".yellow());
    println!("  3. Daftarkan router :");
    println!("     {}", "router.merge(pakem::app::controllers::aurora::routes())".yellow());
    println!("\n{}", "🔐 Akun Bawaan (Default Accounts - Password: password123):".bright_magenta().bold());
    println!("  👑 Admin  : admin@lumina.dev  (Role: admin)");
    println!("  ✍️  Editor : editor@lumina.dev (Role: editor)");
    println!("  👤 Member : member@lumina.dev (Role: member)");
}

// ─────────────────────────────────────────────────────────────────────────────
//  OPENAPI & POSTMAN EXPORTER
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
struct ScannedRoute {
    method: String,
    path: String,
    summary: Option<String>,
    description: Option<String>,
    tags: Vec<String>,
    deprecated: bool,
}

fn collect_rs_files(dir: &Path, list: &mut Vec<std::path::PathBuf>) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let p = entry.path();
            if p.is_dir() {
                collect_rs_files(&p, list);
            } else if p.extension().map_or(false, |ext| ext == "rs") {
                list.push(p);
            }
        }
    }
}

fn scan_routes_in_src() -> Vec<ScannedRoute> {
    let mut files = Vec::new();
    let src_dir = Path::new("src");
    if src_dir.exists() {
        collect_rs_files(src_dir, &mut files);
    }

    let mut routes = Vec::new();
    let re_route = regex::Regex::new(r#"\.(get|post|put|delete|patch)\(\s*"([^"]+)""#).unwrap();
    let re_summary = regex::Regex::new(r#"\.summary\(\s*"([^"]+)""#).unwrap();
    let re_desc = regex::Regex::new(r#"\.description\(\s*"([^"]+)""#).unwrap();
    let re_tag = regex::Regex::new(r#"\.tag\(\s*"([^"]+)""#).unwrap();

    for file_path in files {
        if let Ok(content) = fs::read_to_string(&file_path) {
            for cap in re_route.captures_iter(&content) {
                let method = cap[1].to_uppercase();
                let path = cap[2].to_string();

                let mut summary = None;
                let mut description = None;
                let mut tags = Vec::new();
                let mut deprecated = false;

                if let Some(match_pos) = content.find(&format!(".{}(\"{}\"", cap[1].to_lowercase(), path)) {
                    let slice = &content[match_pos..match_pos.saturating_add(300).min(content.len())];
                    if let Some(s) = re_summary.captures(slice) {
                        summary = Some(s[1].to_string());
                    }
                    if let Some(d) = re_desc.captures(slice) {
                        description = Some(d[1].to_string());
                    }
                    if let Some(t) = re_tag.captures(slice) {
                        tags.push(t[1].to_string());
                    }
                    if slice.contains(".deprecated()") {
                        deprecated = true;
                    }
                }

                routes.push(ScannedRoute {
                    method,
                    path,
                    summary,
                    description,
                    tags,
                    deprecated,
                });
            }
        }
    }

    routes
}

fn build_openapi_spec(routes: &[ScannedRoute], title: &str, version: &str) -> serde_json::Value {
    let mut paths = serde_json::Map::new();

    for r in routes {
        let (openapi_path, path_params) = normalize_cli_path(&r.path);
        let path_obj = paths.entry(openapi_path).or_insert_with(|| serde_json::json!({}));

        let method_lower = r.method.to_lowercase();
        let mut parameters = Vec::new();
        for param in path_params {
            parameters.push(serde_json::json!({
                "name": param,
                "in": "path",
                "required": true,
                "description": format!("Path parameter {}", param),
                "schema": {
                    "type": if param.ends_with("_id") || param == "id" { "integer" } else { "string" }
                }
            }));
        }

        let mut op = serde_json::json!({
            "summary": r.summary.clone().unwrap_or_else(|| format!("{} {}", r.method, r.path)),
            "description": r.description.clone().unwrap_or_default(),
            "tags": r.tags,
            "deprecated": r.deprecated,
            "parameters": parameters,
            "responses": {
                "200": {
                    "description": "Operasi Berhasil",
                    "content": {
                        "application/json": {
                            "schema": {
                                "type": "object",
                                "properties": {
                                    "success": { "type": "boolean" },
                                    "data": { "type": "object" }
                                }
                            }
                        }
                    }
                }
            }
        });

        if matches!(method_lower.as_str(), "post" | "put" | "patch") {
            op["requestBody"] = serde_json::json!({
                "required": true,
                "content": {
                    "application/json": {
                        "schema": {
                            "type": "object"
                        }
                    }
                }
            });
        }

        path_obj[method_lower] = op;
    }

    serde_json::json!({
        "openapi": "3.0.3",
        "info": {
            "title": title,
            "version": version,
            "description": "REST API documentation generated automatically by Lumina CLI."
        },
        "servers": [
            {
                "url": "/",
                "description": "Default Server"
            }
        ],
        "paths": paths
    })
}

fn normalize_cli_path(path: &str) -> (String, Vec<String>) {
    let mut params = Vec::new();
    let segments: Vec<String> = path
        .split('/')
        .map(|segment| {
            if let Some(param) = segment.strip_prefix(':') {
                params.push(param.to_string());
                format!("{{{}}}", param)
            } else if let Some(param) = segment.strip_prefix('*') {
                params.push(param.to_string());
                format!("{{{}}}", param)
            } else if segment.starts_with('{') && segment.ends_with('}') {
                let param = &segment[1..segment.len() - 1];
                params.push(param.to_string());
                segment.to_string()
            } else {
                segment.to_string()
            }
        })
        .collect();

    (segments.join("/"), params)
}

fn cli_json_to_yaml(value: &serde_json::Value, indent: usize) -> String {
    let pad = "  ".repeat(indent);
    match value {
        serde_json::Value::Null => "null".to_string(),
        serde_json::Value::Bool(b) => b.to_string(),
        serde_json::Value::Number(n) => n.to_string(),
        serde_json::Value::String(s) => {
            if s.contains('\n') || s.contains(':') || s.contains('#') || s.contains(' ') || s.is_empty() {
                format!("\"{}\"", s.replace('"', "\\\""))
            } else {
                s.clone()
            }
        }
        serde_json::Value::Array(arr) => {
            if arr.is_empty() {
                "[]".to_string()
            } else {
                let mut out = String::new();
                for item in arr {
                    out.push('\n');
                    out.push_str(&format!("{}- {}", pad, cli_json_to_yaml(item, indent + 1).trim_start()));
                }
                out
            }
        }
        serde_json::Value::Object(map) => {
            if map.is_empty() {
                "{}".to_string()
            } else {
                let mut out = String::new();
                for (k, v) in map {
                    out.push('\n');
                    match v {
                        serde_json::Value::Object(_) | serde_json::Value::Array(_) => {
                            out.push_str(&format!("{}{}:{}", pad, k, cli_json_to_yaml(v, indent + 1)));
                        }
                        _ => {
                            out.push_str(&format!("{}{}: {}", pad, k, cli_json_to_yaml(v, indent)));
                        }
                    }
                }
                out
            }
        }
    }
}

pub fn generate_openapi(output: &str, format: &str, title: &str, version: &str) {
    if !guard_project() {
        return;
    }
    println!("{}", "📖 Scanning project routes for OpenAPI generation...".bright_cyan());

    let routes = scan_routes_in_src();
    println!("   Found {} endpoints.", routes.len().to_string().yellow());

    let spec = build_openapi_spec(&routes, title, version);

    if format.to_lowercase() == "yaml" || format.to_lowercase() == "yml" {
        let yaml = cli_json_to_yaml(&spec, 0);
        if let Err(e) = fs::write(output, yaml) {
            println!("  ❌ Failed to write {}: {}", output, e);
            return;
        }
    } else {
        let json = serde_json::to_string_pretty(&spec).unwrap_or_default();
        if let Err(e) = fs::write(output, json) {
            println!("  ❌ Failed to write {}: {}", output, e);
            return;
        }
    }

    print_success(&format!("OpenAPI v3.0 specification generated successfully at {}", output.bright_green()));
}

pub fn export_postman(output: &str, name: &str, version: &str) {
    if !guard_project() {
        return;
    }
    println!("{}", "📮 Scanning project routes for Postman collection export...".bright_cyan());

    let routes = scan_routes_in_src();
    println!("   Found {} endpoints.", routes.len().to_string().yellow());

    let mut items = Vec::new();
    for r in &routes {
        let path_segments: Vec<&str> = r.path.trim_matches('/').split('/').collect();
        let mut item = serde_json::json!({
            "name": r.summary.clone().unwrap_or_else(|| format!("{} {}", r.method, r.path)),
            "request": {
                "method": r.method,
                "header": [
                    { "key": "Accept", "value": "application/json", "type": "text" },
                    { "key": "Content-Type", "value": "application/json", "type": "text" }
                ],
                "url": {
                    "raw": format!("{{{{base_url}}}}{}", r.path),
                    "host": ["{{base_url}}"],
                    "path": path_segments
                },
                "description": r.description.clone().unwrap_or_default()
            },
            "response": []
        });

        if matches!(r.method.as_str(), "POST" | "PUT" | "PATCH") {
            item["request"]["body"] = serde_json::json!({
                "mode": "raw",
                "raw": "{\n  \n}",
                "options": {
                    "raw": { "language": "json" }
                }
            });
        }
        items.push(item);
    }

    let postman = serde_json::json!({
        "info": {
            "_postman_id": uuid::Uuid::new_v4().to_string(),
            "name": name,
            "description": format!("Collection exported from Lumina project (v{})", version),
            "schema": "https://schema.getpostman.com/json/collection/v2.1.0/collection.json"
        },
        "item": items,
        "variable": [
            {
                "key": "base_url",
                "value": "http://127.0.0.1:3000",
                "type": "string"
            }
        ]
    });

    let json = serde_json::to_string_pretty(&postman).unwrap_or_default();
    if let Err(e) = fs::write(output, json) {
        println!("  ❌ Failed to write {}: {}", output, e);
        return;
    }

    print_success(&format!("Postman collection exported successfully at {}", output.bright_green()));
}

// ─────────────────────────────────────────────────────────────────────────────
//  AI CONTEXT & SKILLS GENERATORS
// ─────────────────────────────────────────────────────────────────────────────

fn to_pascal_case(s: &str) -> String {
    let clean = s.trim().replace('-', "_");
    clean
        .split('_')
        .filter(|part| !part.is_empty())
        .map(capitalize_first)
        .collect()
}

fn to_snake_name(s: &str) -> String {
    let mut result = String::new();
    for (i, c) in s.trim().chars().enumerate() {
        if c == '-' || c == ' ' {
            result.push('_');
        } else if c.is_uppercase() {
            if i > 0 && !result.ends_with('_') {
                result.push('_');
            }
            result.push(c.to_ascii_lowercase());
        } else {
            result.push(c);
        }
    }
    result
}

pub fn generate_ai_context(target: &str, force: bool) {
    generate_ai_context_at(Path::new("."), target, force);
}

pub fn generate_ai_context_at(base: &Path, target: &str, force: bool) {
    if !base.join("Cargo.toml").exists() || !base.join("src/app").exists() {
        println!(
            "{}",
            "❌ Error: This command must be run inside a Lumina project root."
                .red()
                .bold()
        );
        return;
    }

    let app_name = fs::read_to_string(base.join("Cargo.toml"))
        .ok()
        .and_then(|content| {
            for line in content.lines() {
                let trimmed = line.trim();
                if trimmed.starts_with("name =") {
                    let parts: Vec<&str> = trimmed.split('=').collect();
                    if parts.len() >= 2 {
                        let name = parts[1].trim().trim_matches('"').trim_matches('\'');
                        return Some(name.to_string());
                    }
                }
            }
            None
        })
        .unwrap_or_else(|| "LuminaApp".to_string());

    println!(
        "{}",
        format!("🤖 Syncing AI Agent Context for [{}]...", app_name)
            .bright_purple()
            .bold()
    );

    let target_lower = target.to_lowercase();
    let generate_all = target_lower == "all" || target_lower.is_empty();

    let mut generated_count = 0;

    let write_file_safe = |rel_path: &str, content: String, label: &str| -> bool {
        let p = base.join(rel_path);
        if p.exists() && !force {
            println!(
                "  {} {} already exists. (Use {} to overwrite)",
                "⚠️ ".yellow(),
                rel_path.bright_yellow(),
                "--force".bright_cyan()
            );
            return false;
        }

        if let Some(parent) = p.parent() {
            if !parent.exists() {
                let _ = fs::create_dir_all(parent);
            }
        }

        if let Err(e) = fs::write(&p, content) {
            println!("  ❌ Failed to write {}: {}", rel_path, e);
            false
        } else {
            print_success(&format!("{} [{}]", rel_path.bright_green(), label));
            true
        }
    };

    if generate_all || target_lower == "agents" || target_lower == "universal" {
        if write_file_safe(
            "AGENTS.md",
            crate::ai_context::generate_agents_md(&app_name),
            "Universal AI Standard",
        ) {
            generated_count += 1;
        }
    }

    if generate_all || target_lower == "cursor" {
        if write_file_safe(
            ".cursorrules",
            crate::ai_context::generate_cursorrules(&app_name),
            "Cursor IDE Agent",
        ) {
            generated_count += 1;
        }
    }

    if generate_all || target_lower == "copilot" || target_lower == "github" {
        if write_file_safe(
            ".github/copilot-instructions.md",
            crate::ai_context::generate_copilot_instructions(&app_name),
            "GitHub Copilot Workspace",
        ) {
            generated_count += 1;
        }
    }

    if generate_all || target_lower == "claude" {
        if write_file_safe(
            "CLAUDE.md",
            crate::ai_context::generate_claude_md(&app_name),
            "Anthropic Claude Code",
        ) {
            generated_count += 1;
        }
    }

    println!(
        "\n  ✨ Synchronized {} AI context file(s) successfully!",
        generated_count
    );
    print_hint("Open your project in Cursor, Copilot, Antigravity, or Claude Code to start coding with zero hallucinations.");
}

pub fn generate_ai_skill(name: &str, fields: &[String], provider: &str) {
    generate_ai_skill_at(Path::new("."), name, fields, provider);
}

pub fn generate_ai_skill_at(base: &Path, name: &str, fields: &[String], _provider: &str) {
    if !base.join("Cargo.toml").exists() || !base.join("src/app").exists() {
        println!(
            "{}",
            "❌ Error: This command must be run inside a Lumina project root."
                .red()
                .bold()
        );
        return;
    }

    let pascal_name = to_pascal_case(name);
    let snake_name = to_snake_name(name);

    println!(
        "{}",
        format!("🧠 Generating AI Skill & Tool Stub for [{}]...", pascal_name)
            .bright_purple()
            .bold()
    );

    let is_reverse_engineer = snake_name.contains("reverse");

    // Parse fields or use default
    let parsed_fields: Vec<(String, String)> = if is_reverse_engineer {
        vec![
            ("target_url".to_string(), "String".to_string()),
            ("html_source".to_string(), "String".to_string()),
            ("component_name".to_string(), "String".to_string()),
            ("extract_reactive".to_string(), "bool".to_string()),
        ]
    } else if fields.is_empty() {
        vec![
            ("query".to_string(), "String".to_string()),
            ("limit".to_string(), "i64".to_string()),
        ]
    } else {
        fields
            .iter()
            .map(|f| {
                let parts: Vec<&str> = f.split(':').collect();
                if parts.len() == 2 {
                    let field_name = parts[0].to_string();
                    let field_type = match parts[1].to_lowercase().as_str() {
                        "int" | "i64" | "integer" => "i64",
                        "float" | "f64" | "double" => "f64",
                        "bool" | "boolean" => "bool",
                        _ => "String",
                    };
                    (field_name, field_type.to_string())
                } else {
                    (f.clone(), "String".to_string())
                }
            })
            .collect()
    };

    // 1. Ensure directories exist
    let tools_dir = base.join("app/ai/tools");
    let skills_dir = base.join(format!("app/ai/skills/{}", snake_name));
    let service_dir = base.join("src/app/services/ai");

    fs::create_dir_all(&tools_dir).ok();
    fs::create_dir_all(&skills_dir).ok();
    fs::create_dir_all(&service_dir).ok();

    // 2. Build JSON Schema properties
    let mut props_map = serde_json::Map::new();
    let mut required_fields = Vec::new();

    for (fname, ftype) in &parsed_fields {
        let json_type = match ftype.as_str() {
            "i64" => "integer",
            "f64" => "number",
            "bool" => "boolean",
            _ => "string",
        };
        props_map.insert(
            fname.clone(),
            serde_json::json!({
                "type": json_type,
                "description": format!("Parameter {}", fname)
            }),
        );
        required_fields.push(fname.clone());
    }

    let description = if is_reverse_engineer {
        "Reverse engineers a target web page, DOM structure, or API endpoint into idiomatic Lumina Blade views, Tailwind styling, Lumina Live reactive components, and Eloquent models."
    } else {
        "Executes business logic and queries database for the requested action."
    };

    let universal_tool = serde_json::json!({
        "name": snake_name,
        "description": description,
        "openai_tool": {
            "type": "function",
            "function": {
                "name": snake_name,
                "description": description,
                "parameters": {
                    "type": "object",
                    "properties": props_map,
                    "required": required_fields
                }
            }
        },
        "gemini_declaration": {
            "name": snake_name,
            "description": description,
            "parameters": {
                "type": "OBJECT",
                "properties": props_map,
                "required": required_fields
            }
        },
        "claude_tool": {
            "name": snake_name,
            "description": description,
            "input_schema": {
                "type": "object",
                "properties": props_map,
                "required": required_fields
            }
        }
    });

    let tool_json_path = tools_dir.join(format!("{}.json", snake_name));
    let _ = fs::write(
        &tool_json_path,
        serde_json::to_string_pretty(&universal_tool).unwrap_or_default(),
    );
    print_success(&format!("Universal Tool Schema: {}", tool_json_path.display().to_string().bright_green()));

    // 3. Generate SKILL.md for Antigravity & Cursor Agent
    let skill_md_content = if is_reverse_engineer {
        format!(
            r#"---
name: {snake_name}
description: {description}
---

# 🛠️ AI Skill: Lumina UI & System Reverse Engineer

Skill ini digunakan ketika pengguna ingin meniru, mereplikasi, atau merekayasa balik (*reverse-engineer*) website, tampilan HTML, atau alur aplikasi target ke dalam Lumina Framework.

## 📋 Langkah Eksekusi Reverse Engineering
1. **Analisis DOM & Visual Target**:
   - Dekonstruksi layout menjadi template Blade di `resources/views/`.
   - Gunakan utility classes Tailwind CSS murni.
2. **Rekonstruksi Interaktivitas (Lumina Live)**:
   - Jika terdapat pencarian instan, filter live, atau modal dinamis: buat komponen Lumina Live di `src/app/live/` dengan atribut `kilat:*` atau `wire:*`.
3. **Penyusunan Skema Data & ORM**:
   - Bentuk Model `#[derive(LuminaModel)]` di `src/app/models/`.
   - Buat migrasi SQL yang sesuai di `database/migrations/`.
4. **Penyusunan Controller**:
   - Wajib menggunakan satu parameter tunggal: `req: Request`.
   - Kembalikan `View::make("...").render(req).await` untuk web atau `req.ok()` untuk REST API.
"#
        )
    } else {
        format!(
            r#"---
name: {snake_name}
description: {description}
---

# 🛠️ AI Skill: {pascal_name}

Gunakan skill ini untuk memanggil tool `{snake_name}` dengan parameter terverifikasi pada Lumina Framework.
"#
        )
    };

    let skill_md_path = skills_dir.join("SKILL.md");
    let _ = fs::write(&skill_md_path, skill_md_content);
    print_success(&format!("Agent Skill Spec: {}", skill_md_path.display().to_string().bright_green()));

    // 4. Generate Rust Safe Service Handler Stub
    let mut struct_fields = String::new();
    for (fname, ftype) in &parsed_fields {
        struct_fields.push_str(&format!("    pub {}: {},\n", fname, ftype));
    }

    let rust_service_code = if is_reverse_engineer {
        format!(
            r#"use pakem::prelude::*;
use serde::{{Deserialize, Serialize}};
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct {pascal_name}Input {{
{struct_fields}}}

pub struct {pascal_name}Tool;

impl {pascal_name}Tool {{
    /// Execute reverse engineering of target page/component into Lumina code
    pub async fn execute(_db: &DatabasePool, input: {pascal_name}Input) -> Result<serde_json::Value, AppError> {{
        tracing::info!("Running AI Reverse Engineering for component: {{}}", input.component_name);

        // Return hasil dekonstruksi arsitektur Lumina
        let output = serde_json::json!({{
            "status": "success",
            "component": input.component_name,
            "target_url": input.target_url,
            "blade_view_hint": format!("resources/views/components/{{}}.blade.rs", input.component_name.to_lowercase()),
            "message": "Target page analyzed successfully. Ready to generate Blade view and Lumina Live components."
        }});

        Ok(output)
    }}
}}
"#
        )
    } else {
        format!(
            r#"use pakem::prelude::*;
use serde::{{Deserialize, Serialize}};
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct {pascal_name}Input {{
{struct_fields}}}

pub struct {pascal_name}Tool;

impl {pascal_name}Tool {{
    /// Execute tool action safely against database and application services
    pub async fn execute(_db: &DatabasePool, input: {pascal_name}Input) -> Result<serde_json::Value, AppError> {{
        tracing::info!("Executing AI Tool {pascal_name} with input: {{:?}}", input);

        let result = serde_json::json!({{
            "status": "success",
            "data": "Operation executed successfully"
        }});

        Ok(result)
    }}
}}
"#
        )
    };

    let service_file_path = service_dir.join(format!("{}_tool.rs", snake_name));
    let _ = fs::write(&service_file_path, rust_service_code);
    print_success(&format!("Rust Tool Handler: {}", service_file_path.display().to_string().bright_green()));

    println!("\n  ✨ AI Skill & Tool [{}] ready for Gemini, Claude, OpenAI & Antigravity!", pascal_name);
    print_hint(&format!("AI assistants can now call `{}` function safely through your Lumina services.", snake_name));
}
