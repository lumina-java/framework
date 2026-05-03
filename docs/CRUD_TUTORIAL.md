# Tutorial CRUD Lumina Framework: Manajemen Buku (Book Management)

Selamat datang di tutorial dasar CRUD Lumina Framework! Dalam panduan ini, kita akan membuat aplikasi sederhana untuk mengelola daftar buku (Book Management). 

Tutorial ini dirancang agar sangat ramah bagi pemula yang baru mengenal konsep MVC (Model-View-Controller) dan RESTful API.

---

## 🚀 Langkah 1: Persiapan Awal

Pastikan Anda telah menyalin konfigurasi `.env` dan mengisi `DATABASE_URL` yang valid:

```env
DATABASE_URL=mysql://root:password@127.0.0.1:3306/lumina_db
```

Atau jika menggunakan SQLite:

```env
DATABASE_URL=sqlite://database.sqlite
```

---

## 🛠️ Langkah 2: Membuat Blueprint Buku dengan CLI

Lumina menyediakan fitur scaffolding CRUD secara instan. Jalankan perintah berikut di terminal Anda:

```bash
cargo run --bin lumina -- make:crud Book
```

**Perintah di atas otomatis membuat file-file berikut:**
1. **Model** di `src/app/models/book.rs`
2. **Migration file** di `database/migrations/..._create_books_table.sql`
3. **Controller** di `src/app/controllers/book_controller.rs`
4. **Views** lengkap di `resources/views/book/` (`index.html`, `create.html`, `edit.html`)

---

## 🗄️ Langkah 3: Menyiapkan Tabel Database

Buka file migrasi SQL baru Anda yang ada di dalam folder `database/migrations/` dan tentukan struktur tabel untuk menyimpan data buku:

```sql
CREATE TABLE books (
    id BIGINT PRIMARY KEY AUTO_INCREMENT,
    title VARCHAR(255) NOT NULL,
    author VARCHAR(255) NOT NULL,
    description TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);
```

> **Catatan:** Jika Anda menggunakan SQLite, skema di atas dapat disederhanakan tanpa keyword `ON UPDATE`.

---

## 🧩 Langkah 4: Registrasi di Framework

Agar file yang baru kita buat dikenali oleh Lumina, kita perlu mendaftarkannya pada komponen internal:

### 1. Daftarkan Model
Buka file `src/app/models/mod.rs`, tambahkan baris berikut:

```rust
pub mod book;
```

### 2. Daftarkan Controller
Buka file `src/app/controllers/mod.rs`, tambahkan baris berikut:

```rust
pub mod book_controller;
```

### 3. Daftarkan Route
Buka file `routes/web.rs`, lalu masukkan route untuk `BookController`:

```rust
use crate::app::controllers::book_controller::BookController;

// Masukkan ke dalam public routes di fungsi register():
let public = Router::<AppState>::new()
    // ... route yang sudah ada ...
    .get("/books", BookController::index)
    .get("/books/create", BookController::create)
    .post("/books", BookController::store)
    .get("/books/:id/edit", BookController::edit)
    .post("/books/:id", BookController::update)
    .get("/books/:id/delete", BookController::delete);
```

---

## 🖥️ Langkah 5: Menjalankan Aplikasi

Sekarang, jalankan server Lumina Framework:

```bash
cargo run --bin lumina -- serve
```

Server Lumina akan aktif di: [http://127.0.0.1:8000/books](http://127.0.0.1:8000/books). 

Cobalah untuk **menambah**, **mengedit**, **melihat daftar**, dan **menghapus** data buku melalui browser Anda. Selamat, Anda berhasil mempelajari dasar CRUD menggunakan Lumina Framework! 🚀
