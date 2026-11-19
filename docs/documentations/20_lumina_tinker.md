# 🔮 Modul 20 — Lumina Tinker (Interactive CLI REPL Shell)

> *"Look like Laravel, Run at Rust Speed. Menghadirkan konsol interaktif paling intuitif untuk berinteraksi langsung dengan aplikasi, database, dan service Lumina tanpa membuat route pengujian sementara."*

Selamat datang di **Modul 20 Dokumentasi Resmi Lumina Framework**! Pada modul ini, kita akan mempelajari serba-serbi **Lumina Tinker**, alat bantu produktivitas berbasis REPL (*Read-Eval-Print Loop*) yang terinspirasi oleh `php artisan tinker` Laravel, namun dibangun dengan kekuatan native Rust.

---

## 🌟 1. Mengapa Lumina Tinker?

Dalam siklus pengembangan backend, developer sering kali perlu:
- Memeriksa record di database secara cepat.
- Menguji kalkulasi bisnis atau enkripsi password.
- Menghasilkan slug URL atau UUID.
- Menguji query Eloquent sebelum menuliskannya di Controller.

Alih-alih membuat route sementara di `routes/web.rs` lalu membuka browser, Anda cukup membuka terminal dan mengetik:

```bash
lumina tinker
```

---

## 🚀 2. Memulai Sesi Lumina Tinker

Pastikan file konfigurasi `.env` proyek Anda telah memiliki variabel `DATABASE_URL` aktif, lalu jalankan:

```bash
lumina tinker
```

Anda akan disambut oleh banner interaktif:

```text
=========================================================
   ⚡ Lumina Framework — Interactive Tinker Shell (REPL)   
=========================================================
📦 [Database] Terhubung ke basis data: PostgreSQL
Ketik 'help' untuk daftar perintah, 'exit' atau Ctrl+D untuk keluar.

lumina > 
```

---

## 🗄️ 3. Query Model Eloquent

Lumina Tinker memahami sintaksis model Lumina secara otomatis:

### A. Mengambil Semua Record (`Model::all()`)
```rust
lumina > User::all()
```
Menghasilkan tabel ANSI Box-Drawing yang rapi:
```text
┌────┬──────────────┬──────────────────┬────────┐
│ id │ name         │ email            │ role   │
├────┼──────────────┼──────────────────┼────────┤
│  1 │ Budi Santoso │ budi@example.com │ admin  │
│  2 │ Siti Rahma   │ siti@example.com │ user   │
└────┴──────────────┴──────────────────┴────────┘
⚡ Executed in 0.42ms (2 rows)
```

### B. Mencari Record Berdasarkan ID (`Model::find(id)`)
```rust
lumina > User::find(1)
```

### C. Menghitung Total Record (`Model::count()`)
```rust
lumina > Article::count()
=> 42
⚡ Executed in 0.28ms (1 row)
```

---

## ⚡ 4. Fluent Query Builder (`DB::table`)

Selain model, Anda dapat menggunakan Query Builder dinamis:

```rust
// Ambil seluruh data tabel
lumina > DB::table("orders").get()

// Batasi jumlah baris
lumina > DB::table("audit_logs").limit(5).get()

// Hitung baris tabel tertentu
lumina > DB::table("contacts").count()
```

---

## 🛠️ 5. Menjalankan Raw SQL Langsung

Jika Anda ingin menjalankan query analitik kompleks atau DDL, Anda bisa mengetik SQL secara langsung:

```sql
lumina > SELECT role, count(*) as total FROM users GROUP BY role;
┌───────┬───────┐
│ role  │ total │
├───────┼───────┤
│ admin │ 2     │
│ user  │ 18    │
└───────┴───────┘
⚡ Executed in 0.35ms (2 rows)
```

Perintah `INSERT`, `UPDATE`, atau `DELETE` juga didukung dengan pelaporan baris terpengaruh (*rows affected*):
```sql
lumina > UPDATE users SET role = 'editor' WHERE id = 2;
=> Query OK, 1 row affected
⚡ Executed in 0.62ms
```

---

## 🔐 6. Helper & Security Facades

Lumina Tinker menyertakan shortcut evaluator untuk fungsi-fungsi utilitas:

### A. Bcrypt Password Hashing (`Hash::make` & `Hash::check`)
```rust
lumina > Hash::make("secret123")
=> "$2b$10$wKk3j4N...qA"
⚡ Executed in 72.10ms

lumina > Hash::check("secret123", "$2b$10$wKk3j4N...qA")
=> true
⚡ Executed in 71.85ms
```

### B. SEO AutoSlug Generator (`AutoSlug::slugify`)
```rust
lumina > AutoSlug::slugify("Tutorial Rust: Keren & Cepat Tahun 2026!")
=> "tutorial-rust-keren-cepat-tahun-2026"
⚡ Executed in 0.05ms
```

### C. UUID Generator (`Uuid::new()`)
```rust
lumina > Uuid::new()
=> "550e8400-e29b-41d4-a716-446655440000"
⚡ Executed in 0.02ms
```

---

## 🔍 7. Framework Introspection Shortcuts

| Perintah | Deskripsi |
| :--- | :--- |
| `tables` / `models` | Menampilkan seluruh tabel di database beserta model yang terinferensi. |
| `env` | Menampilkan variabel lingkungan `.env` dengan masking sensor pada password/key. |
| `help` | Menampilkan ringkasan seluruh perintah bawaan Tinker. |
| `clear` | Membersihkan layar terminal. |
| `exit` / `quit` / `Ctrl+D` | Keluar dari sesi Lumina Tinker. |

---

## 💡 8. Produktivitas Terminal
- **Riwayat Perintah (*Persistent History*)**: Tekan panah Atas/Bawah (`↑`/`↓`) untuk memanggil kembali perintah-perintah sebelumnya. Riwayat disimpan secara otomatis di file `.lumina_tinker_history`.
- **Zero Configuration**: Bekerja otomatis dengan SQLite, PostgreSQL, MySQL, dan SQL Server.
