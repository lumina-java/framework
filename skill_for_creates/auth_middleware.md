# 🔐 Lumina Framework — Auth Middleware Skill

## 🎯 Goal
Implementasi **JWT Authentication Middleware** pada Lumina Framework. Melindungi route API dengan token-based auth yang stateless, terinspirasi Laravel Sanctum namun sepenuhnya native Rust.

**Fitur yang dibangun:**
- JWT token generator & validator (`jsonwebtoken` crate)
- `Claims` struct — payload token (user_id, role, exp)
- `auth_required` middleware — proteksi route via `Authorization: Bearer <token>`
- `POST /api/auth/login` — endpoint login, kembalikan JWT
- `POST /api/auth/register` — endpoint register user baru
- `GET /api/auth/me` — endpoint profil user (protected)
- Route groups: public vs protected

---

## 📁 Perubahan File

```
Cargo.toml                              ← tambah jsonwebtoken, bcrypt

src/http/
├── middleware.rs                       ← MODIFY: tambah auth_required middleware
└── auth.rs                             ← NEW: JWT generate & validate

src/core/
└── auth.rs                             ← NEW: Claims struct

src/app/controllers/
├── mod.rs                              ← MODIFY: export auth_controller
└── auth_controller.rs                  ← NEW: login, register, me

routes/
└── api.rs                              ← MODIFY: tambah auth routes + protected group

.env.example                            ← MODIFY: tambah JWT_SECRET, JWT_EXPIRY
```

---

## 📝 File Implementations

### 1. `Cargo.toml` — Tambah Dependencies
```toml
jsonwebtoken = "9"
bcrypt        = "0.15"
```

---

### 2. `src/core/auth.rs` — Claims Struct
```rust
use serde::{Serialize, Deserialize};

/// Payload yang disimpan di dalam JWT token.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub:   String,  // user_id sebagai string
    pub email: String,
    pub role:  String,
    pub exp:   usize,   // Unix timestamp expiration
    pub iat:   usize,   // Unix timestamp issued at
}

impl Claims {
    /// Buat claims baru dengan expiry dari sekarang + durasi (dalam jam)
    pub fn new(user_id: i64, email: String, role: String, hours: i64) -> Self {
        use std::time::{SystemTime, UNIX_EPOCH};
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as usize;
        Self {
            sub:   user_id.to_string(),
            email,
            role,
            iat:   now,
            exp:   now + (hours as usize * 3600),
        }
    }
}
```

---

### 3. `src/http/auth.rs` — JWT Generator & Validator
```rust
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use crate::core::auth::Claims;

/// Generate JWT token dari Claims.
pub fn generate_token(claims: &Claims) -> Result<String, jsonwebtoken::errors::Error> {
    let secret = crate::support::env("JWT_SECRET", "lumina-secret-key-change-in-prod");
    encode(
        &Header::default(),
        claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
}

/// Validasi dan decode JWT token.
/// Mengembalikan Claims jika valid, error jika expired/invalid.
pub fn validate_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let secret = crate::support::env("JWT_SECRET", "lumina-secret-key-change-in-prod");
    let data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )?;
    Ok(data.claims)
}
```

---

### 4. `src/http/middleware.rs` — auth_required Middleware
```rust
// Tambahkan di middleware.rs yang sudah ada:
use axum::{
    extract::Request,
    http::{StatusCode, HeaderMap},
    middleware::Next,
    response::{Response, Json},
};
use serde_json::json;

/// Middleware proteksi route — cek JWT di header `Authorization: Bearer <token>`.
/// Jika valid: lanjut ke handler.
/// Jika tidak ada / expired / invalid: kembalikan 401 Unauthorized.
pub async fn auth_required(req: Request, next: Next) -> Result<Response, (StatusCode, axum::Json<serde_json::Value>)> {
    let token = extract_bearer_token(req.headers());

    match token {
        None => Err((
            StatusCode::UNAUTHORIZED,
            Json(json!({ "success": false, "message": "Token tidak ditemukan" }))
        )),
        Some(t) => match crate::http::auth::validate_token(&t) {
            Ok(_claims) => Ok(next.run(req).await),
            Err(_) => Err((
                StatusCode::UNAUTHORIZED,
                Json(json!({ "success": false, "message": "Token invalid atau expired" }))
            )),
        }
    }
}

fn extract_bearer_token(headers: &HeaderMap) -> Option<String> {
    headers
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .map(|t| t.to_string())
}
```

