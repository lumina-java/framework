# 💎 Lumina Eloquent ORM — Dokumentasi Resmi

Selamat datang di dokumentasi resmi **Lumina Eloquent ORM**. 
Lumina ORM adalah Object-Relational Mapper independen untuk Rust yang dirancang dengan filosofi **Laravel Eloquent**: sintaks yang ekspresif, intuitif, minim boilerplate (*zero boilerplate*), dan berkinerja tinggi (*zero-cost abstractions*).

---

## 📑 Daftar Isi
1. [Definisi Model & Derive Macro](#1-definisi-model--derive-macro)
2. [Operasi CRUD Dasar (Active Record)](#2-operasi-crud-dasar-active-record)
3. [Fluent Query Builder](#3-fluent-query-builder)
4. [Sistem Relasi (Relationships)](#4-sistem-relasi-relationships)
   - [One to Many (HasMany)](#a-one-to-many-hasmany)
   - [Belongs To (BelongsTo)](#b-belongs-to-belongsto)
   - [One to One (HasOne)](#c-one-to-one-hasone)
   - [Many to Many (BelongsToMany)](#d-many-to-many-belongstomany)
   - [Subquery Relasi (whereHas & whereDoesntHave)](#e-subquery-relasi-wherehas--wheredoesnthave)
   - [Eager Loading (.with) — Solusi N+1](#f-eager-loading-with--solusi-n1)
5. [Paginasi Bergaya Laravel](#5-paginasi-bergaya-laravel)
6. [Integrasi dengan Controller & LuminaResult](#6-integrasi-dengan-controller--luminaresult)

---

## 1. Definisi Model & Derive Macro

Model di Lumina didefinisikan menggunakan struct Rust standar dengan derive macro `LuminaModel`:

```rust
use lumina::prelude::*;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow, Clone, Default, LuminaModel)]
#[lumina(table = "users")]
pub struct User {
    pub id: i64,
    pub name: String,
    pub email: String,
    
    // Otomatis dikelola oleh Lumina ORM
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub deleted_at: Option<String>,
}
```

### Konvensi & Atribut:
- `#[lumina(table = "nama_tabel")]`: Menentukan nama tabel di database. Jika tidak diberikan, defaultnya adalah nama struct dalam bentuk lowercase jamak (misal `User` -> `users`).
- `#[primary_key]`: Menentukan kolom primary key (default: `id`).
- `#[column("nama_kolom")]`: Mapping nama field ke kolom database yang berbeda.
- **Soft Delete**: Jika model memiliki field `deleted_at: Option<String>`, Lumina otomatis mengaktifkan fitur Soft Delete (record tidak langsung dihapus fisik dari database).

---

## 2. Operasi CRUD Dasar (Active Record)

Lumina menyediakan dua gaya: **Static Model API** dan **Active Record Instance API**.

### A. Mengambil Data (Read)
```rust
// Ambil semua data (aktif/bukan soft deleted)
let users: Vec<User> = User::all(db).await?;

// Ambil record berdasarkan ID (mengembalikan Option<User>)
let user: Option<User> = User::find(db, 1).await?;

// Ambil record berdasarkan ID atau lempar Error (Laravel: findOrFail)
let user: User = User::find_or_fail(db, 1).await?;

// Hitung total record aktif
let total: i64 = User::count(db).await?;
```

### B. Membuat & Memperbarui Data (Create & Update)
Metode `$model.save(db)` bekerja cerdas:
- Jika `id == 0`: Melakukan operasi **INSERT**.
- Jika `id > 0`: Melakukan operasi **UPDATE**.

```rust
// 1. INSERT Baru
let mut new_user = User {
    name: "Budi Santoso".to_string(),
    email: "budi@example.com".to_string(),
    ..Default::default()
};
let id = new_user.save(db).await?;
new_user.id = id;

// 2. UPDATE Record
if let Some(mut user) = User::find(db, 1).await? {
    user.name = "Budi Santoso Updated".to_string();
    user.save(db).await?; // Otomatis UPDATE users SET name = ... WHERE id = 1
}
```

### C. Menghapus Data (Delete)
```rust
// 1. Hapus via instance (Active Record)
user.delete(db).await?;

// 2. Hapus via ID langsung (Laravel: User::destroy(id))
User::destroy(db, 1).await?;
```

---

## 3. Fluent Query Builder

Untuk query dinamis dan filter kompleks, gunakan `User::query(db)`:

```rust
// WHERE Clauses
let users = User::query(db)
    .where_eq("status", "active")
    .where_like("name", "%budi%")
    .where_null("verified_at")
    .order_by("created_at", "DESC")
    .limit(10)
    .get()
    .await?;

// First Record
let first_user: Option<User> = User::query(db)
    .where_eq("email", "budi@example.com")
    .first()
    .await?;

// First or Fail
let first_user: User = User::query(db)
    .where_eq("email", "budi@example.com")
    .first_or_fail()
    .await?;

// Shortcuts pada Model
let active_users = User::where_eq(db, "status", "active").get().await?;
let latest_users = User::latest(db).limit(5).get().await?;
```

---

## 4. Sistem Relasi (Relationships)

Lumina ORM mendukung relasi database lengkap layaknya Eloquent Laravel:

### A. One to Many (HasMany)
Contoh: 1 `User` memiliki banyak `Post`.

```rust
impl User {
    /// Relasi Has Many: $user->posts()
    pub async fn posts(&self, db: &DatabasePool) -> Result<Vec<Post>, sqlx::Error> {
        self.has_many(db, "user_id").await
    }
}

// Cara Penggunaan:
let user = User::find(db, 1).await?.unwrap();
let posts = user.posts(db).await?;
```

### B. Belongs To (BelongsTo)
Contoh: 1 `Post` dimiliki oleh 1 `User`.

```rust
impl Post {
    /// Relasi Belongs To: $post->author()
    pub async fn author(&self, db: &DatabasePool) -> Result<User, sqlx::Error> {
        self.belongs_to(db, self.user_id).await
    }
}

// Cara Penggunaan:
let post = Post::find(db, 10).await?.unwrap();
let author = post.author(db).await?;
```

### C. One to One (HasOne)
Contoh: 1 `User` memiliki 1 `Profile`.

```rust
impl User {
    /// Relasi Has One: $user->profile()
    pub async fn profile(&self, db: &DatabasePool) -> Result<Profile, sqlx::Error> {
        self.has_one(db, "user_id").await
    }
}

// Cara Penggunaan:
let user = User::find(db, 1).await?.unwrap();
let profile = user.profile(db).await?;
```

### D. Many to Many (BelongsToMany)
Contoh: 1 `Post` memiliki banyak `Tag`, dan 1 `Tag` dimiliki oleh banyak `Post` via pivot table `post_tag`.

```rust
impl Post {
    /// Relasi Belongs To Many: $post->tags()
    pub async fn tags(&self, db: &DatabasePool) -> Result<Vec<Tag>, sqlx::Error> {
        self.belongs_to_many(db, "post_tag", "post_id", "tag_id").await
    }
}

// Cara Penggunaan:
let post = Post::find(db, 10).await?.unwrap();
let tags = post.tags(db).await?;
```

### E. Subquery Relasi (whereHas & whereDoesntHave)
Ingin mencari pengguna yang **memiliki postingan** atau **belum pernah membuat postingan**? Gunakan `where_has` dan `where_doesnt_have`:

```rust
// Cari User yang memiliki minimal 1 Postingan (Laravel: User::whereHas('posts'))
let active_authors = User::where_has(db, "posts", "user_id", "id")
    .get()
    .await?;

// Cari User yang belum pernah memposting apa pun (Laravel: User::whereDoesntHave('posts'))
let new_users = User::where_doesnt_have(db, "posts", "user_id", "id")
    .get()
    .await?;
```

### F. Eager Loading (.with) — Solusi N+1
Untuk mencegah query berulang dalam loop, gunakan `.with("relasi")`:

```rust
// Hanya membutuhkan 2 query SQL, bukan 100 query!
let posts = Post::query(db)
    .with("author")
    .with("tags")
    .get()
    .await?;

for post in posts {
    println!("Title: {}", post.title);
    if let Some(author) = &post.author {
        println!("Author: {}", author.name);
    }
}
```

---

## 5. Paginasi Bergaya Laravel

Lumina menyediakan paginasi otomatis yang mengembalikan struktur data lengkap (items, total, per_page, current_page, last_page):

```rust
// Paginasi: Halaman 1, 15 item per halaman
let paginator = Contact::paginate(db, 1, 15).await?;

println!("Total records: {}", paginator.total);
println!("Last page: {}", paginator.last_page);
for item in paginator.items {
    println!("Contact: {}", item.name);
}
```

---

## 6. Integrasi dengan Controller & LuminaResult

Lumina ORM menyatu sempurna dengan Controller dan `LuminaResult`:

```rust
use crate::app::prelude::*;

pub struct ContactController;

impl ContactController {
    /// GET /contacts — List semua kontak
    pub async fn index(req: Request) -> LuminaResult {
        let contacts = Contact::all(req.db()).await?;
        req.ok(contacts)
    }

    /// GET /contacts/paged — Paginasi otomatis dari query string (?page=1&per_page=10)
    pub async fn paginate(req: Request) -> LuminaResult {
        let (page, per_page) = req.pagination();
        let paginator = Contact::paginate(req.db(), page, per_page).await?;
        req.paginate(paginator)
    }

    /// GET /contacts/:id — Ambil satu kontak
    pub async fn show(req: Request) -> LuminaResult {
        let id = req.id_or_fail()?;
        match Contact::find(req.db(), id).await? {
            Some(contact) => req.ok(contact),
            None => req.not_found("Kontak tidak ditemukan"),
        }
    }

    /// POST /contacts — Buat kontak baru
    pub async fn store(req: Request, ValidatedJson(mut contact): ValidatedJson<Contact>) -> LuminaResult {
        let id = contact.save(req.db()).await?;
        contact.id = id;
        req.created_with(contact, "Kontak berhasil dibuat")
    }

    /// PUT /contacts/:id — Perbarui kontak
    pub async fn update(req: Request, ValidatedJson(payload): ValidatedJson<Contact>) -> LuminaResult {
        let id = req.id_or_fail()?;
        let mut contact = match Contact::find(req.db(), id).await? {
            Some(c) => c,
            None => return req.not_found("Kontak tidak ditemukan"),
        };
        contact.name = payload.name;
        contact.save(req.db()).await?;
        req.success(contact, "Kontak berhasil diperbarui")
    }

    /// DELETE /contacts/:id — Hapus kontak
    pub async fn destroy(req: Request) -> LuminaResult {
        let id = req.id_or_fail()?;
        if Contact::destroy(req.db(), id).await? {
            req.success(serde_json::json!({ "id": id }), "Kontak berhasil dihapus")
        } else {
            req.not_found("Kontak tidak ditemukan")
        }
    }
}
```

---

## 7. Multi-Database Support (PostgreSQL, SQLite, MySQL)

Lumina ORM secara cerdas mendeteksi driver database yang sedang digunakan:
- **PostgreSQL**: Menggunakan format binding `$1, $2`, klausa `RETURNING id`, dan konversi `to_char` untuk timestamp.
- **SQLite / MySQL**: Menggunakan binding `?`, `last_insert_rowid()`, dan fungsi timestamp lokal.

Anda dapat beralih database cukup dengan mengubah file `.env`:
```env
DB_CONNECTION=postgres
DB_HOST=127.0.0.1
DB_PORT=5432
DB_DATABASE=lumina_db
DB_USERNAME=postgres
DB_PASSWORD=secret
```
Tanpa perlu mengubah satu baris kode Rust pun!
