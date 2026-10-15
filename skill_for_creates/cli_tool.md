# 🛠️ Lumina Framework — CLI Tool Skill

## 🎯 Goal
Implementasi **Command Line Interface (CLI)** untuk Lumina Framework. Terinspirasi dari Laravel Artisan, CLI ini bernama `lumina` dan bertugas mempercepat *Developer Experience* (DX) melalui scaffolding file otomatis.

**Fitur yang dibangun:**
- Integrasi crate `clap` untuk parsing argumen command line.
- Entry point bin terpisah di `src/bin/lumina.rs` (bisa di-run dengan `cargo run --bin lumina -- <cmd>`).
- `make:controller <name>` — Generate boilerplate controller.
- `make:model <name>` — Generate boilerplate model (struct & trait impl).
- `make:migration <name>` — Generate file SQL migrasi kosong dengan prefix timestamp.

---

## 📁 Perubahan File

```
Cargo.toml                              ← MODIFY: tambah clap, tambah [[bin]] lumina

src/bin/                                ← NEW DIR
└── lumina.rs                           ← NEW: Entry point CLI

src/cli/                                ← NEW DIR
├── mod.rs                              ← NEW
├── commands.rs                         ← NEW: Definisi argumen clap
└── stubs/                              ← NEW: Template/boilerplate file
    ├── controller.stub
    └── model.stub
```

---

## 📝 Konsep Implementasi

### 1. `Cargo.toml`
```toml
[dependencies]
clap = { version = "4", features = ["derive"] }
chrono = "0.4" # Untuk timestamp migration

[[bin]]
name = "lumina"
path = "src/bin/lumina.rs"
```

### 2. `src/bin/lumina.rs` (Entry Point)
Parsing argumen menggunakan `clap` dan memanggil eksekutor dari module `cli`.

```bash
# Contoh pemakaian setelah di-build:
cargo run --bin lumina -- make:controller ProductController
cargo run --bin lumina -- make:model Product
cargo run --bin lumina -- make:migration create_products_table
```

### 3. Eksekutor `make:migration`
Akan mengambil waktu saat ini (contoh `20260427103000`) dan membuat file:
`database/migrations/20260427103000_create_products_table.sql`

### 4. Eksekutor `make:controller`
Membaca `stubs/controller.stub`, mengganti placeholder `{{name}}` dengan nama yang diinput, lalu menyimpannya di `src/app/controllers/`.
File `mod.rs` di controllers sebaiknya juga di-update secara otomatis jika memungkinkan.

---

## ✅ Validation Checklist

1. Jalankan `cargo run --bin lumina -- make:controller TestController`
   - Harapan: File `src/app/controllers/test_controller.rs` terbuat dengan format yang benar.
2. Jalankan `cargo run --bin lumina -- make:migration create_test_table`
   - Harapan: File `database/migrations/<timestamp>_create_test_table.sql` terbuat.
3. Jalankan `cargo build`
   - Harapan: Berhasil tanpa error.

---

## 📌 Urutan Eksekusi
Skill ini akan dieksekusi via pull request ke master (workflow standar).
