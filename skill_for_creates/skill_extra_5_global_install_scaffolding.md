# Skill Extra 5: Global Installation & Smart Scaffolding

Skill ini mencakup alur instalasi global framework Lumina dan sistem scaffolding CRUD yang dinamis.

## 1. Global Installation Flow
Agar Lumina dapat diinstall via `cargo install`, project harus memiliki struktur binary yang jelas.

### Alur Eksekusi:
1. **Instalasi**: Developer menjalankan `cargo install lumina-framework`.
2. **Binary**: Cargo menginstall binary `lumina` (CLI) dan `lumina-server` (Runtime).
3. **Inisialisasi**: Developer menjalankan `lumina new <project_name>`.
4. **Logic**: CLI melakukan `git clone` skeleton terbaru, membersihkan metadata `.git`, dan menyiapkan `.env`.

### Kode Penting:
- **`src/bin/lumina.rs`**: Entry point CLI yang menangani subcommand `new`.
- **`src/cli/mod.rs`**: Implementasi `handle_new` yang menggunakan `std::process::Command` untuk cloning.

---

## 2. Smart CRUD Scaffolding
Sistem yang memungkinkan pembuatan satu modul lengkap (MVC + Migration + Views) hanya dengan satu perintah.

### Format Perintah:
```bash
lumina make:crud Contact name:string:required email:string:email phone:string
```

### Supported Types:
- `string`: String / VARCHAR
- `text`: String / TEXT
- `integer`: i64 / INTEGER
- `boolean`: bool / BOOLEAN
- `date`: NaiveDate / DATE

### Alur Scaffolding:
1. **Parsing**: CLI memecah argumen fields.
2. **Generation**: Menggunakan `stubs/*.stub` untuk menyuntikkan kode dinamis ke:
   - Model (SQL Binding)
   - Migration (SQL Schema)
   - Service (Logic)
   - Request (Validation Rules)
   - Controller (Assignment logic)
   - Views (HTMX-ready Blade templates)
3. **Auto-Registration**: Mendaftarkan module baru ke `mod.rs` dan `routes/web.rs`.

---

## 3. Deployment Tooling
Otomasi pembuatan konfigurasi infrastruktur produksi.

### Commands:
- `make:docker`: Generate multi-stage Dockerfile & docker-compose (App, DB, Redis, Nginx).
- `make:nginx`: Generate hardened reverse proxy config.
- `make:supervisor`: Generate process manager config.
- `backup:run`: Otomasi backup database dan storage.

---

## 4. Best Practices for Creators:
- Gunakan `include_str!` untuk stubs agar binary self-contained.
- Selalu jalankan `cargo fmt` pada stubs untuk menjaga kerapihan.
- Pastikan `README.md` selalu update dengan flow instalasi terbaru.
