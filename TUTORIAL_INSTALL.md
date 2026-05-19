# 🚀 Panduan Instalasi & Penggunaan Lumina Framework (Untuk Pemula)

Selamat datang di **Lumina Framework**! Lumina adalah framework web MVC modern berbasis Rust yang menggabungkan kecepatan super cepat dan keamanan memori dari Rust dengan kemudahan dan kenyamanan pengembangan seperti Laravel.

Panduan ini dirancang khusus untuk pemula agar dapat menginstal dan menggunakan CLI Lumina dengan mudah dan tanpa hambatan.

---

## 📋 Prasyarat Sistem

Sebelum menginstal Lumina, pastikan komputer Anda telah terpasang:
1. **Rust & Cargo** (Gunakan [Rustup](https://rustup.rs/) untuk instalasi mudah).
2. **Git** (Diperlukan untuk inisialisasi repositori project baru).
3. **SQLite / PostgreSQL / MySQL** (Tergantung database yang ingin Anda gunakan, SQLite adalah default).

---

## ⚡ Langkah 1: Opsi Metode Instalasi (Pilih Salah Satu)

Sama seperti Laravel yang mendukung instalasi melalui global installer, composer, maupun manual cloning, Lumina Framework menyediakan **tiga metode instalasi modern** agar dapat digunakan secara fleksibel:

### 🌟 Metode A: Menggunakan Lumina Installer (`cargo-lumina`) — *Sangat Direkomendasikan (Ala Laravel Installer)*
Ini adalah cara tercepat dan paling elegan menggunakan Cargo subcommand. CLI terintegrasi langsung sebagai modul internal toolchain Rust Anda.
1. **Instal installer-nya secara global:**
   ```bash
   cargo install cargo-lumina
   ```
2. **Buat proyek baru dari direktori mana saja dengan mudah:**
   ```bash
   cargo lumina new nama_project_anda
   ```

### 📦 Metode B: Menggunakan Standalone CLI (`lumina`) dari Git Source
Jika Anda ingin mengompilasi langsung dari source code repositori GitHub resmi untuk mendapatkan fitur paling mutakhir:
1. **Clone repositori dan masuk ke folder framework:**
   ```bash
   git clone https://github.com/lumina-java/framework.git
   cd framework
   ```
2. **Instal binary `lumina` secara global:**
   ```bash
   cargo install --path .
   ```
3. **Buat proyek baru menggunakan command standalone:**
   ```bash
   lumina new nama_project_anda
   ```

### 📁 Metode C: Mengunduh Template Skeleton Secara Langsung
Jika Anda ingin mengunduh folder boilerplate secara manual tanpa memasang installer global:
1. **Cukup unduh / clone berkas skeleton Lumina dan ubah nama foldernya:**
   ```bash
   git clone https://github.com/lumina-java/framework.git
   # Ambil folder skeleton bawaan di dalam folder 'src/cli/stubs' atau salin folder kerangka dasar.
   ```

---

### 🔍 Verifikasi Instalasi CLI
Setelah memasang Metode A atau Metode B, pastikan CLI Lumina sudah aktif dan terdaftar dengan menjalankan salah satu perintah di bawah ini:
```bash
# Jika menggunakan Metode A:
cargo lumina --help

# Jika menggunakan Metode B:
lumina --help
```
*Lumina akan menampilkan antarmuka bantuan yang sangat premium dengan daftar perintah lengkap!*

---

## 🆕 Langkah 2: Membuat Project Baru

Tergantung metode instalasi yang Anda pilih di Langkah 1, buat proyek baru Anda dengan perintah di bawah ini:

* **Untuk Pengguna Metode A (`cargo-lumina`):**
  ```bash
  cargo lumina new nama_project_anda
  ```
* **Untuk Pengguna Metode B (`lumina` standalone):**
  ```bash
  lumina new nama_project_anda
  ```

### Apa yang terjadi di balik layar?
Lumina secara otomatis me-generate seluruh kerangka kerja (scaffolding) MVC secara mandiri di komputer Anda:
- 📁 Struktur folder lengkap (`src/app/controllers`, `src/app/models`, `resources/views`, dsb).
- ⚙️ Konfigurasi `.env` dan `.env.example` yang siap pakai.
- 📦 File dependensi `Cargo.toml` yang menunjuk ke core framework Lumina.
- 🐙 Inisialisasi repositori Git baru.

---

## 🏃 Langkah 3: Menjalankan Project

Masuk ke folder project yang baru dibuat dan jalankan servernya:

```bash
cd nama_project_anda
cargo run
```

Buka browser Anda dan akses:
👉 **[http://localhost:8000](http://localhost:8000)**

Anda akan disambut oleh halaman **Welcome Lumina** dengan desain premium dan modern!

---

## 🛠️ Langkah 4: Menggunakan Fitur CLI (Scaffolding Cepat)

Semua perintah pembuatan fitur CLI dapat dijalankan di dalam direktori project Anda untuk mempercepat pengembangan:

### A. Membuat Controller Baru
```bash
lumina make:controller ProductController
```
*Akan membuat file di `src/app/controllers/product_controller.rs` dan otomatis mendaftarkannya di `controllers/mod.rs`.*

### B. Membuat Model & Migrasi Baru
```bash
lumina make:model Product
lumina make:migration create_products_table
```
*Membuat model di `src/app/models/product.rs` dan file SQL migrasi di `database/migrations/`.*

### C. Membuat CRUD Lengkap (Model, Controller, Migrasi, View, Rute & Menu)
```bash
lumina make:crud Book title:string price:integer description:text
```
*Ini adalah fitur ajaib Lumina! Perintah ini akan langsung menghasilkan seluruh halaman CRUD lengkap yang fungsional beserta form, tabel, sidebar menu, rute, controller, dan migrasi.*

### D. Membuat Sistem Autentikasi Instan (Register & Login)
```bash
lumina make:auth
```
*Ini adalah fitur praktis Lumina untuk keamanan sistem! Hanya dengan satu perintah, CLI secara otomatis melakukan seluruh alur berikut tanpa konfigurasi manual:*
* **Membuat Halaman View:** Menghasilkan file tampilan premium untuk form **Register** dan **Login** (`resources/views/auth/`).
* **Membuat & Mendaftarkan Controller:** Menghasilkan file `src/app/controllers/auth_controller.rs` dan mendaftarkannya otomatis di `controllers/mod.rs`.
* **Sinkronisasi Rute Pintar:** Mendeteksi berkas rute Anda (`routes/web.rs`), menghapus rute welcome statis yang lama, dan langsung menyuntikkan rute dinamis `GET` & `POST` lengkap untuk alur Login, Register, dan Logout.
* **Integrasi Model User:** Memeriksa dan memperbarui model database `User` (`src/app/models/user.rs`) untuk menambahkan field `role` serta method verifikasi email bawaan.

*Setelah perintah selesai dijalankan, Anda hanya perlu mengetik perintah `lumina migrate` untuk memastikan tabel database siap, lalu jalankan `lumina serve` untuk langsung menguji sistem autentikasi fungsional Anda!*

---

## 🐳 Langkah 5: Fitur Deployment & Auto Host (Docker / Nginx / Supervisor)

Lumina menyediakan automasi untuk deployment ke server produksi (auto-hosting/VPS) dengan standar industri:

### 1. Auto-Config Docker
Jalankan perintah berikut di root project:
```bash
lumina make:docker
```
*Akan menghasilkan file `Dockerfile` dan `docker-compose.yml` produksi yang terintegrasi dengan Nginx proxy, PostgreSQL, dan Redis.*

Untuk menjalankannya secara kontainerisasi di VPS:
```bash
docker-compose up -d --build
```

### 2. Auto-Config Nginx Reverse Proxy
Untuk mendeploy manual di server Ubuntu tanpa Docker:
```bash
lumina make:nginx
```
*Akan me-generate konfigurasi Nginx di folder `nginx/nginx.conf`. Cukup salin file ini ke `/etc/nginx/sites-available/` di VPS Anda.*

### 3. Auto-Config Supervisor (Auto Restart Process)
Agar server web Rust Anda selalu berjalan dan otomatis menyala kembali jika server restart atau crash:
```bash
lumina make:supervisor
```
*Akan menghasilkan file `supervisor/lumina.conf` yang siap Anda pasang di Supervisor daemon server VPS Anda.*

---

## 📚 Ringkasan Perintah CLI Global (`lumina --help`)

| Perintah | Deskripsi |
| :--- | :--- |
| `lumina new <name>` | Membuat project Lumina baru dari skeleton |
| `lumina serve` | Menjalankan server aplikasi |
| `lumina watch` | Menjalankan server dengan fitur auto-reload saat kode diubah |
| `lumina make:controller <name>` | Membuat Controller baru dan mendaftarkannya otomatis |
| `lumina make:model <name>` | Membuat Model database baru |
| `lumina make:migration <name>` | Membuat file migrasi database SQL |
| `lumina make:crud <name> [fields]`| Membuat halaman CRUD komplit secara instan |
| `lumina make:auth` | Membuat sistem autentikasi siap pakai (Register & Login) |
| `lumina migrate` | Menjalankan semua migrasi database yang belum diaplikasikan |
| `lumina migrate:rollback` | Membatalkan migrasi terakhir |
| `lumina db:seed` | Mengisi database dengan data dummy awal |
| `lumina make:docker` | Membuat Dockerfile & docker-compose.yml produksi |
| `lumina make:nginx` | Membuat konfigurasi Nginx reverse proxy produksi |
| `lumina make:supervisor` | Membuat konfigurasi Supervisor daemon |

Selamat berkarya dengan **Lumina Framework**! Jika ada pertanyaan atau kendala, silakan periksa file `README.md` pada project Anda atau buka dokumentasi resmi Lumina. 🦀✨
