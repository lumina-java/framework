use std::fs;
use std::io;
use std::path::Path;

/// Generate universal `AGENTS.md` context instructions for any AI agent (Antigravity, Cline, Windsurf, ChatGPT, etc.)
pub fn generate_agents_md(app_name: &str) -> String {
    format!(
        r#"# 🤖 Lumina Framework — Developer & AI Agent Guidelines ({app_name})

> **Visi Inti Lumina:**  
> *"Look like Laravel, Run at Rust Speed. Cuantik, elegan, tercepat, dan menghadirkan pengalaman pengembangan (Developer Experience) paling menyenangkan di dunia backend."*

Dokumen ini adalah pedoman mutlak (**Rules & Standards**) bagi seluruh AI Agent (Antigravity, Cursor, Copilot, Claude Code, ChatGPT, dsb.) dan developer yang menulis, mengedit, atau melakukan maintenance kode pada proyek **{app_name}**.

---

## 🏛️ 1. Prinsip Utama Desain Kode

1. **Laravel Elegance**: Sintaks harus ekspresif, ringkas, bersih (*human-readable*), dan deklaratif. Hindari kode bertele-tele (*boilerplate*) yang menghambat kecepatan berpikir developer.
2. **Rust Raw Speed & Safety**: Menggunakan Tokio async runtime, Axum engine, dan zero-cost abstractions tanpa Garbage Collector.
3. **Batteries-Included**: ORM, Blade view engine, autentikasi, router, WebSocket, scheduler, dan background job queue sudah terintegrasi secara harmonis.

---

## 🎮 2. Rules Mutlak untuk Controller (`src/app/controllers/`)

### A. Wajib Menggunakan Unified `Request` Bundle
- ❌ **DILARANG KERAS** menggunakan puluhan extractor mentah Axum (`State(state)`, `Path(id)`, `Query(q)`, `Session(s)`, `CsrfToken`, dsb.) pada controller aplikasi.
- ✅ **WAJIB** menggunakan satu parameter tunggal: `req: Request` (dari `pakem::prelude::*`).

### B. Standardisasi Return Type
- **Controller Halaman Web (HTML View)**:
  - **WAJIB** mengembalikan `impl IntoResponse`.
  - **WAJIB** merender view menggunakan `View::make("nama.view").with("kunci", nilai)...render(req).await` (Bebas tanda ampersand `&` yang merepotkan).
  - Contoh:
    ```rust
    pub async fn index(req: Request) -> impl IntoResponse {{
        let user = req.user.clone().expect("Unauthorized");
        let users = User::query(req.db()).with("posts").get().await.unwrap_or_default();

        View::make("dashboard.index")
            .with("title", "Dashboard — Lumina")
            .with("user", user)
            .with("users", users)
            .render(req)
            .await
    }}
    ```

- **Controller RESTful API (JSON)**:
  - **WAJIB** mengembalikan `LuminaResult` (alias untuk `Result<Response, AppError>`).
  - **WAJIB** memanfaatkan shortcut helper response bawaan `Request`:
    - `req.ok(data)` ➔ HTTP 200 `{{ "success": true, "data": ... }}`
    - `req.success(data, "Pesan sukses")` ➔ HTTP 200 `{{ "success": true, "message": "...", "data": ... }}`
    - `req.created_with(data, "Pesan dibuat")` ➔ HTTP 201 `{{ "success": true, "message": "...", "data": ... }}`
    - `req.paginate(paginator)` ➔ HTTP 200 dengan metadata paginasi lengkap
    - `req.not_found("Pesan error")` ➔ HTTP 404
    - `req.bad_request("Pesan error")` ➔ HTTP 400
    - `req.unauthorized("Pesan error")` ➔ HTTP 401
  - Contoh:
    ```rust
    pub async fn api_index(req: Request) -> LuminaResult {{
        let (page, per_page) = req.pagination();
        let paginator = Product::paginate(req.db(), page, per_page).await?;
        req.paginate(paginator)
    }}
    ```

### C. Validasi Input Otomatis
- **Form Web**: Gunakan `ValidatedForm<T>`. Jika validasi gagal, engine Lumina otomatis me-redirect balik dengan flash error dan input lama (*old input*).
- **REST API**: Gunakan `ValidatedJson<T>`. Jika validasi gagal, engine otomatis mengembalikan HTTP 422 Unprocessable Entity berformat JSON.

### D. Akses Data Kontekstual via `Request`
- Database: `req.db()` atau `req.db_arc()`
- User Login: `req.user` atau `req.user()`
- Path Param / ID: `let id = req.id_or_fail()?;` atau `req.param("id")`
- Query Param: `let q = req.query_val("search");`
- Paginasi: `let (page, per_page) = req.pagination();`
- Redirect: `req.redirect("/products").with_success("Berhasil disimpan!").go(&req).await`

---

## 🗄️ 3. Rules untuk Model & ORM (`src/app/models/`)

1. **Macro Derivation**:
   Setiap entitas model **WAJIB** menambahkan derive:
   ```rust
   #[derive(Debug, Serialize, Deserialize, FromRow, Clone, Default, LuminaModel)]
   #[table("products")]
   pub struct Product {{
       pub id: i64,
       pub name: String,
       pub price: f64,
       pub created_at: Option<String>,
       pub updated_at: Option<String>,
       pub deleted_at: Option<String>,
   }}
   ```
2. **Fluent Query Builder**:
   Gunakan query builder yang lancar ala Laravel:
   ```rust
   let active_items = Product::query(req.db())
       .where_eq("is_active", true)
       .where_gt("price", 10000.0)
       .order_by("created_at", "DESC")
       .limit(10)
       .get()
       .await?;
   ```
3. **One-Line Relationships**:
   ```rust
   has_many!(Comment, "product_id");
   belongs_to!(Category, "category_id");
   ```
4. **Cegah Masalah N+1 Query**:
   Gunakan `.with("relation_name")` saat mengambil data relasi untuk koleksi:
   ```rust
   let products = Product::query(req.db()).with("category").get().await?;
   ```

---

## 🎨 4. Rules untuk View (Blade Engine — `resources/views/`)

1. **Template Native Blade**:
   - File template bertipe `.blade.rs` dan disimpan di bawah folder `resources/views/`.
   - Hindari logika bisnis berat di dalam view. View hanya bertugas untuk presentasi.
2. **Pewarisan Layout**:
   - Gunakan `@extends('layouts.app')`, `@section('content')`, dan `@yield('content')`.
3. **Looping & Kontrol Alur**:
   - Gunakan `@foreach($items as $item)` dan manfaatkan objek `$loop` (`$loop.index`, `$loop.first`, `$loop.last`).
   - Gunakan `@if`, `@elseif`, `@else`, `@endif`.
   - Gunakan `@auth` / `@guest`.
4. **Form Security**:
   - Form method `POST` wajib menyertakan `@csrf`.
   - Gunakan `@method('PUT')` atau `@method('DELETE')` untuk spoofing HTTP method.

---

## ⚡ 5. Lumina Live (Reactive Server-Driven UI)

Jika membuat komponen interaktif tanpa menulis JavaScript/SPA:
- Gunakan direktif `kilat:*`, `wire:*`, atau `live:*`.
- Contoh: `<button wire:click="increment">+</button>` atau `<input wire:model="search" />`.
- Komponen backend mengimplementasikan trait `LuminaLiveComponent`.

---

## 📋 6. Standard CRUD Blueprint (Resep Baku Pembuatan Fitur)

Ketika diminta membuat fitur CRUD baru (misal: `Product`), ikuti 6 langkah terstruktur ini:

### Langkah 1: Migrasi Database (`database/migrations/`)
```sql
CREATE TABLE IF NOT EXISTS products (
    id INTEGER PRIMARY KEY AUTOINCREMENT, -- atau BIGSERIAL di Postgres
    name VARCHAR(255) NOT NULL,
    price DOUBLE PRECISION NOT NULL DEFAULT 0,
    description TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP NULL
);
```

### Langkah 2: Model (`src/app/models/product.rs`)
```rust
use pakem::prelude::*;

#[derive(Debug, Serialize, Deserialize, FromRow, Clone, Default, LuminaModel)]
#[table("products")]
pub struct Product {{
    pub id: i64,
    pub name: String,
    pub price: f64,
    pub description: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub deleted_at: Option<String>,
}}
```

### Langkah 3: Request Validation (`src/app/requests/product_request.rs`)
```rust
use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct CreateProductRequest {{
    #[validate(length(min = 3, message = "Nama produk minimal 3 karakter"))]
    pub name: String,
    #[validate(range(min = 0.0, message = "Harga tidak boleh minus"))]
    pub price: f64,
    pub description: Option<String>,
}}
```

### Langkah 4: Controller 7 RESTful Actions (`src/app/controllers/product_controller.rs`)
```rust
use pakem::prelude::*;
use crate::app::models::product::Product;
use crate::app::requests::product_request::CreateProductRequest;

pub struct ProductController;

impl ProductController {{
    // 1. GET /products (Index)
    pub async fn index(req: Request) -> impl IntoResponse {{
        let products = Product::query(req.db()).order_by("id", "DESC").get().await.unwrap_or_default();
        View::make("products.index").with("products", products).render(req).await
    }}

    // 2. GET /products/create (Form Tambah)
    pub async fn create(req: Request) -> impl IntoResponse {{
        View::make("products.create").render(req).await
    }}

    // 3. POST /products (Simpan)
    pub async fn store(req: Request) -> impl IntoResponse {{
        let form = match req.validate_form::<CreateProductRequest>().await {{
            Ok(f) => f,
            Err(e) => return req.redirect("/products/create").with_errors(e).with_input().go(&req).await,
        }};

        let new_item = Product {{
            name: form.name,
            price: form.price,
            description: form.description,
            ..Default::default()
        }};

        if let Err(e) = new_item.save(req.db()).await {{
            return req.redirect("/products/create").with_error(format!("Gagal: {{}}", e)).go(&req).await;
        }}

        req.redirect("/products").with_success("Produk berhasil ditambahkan!").go(&req).await
    }}

    // 4. GET /products/:id (Detail)
    pub async fn show(req: Request) -> impl IntoResponse {{
        let id: i64 = req.id_or_fail().unwrap_or(0);
        let product = Product::find(req.db(), id).await.ok().flatten();
        View::make("products.show").with("product", product).render(req).await
    }}

    // 5. GET /products/:id/edit (Form Edit)
    pub async fn edit(req: Request) -> impl IntoResponse {{
        let id: i64 = req.id_or_fail().unwrap_or(0);
        let product = Product::find(req.db(), id).await.ok().flatten();
        View::make("products.edit").with("product", product).render(req).await
    }}

    // 6. PUT /products/:id (Update)
    pub async fn update(req: Request) -> impl IntoResponse {{
        let id: i64 = req.id_or_fail().unwrap_or(0);
        let form = match req.validate_form::<CreateProductRequest>().await {{
            Ok(f) => f,
            Err(e) => return req.redirect(&format!("/products/{{}}/edit", id)).with_errors(e).go(&req).await,
        }};

        let mut product = match Product::find(req.db(), id).await.ok().flatten() {{
            Some(p) => p,
            None => return req.redirect("/products").with_error("Produk tidak ditemukan!").go(&req).await,
        }};

        product.name = form.name;
        product.price = form.price;
        product.description = form.description;
        let _ = product.save(req.db()).await;

        req.redirect("/products").with_success("Produk berhasil diperbarui!").go(&req).await
    }}

    // 7. DELETE /products/:id (Hapus)
    pub async fn destroy(req: Request) -> impl IntoResponse {{
        let id: i64 = req.id_or_fail().unwrap_or(0);
        if let Ok(Some(product)) = Product::find(req.db(), id).await {{
            let _ = product.delete(req.db()).await;
        }}
        req.redirect("/products").with_success("Produk berhasil dihapus!").go(&req).await
    }}

    // --- RESTful API Handlers (JSON) ---
    pub async fn api_index(req: Request) -> LuminaResult {{
        let (page, per_page) = req.pagination();
        let paginator = Product::paginate(req.db(), page, per_page).await?;
        req.paginate(paginator)
    }}

    pub async fn api_store(req: Request) -> LuminaResult {{
        let payload = req.validate_json::<CreateProductRequest>().await?;
        let product = Product {{
            name: payload.name,
            price: payload.price,
            description: payload.description,
            ..Default::default()
        }};
        let saved = product.save(req.db()).await?;
        req.created_with(saved, "Produk berhasil dibuat")
    }}
}}
```

### Langkah 5: Registrasi Route (`src/routes/web.rs` & `api.rs`)
```rust
// Web Routes (src/routes/web.rs)
router = router
    .route("/products", get(ProductController::index).post(ProductController::store))
    .route("/products/create", get(ProductController::create))
    .route("/products/:id", get(ProductController::show).put(ProductController::update).delete(ProductController::destroy))
    .route("/products/:id/edit", get(ProductController::edit));

// API Routes (src/routes/api.rs)
router = router
    .route("/api/products", get(ProductController::api_index).post(ProductController::api_store));
```

---

## 🚫 7. Larangan Keras (Strict Prohibitions)

1. ❌ **DILARANG** mengembalikan boilerplate Axum mentah di Controller (`State(state)`, `Path(id)`). Selalu gunakan parameter tunggal `req: Request`.
2. ❌ **DILARANG** menulis query mentah SQL yang rentan SQL Injection — gunakan query builder atau parameterized statement.
3. ❌ **DILARANG** menggunakan `unwrap()` atau `expect()` secara sembarangan di handler publik — selalu tangani error dengan operator `?` atau `AppError`.

---

*Lumina Framework: Keringat performa Rust di balik keindahan dan kemudahan Laravel.*
"#,
        app_name = app_name
    )
}

