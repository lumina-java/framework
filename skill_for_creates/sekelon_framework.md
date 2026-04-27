# 🦀 Lumina Framework - Skeleton Skill

## 🎯 Goal
Buat skeleton framework web Rust bernama "Lumina" dengan arsitektur Laravel-like (clean, readable, beautiful).

---

## 📁 Struktur Direktori

```
lumina/
├── src/
│   ├── main.rs
│   ├── lib.rs
│   │
│   ├── core/
│   │   ├── mod.rs
│   │   ├── application.rs
│   │   ├── router.rs
│   │   ├── request.rs
│   │   ├── response.rs
│   │   ├── middleware.rs
│   │   └── container.rs
│   │
│   ├── http/
│   │   ├── mod.rs
│   │   ├── server.rs
│   │   └── kernel.rs
│   │
│   ├── app/
│   │   ├── mod.rs
│   │   ├── controllers/
│   │   │   ├── mod.rs
│   │   │   └── home_controller.rs
│   │   ├── models/
│   │   │   └── mod.rs
│   │   └── providers/
│   │       └── mod.rs
│   │
│   ├── database/
│   │   ├── mod.rs
│   │   └── connection.rs
│   │
│   ├── config/
│   │   ├── mod.rs
│   │   └── app.rs
│   │
│   └── support/
│       └── mod.rs
│
├── routes/
│   ├── web.rs
│   └── api.rs
│
├── resources/
│   └── views/
│       └── home/
│           └── index.html
│
├── database/
│   ├── migrations/
│   └── seeders/
│
├── storage/
│   ├── logs/
│   └── cache/
│
├── public/
│   ├── css/
│   ├── js/
│   └── images/
│
├── tests/
│   ├── unit/
│   └── integration/
│
├── .env.example
├── Cargo.toml
└── README.md
```

---

## 📝 File Implementations

### 1. `Cargo.toml`
```toml
[package]
name = "lumina"
version = "0.1.0"
edition = "2026"
authors = ["Slamet Sugandi <packercyber@gmail.com>"]
description = "A beautiful, fast, and elegant web framework for Rust"
license = "MIT"

[lib]
name = "lumina"
path = "src/lib.rs"

[[bin]]
name = "lumina"
path = "src/main.rs"

[dependencies]
tokio = { version = "1", features = ["full"] }
axum = "0.7"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
dotenv = "0.15"
async-trait = "0.1"
tracing = "0.1"
tracing-subscriber = "0.3"
```

---

### 2. `src/lib.rs`
```rust
pub mod core;
pub mod http;
pub mod app;
pub mod database;
pub mod config;
pub mod support;

pub mod prelude {
    pub use crate::core::application::Application;
    pub use crate::core::router::Router;
    pub use crate::http::kernel::HttpKernel;
}
```

---

### 3. `src/main.rs`
```rust
use lumina::prelude::*;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    
    let app = Application::new();
    
    println!("✨ Lumina Framework");
    println!("🚀 Server starting at http://127.0.0.1:8000");
    
    app.serve("127.0.0.1:8000").await;
}
```

---

### 4. `src/core/mod.rs`
```rust
pub mod application;
pub mod router;
pub mod request;
pub mod response;
pub mod middleware;
pub mod container;
```

---

### 5. `src/core/application.rs`
```rust
use crate::core::router::Router;
use crate::core::container::Container;

pub struct Application {
    router: Router,
    container: Container,
}

impl Application {
    pub fn new() -> Self {
        Self {
            router: Router::new(),
            container: Container::new(),
        }
    }
    
    pub async fn serve(self, addr: &str) {
        println!("🌐 Listening on http://{}", addr);
        // HTTP server implementation akan ditambah nanti
    }
}
```

---

### 6. `src/core/router.rs`
```rust
pub struct Router {
    routes: Vec<Route>,
}

pub struct Route {
    pub method: String,
    pub path: String,
}

impl Router {
    pub fn new() -> Self {
        Self { routes: Vec::new() }
    }
    
    pub fn add_route(&mut self, method: &str, path: &str) {
        self.routes.push(Route {
            method: method.to_string(),
            path: path.to_string(),
        });
    }
}
```

---

### 7. `src/core/request.rs`
```rust
pub struct Request {
    pub method: String,
    pub uri: String,
    pub body: String,
}

impl Request {
    pub fn new(method: String, uri: String) -> Self {
        Self {
            method,
            uri,
            body: String::new(),
        }
    }
}
```

---

### 8. `src/core/response.rs`
```rust
pub struct Response {
    pub status: u16,
    pub body: String,
}

impl Response {
    pub fn ok(body: String) -> Self {
        Self {
            status: 200,
            body,
        }
    }
    
    pub fn json<T: serde::Serialize>(data: T) -> Self {
        let body = serde_json::to_string(&data).unwrap();
        Self {
            status: 200,
            body,
        }
    }
}
```

---

### 9. `src/core/middleware.rs`
```rust
use async_trait::async_trait;
use crate::core::{request::Request, response::Response};

#[async_trait]
pub trait Middleware: Send + Sync {
    async fn handle(&self, request: Request) -> Result<Response, String>;
}
```

---

### 10. `src/core/container.rs`
```rust
use std::collections::HashMap;
use std::any::Any;

pub struct Container {
    bindings: HashMap<String, Box<dyn Any>>,
}

impl Container {
    pub fn new() -> Self {
        Self {
            bindings: HashMap::new(),
        }
    }
    
    pub fn bind<T: 'static>(&mut self, key: String, value: T) {
        self.bindings.insert(key, Box::new(value));
    }
}
```

