# Skill 1: Implementasi Macro-based ORM (Query Builder)

## Deskripsi / Tujuan
Saat ini framework Lumina menggunakan raw query SQL via `sqlx`. Tujuan tugas ini adalah membangun sebuah *Macro-based ORM* (ActiveRecord pattern) atau abstraksi *Query Builder* sederhana. Hal ini bertujuan agar *developer* dapat mengambil data dengan sintaks yang mirip dengan Eloquent di Laravel (misal: `User::find(1)`).

## Kebutuhan Teknis
- Jangan menghapus koneksi `sqlx` yang sudah ada, melainkan buat *wrapper* di atasnya.
- Buat abstraksi struct/trait `Model` atau `QueryBuilder`.
- Pertimbangkan untuk membuat *Procedural Macro* (opsional tapi disarankan) agar model yang ada di `src/app/models/` bisa otomatis mendapatkan method CRUD dasar (`find`, `all`, `create`, `update`, `delete`).

## Langkah-langkah Implementasi
1. Buat direktori atau file baru `src/database/orm.rs` atau `src/core/orm.rs`.
2. Buat trait `Model` yang mendefinisikan *interface* ORM.
3. Implementasikan fungsi `find`, `where`, dan `get`.
4. (Opsional) Buat sub-crate terpisah (contoh: `lumina-macros`) di dalam workspace untuk Procedural Macros `#[derive(Model)]`.
5. Modifikasi contoh `User` model (jika ada) untuk menggunakan abstraksi ORM ini.

## Instruksi Git & Pull Request
- Buat *branch* baru: `git checkout -b feature/macro-based-orm`.
- Kerjakan perubahan pada *branch* tersebut.
- Jangan melakukan *push* langsung ke `master`.
- Setelah selesai, *push branch* dan buat Pull Request (PR) ke `master`.

## Panduan User Test Manual
1. Pastikan database aktif dan migrasi sudah berjalan.
2. Buat sebuah rute pengujian di `src/core/router.rs` (contoh: `GET /test-orm`).
3. Di dalam *handler* rute tersebut, cobalah memanggil `User::find(1).await` (atau model lain).
4. Kembalikan hasilnya dalam bentuk JSON.
5. Akses `http://localhost:3000/test-orm` melalui browser atau Postman.
6. Pastikan respon mengembalikan data *record* dari database dengan benar tanpa harus menulis query mentah `SELECT * FROM users WHERE id = 1`.
