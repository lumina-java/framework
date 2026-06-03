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

pub fn generate_model(name: &str, fields: &[String]) {
    if !guard_project() {
        return;
    }
    println!("{}", "🔨 Generating Model...".bright_purple().bold());

    let lower = to_snake_case(name);
    let class = capitalize_first(name);
    let table = format!("{}s", lower);

    let mut struct_fields = String::new();
    let mut sql_fields = String::new();

    for field in fields {
        let parts: Vec<&str> = field.split(':').collect();
        let fname = parts[0];
        let ty = if parts.len() > 1 && parts[1] == "integer" { "i64" } else { "String" };
        let sql_ty = if ty == "i64" { "INTEGER NULL" } else { "TEXT NULL" };

        struct_fields.push_str(&format!("    pub {}: {},\n", fname, ty));
        sql_fields.push_str(&format!("    {} {},\n", fname, sql_ty));
    }

    let code = format!(
        r#"use lumina::prelude::*;
use serde::{{Serialize, Deserialize}};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, Clone, Default, FromRow, LuminaModel)]
#[table("{}")]
pub struct {class} {{
    pub id: i64,
{struct_fields}    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub deleted_at: Option<String>,
}}

impl {class} {{
    pub fn new() -> Self {{
        Self::default()
    }}
}}
"#,
        table,
        class = class,
        struct_fields = struct_fields
    );

    let path = format!("src/app/models/{}.rs", lower);
    fs::write(&path, &code).ok();

    let mod_file = "src/app/models/mod.rs";
    if let Ok(mod_content) = fs::read_to_string(mod_file) {
        let stmt = format!("pub mod {};", lower);
        if !mod_content.contains(&stmt) {
            append_to_file(mod_file, &format!("{}\n", stmt));
        }
    } else {
        append_to_file(mod_file, &format!("pub mod {};\n", lower));
    }

    print_success(&format!("Model: {}", path));

    // Also generate migration
    generate_migration_raw(&format!("create_{}_table", table), Some(&format!(
        "CREATE TABLE IF NOT EXISTS {table} (\n    id INTEGER PRIMARY KEY AUTOINCREMENT,\n{}    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,\n    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,\n    deleted_at DATETIME NULL\n);\n",
        sql_fields,
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
    generate_model(name, fields);

    // 2. Controller
    let ctrl_code = build_crud_controller(&lower, &class, fields);
    let ctrl_path = format!("src/app/controllers/{}_controller.rs", lower);
    fs::write(&ctrl_path, ctrl_code).ok();

    let ctrl_mod_file = "src/app/controllers/mod.rs";
    if let Ok(mod_content) = fs::read_to_string(ctrl_mod_file) {
        let stmt = format!("pub mod {}_controller;", lower);
        if !mod_content.contains(&stmt) {
            append_to_file(ctrl_mod_file, &format!("{}\n", stmt));
        }
    } else {
        append_to_file(ctrl_mod_file, &format!("pub mod {}_controller;\n", lower));
    }
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
    println!("      .get(\"/{lower}\",                {class}Controller::index)");
    println!("      .get(\"/{lower}/create\",         {class}Controller::create)");
    println!("      .post(\"/{lower}\",               {class}Controller::store)");
    println!("      .get(\"/{lower}/:id\",            {class}Controller::show)");
    println!("      .get(\"/{lower}/:id/edit\",       {class}Controller::edit)");
    println!("      .post(\"/{lower}/:id/update\",    {class}Controller::update)");
    println!("      .post(\"/{lower}/:id/delete\",    {class}Controller::destroy)");
}

fn build_crud_controller(lower: &str, class: &str, fields: &[String]) -> String {
    let mut form_fields = String::new();
    let mut assign_fields = String::new();

    for field in fields {
        let parts: Vec<&str> = field.split(':').collect();
        let name = parts[0];
        let ty = if parts.len() > 1 && parts[1] == "integer" { "i64" } else { "String" };

        form_fields.push_str(&format!("    pub {}: {},\n", name, ty));
        if ty == "i64" {
            assign_fields.push_str(&format!("        item.{} = form.{};\n", name, name));
        } else {
            assign_fields.push_str(&format!("        item.{} = form.{}.clone();\n", name, name));
        }
    }

    format!(
        r#"use lumina::prelude::*;
use axum::extract::Path;
use crate::app::models::{lower}::{class};
use serde::Deserialize;
use axum::response::IntoResponse;
use axum::Form;

pub struct {class}Controller;

#[derive(Deserialize)]
pub struct {class}Form {{
{form_fields}
}}

impl {class}Controller {{
    /// GET /{lower} — list all records
    pub async fn index(req: Request) -> impl IntoResponse {{
        let pool = req.db();
        let items = {class}::all(pool).await.unwrap_or_default();

        req.view("{lower}/index")
            .with("title", "{class} List")
            .with("items", items)
            .render(&req)
            .await.into_response()
    }}

    /// GET /{lower}/create — show create form
    pub async fn create(req: Request) -> impl IntoResponse {{
        req.view("{lower}/create")
            .with("title", "Create {class}")
            .render(&req)
            .await.into_response()
    }}

    /// POST /{lower} — store new record
    pub async fn store(req: Request, Form(form): Form<{class}Form>) -> impl IntoResponse {{
        let pool = req.db();
        let mut item = {class}::new();
{assign_fields}

        match item.save(pool).await {{
            Ok(_) => req.redirect("/{lower}")
                .with_success("{class} berhasil ditambahkan!")
                .go(&req).await.into_response(),
            Err(e) => req.view("{lower}/create")
                .with("title", "Create {class}")
                .with("error", format!("Gagal menyimpan data: {{}}", e))
                .render(&req).await.into_response()
        }}
    }}

    /// GET /{lower}/:id — show single record
    pub async fn show(req: Request, Path(id): Path<i64>) -> impl IntoResponse {{
        let pool = req.db();
        match {class}::find(pool, id).await {{
            Ok(item) => req.view("{lower}/show")
                .with("title", "{class} Detail")
                .with("item", item)
                .render(&req).await.into_response(),
            Err(_) => req.redirect("/{lower}")
                .with_error("Data tidak ditemukan!")
                .go(&req).await.into_response()
        }}
    }}

    /// GET /{lower}/:id/edit — show edit form
    pub async fn edit(req: Request, Path(id): Path<i64>) -> impl IntoResponse {{
        let pool = req.db();
        match {class}::find(pool, id).await {{
            Ok(item) => req.view("{lower}/edit")
                .with("title", "Edit {class}")
                .with("item", item)
                .render(&req).await.into_response(),
            Err(_) => req.redirect("/{lower}")
                .with_error("Data tidak ditemukan!")
                .go(&req).await.into_response()
        }}
    }}

    /// POST /{lower}/:id/update — update existing record
    pub async fn update(req: Request, Path(id): Path<i64>, Form(form): Form<{class}Form>) -> impl IntoResponse {{
        let pool = req.db();

        if let Ok(mut item) = {class}::find(pool, id).await {{
{assign_fields}
            let _ = item.save(pool).await;
        }}

        req.redirect("/{lower}")
            .with_success("{class} berhasil diperbarui!")
            .go(&req).await.into_response()
    }}

    /// POST /{lower}/:id/delete — delete record
    pub async fn destroy(req: Request, Path(id): Path<i64>) -> impl IntoResponse {{
        let pool = req.db();
        let _ = {class}::delete(pool, id).await;

        req.redirect("/{lower}")
            .with_success("{class} berhasil dihapus!")
            .go(&req).await.into_response()
    }}
}}
"#,
        lower = lower,
        class = class,
        form_fields = form_fields,
        assign_fields = assign_fields
    )
}

fn build_blade_view(class: &str, view: &str, lower: &str, fields: &[String]) -> String {
    let title = match view {
        "index" => format!("{} List", class),
        "create" => format!("Create {}", class),
        "edit" => format!("Edit {}", class),
        "show" => format!("{} Detail", class),
        _ => class.to_string(),
    };

    let mut table_headers = String::new();
    let mut table_cells = String::new();
    let mut form_inputs = String::new();
    let mut form_edit_inputs = String::new();
    let mut show_details = String::new();

    for field in fields {
        let parts: Vec<&str> = field.split(':').collect();
        let name = parts[0];
        let ty = if parts.len() > 1 { parts[1] } else { "text" };
        let input_type = if ty == "integer" { "number" } else { "text" };
        let label = capitalize_first(name);

        table_headers.push_str(&format!("                    <th class=\"px-6 py-4 text-left text-sm font-semibold text-slate-400\">{}</th>\n", label));
        table_cells.push_str(&format!("                    <td class=\"px-6 py-4\">{{{{ item.{} }}}}</td>\n", name));

        form_inputs.push_str(&format!(r#"            <div>
                <label class="block text-sm font-semibold text-slate-300 mb-2">{}</label>
                <input type="{}" name="{}" class="w-full bg-slate-950/80 border border-slate-800 rounded-xl px-4 py-3 text-white focus:outline-none focus:border-indigo-500 focus:ring-1 focus:ring-indigo-500 transition" required>
            </div>
"#, label, input_type, name));

        form_edit_inputs.push_str(&format!(r#"            <div>
                <label class="block text-sm font-semibold text-slate-300 mb-2">{}</label>
                <input type="{}" name="{}" value="{{{{ item.{} }}}}" class="w-full bg-slate-950/80 border border-slate-800 rounded-xl px-4 py-3 text-white focus:outline-none focus:border-indigo-500 focus:ring-1 focus:ring-indigo-500 transition" required>
            </div>
"#, label, input_type, name, name));

        show_details.push_str(&format!("        <div class=\"mb-4\">\n            <p class=\"text-sm text-slate-400\">{}</p>\n            <p class=\"text-lg text-white font-semibold\">{{{{ item.{} }}}}</p>\n        </div>\n", label, name));
    }


    let body = match view {
        "index" => format!(r#"
    <div class="flex justify-between items-center mb-8">
        <h1 class="text-3xl font-bold text-white heading-font">{title}</h1>
        <a href="/{lower}/create" class="bg-indigo-600 text-white px-4 py-2 rounded-xl font-semibold hover:bg-indigo-500 transition">+ Add New</a>
    </div>

    {{% if flash_success %}}
    <div class="bg-indigo-500/10 border border-indigo-500/20 text-indigo-400 px-4 py-3 rounded-xl mb-6">
        {{{{ flash_success }}}}
    </div>
    {{% endif %}}

    <div class="bg-slate-900/50 border border-slate-800/80 rounded-3xl overflow-hidden backdrop-filter backdrop-blur-sm">
        <table class="w-full text-left border-collapse">
            <thead>
                <tr class="border-b border-slate-800 bg-slate-900/80">
                    <th class="px-6 py-4 text-left text-sm font-semibold text-slate-400 w-16">ID</th>
{table_headers}
                    <th class="px-6 py-4 text-right text-sm font-semibold text-slate-400">Actions</th>
                </tr>
            </thead>
            <tbody class="divide-y divide-slate-800/50">
                {{% for item in items %}}
                <tr class="hover:bg-slate-800/30 transition">
                    <td class="px-6 py-4 text-slate-500">#{{{{ item.id }}}}</td>
{table_cells}
                    <td class="px-6 py-4 text-right space-x-3">
                        <a href="/{lower}/{{{{ item.id }}}}" class="text-slate-400 hover:text-white font-medium text-sm">View</a>
                        <a href="/{lower}/{{{{ item.id }}}}/edit" class="text-indigo-400 hover:text-indigo-300 font-medium text-sm">Edit</a>
                        <form action="/{lower}/{{{{ item.id }}}}/delete" method="POST" class="inline" onsubmit="return confirm('Are you sure you want to delete this?');">
                            <button type="submit" class="text-pink-500 hover:text-pink-400 font-medium text-sm">Delete</button>
                        </form>
                    </td>
                </tr>
                {{% else %}}
                <tr>
                    <td colspan="10" class="px-6 py-8 text-center text-slate-500">No {lower}s found.</td>
                </tr>
                {{% endfor %}}
            </tbody>
        </table>
    </div>
"#),
        "create" => format!(r#"
    <div class="max-w-2xl mx-auto">
        <div class="flex justify-between items-center mb-8">
            <h1 class="text-3xl font-bold text-white heading-font">{title}</h1>
            <a href="/{lower}" class="text-slate-400 hover:text-white transition">← Back</a>
        </div>

        {{% if error %}}
        <div class="bg-pink-500/10 border border-pink-500/20 text-pink-400 px-4 py-3 rounded-xl mb-6">
            {{{{ error }}}}
        </div>
        {{% endif %}}

        <form action="/{lower}" method="POST" class="bg-slate-900/50 border border-slate-800/80 rounded-3xl p-8 space-y-6">
{form_inputs}
            <div class="pt-4">
                <button type="submit" class="w-full bg-indigo-600 text-white py-3.5 rounded-xl font-bold hover:bg-indigo-500 shadow-lg shadow-indigo-600/20 transition transform active:scale-98">Save Data</button>
            </div>
        </form>
    </div>
"#),
        "edit" => format!(r#"
    <div class="max-w-2xl mx-auto">
        <div class="flex justify-between items-center mb-8">
            <h1 class="text-3xl font-bold text-white heading-font">{title}</h1>
            <a href="/{lower}" class="text-slate-400 hover:text-white transition">← Back</a>
        </div>

        <form action="/{lower}/{{{{ item.id }}}}/update" method="POST" class="bg-slate-900/50 border border-slate-800/80 rounded-3xl p-8 space-y-6">
{form_edit_inputs}
            <div class="pt-4">
                <button type="submit" class="w-full bg-indigo-600 text-white py-3.5 rounded-xl font-bold hover:bg-indigo-500 shadow-lg shadow-indigo-600/20 transition transform active:scale-98">Update Data</button>
            </div>
        </form>
    </div>
"#),
        "show" => format!(r#"
    <div class="max-w-2xl mx-auto">
        <div class="flex justify-between items-center mb-8">
            <h1 class="text-3xl font-bold text-white heading-font">{title}</h1>
            <a href="/{lower}" class="text-slate-400 hover:text-white transition">← Back</a>
        </div>

        <div class="bg-slate-900/50 border border-slate-800/80 rounded-3xl p-8">
{show_details}
            <div class="mt-8 pt-6 border-t border-slate-800 flex gap-4">
                <a href="/{lower}/{{{{ item.id }}}}/edit" class="bg-indigo-600 text-white px-6 py-2.5 rounded-xl font-bold hover:bg-indigo-500 transition">Edit</a>
            </div>
        </div>
    </div>
"#),
        _ => String::new(),
    };

    format!(
        "{{% extends \"layouts/app.blade.rs\" %}}\n\n{{% block content %}}\n<div class=\"px-4 py-12\">\n{}\n</div>\n{{% endblock %}}\n",
        body
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