/// Generate `.cursorrules` optimized for Cursor IDE AI Agent and Composer
pub fn generate_cursorrules(app_name: &str) -> String {
    format!(
        r#"# Cursor Rules for Lumina Framework ({app_name})
# Philosophy: "Look like Laravel, Run at Rust Speed"

## Core Controller Rules
- ALWAYS use a single parameter for controllers: `req: Request` from `pakem::prelude::*`.
- NEVER use raw Axum extractors (`State(...)`, `Path(...)`, `Query(...)`, `Form(...)`) in application controllers.
- Web HTML Controllers:
  - Return `impl IntoResponse`.
  - Render with `View::make("view.name").with("key", val).render(req).await`.
- REST API Controllers:
  - Return `LuminaResult` (alias for `Result<Response, AppError>`).
  - Use helpers: `req.ok(data)`, `req.created_with(data, msg)`, `req.paginate(paginator)`, `req.not_found(msg)`.

## Model & ORM Rules
- Entities MUST derive: `#[derive(Debug, Serialize, Deserialize, FromRow, Clone, Default, LuminaModel)]`.
- Entities MUST specify table: `#[table("table_name")]`.
- Always use the fluent query builder: `Model::query(req.db()).where_eq("col", val).get().await?`.
- Relationships: `has_many!(Child, "parent_id");`, `belongs_to!(Parent, "parent_id");`.
- Eager load with `.with("relation")` to prevent N+1 queries.

## Validation Rules
- Web forms: `req.validate_form::<FormType>().await` or `ValidatedForm<T>`.
- API endpoints: `req.validate_json::<JsonType>().await` or `ValidatedJson<T>`.

## Blade Views & Livewire
- Files in `resources/views/` must end in `.blade.rs`.
- Form methods POST require `@csrf`.
- Reactive UI components use Lumina Live directives: `kilat:*`, `wire:*`, `live:*`.

## CLI Commands
- `lumina serve` : Run app
- `lumina watch` : Run app with hot auto-reload
- `lumina tinker` : Interactive REPL shell
- `lumina migrate` : Run database migrations
- `lumina make:crud <Name>` : Scaffold complete CRUD
- `lumina make:ai-context` : Refresh AI instructions
"#
    )
}

