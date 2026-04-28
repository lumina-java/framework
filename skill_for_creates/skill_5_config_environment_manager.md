# Skill 5: Implementasi Config & Environment Manager

## Deskripsi / Tujuan
Aplikasi sering perlu membaca konfigurasi dari file `.env` di berbagai tempat (misalnya untuk parameter *third-party API*, *database credentials*, dsb). Modul ini akan menjadi Singleton/Global Config Manager yang memuat semua data `.env` ke *memory* sekali saat aplikasi *startup*, dan dapat diakses dengan metode `Config::get("key")`.

## Kebutuhan Teknis
- Menggunakan crate `dotenv` yang sudah ada di dependensi.
- Tambahkan kemampuan untuk mengelompokkan konfigurasi atau mem-parsing `.env` ke dalam *struct* dengan skema statis agar *type-safe* menggunakan `serde` atau `envconfig`.
- Mencegah inisialisasi `.env` berulang kali di tiap controller.

## Langkah-langkah Implementasi
1. Buat direktori `src/config/` (jika belum komplit) dan file `src/config/manager.rs`.
2. Definisikan struct `AppConfig` yang menampung konfigurasi dari `.env` (contoh: host, port, database_url, jwt_secret).
3. Buat mekanisme singleton atau gunakan pola `Arc<AppConfig>` pada inisialisasi `AppState`.
4. Ganti penggunaan raw `env("DATABASE_URL")` di `application.rs` menjadi menggunakan metode manajer konfigurasi terpusat.
5. (Opsional tapi disarankan) buat helper global `Config::get("app.url")` jika memakai implementasi global statis (`once_cell` / `lazy_static`).

## Instruksi Git & Pull Request
- Buat *branch*: `git checkout -b feature/config-manager`.
- Setelah selesai implementasi, jangan komit file `.env` yang sebenarnya (pastikan masuk ke `.gitignore`). Hanya tambahkan properti ke `.env.example`.
- Lakukan Pull Request ke *master*.

## Panduan User Test Manual
1. Buka `.env` dan tambahkan variabel *custom*, misalnya `APP_NAME=Lumina_Test_123`.
2. Di dalam sebuah controller/handler web (misalnya beranda `/`), ambil konfigurasi tersebut dengan `Config::get("APP_NAME")` atau via ekstensi state.
3. Cetak variabel tersebut ke respon teks atau HTML.
4. Buka `http://localhost:3000` di *browser*.
5. Pastikan teks "Lumina_Test_123" tampil di layar.
6. Ubah nilai di `.env` menjadi "Lumina_Test_456" dan restart server, pastikan nilai baru ikut berubah.