---

### 5. `src/app/controllers/auth_controller.rs` — Login / Register / Me
```rust
use axum::{extract::Json as JsonBody, Json};
use serde::{Serialize, Deserialize};
use serde_json::{json, Value};

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email:    String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub name:     String,
    pub email:    String,
    pub password: String,
}

pub struct AuthController;

impl AuthController {
    /// POST /api/auth/login
    /// Verifikasi kredensial, kembalikan JWT token jika valid.
    pub async fn login(JsonBody(body): JsonBody<LoginRequest>) -> Json<Value> {
        // TODO: query DB, verify bcrypt hash
        // Simulasi untuk skeleton:
        if body.email == "admin@lumina.rs" && body.password == "secret" {
            let claims = crate::core::auth::Claims::new(
                1, body.email, "admin".to_string(), 24
            );
            match crate::http::auth::generate_token(&claims) {
                Ok(token) => Json(json!({
                    "success": true,
                    "message": "Login berhasil",
                    "token": token,
                    "token_type": "Bearer",
                    "expires_in": 86400
                })),
                Err(_) => Json(json!({
                    "success": false,
                    "message": "Gagal generate token"
                })),
            }
        } else {
            Json(json!({
                "success": false,
                "message": "Email atau password salah"
            }))
        }
    }

    /// POST /api/auth/register
    pub async fn register(JsonBody(body): JsonBody<RegisterRequest>) -> Json<Value> {
        // TODO: hash password dengan bcrypt, insert ke DB
        Json(json!({
            "success": true,
            "message": "Registrasi berhasil",
            "data": {
                "id":    99,
                "name":  body.name,
                "email": body.email,
                "role":  "user"
            }
        }))
    }

    /// GET /api/auth/me — Protected route, butuh token valid
    pub async fn me() -> Json<Value> {
        // TODO: ambil user dari Claims yang di-inject middleware
        Json(json!({
            "success": true,
            "data": {
                "id":    1,
                "name":  "Slamet Sugandi",
                "email": "admin@lumina.rs",
                "role":  "admin"
            }
        }))
    }
}
```

---

### 6. `routes/api.rs` — Public & Protected Route Groups
```rust
use axum::middleware::from_fn;
use crate::core::router::Router;
use crate::app::controllers::{
    api_controller::ApiController,
    auth_controller::AuthController,
};
use crate::http::middleware::auth_required;

pub fn register() -> Router {
    // === Public routes (tidak perlu token) ===
    let public = Router::new()
        .post("/auth/login",    AuthController::login)
        .post("/auth/register", AuthController::register);

    // === Protected routes (butuh Authorization: Bearer <token>) ===
    // Gunakan .layer(from_fn(auth_required)) pada router group ini
    let protected = Router::new()
        .get("/auth/me",    AuthController::me)
        .get("/users",      ApiController::index)
        .get("/users/:id",  ApiController::show)
        .post("/users",     ApiController::store);

    // Gabungkan public + protected
    public.merge(protected)
}
```

---

### 7. `.env.example` — JWT Config
```env
# JWT Authentication
JWT_SECRET=lumina-super-secret-key-change-this-in-production
JWT_EXPIRY_HOURS=24
```

---

## ✅ Validation Checklist

```bash
cargo build    # 0 errors

# Test login (dapat token)
curl -X POST http://localhost:8000/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{"email":"admin@lumina.rs","password":"secret"}'
# Response: {"success":true,"token":"eyJ..."}

# Test protected route TANPA token (harus 401)
curl http://localhost:8000/api/users
# Response: {"success":false,"message":"Token tidak ditemukan"}

# Test protected route DENGAN token (harus 200)
curl http://localhost:8000/api/users \
  -H "Authorization: Bearer eyJ..."
# Response: {"success":true,"data":[...]}
```

### Logger output di terminal:
```
  [POST]   /api/auth/login    → 200  (0.45ms)
  [GET]    /api/users          → 401  (0.08ms)   ← no token
  [GET]    /api/users          → 200  (0.12ms)   ← with token
```

---

## 📌 Urutan Implementasi
1. ✅ Skeleton Framework (Issue #0 / commit awal)
2. ✅ Routing Engine (Issue #1)
3. ⏳ **Database Layer (Issue #2)** ← saat ini
4. ⏳ **Auth Middleware (Issue #3)** ← setelah database layer selesai

---
**End of Skill** 🚀
