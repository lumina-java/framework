# 📘 Modul 01: Persyaratan Sistem & Instalasi

Selamat datang di dokumentasi resmi **Lumina Framework**! Modul ini membahas semua hal yang Anda butuhkan untuk memulai pengembangan aplikasi dengan Lumina dari awal.

---

## 1. Persyaratan Sistem (System Requirements)

Sebelum menggunakan Lumina, pastikan sistem komputer Anda telah terpasang:

- **Rust Compiler & Cargo**: Versi **1.75+** (Stable).
  - Cek versi: `rustc --version` dan `cargo --version`
  - Panduan instalasi: [rustup.rs](https://rustup.rs)
- **Database Engine** (Pilih salah satu atau beberapa sesuai kebutuhan):
  - **SQLite**: Cocok untuk pemula, prototyping cepat, zero-config tanpa server database tambahan.
  - **PostgreSQL**: Direkomendasikan untuk production berkinerja tinggi.
  - **MySQL / MariaDB**: Alternatif populer untuk aplikasi web standar.
- **Git**: Untuk inisialisasi repositori otomatis dan version control.
- *(Opsional)* **WebAssembly Target**: Jika Anda ingin mengembangkan plugin WASM:
  ```bash
  rustup target add wasm32-unknown-unknown
  ```

---

## 2. Instalasi Lumina CLI (`lumina-cli`)

Lumina dilengkapi dengan toolchain Command Line Interface (CLI) bawaan yang elegan dan canggih bernama `lumina`.

### Cara 1: Pasang Global via Crates.io
```bash
cargo install lumina-cli --force
```

### Cara 2: Pasang Langsung dari Repositori GitHub Resmi
```bash
cargo install --git https://github.com/lumina-java/framework.git lumina-cli --force
```

Setelah instalasi selesai, verifikasi dengan menjalankan:
```bash
lumina --version
lumina --help
```

### Cara 3: Auto-Upgrade Toolchain & Dependensi Proyek
```bash
lumina upgrade
```

### Cara 4: Instalasi Lokal dari Source Code
Jika Anda berkontribusi atau memodifikasi source code Lumina:
```bash
# Clone repositori framework
git clone https://github.com/lumina-java/framework.git
cd framework

# Install biner CLI ke lingkungan Cargo lokal
cargo install --path lumina-cli --force
```

---

## 3. Membuat Proyek Baru (`lumina new`)

Lumina menyediakan flag `--db` untuk mengonfigurasi driver database dan file `.env` secara instan, serta memangkas waktu kompilasi `cargo build` awal hingga **45%**.

### Opsi A: SQLite (Tercepat, Zero-Config)
```bash
lumina new my-blog --db sqlite
```
> Otomatis membuat file `database.sqlite` dan menyiapkan dependensi SQLite yang ramping.

### Opsi B: PostgreSQL (Production Ready)
```bash
lumina new enterprise-app --db postgres
```
> Menyiapkan driver PostgreSQL berkinerja tinggi dan template koneksi standard.

### Opsi C: MySQL / MariaDB
```bash
lumina new store-app --db mysql
```

### Opsi D: Multi-Driver (Semua Driver Aktif)
```bash
lumina new portal-app --db all
```

---

## 4. Struktur Direktori Proyek

Lumina menerapkan arsitektur **Model-View-Controller (MVC)** yang terorganisir rapi dan intuitif:

```text
my-app/
├── app/
│   └── scripts/             # Skrip Indonesian Script (.is) backend
├── database/
│   ├── migrations/          # File migrasi SQL skema tabel
│   └── seeders/             # Data seeding untuk pengujian/dummy
├── plugins/                 # Plugin WebAssembly (.wasm) dan source plugin
├── public/                  # Static assets publik (gambar, font, favicon)
├── resources/
│   ├── css/                 # Stylesheet CSS kustom
│   ├── js/                  # JavaScript frontend
│   └── views/               # Template Blade (.blade.rs)
│       ├── layouts/         # Layout dasar (@extends)
│       └── auth/            # Halaman login, register, profil
├── src/
│   ├── app/
│   │   ├── controllers/     # Controller handlers HTTP
│   │   ├── models/          # Model data (LuminaModel derive)
│   │   ├── requests/        # Validasi Form/JSON request
│   │   └── services/        # Business logic layer
│   ├── routes/
│   │   ├── api.rs           # Endpoint RESTful API
│   │   └── web.rs           # Route halaman web
│   └── main.rs              # Entrypoint server aplikasi
├── storage/
│   ├── app/public/          # Berkas upload user
│   ├── cache/               # Cache aplikasi
│   └── logs/                # File log perputaran harian
├── .env                     # Konfigurasi environment lokal aktif
├── .env.example             # Template konfigurasi environment
└── Cargo.toml               # Dependensi Rust & Cargo crate
```

---

## 5. Konfigurasi Environment (`.env`)

File `.env` otomatis digenerate saat project dibuat. Contoh konfigurasi SQLite:

```env
# Pengaturan Aplikasi
APP_NAME=Lumina
APP_ENV=local
APP_DEBUG=true
APP_URL=http://localhost:8000
APP_KEY=lumina-super-secret-key-change-me-in-production

# Logging
LOG_LEVEL=info
LOG_TO_FILE=true

# Database Configuration (SQLite)
DB_CONNECTION=sqlite
DB_DATABASE=./database.sqlite
DATABASE_URL=sqlite:./database.sqlite

# Session & JWT
JWT_SECRET=lumina-jwt-secret-key-change-me-in-production
RATE_LIMIT_MAX=60
RATE_LIMIT_WINDOW=60
```

---

## 6. Menjalankan Server Aplikasi

Masuk ke folder proyek yang baru dibuat:
```bash
cd my-blog
```

### Menjalankan Server Standar:
```bash
lumina serve
# atau langsung menggunakan: cargo run
```

### Menjalankan Server dengan Auto-Reload (Hot-Reloading):
Jika Anda menginstal `cargo-watch`, server akan otomatis restart setiap kali Anda menyimpan perubahan kode:
```bash
lumina watch
```

Buka browser Anda dan akses:
👉 **`http://localhost:8000`**

---

### Navigasi Modul:
- **Berikutnya**: [Modul 02: Routing & Middleware](./02_routing_dan_middleware.md) ➔
- **Kembali ke**: [Daftar Isi Dokumentasi](./README.md)
