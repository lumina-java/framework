# ⚡ Lumina CLI (`lumina`)

> **"Look like Laravel, Run at Rust Speed."**  
> *The official Artisan-like command-line toolchain for Lumina Framework.*

[![License: MIT](https://img.shields.io/badge/License-MIT-emerald.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-2021%20edition-orange.svg)](https://www.rust-lang.org)
[![Lumina Framework](https://img.shields.io/badge/ecosystem-Lumina%20Framework-blue.svg)](https://github.com/lumina-java/framework)

---

## 🌟 Tentang Lumina CLI

`lumina-cli` menghadirkan keajaiban pengalaman pengembang (*Developer Experience*) ala **Laravel Artisan** ke dalam dunia **Rust**. Dirancang untuk membangkitkan proyek, merancang boilerplate secara otomatis (*scaffolding*), mengeksekusi migrasi database, mengelola antrean latar belakang (*background queue*), hingga menjalankan audit keamanan proyek secara instan.

---

## 📦 Instalasi

### Dari GitHub (Rekomendasi Terkini)
```bash
cargo install --git https://github.com/lumina-java/lumina-cli
```

### Atau Build dari Sumber
```bash
git clone https://github.com/lumina-java/lumina-cli.git
cd lumina-cli
cargo install --path .
```

Verifikasi instalasi dengan mengetik:
```bash
lumina --version
```

---

## 🚀 Panduan Penggunaan & Perintah Lengkap

### 1. Inisialisasi Proyek Baru
Membangkitkan struktur aplikasi Lumina lengkap dengan pilihan database:
```bash
# Proyek baru dengan database SQLite (default)
lumina new my_app

# Proyek baru dengan database PostgreSQL
lumina new my_app --db postgres

# Proyek baru dengan database MySQL
lumina new my_app --db mysql

# Mode offline (menggunakan embedded template)
lumina new my_app --offline
```

---

### 2. Menjalankan Server & Pengembangan
```bash
# Menjalankan server aplikasi (cargo run)
lumina serve

# Menjalankan server dengan live-reload otomatis (cargo watch)
lumina watch
```

---

### 3. Artisan-Style Generators (`make:*`)

Bebaskan diri Anda dari penulisan *boilerplate* manual:

```bash
# Membangkitkan Controller baru
lumina make:controller UserController

# Membangkitkan Model ORM + Berkas Migrasi otomatis
lumina make:model Product

# Membangkitkan berkas migrasi database kosong
lumina make:migration create_orders_table

# Membangkitkan Full CRUD instan (Model, Controller, Migrasi, Blade Views)
lumina make:crud Post title:string content:text is_published:boolean

# Membangkitkan sistem Autentikasi lengkap (Register, Login, Session, Model User)
lumina make:auth

# Membangkitkan Model Factory & Faker untuk unit testing dan seeding
lumina make:factory UserFactory

# Membangkitkan template view Blade Native
lumina make:view dashboard.analytics

# Membangkitkan Background Queue Job
lumina make:job SendInvoiceEmail

# Membangkitkan Komponen Reaktif Lumina Live (Server-Driven Reactive UI)
lumina make:live Counter
```

---

### 4. Database Migration Engine
Eksekusi migrasi database transaksional tanpa memerlukan dependensi eksternal:
```bash
# Jalankan semua migrasi pending
lumina migrate

# Batalkan (rollback) batch migrasi terakhir
lumina migrate:rollback
```

---

### 5. Background Queue Worker
Jalankan pemrosesan antrean asinkron untuk email, notifikasi, dan kalkulasi berat:
```bash
# Menjalankan worker pada queue default
lumina queue:work

# Menjalankan worker pada antrean khusus
lumina queue:work --queue high-priority
```

---

### 6. Keamanan & Pemeliharaan Proyek
```bash
# Lumina Shield: Audit keamanan otomatis untuk file .env, dependensi, dan secret leak
lumina audit

# Membersihkan cache kompilasi compiler & build artifacts (membebaskan ruang disk)
lumina clean
lumina clean --purge

# Memperbarui toolchain Lumina CLI dan dependensi framework
lumina upgrade
```

---

## 📋 Ringkasan Perintah

| Perintah | Deskripsi |
| :--- | :--- |
| `lumina new <name>` | Membangkitkan skeleton aplikasi baru (*batteries-included*) |
| `lumina serve` | Menjalankan server aplikasi di port 8000 |
| `lumina watch` | Server dengan hot-reloading otomatis saat kode berubah |
| `lumina make:controller` | Membuat controller web/API |
| `lumina make:model` | Membuat model ORM `LuminaModel` |
| `lumina make:crud` | Membangkitkan seluruh alur CRUD dalam 1 detik |
| `lumina make:auth` | Sistem autentikasi instan (Login, Register, Session) |
| `lumina make:live` | Komponen reaktif fullstack Lumina Live |
| `lumina make:factory` | Model Factory & Faker Indonesia |
| `lumina migrate` | Menjalankan migrasi database SQLx |
| `lumina migrate:rollback`| Rollback migrasi database terakhir |
| `lumina queue:work` | Menjalankan background queue consumer |
| `lumina audit` | Pengecekan audit keamanan konfigurasi |
| `lumina clean` | Membersihkan cache build target yang membengkak |
| `lumina upgrade` | Memperbarui CLI ke versi termutakhir |

---

## 🤝 Berkontribusi (Contributing)

Kontribusi dari komunitas sangat disambut dengan antusias!
1. Fork repositori ini.
2. Buat branch fitur baru (`git checkout -b feature/nama-fitur`).
3. Commit perubahan Anda (`git commit -m 'feat: tambah perintah baru'`).
4. Push ke branch Anda (`git push origin feature/nama-fitur`).
5. Buat **Pull Request**.

---

## 📜 Lisensi

Lumina CLI dirilis di bawah lisensi terbuka [MIT License](LICENSE).  
Dikelola dengan bangga oleh tim pengembang [Lumina Framework](https://github.com/lumina-java/framework).

