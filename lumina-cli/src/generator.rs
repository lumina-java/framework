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

    // 3. Bersihkan log lama di storage/logs
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
        r#"use lumina::prelude::*;
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
use lumina::prelude::*;
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
use lumina::prelude::*;
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
    ];

    for dir in view_dirs {
        let _ = fs::create_dir_all(dir);
    }

    let guest_layout = include_str!("../../resources/views/aurora/layouts/guest.blade.rs");
    let app_layout = include_str!("../../resources/views/aurora/layouts/app.blade.rs");
    let login_view = include_str!("../../resources/views/aurora/auth/login.blade.rs");
    let register_view = include_str!("../../resources/views/aurora/auth/register.blade.rs");
    let dashboard_view = include_str!("../../resources/views/aurora/dashboard/index.blade.rs");
    let profile_view = include_str!("../../resources/views/aurora/profile/edit.blade.rs");

    let auth_ctrl = include_str!("../../src/app/controllers/aurora/auth_controller.rs");
    let dash_ctrl = include_str!("../../src/app/controllers/aurora/dashboard_controller.rs");
    let prof_ctrl = include_str!("../../src/app/controllers/aurora/profile_controller.rs");
    let mod_ctrl = include_str!("../../src/app/controllers/aurora/mod.rs");

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
    ];

    for (path, content) in files {
        if let Err(e) = fs::write(path, content) {
            println!("  ❌ Failed to write {}: {}", path, e);
        } else {
            print_success(&format!("Generated {}", path));
        }
    }

    println!("\n{}", "🎉 Lumina Aurora Starter Kit installed successfully!".bright_green().bold());
    println!("{}", "Mount all Aurora routes in your router:".bright_cyan());
    println!("  {}", "router.merge(lumina::app::controllers::aurora::routes())".yellow());
}
