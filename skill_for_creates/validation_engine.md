# Lumina Skill: Validation Engine (Request Validation)

## Overview
Lumina menyediakan sistem validasi request yang terintegrasi dengan Axum menggunakan crate `validator`. Developer dapat mendefinisikan aturan validasi langsung pada struct request menggunakan attribute macros.

## Standards & Patterns

### 1. Definisi Request Struct
Setiap data yang masuk harus didefinisikan dalam struct yang mengimplementasikan `Deserialize` dan `Validate`.

```rust
#[derive(Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(length(min = 3, message = "Nama minimal 3 karakter"))]
    pub name: String,

    #[validate(email(message = "Format email tidak valid"))]
    pub email: String,

    #[validate(length(min = 8, message = "Password minimal 8 karakter"))]
    pub password: String,
}
```

### 2. Custom Extractor: `ValidatedJson<T>`
Gunakan extractor `ValidatedJson<T>` di handler controller untuk memicu validasi otomatis. Jika validasi gagal, Lumina akan secara otomatis mengembalikan HTTP 422 dengan detail error dalam format JSON.

```rust
pub async fn register(ValidatedJson(payload): ValidatedJson<RegisterRequest>) -> impl IntoResponse {
    // Logic hanya akan berjalan jika payload valid
}
```

### 3. Format Error Response (422)
Lumina mengembalikan daftar error yang terstruktur agar mudah dikonsumsi oleh frontend.

```json
{
  "errors": {
    "email": ["Format email tidak valid"],
    "password": ["Password minimal 8 karakter"]
  }
}
```

## Implementation Logic (Under the hood)
- Implementasi `FromRequest` untuk `ValidatedJson<T>`.
- Pemanggilan `payload.validate()`.
- Transformasi `ValidationErrors` menjadi `HashMap<String, Vec<String>>`.
- Mapping HTTP status ke `UNPROCESSABLE_ENTITY`.