/// Generate `.github/copilot-instructions.md` for GitHub Copilot Workspace
pub fn generate_copilot_instructions(app_name: &str) -> String {
    format!(
        r#"# GitHub Copilot Instructions for {app_name} (Lumina Framework)

You are an expert Lumina Framework developer. Lumina is an enterprise Rust web framework that combines Laravel's DX and elegance with Rust's raw speed and memory safety.

## Architectural Mandates
1. **Controllers**:
   - Single signature: `pub async fn handler_name(req: Request) -> impl IntoResponse` (or `-> LuminaResult` for JSON APIs).
   - Do NOT suggest raw Axum extractors like `State(state)`, `Path(id)`. Access context via `req.db()`, `req.id_or_fail()?`, `req.user()`, `req.query_val()`.
   - Render views with `View::make("path").with("key", value).render(req).await`.
   - Return API responses with `req.ok(data)`, `req.paginate(paginator)`, or `req.created_with(data, msg)`.

2. **Models & Database**:
   - Must derive `LuminaModel`: `#[derive(Debug, Serialize, Deserialize, FromRow, Clone, Default, LuminaModel)]`.
   - Fluent queries: `User::query(req.db()).where_eq("is_active", true).get().await?`.
   - Relationship macros: `has_many!(Post, "user_id")` and `belongs_to!(User, "user_id")`.

3. **Views**:
   - Blade engine templates in `resources/views/**/*.blade.rs`.
   - Use `@extends('layouts.app')`, `@section('content')`, `@yield('content')`, `@csrf`.

4. **Safety & Style**:
   - Avoid raw `unwrap()` in production handlers; use `?` operator or return `AppError`.
   - Avoid raw SQL strings vulnerable to injection; use Lumina query builder or prepared parameters.
"#
    )
}

