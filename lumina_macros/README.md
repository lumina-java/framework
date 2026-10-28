# 🔮 Lumina Macros (`lumina_macros`)

> **"Look like Laravel, Run at Rust Speed."**  
> *Procedural macros powering Lumina Framework — Model ORM derivation, active record methods, and human-readable form validation.*

[![License: MIT](https://img.shields.io/badge/License-MIT-emerald.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-2021%20edition-orange.svg)](https://www.rust-lang.org)
[![Lumina Framework](https://img.shields.io/badge/ecosystem-Lumina%20Framework-blue.svg)](https://github.com/lumina-java/framework)

---

## 🌟 Fitur Utama

`lumina_macros` menyediakan makro prosedural (*procedural macros*) yang membebaskan pengembang dari penulisan kode repetitif (*boilerplate*):

1. **`#[derive(LuminaModel)]`**:
   - Membangkitkan implementasi trait `Model` secara otomatis.
   - Menyediakan fungsi CRUD statis dan instance: `find()`, `all()`, `create()`, `save()`, `delete()`, `paginate()`.
   - Menghubungkan model ke **Fluent Query Builder** (`User::query(db)`).
   - Mendukung kustomisasi tabel (`#[table("nama_tabel")]`), primary key kustom (`#[primary_key("id")]`), dan mapping nama kolom SQL.

2. **`#[lumina_form]`**:
   - Validasi form deklaratif dengan sintaks yang ekspresif ala Laravel: `#[rule("required|email|min:3|max:50")]`.
   - Mengubah aturan teks secara otomatis menjadi validator bawaan tanpa konfigurasi manual yang berbelit-belit.

---

## 📦 Instalasi

Tambahkan dependensi ke berkas `Cargo.toml` Anda:

```toml
[dependencies]
lumina_macros = { git = "https://github.com/lumina-java/lumina-macros" }
```

*(Catatan: Jika Anda menggunakan `lumina` atau `lumina-framework`, makro ini sudah diekspor ulang secara otomatis melalui `lumina::prelude::*`)*.

---

## 🚀 Panduan Penggunaan

### 1. Derive Model ORM (`#[derive(LuminaModel)]`)

Cukup tambahkan derive pada struct entitas Anda:

```rust
use lumina::prelude::*;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow, Clone, Default, LuminaModel)]
#[table("users")]
pub struct User {
    pub id: i64,
    pub name: String,
    pub email: String,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}
```

Setelah didefinisikan, struct otomatis memiliki metode ekspresif:
```rust
// 1. Ambil data berdasarkan ID
let user = User::find(req.db(), 1).await?;

// 2. Query builder chaining
let active_users = User::query(req.db())
    .where_eq("email", "john@example.com")
    .order_by("name", "ASC")
    .get()
    .await?;

// 3. Paginasi otomatis
let paginator = User::paginate(req.db(), page, per_page).await?;
```

---

### 2. Validasi Form Deklaratif (`#[lumina_form]`)

Definisikan form input web atau REST API dengan aturan validasi deklaratif:

```rust
use lumina_macros::lumina_form;

#[lumina_form]
pub struct RegisterForm {
    #[rule("required|min:3|max:50")]
    pub name: String,

    #[rule("required|email")]
    pub email: String,

    #[rule("required|min:8")]
    pub password: String,
}
```

Aturan yang didukung:
- `required` — Field wajib diisi dan tidak boleh kosong.
- `email` — Format alamat surel yang valid.
- `url` — Format URL yang valid.
- `min:X` — Panjang karakter minimal `X`.
- `max:X` — Panjang karakter maksimal `X`.

---

## 🤝 Berkontribusi (Contributing)

Kontribusi dari komunitas sangat dinantikan!
1. Fork repositori ini.
2. Buat branch fitur (`git checkout -b feature/nama-fitur`).
3. Jalankan pengujian (`cargo check`).
4. Commit perubahan Anda (`git commit -m 'feat: tambah fitur macro baru'`).
5. Push ke branch Anda (`git push origin feature/nama-fitur`).
6. Buka **Pull Request**.

---

## 📜 Lisensi

Lumina Macros dirilis di bawah lisensi terbuka [MIT License](LICENSE).  
Dikelola oleh tim pengembang [Lumina Framework](https://github.com/lumina-java/framework).

