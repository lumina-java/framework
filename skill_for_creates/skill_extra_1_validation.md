# Skill Extra 1: Automatic Validation Extractor (Laravel-like)

## 🎯 Objektif
Mengurangi kode duplikat (boilerplate) di Controller dengan memindahkan logika validasi dan *redirect* ke dalam Axum Extractor. Kita ingin agar `ValidatedForm` tidak hanya memvalidasi data, tapi juga otomatis melakukan *redirect back* (ke halaman sebelumnya) dengan membawa pesan error (flashes) dan input lama (old input) jika validasi gagal.

## 📋 Deskripsi Tugas
Saat ini, setiap endpoint POST yang memproses form HTML memiliki blok kode seperti ini:
```rust
if let Err(e) = form.validate() {
    return (token, Redirect::to("/auth/register")
        .with_errors(e.to_map())
        .with_input(json!({"email": form.email}))
        .with_error("Validasi gagal.")
        .send(&session).await).into_response();
}
```
Ini sangat tidak DRY (Don't Repeat Yourself) dan sulit dibaca. Kita ingin menyederhanakannya menjadi:
```rust
pub async fn register(
    ValidatedForm(form): ValidatedForm<RegisterForm>
) -> impl IntoResponse {
    // Controller HANYA berisi logika ketika form SUDAH valid.
    // Jika tidak valid, eksekusi kode ini bahkan tidak akan dipanggil.
}
```

## 🛠️ Langkah Eksekusi (Untuk Junior Programmer / AI)

### Langkah 1: Buat tipe `ValidatedForm`
Buka file `src/core/validation.rs` (atau buat modul baru `src/core/request.rs` jika lebih relevan). Buat struct pembungkus:
```rust
pub struct ValidatedForm<T>(pub T);
```

### Langkah 2: Implementasi Trait `FromRequest` dari Axum
Buat implementasi `FromRequest` untuk `ValidatedForm<T>`. Di dalam `from_request`:
1. Ekstrak data dari Request menggunakan `axum::Form::<T>::from_request`.
2. Jika sukses diekstrak, panggil `.validate()` pada data tersebut.
3. Jika `.validate()` menghasilkan Error (`Err`):
   - Ambil `tower_sessions::Session` dari request extensions (menggunakan `Extension<Session>`).
   - Gunakan `crate::core::response::Redirect` untuk merakit response.
   - Ambil header `Referer` dari request untuk mengetahui ke mana user harus dikembalikan (redirect back).
   - Masukkan error validasi (`e.to_map()`) ke session `_errors`.
   - Masukkan raw form bytes (atau data yang ada) ke session `_old` agar form input pengguna tidak hilang.
   - Return axum `Response` (HTTP 302 Redirect) sebagai Error Rejection.
4. Jika validasi lolos (`Ok`), return `Ok(ValidatedForm(data))`.

### Langkah 3: Modifikasi `IntoResponse` untuk Error Validasi Web
Penting: Extractor Axum mengembalikan tipe `Rejection` jika gagal. Anda harus mendefinisikan tipe error baru (misal `WebValidationError`) yang mengimplementasikan `IntoResponse` dan mengembalikan aksi redirect 302 ke header Referer.

### Langkah 4: Refactor `AuthController.rs`
Hapus manual validasi di `AuthController::register` dan `AuthController::login`. Ganti parameter `axum::Form(form)` menjadi `ValidatedForm(form)`.

## ✅ Kriteria Penerimaan
- Saat form login disubmit tanpa email, browser otomatis memantul kembali (redirect) ke halaman login dengan menampilkan pesan error validasi di bawah kolom input, TANPA ada kode pengecekan error di dalam `login` controller.
