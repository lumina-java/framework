# Skill: Beginner-Friendly Developer Experience (DX)

Dokumen ini berisi panduan dan ide implementasi untuk menjadikan Lumina Framework sangat ramah bagi pemula ("manusia awam"). Skill ini ditujukan bagi AI / Junior Programmer yang akan mengeksekusi penyempurnaan framework.

## 1. Beautiful Error Page (Ignition Style)
**Tujuan:** Mengganti panic stack trace dan error Tera/SQL yang kaku dengan tampilan HTML yang ramah.
**Langkah Implementasi:**
- Buat sebuah `PanicHook` custom atau middleware global yang menangkap `panic` atau `Error`.
- Siapkan file template internal (misal `resources/views/errors/exception.html`).
- Tampilkan stack trace secara visual, menyoroti baris kode (code snippet) di mana error terjadi.
- Berikan saran solusi (contoh: "Tabel tidak ditemukan. Jalankan ./lumina migrate").

## 2. CLI Magic: make:crud & make:auth
**Tujuan:** Mempercepat scaffolding aplikasi.
**Langkah Implementasi:**
- Buka `src/bin/lumina.rs` (atau script CLI yang ada).
- Tambahkan argumen `make:auth`:
  - Otomatis membuat `AuthController` lengkap dengan login/register logic.
  - Generate view `login.html` & `register.html` menggunakan CSS bawaan.
  - Daftarkan route di `routes/web.rs`.
- Tambahkan argumen `make:crud <Name>`:
  - Generate `<Name>Controller` (index, create, store, edit, update, delete).
  - Generate `<Name>` Model dan file Migration SQL-nya.
  - Generate folder view `resources/views/<name>/` dengan file `index.html`, `create.html`, dan `edit.html`.

## 3. Smart .env Checker
**Tujuan:** Mencegah error membingungkan di awal instalasi karena lupa konfigurasi `.env`.
**Langkah Implementasi:**
- Di fungsi `main()` (`src/main.rs`), sebelum inisialisasi Database Pool.
- Cek keberadaan file `.env`. Jika tidak ada, print peringatan berwarna kuning/merah: "Peringatan: File .env tidak ditemukan. Silakan copy dari .env.example".
- Cek variabel krusial seperti `DATABASE_URL`. Jika kosong, berikan panduan cara mengisinya.

## 4. Built-in View Macros (Komponen UI)
**Tujuan:** Mempermudah penulisan HTML, mirip seperti komponen Blade Laravel (`<x-alert>`).
**Langkah Implementasi:**
- Di dalam setup `Tera` (`src/core/view.rs`), daftarkan custom macro atau simpan file `resources/views/macros.html` yang diload otomatis.
- Buat macro untuk:
  - `{{ form_error(field="nama") }}`: Menampilkan text merah jika ada error validasi di field tersebut.
  - `{{ alert_flash() }}`: Merender HTML alert berdasarkan flash session (success/error).

## 5. Quick Start Tutorial (10-Menit)
**Tujuan:** Memberikan 'Aha! moment' tercepat.
**Langkah Implementasi:**
- Buat file `docs/10_MINUTES_TUTORIAL.md`.
- Tulis langkah-langkah membuat aplikasi "To-Do List" lengkap dari `git clone` hingga selesai dalam 5-10 langkah menggunakan CLI `make:crud`.
