# 📘 Modul 08: Autentikasi & Keamanan

Lumina Framework menyediakan sistem keamanan ganda (**Dual Authentication Suite**) yang mencakup autentikasi berbasis **Session/Cookie** untuk aplikasi web tradisional, serta **JSON Web Token (JWT)** untuk API mobile/frontend modern.

---

## 1. Scaffolding Autentikasi Instan (`lumina make:auth`)

Buat modul registrasi, login, logout, dan dashboard siap pakai hanya dengan satu perintah:

```bash
lumina make:auth
```
Perintah ini akan secara otomatis membuat:
- Controller: `src/app/controllers/auth_controller.rs`
- Model & Migrasi: `User` model dengan hashing password
- Halaman View Blade: `resources/views/auth/login.blade.rs`, `register.blade.rs`, dan `dashboard.blade.rs`.

---

## 2. Password Hashing (Bcrypt)

Lumina menggunakan algoritma hashing **Bcrypt** standar industri:

```rust
use bcrypt::{hash, verify, DEFAULT_COST};

// Hashing password baru
let raw_password = "password_rahasia_123";
let hashed = hash(raw_password, DEFAULT_COST)?;

// Verifikasi password saat login
let is_valid = verify("password_input_user", &hashed)?;
if is_valid {
    println!("Password cocok!");
}
```

---

## 3. Autentikasi JWT (JSON Web Token)

### A. Model Pengguna Terautentikasi (`AuthUser`)
```rust
use lumina::core::auth::AuthUser;
use lumina::http::auth::{generate_token, validate_token};

// 1. Buat instance AuthUser setelah login sukses (berlaku 24 jam)
let user = AuthUser::new(
    1,                                  // user_id
    "user@example.com".to_string(),     // email
    "admin".to_string(),                // role
    vec!["create_post".to_string()],    // permissions
    24,                                 // masa aktif (jam)
);

// 2. Generate token JWT string
let token = generate_token(&user)?;
```

### B. Memproteksi Endpoint dengan `AuthUser` Extractor
Cukup letakkan `auth_user: AuthUser` di parameter fungsi controller Anda. Lumina akan otomatis memvalidasi header `Authorization: Bearer <token>`:

```rust
use axum::Json;
use lumina::core::auth::AuthUser;

pub async fn profile(auth_user: AuthUser) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "authorized",
        "user_id": auth_user.sub,
        "email": auth_user.email,
        "role": auth_user.role,
    }))
}
```

---

## 4. Role & Permission Guard (`can`)

Periksa hak akses user menggunakan method `auth_user.can(...)`:

```rust
use axum::http::StatusCode;

pub async fn delete_article(auth_user: AuthUser) -> Result<String, StatusCode> {
    if !auth_user.can("delete_post") && auth_user.role != "admin" {
        return Err(StatusCode::FORBIDDEN);
    }

    Ok("Artikel berhasil dihapus".to_string())
}
```

---

## 5. Direktif Autentikasi di Blade (`@auth`, `@guest`)

Sembunyikan atau tampilkan elemen navigasi berdasarkan status login:

```html
@auth
    <span>Halo, {{ $auth.email }}!</span>
    <form method="POST" action="/logout">
        @csrf
        <button type="submit">Keluar</button>
    </form>
@endauth

@guest
    <a href="/login">Masuk</a>
    <a href="/register">Daftar Akun</a>
@endguest
```

---

## 6. Lumina Shield Security Audit (`lumina audit`)

Sebelum melakukan deployment ke server produksi, Lumina menyediakan alat audit keamanan otomatis:

```bash
lumina audit
```

### Lingkup Pemeriksaan Otomatis:
1. **Secret & Key Entropy**: Memverifikasi kekuatan `APP_KEY` dan `JWT_SECRET` (panjang minimal 32 karakter dan bukan placeholder default).
2. **Debug Mode in Production**: Mencegah kebocoran stack trace dan kredensial sensitif jika `APP_DEBUG=true` aktif di lingkungan `production`.
3. **CSRF Protection on Forms**: Memindai seluruh template tampilan (`resources/views/`) untuk memastikan setiap formulir bermetode `POST` memiliki direktif `@csrf`.
4. **Database & Storage Hardening**: Memeriksa koneksi database dan isolasi folder upload publik.
5. **Scoreboard & Actionable Fixes**: Memberikan skor keamanan (*0–100*), peringkat mutu (*Grade A+*), serta rekomendasi perintah perbaikan secara instan.

---

### Navigasi Modul:
- **Sebelumnya**: [Modul 07: Indonesian Script](./07_indonesian_script.md) 
- **Berikutnya**: [Modul 09: Broadcasting & WebSocket](./09_broadcasting_dan_websocket.md) ➔
- **Kembali ke**: [Daftar Isi Dokumentasi](./README.md)
