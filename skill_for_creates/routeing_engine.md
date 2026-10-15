# 🛣️ Lumina Framework — Routing Engine Skill

## 🎯 Goal
Implementasi **Routing Engine** pada framework Lumina yang menggabungkan kekuatan Axum dengan sintaks deklaratif ala Laravel. Fitur utama:
- Lumina Router sebagai fluent builder wrapper di atas `axum::Router`
- Dynamic routing dengan path parameter (`:id`, `:slug`, dll.)
- Route declaration terpisah di `routes/web.rs` dan `routes/api.rs`
- Request Logger Middleware dengan output berwarna

---

## 📁 File yang Dibuat / Dimodifikasi

```
src/core/router.rs                      ← MODIFY: Rewrite total
src/http/middleware.rs                  ← NEW: Logger middleware
src/http/mod.rs                         ← MODIFY: tambah mod middleware
src/app/controllers/user_controller.rs  ← NEW: Demo dynamic routing
src/app/controllers/api_controller.rs   ← NEW: JSON API controller
src/app/controllers/mod.rs              ← MODIFY: export controllers baru
routes/web.rs                           ← MODIFY: deklaratif route
routes/api.rs                           ← MODIFY: API routes
src/lib.rs                              ← MODIFY: include routes via #[path]
src/core/application.rs                 ← MODIFY: gunakan Lumina Router
```

---

## 📝 File Implementations

### 1. `src/core/router.rs` — Lumina Router
```rust
use axum::{
    Router as AxumRouter,
    handler::Handler,
    routing,
};

/// Lumina Router — fluent builder API yang membungkus axum::Router.
pub struct Router {
    inner: AxumRouter,
}

impl Router {
    pub fn new() -> Self {
        Self { inner: AxumRouter::new() }
    }

    pub fn get<H, T>(mut self, path: &str, handler: H) -> Self
    where H: Handler<T, ()>, T: 'static
    {
        self.inner = self.inner.route(path, routing::get(handler));
        self
    }

    pub fn post<H, T>(mut self, path: &str, handler: H) -> Self
    where H: Handler<T, ()>, T: 'static
    {
        self.inner = self.inner.route(path, routing::post(handler));
        self
    }

    pub fn put<H, T>(mut self, path: &str, handler: H) -> Self
    where H: Handler<T, ()>, T: 'static
    {
        self.inner = self.inner.route(path, routing::put(handler));
        self
    }

    pub fn delete<H, T>(mut self, path: &str, handler: H) -> Self
    where H: Handler<T, ()>, T: 'static
    {
        self.inner = self.inner.route(path, routing::delete(handler));
        self
    }

    pub fn patch<H, T>(mut self, path: &str, handler: H) -> Self
    where H: Handler<T, ()>, T: 'static
    {
        self.inner = self.inner.route(path, routing::patch(handler));
        self
    }

    pub fn nest(mut self, prefix: &str, other: Router) -> Self {
        self.inner = self.inner.nest(prefix, other.inner);
        self
    }

    pub fn merge(mut self, other: Router) -> Self {
        self.inner = self.inner.merge(other.inner);
        self
    }

    pub fn into_axum(self) -> AxumRouter {
        self.inner
    }
}

impl Default for Router {
    fn default() -> Self { Self::new() }
}
```

---

### 2. `src/http/middleware.rs` — Logger Middleware
```rust
use axum::{extract::Request, middleware::Next, response::Response};
use std::time::Instant;

pub async fn logger(req: Request, next: Next) -> Response {
    let method = req.method().clone();
    let path   = req.uri().path().to_string();
    let start  = Instant::now();

    let response = next.run(req).await;

    let elapsed = start.elapsed();
    let status  = response.status();
    let ms      = elapsed.as_secs_f64() * 1000.0;

    let method_color = match method.as_str() {
        "GET"    => "\x1b[34m",
        "POST"   => "\x1b[32m",
        "PUT"    => "\x1b[33m",
        "DELETE" => "\x1b[31m",
        "PATCH"  => "\x1b[35m",
        _        => "\x1b[37m",
    };

    let status_color = if status.is_success() { "\x1b[32m" }
        else if status.is_client_error() { "\x1b[33m" }
        else if status.is_server_error() { "\x1b[31m" }
        else { "\x1b[37m" };

    println!(
        "  {}{:7}\x1b[0m {:<30} \x1b[90m→\x1b[0m {}{}\x1b[0m  \x1b[90m({:.2}ms)\x1b[0m",
        method_color, format!("[{}]", method), path,
        status_color, status.as_u16(), ms,
    );

    response
}
```

---

### 3. `src/http/mod.rs`
```rust
pub mod server;
pub mod kernel;
pub mod middleware;
```

---

### 4. `src/app/controllers/user_controller.rs` — Dynamic Routing Demo
```rust
use axum::{extract::Path, response::Html};

pub struct UserController;

impl UserController {
    /// GET /users
    pub async fn index() -> Html<String> {
        Html(r#"
        <!DOCTYPE html>
        <html>
        <!-- premium HTML response with user list -->
        </html>
        "#.to_string())
    }

    /// GET /users/:id  ← Dynamic path parameter
    pub async fn show(Path(id): Path<u32>) -> Html<String> {
        Html(format!(r#"
        <!DOCTYPE html>
        <html>
        <!-- premium HTML response showing user #{id} -->
        </html>
        "#))
    }
}
```

---

