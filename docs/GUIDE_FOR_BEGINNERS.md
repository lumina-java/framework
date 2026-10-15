# 🚀 Panduan Lumina untuk Pemula

Selamat datang di Lumina Framework! Panduan ini dirancang untuk membantu Anda memahami konsep dasar dan alur kerja Lumina dalam hitungan menit.

## 1. Filosofi Lumina
Lumina dirancang agar developer yang terbiasa dengan framework seperti Laravel dapat langsung produktif di ekosistem Rust. Lumina menyembunyikan kompleksitas Rust di balik API yang *fluent* dan alat bantu CLI yang kuat.

## 2. Struktur Proyek
*   `routes/`: Tempat mendefinisikan URL aplikasi (`web.rs` untuk HTML, `api.rs` untuk JSON).
*   `src/app/controllers/`: Logika untuk menangani request.
*   `src/app/models/`: Definisi tabel database dan strukturnya.
*   `src/app/services/`: (Baru!) Logika bisnis yang dipisahkan dari controller.
*   `resources/views/`: Template UI menggunakan sistem `.blade.rs`.
*   `database/migrations/`: File SQL untuk mengubah struktur database.

## 3. Alur Kerja "Blueprint" (Scaffolding)
Cara tercepat membuat fitur di Lumina adalah menggunakan perintah `make:crud`.
```bash
./lumina make:crud Patient
```
Perintah ini akan membuatkan Anda:
1.  **Model & Migration** (Database)
2.  **Service Layer** (Bisnis Logik)
3.  **Request Validator** (Validasi Form)
4.  **Controller** (Handler)
5.  **Views** (UI: Index, Create, Edit)
6.  **Auto-Registration**: Semuanya otomatis terdaftar di routes dan menu!

## 4. Routing & Grouping
Gunakan grup untuk mengelola route yang memiliki prefix atau middleware yang sama.
```rust
router.group("/admin", |r| {
    r.middleware(from_fn(auth_required))
     .get("/dashboard", DashboardController::index)
});
```

## 5. Storage (Multi-Disk)
Lumina mendukung penyimpanan lokal dan S3 secara transparan.
```rust
// Simpan file ke disk default
req.storage().put("foto.jpg", data).await;

// Simpan ke S3
req.storage().disk("s3").put("backup.zip", data).await;
```

## 6. Mail & Notification
Mengirim email sangat mudah dengan API yang fluent.
```rust
req.mail()
    .to("user@example.com")
    .subject("Welcome")
    .send("emails.welcome", json!({"name": "Budi"}))
    .await;
```

## 7. Deployment (Instan)
Lumina sudah siap untuk produksi. Gunakan perintah berikut untuk menyiapkan server:
```bash
./lumina make:docker      # Siapkan Docker
./lumina make:nginx       # Siapkan Reverse Proxy
./lumina make:supervisor  # Siapkan Process Manager
```

---
**Tips**: Gunakan `./lumina --help` untuk melihat semua perintah yang tersedia. Selamat berkarya dengan Rust! 🦀✨
