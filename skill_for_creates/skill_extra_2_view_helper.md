# Skill Extra 2: Elegant View Helper

## 🎯 Objektif
Membuat cara merender tampilan (views) HTML dari template Tera menjadi jauh lebih cantik dan *readable* menyerupai fungsi `view()` milik Laravel. 

## 📋 Deskripsi Tugas
Saat ini, untuk merender sebuah template, kita harus menulis:
```rust
let mut context = tera::Context::new();
context.insert("csrf_token", &token.authenticity_token().unwrap());
context.insert("user", &user);

let rendered = state.view.render_with_session("auth/login.html", context, &session).await;
return (token, Html(rendered)).into_response();
```
Kode ini terlalu mekanis dan panjang. Kita ingin mengubahnya menjadi sebuah rantai (*builder pattern*) yang indah:
```rust
return View::make("auth/login")
    .with("csrf_token", token.authenticity_token().unwrap())
    .with("user", user)
    .render(&state, &session)
    .await
    .into_response(token); // otomatis membungkus HTML dan CsrfToken
```

## 🛠️ Langkah Eksekusi (Untuk Junior Programmer / AI)

### Langkah 1: Buat Struct `ViewBuilder`
Buat struktur baru di `src/core/view.rs` bernama `ViewBuilder`:
```rust
pub struct ViewBuilder {
    template: String,
    context: tera::Context,
}
```

### Langkah 2: Implementasi Builder Methods
1. Tambahkan metode statis `pub fn make(template: &str) -> Self`. Inisialisasi struktur `ViewBuilder` dengan context baru.
2. Tambahkan metode berantai `pub fn with<T: Serialize>(mut self, key: &str, value: T) -> Self` untuk memasukkan variabel ke dalam `tera::Context`.

### Langkah 3: Implementasi Render Method
Tambahkan metode eksekusi `.render(state: &AppState, session: &tower_sessions::Session)` yang akan secara asinkron memanggil `state.view.render_with_session`. Metode ini harus mengembalikan struktur perantara yang menampung string hasil render.

### Langkah 4: Implementasi Helper untuk `IntoResponse`
Kombinasikan string hasil render dengan `CsrfToken` (yang selalu wajib ada di setiap respons Axum Lumina) menjadi sebuah helper function `into_response(self, token: CsrfToken)` sehingga menghasilkan `impl IntoResponse` yang secara otomatis terbungkus di dalam `axum::response::Html`.

### Langkah 5: Refactoring Controller
Perbarui semua Controller (`HomeController`, `AuthController`) yang merender halaman HTML agar menggunakan struktur baru `View::make()`.

## ✅ Kriteria Penerimaan
- Minimal 30% baris kode di setiap Controller yang merender HTML berkurang.
- Tidak ada lagi pemanggilan `tera::Context::new()` manual di Controller.
- Response secara konsisten dan aman dapat di-return lengkap dengan header HTML dan sinkronisasi `CsrfToken`.
