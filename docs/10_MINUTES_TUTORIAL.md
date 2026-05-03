# Lumina 10-Minute Tutorial: Aplikasi To-Do List Sederhana

Tutorial ini akan memandu Anda membuat aplikasi To-Do List lengkap dengan database, CRUD (Create, Read, Update, Delete), dan tampilan (Views) dalam waktu kurang dari 10 menit menggunakan Lumina CLI.

---

## Prasyarat
- **Rust (Cargo)** terinstal di sistem Anda.
- **Database** (MySQL / SQLite / Postgres) sudah menyala.

---

## Langkah 1: Kloning Repositori & Konfigurasi

Pertama, siapkan repositori dan salin konfigurasi dasar `.env`:

```bash
git clone https://github.com/lumina-java/lumina.git todo-app
cd todo-app
cp .env.example .env
```

Buka file `.env` dan sesuaikan koneksi database Anda pada variabel `DATABASE_URL`, misalnya untuk SQLite:

```env
DATABASE_URL=sqlite://database.sqlite
```

---

## Langkah 2: Scaffolding To-Do CRUD

Gunakan CLI ajaib Lumina untuk membuat blueprint aplikasi To-Do:

```bash
cargo run --bin lumina -- make:crud Todo
```

**Perintah di atas otomatis membuat:**
1. **Model** To-Do di `src/app/models/todo.rs`.
2. **Migration file** di `database/migrations/..._create_todos_table.sql`.
3. **Controller** di `src/app/controllers/todo_controller.rs`.
4. **Views** lengkap di `resources/views/todo/` (`index.html`, `create.html`, `edit.html`).

---

## Langkah 3: Sesuaikan Model & Migrasi

Buka file migrasi SQL baru Anda di `database/migrations/` dan definisikan skema tabel `todos`:

```sql
CREATE TABLE todos (
    id BIGINT PRIMARY KEY AUTO_INCREMENT,
    title VARCHAR(255) NOT NULL,
    completed BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);
```

Untuk **SQLite**, sesuaikan query dengan skema yang sesuai.

---

## Langkah 4: Registrasi Route dan Controller

Daftarkan controller baru Anda di file `src/app/controllers/mod.rs`:

```rust
pub mod todo_controller;
```

Dan daftarkan routes di `routes/web.rs`:

```rust
use crate::app::controllers::todo_controller::TodoController;

// Pada fungsi register():
public
    .get("/todos", TodoController::index)
    .get("/todos/create", TodoController::create)
    .post("/todos", TodoController::store)
    .get("/todos/:id/edit", TodoController::edit)
    .post("/todos/:id", TodoController::update)
    .get("/todos/:id/delete", TodoController::delete);
```

---

## Langkah 5: Jalankan Server & Nikmati Hasilnya!

Sekarang, jalankan migrasi dan server Lumina Anda:

```bash
cargo run --bin lumina -- serve
```

Buka browser Anda di: [http://127.0.0.1:8000/todos](http://127.0.0.1:8000/todos)

✨ **Selesai!** Anda telah berhasil membuat aplikasi To-Do List pertama Anda dengan Lumina Framework!