---

### 11. `src/http/mod.rs`
```rust
pub mod server;
pub mod kernel;
```

---

### 12. `src/http/server.rs`
```rust
pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }
    
    pub async fn start(&self) {
        println!("🔥 HTTP Server running on {}", self.addr);
        // Axum server implementation
    }
}
```

---

### 13. `src/http/kernel.rs`
```rust
pub struct HttpKernel;

impl HttpKernel {
    pub fn new() -> Self {
        Self
    }
    
    pub fn bootstrap(&self) {
        println!("⚙️  HTTP Kernel bootstrapped");
    }
}
```

---

### 14. `src/app/mod.rs`
```rust
pub mod controllers;
pub mod models;
pub mod providers;
```

---

### 15. `src/app/controllers/mod.rs`
```rust
pub mod home_controller;
```

---

### 16. `src/app/controllers/home_controller.rs`
```rust
use crate::core::response::Response;

pub struct HomeController;

impl HomeController {
    pub fn index() -> Response {
        Response::ok("Welcome to Lumina Framework! 🚀".to_string())
    }
    
    pub fn about() -> Response {
        Response::ok("About Lumina Framework".to_string())
    }
}
```

---

### 17. `src/app/models/mod.rs`
```rust
// Models akan ditambahkan di sini
```

---

### 18. `src/app/providers/mod.rs`
```rust
pub trait ServiceProvider {
    fn register(&self);
    fn boot(&self);
}
```

---

### 19. `src/database/mod.rs`
```rust
pub mod connection;
```

---

### 20. `src/database/connection.rs`
```rust
pub struct Database {
    connection_string: String,
}

impl Database {
    pub fn new(connection_string: String) -> Self {
        Self { connection_string }
    }
    
    pub async fn connect(&self) -> Result<(), String> {
        println!("📦 Database connected");
        Ok(())
    }
}
```

---

### 21. `src/config/mod.rs`
```rust
pub mod app;
```

---

### 22. `src/config/app.rs`
```rust
pub struct AppConfig {
    pub name: String,
    pub env: String,
    pub debug: bool,
}

impl AppConfig {
    pub fn load() -> Self {
        Self {
            name: "Lumina".to_string(),
            env: "development".to_string(),
            debug: true,
        }
    }
}
```

---

### 23. `src/support/mod.rs`
```rust
// Helper functions dan utilities
pub fn env(key: &str, default: &str) -> String {
    std::env::var(key).unwrap_or_else(|_| default.to_string())
}
```

---

### 24. `routes/web.rs`
```rust
use crate::app::controllers::home_controller::HomeController;

pub fn register() {
    // Route definitions
    // GET  / -> HomeController::index
    // GET  /about -> HomeController::about
}
```

---

### 25. `routes/api.rs`
```rust
pub fn register() {
    // API route definitions
}
```

---

### 26. `.env.example`
```env
APP_NAME=Lumina
APP_ENV=development
APP_DEBUG=true
APP_URL=http://localhost:8000

DB_CONNECTION=postgres
DB_HOST=127.0.0.1
DB_PORT=5432
DB_DATABASE=lumina
DB_USERNAME=root
DB_PASSWORD=
```

---

### 27. `resources/views/home/index.html`
```html
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Lumina Framework</title>
</head>
<body>
    <h1>✨ Welcome to Lumina Framework</h1>
    <p>A beautiful, fast, and elegant web framework for Rust</p>
</body>
</html>
```

---

### 28. `README.md`
```markdown
# ✨ Lumina Framework

A beautiful, fast, and elegant web framework for Rust.

Inspired by Laravel's clean and readable syntax.

## Features

- 🚀 Fast and efficient (built with Rust)
- 🎨 Clean and beautiful syntax
- 🏗️ MVC Architecture
- 🔌 Modular design
- 📦 Service Container
- 🛣️ Elegant routing

## Quick Start

```bash
cargo run
```

Server akan berjalan di `http://127.0.0.1:8000`

## License

MIT
```

---

## ✅ Validation Checklist

Setelah generate semua file:

```bash
# 1. Build project
cargo build

# 2. Run project
cargo run

# 3. Expected output:
# ✨ Lumina Framework
# 🚀 Server starting at http://127.0.0.1:8000
# 🌐 Listening on http://127.0.0.1:8000
```

---

## 🎯 Output

Project skeleton Lumina siap dikembangkan dengan:
- ✅ Struktur folder lengkap
- ✅ Module system terorganisir
- ✅ Placeholder implementation
- ✅ Compile tanpa error
- ✅ Siap untuk tahap berikutnya (routing, HTTP server, database)

---

## 📌 Next Steps (Pilih salah satu)

1. **HTTP Server Implementation** (Axum integration)
2. **Routing Engine** (URL mapping ke controller)
3. **Database Layer** (Connection pool, query builder)
4. **Template Engine** (View rendering)
5. **CLI Tool** (`lumina serve`, `lumina make:controller`)

---

## 🔖 Notes

- Framework ini menggunakan **async/await** (Tokio runtime)
- Design pattern mengikuti **Laravel conventions**
- Fokus pada **DX (Developer Experience)**
- Modular dan extensible

---

**End of Skill** 🚀