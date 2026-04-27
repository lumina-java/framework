# 🛡️ Lumina Framework — Request Validation Skill

## 🎯 Goal
Mencegah data kotor atau invalid (bad request) masuk ke layer Controller. Validasi ini berjalan secara otomatis dengan memanfaatkan crate `validator` dan *Axum Extractor Wrapper*. Terinspirasi oleh Laravel Form Request Validation.

**Fitur yang dibangun:**
- Integrasi crate `validator`.
- Wrapper `ValidatedJson<T>` sebagai ganti `axum::Json<T>`.
- Otomatis mereturn HTTP 422 (Unprocessable Entity) beserta daftar error jika validasi gagal.
- Integrasi rule validasi pada struct request (contoh: `@email`, `@length(min=8)`).

---

## 📁 Perubahan File

```
Cargo.toml                              ← MODIFY: tambah validator

src/core/
└── request.rs                          ← MODIFY: tambah ValidatedJson extractor

src/app/controllers/
└── auth_controller.rs                  ← MODIFY: gunakan ValidatedJson di register/login
```

---

## 📝 Konsep Implementasi

### 1. `Cargo.toml`
```toml
[dependencies]
validator = { version = "0.18", features = ["derive"] }
```

### 2. `src/core/request.rs`
Membuat wrapper struct `ValidatedJson<T>` yang mengimplementasikan trait `FromRequest` dari Axum.

```rust
use axum::{
    async_trait,
    extract::{FromRequest, Request, rejection::JsonRejection},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use validator::Validate;
use serde_json::json;

#[derive(Debug, Clone, Copy, Default)]
pub struct ValidatedJson<T>(pub T);

#[async_trait]
impl<T, S> FromRequest<S> for ValidatedJson<T>
where
    T: serde::de::DeserializeOwned + Validate,
    S: Send + Sync,
    Json<T>: FromRequest<S, Rejection = JsonRejection>,
{
    type Rejection = Response;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(value) = Json::<T>::from_request(req, state)
            .await
            .map_err(|err| {
                (StatusCode::BAD_REQUEST, Json(json!({"success": false, "message": err.body_text()}))).into_response()
            })?;

        if let Err(errors) = value.validate() {
            let body = Json(json!({
                "success": false,
                "message": "Validasi gagal",
                "errors": errors
            }));
            return Err((StatusCode::UNPROCESSABLE_ENTITY, body).into_response());
        }

        Ok(ValidatedJson(value))
    }
}
```

### 3. Controller Implementation
Di `auth_controller.rs`:

```rust
use crate::core::request::ValidatedJson;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(length(min = 3, message = "Nama minimal 3 karakter"))]
    pub name: String,
    
    #[validate(email(message = "Format email tidak valid"))]
    pub email: String,
    
    #[validate(length(min = 6, message = "Password minimal 6 karakter"))]
    pub password: String,
}

// Controller parameter diganti menjadi ValidatedJson
pub async fn register(ValidatedJson(body): ValidatedJson<RegisterRequest>) -> Json<Value> {
    // Jika sampai di sini, data SUDAH PASTI VALID.
    // ...
}
```

---

## ✅ Validation Checklist

1. Kirim `POST /api/auth/register` dengan email format salah.
   - Harapan: Menerima respons HTTP 422 (Unprocessable Entity) dengan JSON error detail `Format email tidak valid`.
   - Harapan: Logic di dalam fungsi `register()` tidak pernah tereksekusi.
2. Kirim payload benar.
   - Harapan: Logic berjalan normal dan return 200 OK.

---

## 📌 Urutan Eksekusi
Skill ini akan dieksekusi via pull request ke master setelah fitur Template Engine selesai.
