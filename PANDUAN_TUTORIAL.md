# 🚀 PANDUAN LENGKAP LUMINA FRAMEWORK (STYLE LARAVEL)

Panduan praktis pembuatan project web dari nol hingga berjalan menggunakan **Lumina Framework**. Alur ini dirancang agar terasa sangat akrab bagi developer yang terbiasa dengan **Laravel & Composer / PHP Artisan**.

---

## ⚡ Perbandingan Alur (Laravel vs Lumina)

| Tahap | 🐘 Laravel (PHP) | 🦀 Lumina (Rust) |
| :--- | :--- | :--- |
| **1. Install CLI/Tool** | `composer` | `cargo` (Sudah include saat install Rust) |
| **2. Install Framework CLI** | `composer global require laravel/installer` | `cargo install cargo-lumina` |
| **3. Buat Project Baru** | `composer create-project laravel/laravel my-app` <br>*atau* `laravel new my-app` | `cargo lumina new my-app` |
| **4. Masuk ke Folder** | `cd my-app` | `cd my-app` |
| **5. Jalankan Server** | `php artisan serve` | `cargo lumina serve` |
| **6. Buka Browser** | `http://localhost:8000` | `http://localhost:8000` |

---

## 🛠️ Langkah Demi Langkah (Dari Nol sampai Berjalan)

### 1️⃣ Prasyarat: Install Rust (Cukup Sekali)
Pastikan komputer kamu sudah memiliki Rust. Jika belum, install melalui perintah resmi:

* **Linux / macOS:**
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```
* **Windows:**
  Unduh installer `rustup-init.exe` dari [https://rustup.rs](https://rustup.rs/) dan jalankan.

---

### 2️⃣ Install Lumina CLI (Global)
Install perkakas CLI Lumina menggunakan `cargo` (Package Manager Rust):

```bash
cargo install cargo-lumina
```

*Catatan: Setelah perintah di atas selesai, perintah `cargo lumina` atau `lumina` sudah dapat dipanggil dari mana saja di terminal kamu.*

---

### 3️⃣ Buat Project Baru (`cargo lumina new`)
Sama seperti `composer create-project laravel/laravel toko_online`, jalankan perintah berikut untuk membuat kerangka project Lumina baru:

```bash
cargo lumina new toko_online
```

Lumina akan otomatis menyiapkan seluruh struktur folder MVC (Controllers, Models, Views, Routes, Database SQLite otomatis, `.env`, dan konfigurasi Cargo).

---

### 4️⃣ Jalankan Server (`cargo lumina serve`)
Masuk ke folder project yang baru dibuat, lalu jalankan server pengembangannya:

```bash
cd toko_online
cargo lumina serve
```

*(Atau kamu juga bisa menggunakan perintah standar `cargo run`)*.

---

### 5️⃣ Buka Browser
Buka browser favorit kamu dan kunjungi alamat:

👉 **http://localhost:8000**

🎉 **Selamat!** Halaman awal (Homepage) Lumina Framework kamu sudah berhasil tampil!

---

## 💡 Fitur Perintah Tambahan (Mirip `php artisan`)

Lumina CLI menyediakan perintah-perintah generator ala `php artisan`:

* **`cargo lumina serve`** ➔ Jalankan server lokal (`php artisan serve`)
* **`cargo lumina watch`** ➔ Jalankan server dengan *hot-reload* otomatis saat kode diubah
* **`cargo lumina make:controller Product`** ➔ Buat Controller baru (`php artisan make:controller`)
* **`cargo lumina make:model Product`** ➔ Buat Model & Migrasi baru (`php artisan make:model`)
* **`cargo lumina make:crud Product name:string price:integer`** ➔ Generate CRUD lengkap (Model, Controller, Migration, Views)
* **`cargo lumina make:auth`** ➔ Generate scaffolding Login & Register siap pakai
* **`cargo lumina migrate`** ➔ Jalankan migrasi database (`php artisan migrate`)
