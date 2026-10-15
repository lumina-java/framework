# Skill 2: Implementasi Global Exception Handler

## Deskripsi / Tujuan
Membangun layer *Global Exception Handler* terpusat yang akan menangkap semua panic, error dari *database*, atau error aplikasi (`Result::Err`). Saat ini, penanganan error mungkin masih mengandalkan manual `.expect()` atau *return tuple* dari axum.

## Kebutuhan Teknis
- Mengubah standar respon error menjadi struktur tipe `AppError`.
- `AppError` harus mengimplementasikan trait `IntoResponse` dari `axum`.
- Respon harus dibedakan berdasarkan asal request:
  - Jika request API (mengharapkan JSON): Kembalikan JSON `{ "success": false, "message": "..." }` dengan status 500/404/dll.
  - Jika request Web (Browser): Render file HTML dari `resources/views/errors/500.html` atau `404.html`.

## Langkah-langkah Implementasi
1. Buat file baru `src/core/error.rs` (atau `src/http/exception.rs`).
2. Definisikan `enum AppError` yang menampung error seperti `DatabaseError(sqlx::Error)`, `ValidationError`, `NotFound`, dll.
3. Tulis implementasi `axum::response::IntoResponse` untuk `AppError`. Di dalamnya periksa `Accept` header. Jika memuat `application/json`, return JSON. Jika `text/html`, return HTML view.
4. Buat template default untuk error di `resources/views/errors/404.html` dan `500.html`.
5. Integrasikan `AppError` sebagai tipe *return* standar di berbagai controller handler `Result<Response, AppError>`.

## Instruksi Git & Pull Request
- Buat *branch* baru: `git checkout -b feature/global-exception-handler`.
- Lakukan komit secara bertahap.
- Jangan *push* ke `master` secara langsung.
- Ajukan Pull Request untuk *review*.

## Panduan User Test Manual
1. Buat sebuah endpoint pengujian: `GET /test-error` yang secara sengaja me-return `AppError::InternalServerError("Testing Error")`.
2. Buka Postman/cURL, *hit* `/test-error` dengan header `Accept: application/json`. Pastikan mendapat respons JSON dengan status code 500.
3. Buka *browser* (Chrome/Firefox), navigasikan ke `http://localhost:3000/test-error`. Pastikan melihat halaman web (HTML) bertuliskan "500 Internal Server Error" yang di-render dari template Tera, bukan raw text/JSON.
