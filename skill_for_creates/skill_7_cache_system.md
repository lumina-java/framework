# Skill 7: Implementasi Cache System (Cache Manager)

## Deskripsi / Tujuan
Sistem Cache sangat penting dalam framework untuk menyimpan sementara data yang sering diakses agar tidak terus-menerus melakukan query ke database. Fitur ini mengadopsi gaya Laravel (`Cache::get`, `Cache::put`, `Cache::remember`), memungkinkan aplikasi berjalan lebih cepat dan efisien.

## Kebutuhan Teknis
- Buat abstraksi **Cache Manager** di `src/core/cache/mod.rs`.
- Untuk MVP, gunakan **In-Memory Cache** (misal menggunakan `moka` atau sekadar `std::collections::HashMap` dengan `RwLock` dan dukungan *Time-To-Live* / TTL).
- (Opsional/Lanjutan) Mendukung driver **Redis** menggunakan crate `redis`.
- Metode yang wajib ada:
  - `put(key, value, ttl)`
  - `get(key)`
  - `forget(key)`
  - `remember(key, ttl, closure)` (Jika memungkinkan di Rust menggunakan async closure).

## Langkah-langkah Implementasi
1. Buat modul `src/core/cache/mod.rs`.
2. Buat struct `CacheManager` yang membungkus mekanisme penyimpanan (In-Memory).
3. Tambahkan `cache: Arc<CacheManager>` ke dalam `AppState` di `src/core/application.rs`.
4. Buat perintah CLI `./lumina make:cache` jika diperlukan, atau sekadar tambahkan dokumentasi.
5. Daftarkan di `src/core/mod.rs`.

## Instruksi Git & Pull Request
- Checkout branch baru: `git checkout -b feature/cache-system`.
- Tulis kode dengan rapi dan pastikan *thread-safe* (`Send + Sync`).
- Buka Pull Request dan beri judul "feat(cache): Implement Cache System (Skill 7)".

## Panduan User Test Manual
1. Buat route `/api/test-cache`.
2. Di dalam handler, coba ambil data dari cache menggunakan key `test_key`.
3. Jika kosong, simpan data string "Lumina Cache Berhasil!" dengan TTL 10 detik, lalu return response.
4. Hit endpoint tersebut berulang-ulang. Anda akan melihat bahwa pada hit pertama data diset, dan pada hit berikutnya data langsung diambil dari cache.
5. Tunggu 11 detik, lalu hit kembali. Data seharusnya sudah terhapus (expired) dan diset ulang.