### 5. `src/app/controllers/api_controller.rs` — JSON API
```rust
use axum::{extract::Path, Json};
use serde_json::{json, Value};

pub struct ApiController;

impl ApiController {
    /// GET /api/users
    pub async fn index() -> Json<Value> {
        Json(json!({
            "success": true,
            "data": [
                {"id": 1, "name": "Alice", "email": "alice@lumina.rs"},
                {"id": 2, "name": "Bob",   "email": "bob@lumina.rs"},
            ]
        }))
    }

    /// GET /api/users/:id  ← Dynamic path parameter
    pub async fn show(Path(id): Path<u32>) -> Json<Value> {
        Json(json!({
            "success": true,
            "data": {"id": id, "name": "Alice", "email": "alice@lumina.rs"}
        }))
    }

    /// POST /api/users
    pub async fn store() -> Json<Value> {
        Json(json!({
            "success": true,
            "message": "User created successfully",
            "data": {"id": 3}
        }))
    }
}
```

---

### 6. `src/app/controllers/mod.rs`
```rust
pub mod home_controller;
pub mod user_controller;
pub mod api_controller;
```

---

### 7. `routes/web.rs` — Deklaratif Web Routes
```rust
use crate::core::router::Router;
use crate::app::controllers::home_controller::HomeController;
use crate::app::controllers::user_controller::UserController;

pub fn register() -> Router {
    Router::new()
        .get("/",           HomeController::index)
        .get("/about",      HomeController::about)
        .get("/users",      UserController::index)
        .get("/users/:id",  UserController::show)  // ← Dynamic!
}
```

---

### 8. `routes/api.rs` — API Routes
```rust
use crate::core::router::Router;
use crate::app::controllers::api_controller::ApiController;

pub fn register() -> Router {
    Router::new()
        .get("/users",      ApiController::index)
        .get("/users/:id",  ApiController::show)   // ← Dynamic!
        .post("/users",     ApiController::store)
}
// Semua route di sini akan diprefix /api oleh Application
```

---

### 9. `src/lib.rs` — Include Routes via `#[path]`
```rust
pub mod core;
pub mod http;
pub mod app;
pub mod database;
pub mod config;
pub mod support;

// Include route files sebagai bagian dari crate
// Teknik #[path] membuat routes/web.rs bisa menggunakan crate:: paths
#[path = "../routes/web.rs"]
pub mod web_routes;

#[path = "../routes/api.rs"]
pub mod api_routes;

pub mod prelude {
    pub use crate::core::application::Application;
    pub use crate::core::router::Router;
    pub use crate::http::kernel::HttpKernel;
}
```

---

### 10. `src/core/application.rs` — Gunakan Lumina Router + Middleware
```rust
use axum::{Router as AxumRouter, middleware::from_fn};
use crate::http::server::Server;
use crate::http::middleware::logger;
use crate::core::container::Container;

#[allow(dead_code)]
pub struct Application {
    container: Container,
}

impl Application {
    pub fn new() -> Self {
        Self { container: Container::new() }
    }

    fn build_router(&self) -> AxumRouter {
        let web = crate::web_routes::register().into_axum();
        let api = crate::api_routes::register().into_axum();

        AxumRouter::new()
            .merge(web)
            .nest("/api", api)        // prefix semua API route dengan /api
            .layer(from_fn(logger))   // apply logger middleware ke semua route
    }

    pub async fn serve(self, addr: &str) {
        println!("🌐 Listening on http://{}", addr);
        let router = self.build_router();
        let server = Server::new(addr.to_string());
        server.start(router).await;
    }
}
```

---

## ✅ Validation Checklist

```bash
# 1. Build
cargo build
# Expected: Finished dev profile, 0 errors, 0 warnings

# 2. Run
cargo run
# Expected output:
# ✨ Lumina Framework
# 🚀 Server starting at http://127.0.0.1:8000
# 🌐 Listening on http://127.0.0.1:8000
# 🔥 HTTP Server running on http://127.0.0.1:8000
```

### Endpoint Test (bisa pakai curl atau browser)
```bash
curl http://localhost:8000/              # → HTML Home
curl http://localhost:8000/about         # → HTML About
curl http://localhost:8000/users         # → HTML User List
curl http://localhost:8000/users/42      # → HTML User #42 (dynamic)
curl http://localhost:8000/api/users     # → JSON array
curl http://localhost:8000/api/users/1   # → JSON user #1 (dynamic)
curl -X POST http://localhost:8000/api/users  # → JSON created
```

### Terminal Logger Output (saat server running)
```
  [GET]    /                              → 200  (0.12ms)
  [GET]    /users                         → 200  (0.08ms)
  [GET]    /users/42                      → 200  (0.09ms)
  [GET]    /api/users                     → 200  (0.11ms)
  [GET]    /api/users/1                   → 200  (0.10ms)
  [POST]   /api/users                     → 200  (0.13ms)
```

---

## 🎯 Output

Setelah skill ini dijalankan, Lumina memiliki:
- ✅ Lumina Router sebagai abstraksi bersih di atas Axum
- ✅ Dynamic routing dengan path parameter (`:id`)
- ✅ Route file deklaratif di `routes/web.rs` dan `routes/api.rs`
- ✅ API routes dengan prefix `/api` otomatis
- ✅ Request Logger Middleware dengan colorized terminal output
- ✅ HTML controllers dan JSON API controllers terpisah

---

## 📌 Next Steps (Pilih salah satu)

1. **Database Layer** — Query builder, connection pool (sqlx)
2. **Request Validation** — Validasi body request JSON
3. **Authentication Middleware** — JWT / Session-based auth
4. **CLI Tool** — `lumina serve`, `lumina make:controller`
5. **Template Engine** — View rendering dengan Tera/MiniJinja

---

**End of Skill** 🚀
