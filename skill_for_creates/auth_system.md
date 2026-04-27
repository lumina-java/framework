# Lumina Skill: Authentication System (Secure Flow)

## Overview
Lumina menyediakan sistem autentikasi terintegrasi yang mencakup password hashing, validasi input, dan token management (JWT).

## Standards & Patterns

### 1. Password Hashing
Gunakan `bcrypt` untuk mengamankan password. Jangan pernah menyimpan password dalam bentuk plain text.

```rust
// Helper pattern
let hashed = lumina::auth::hash::make("secret");
let is_valid = lumina::auth::hash::verify("secret", &hashed);
```

### 2. Registration Flow
1. Terima data via `ValidatedJson<RegisterRequest>`.
2. Hash password.
3. Simpan ke database menggunakan model `User`.
4. Kembalikan response sukses atau otomatis login.

### 3. Login Flow
1. Cari user berdasarkan email.
2. Bandingkan password menggunakan `hash::verify`.
3. Jika cocok, generate JWT menggunakan `jsonwebtoken`.
4. Simpan token di Cookie (untuk Web) atau Header (untuk API).

### 4. Auth Views
Letakkan template autentikasi di `resources/views/auth/`. Gunakan layout master untuk konsistensi UI.

## Security Best Practices
- **Cost Factor**: Gunakan cost factor minimal 10 untuk bcrypt.
- **JWT Secret**: Pastikan `JWT_SECRET` di `.env` sangat kuat.
- **HttpOnly Cookies**: Simpan JWT dalam HttpOnly cookie untuk mencegah serangan XSS pada aplikasi web.