/// Generate `CLAUDE.md` for Anthropic Claude Code CLI
pub fn generate_claude_md(app_name: &str) -> String {
    format!(
        r#"# CLAUDE.md — {app_name} (Lumina Framework)

This document guides Claude Code when working in this Lumina repository.

## Commands
- Run server: `lumina serve` (or `cargo run`)
- Watch server (auto-reload): `lumina watch`
- Interactive REPL shell: `lumina tinker`
- Database migrations: `lumina migrate` (rollback: `lumina migrate:rollback`)
- Database seeding: `lumina db:seed`
- Generate CRUD: `lumina make:crud <ModelName>`
- Run tests: `cargo test`

## Architecture & Code Conventions
- **Controller signature**: Always `pub async fn action(req: Request) -> impl IntoResponse` (Web) or `-> LuminaResult` (API). Never use raw Axum extractors.
- **ORM**: Derive `LuminaModel` on all models. Use `Model::query(req.db())` for queries.
- **Views**: Blade templates live in `resources/views/` as `.blade.rs`.
- **Validation**: `req.validate_form::<T>()` for HTML forms, `req.validate_json::<T>()` for REST APIs.
- **Error Handling**: Use `AppError` and the `?` operator instead of `unwrap()`.
"#
    )
}

/// Scaffold all universal AI context files in the given project base path
pub fn scaffold_ai_context(base: &Path, app_name: &str) -> io::Result<()> {
    // 1. Ensure .github folder exists
    let github_dir = base.join(".github");
    if !github_dir.exists() {
        fs::create_dir_all(&github_dir)?;
    }

    // 2. Write AGENTS.md
    fs::write(base.join("AGENTS.md"), generate_agents_md(app_name))?;

    // 3. Write .cursorrules
    fs::write(base.join(".cursorrules"), generate_cursorrules(app_name))?;

    // 4. Write .github/copilot-instructions.md
    fs::write(
        github_dir.join("copilot-instructions.md"),
        generate_copilot_instructions(app_name),
    )?;

    // 5. Write CLAUDE.md
    fs::write(base.join("CLAUDE.md"), generate_claude_md(app_name))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_agents_md_contains_lumina_rules_and_crud() {
        let content = generate_agents_md("TokoApp");
        assert!(content.contains("Lumina Framework — Developer & AI Agent Guidelines (TokoApp)"));
        assert!(content.contains("req: Request"));
        assert!(content.contains("LuminaModel"));
        assert!(content.contains("Standard CRUD Blueprint"));
        assert!(content.contains("View::make"));
        assert!(content.contains("req.paginate"));
        assert!(content.contains("DILARANG KERAS"));
    }

    #[test]
    fn test_generate_cursorrules_contains_rules() {
        let content = generate_cursorrules("TokoApp");
        assert!(content.contains("Cursor Rules for Lumina Framework (TokoApp)"));
        assert!(content.contains("ALWAYS use a single parameter for controllers: `req: Request`"));
        assert!(content.contains("NEVER use raw Axum extractors"));
    }

    #[test]
    fn test_generate_copilot_instructions_contains_mandates() {
        let content = generate_copilot_instructions("TokoApp");
        assert!(content.contains("GitHub Copilot Instructions for TokoApp (Lumina Framework)"));
        assert!(content.contains("Do NOT suggest raw Axum extractors"));
    }

    #[test]
    fn test_generate_claude_md_contains_commands_and_conventions() {
        let content = generate_claude_md("TokoApp");
        assert!(content.contains("CLAUDE.md — TokoApp (Lumina Framework)"));
        assert!(content.contains("lumina serve"));
        assert!(content.contains("lumina tinker"));
        assert!(content.contains("lumina make:crud"));
    }

    #[test]
    fn test_scaffold_ai_context_creates_all_files() {
        let temp_dir = std::env::temp_dir().join(format!("lumina_ai_test_{}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()));
        fs::create_dir_all(&temp_dir).expect("Failed to create temp dir");

        let res = scaffold_ai_context(&temp_dir, "TestApp");
        assert!(res.is_ok());

        assert!(temp_dir.join("AGENTS.md").exists());
        assert!(temp_dir.join(".cursorrules").exists());
        assert!(temp_dir.join(".github/copilot-instructions.md").exists());
        assert!(temp_dir.join("CLAUDE.md").exists());

        let agents = fs::read_to_string(temp_dir.join("AGENTS.md")).unwrap();
        assert!(agents.contains("TestApp"));

        // Clean up
        let _ = fs::remove_dir_all(temp_dir);
    }

    #[test]
    fn test_skeleton_includes_ai_context() {
        let temp_dir = std::env::temp_dir().join(format!("lumina_skel_test_{}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()));
        fs::create_dir_all(&temp_dir).expect("Failed to create temp dir");

        crate::skeleton::create_full_skeleton(&temp_dir, "SkelApp", "sqlite");

        assert!(temp_dir.join("Cargo.toml").exists());
        assert!(temp_dir.join("AGENTS.md").exists());
        assert!(temp_dir.join(".cursorrules").exists());
        assert!(temp_dir.join(".github/copilot-instructions.md").exists());
        assert!(temp_dir.join("CLAUDE.md").exists());

        // Clean up
        let _ = fs::remove_dir_all(temp_dir);
    }

    #[test]
    fn test_generate_ai_context_command() {
        let temp_dir = std::env::temp_dir().join(format!("lumina_ctx_cmd_test_{}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()));
        fs::create_dir_all(temp_dir.join("src/app")).expect("Failed to create src/app");
        fs::write(temp_dir.join("Cargo.toml"), "[package]\nname = \"test_ctx_app\"\nversion = \"0.1.0\"\n").unwrap();

        crate::generator::generate_ai_context_at(&temp_dir, "all", true);

        assert!(temp_dir.join("AGENTS.md").exists());
        assert!(temp_dir.join(".cursorrules").exists());
        assert!(temp_dir.join(".github/copilot-instructions.md").exists());
        assert!(temp_dir.join("CLAUDE.md").exists());

        let agents_content = fs::read_to_string(temp_dir.join("AGENTS.md")).unwrap();
        assert!(agents_content.contains("test_ctx_app"));

        let _ = fs::remove_dir_all(temp_dir);
    }

    #[test]
    fn test_generate_ai_skill_creates_schemas_and_stubs() {
        let temp_dir = std::env::temp_dir().join(format!("lumina_skill_test_{}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()));
        fs::create_dir_all(temp_dir.join("src/app")).expect("Failed to create src/app");
        fs::write(temp_dir.join("Cargo.toml"), "[package]\nname = \"test_skill_app\"\nversion = \"0.1.0\"\n").unwrap();

        let fields = vec!["keyword:string".to_string(), "limit:i64".to_string()];
        crate::generator::generate_ai_skill_at(&temp_dir, "SearchProducts", &fields, "all");

        assert!(temp_dir.join("app/ai/tools/search_products.json").exists());
        assert!(temp_dir.join("app/ai/skills/search_products/SKILL.md").exists());
        assert!(temp_dir.join("src/app/services/ai/search_products_tool.rs").exists());

        let json_content = fs::read_to_string(temp_dir.join("app/ai/tools/search_products.json")).unwrap();
        assert!(json_content.contains("openai_tool"));
        assert!(json_content.contains("gemini_declaration"));
        assert!(json_content.contains("claude_tool"));

        let rust_code = fs::read_to_string(temp_dir.join("src/app/services/ai/search_products_tool.rs")).unwrap();
        assert!(rust_code.contains("pub struct SearchProductsInput"));
        assert!(rust_code.contains("pub keyword: String"));
        assert!(rust_code.contains("pub limit: i64"));
        assert!(rust_code.contains("SearchProductsTool"));

        let _ = fs::remove_dir_all(temp_dir);
    }

    #[test]
    fn test_generate_ai_skill_reverse_engineer() {
        let temp_dir = std::env::temp_dir().join(format!("lumina_rev_test_{}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()));
        fs::create_dir_all(temp_dir.join("src/app")).expect("Failed to create src/app");
        fs::write(temp_dir.join("Cargo.toml"), "[package]\nname = \"test_rev_app\"\nversion = \"0.1.0\"\n").unwrap();

        crate::generator::generate_ai_skill_at(&temp_dir, "ReverseEngineer", &[], "all");

        assert!(temp_dir.join("app/ai/tools/reverse_engineer.json").exists());
        assert!(temp_dir.join("app/ai/skills/reverse_engineer/SKILL.md").exists());
        assert!(temp_dir.join("src/app/services/ai/reverse_engineer_tool.rs").exists());

        let skill_doc = fs::read_to_string(temp_dir.join("app/ai/skills/reverse_engineer/SKILL.md")).unwrap();
        assert!(skill_doc.contains("Reverse Engineer"));
        assert!(skill_doc.contains("Lumina Live"));
        assert!(skill_doc.contains("Tailwind"));

        let rust_code = fs::read_to_string(temp_dir.join("src/app/services/ai/reverse_engineer_tool.rs")).unwrap();
        assert!(rust_code.contains("ReverseEngineerInput"));
        assert!(rust_code.contains("ReverseEngineerTool"));

        let _ = fs::remove_dir_all(temp_dir);
    }
}
