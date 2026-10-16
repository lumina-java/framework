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
*(Di Rust, tipe data otomatis terdeteksi tanpa perlu mengetik `: Option<User>` atau `: Vec<User>`)*

```rust
// 1. Ambil semua data
let users = User::all(db).await?;

// 2. Ambil record atau 404 otomatis (Paling Manusiawi untuk Controller)
let user = User::find(db, 1).await?.or_404("User tidak ditemukan")?;

// 3. Ambil record berdasarkan ID (Laravel: findOrFail)
let user = User::find_or_fail(db, 1).await?;

// 4. Ambil record opsional (jika ingin cek manual dengan if let Some)
let maybe_user = User::find(db, 1).await?;

// 5. Hitung total record aktif
let total = User::count(db).await?;
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

// Ambil record pertama (atau 404 otomatis)
let user = User::query(db)
    .where_eq("email", "budi@example.com")
    .first()
    .await?
    .or_404("User tidak ditemukan")?;

// Ambil record pertama (Laravel: firstOrFail)
let user = User::query(db)
    .where_eq("email", "budi@example.com")
    .first_or_fail()
    .await?;

// Shortcuts pada Model
let active_users = User::where_eq(db, "status", "active").get().await?;
let latest_users = User::latest(db).limit(5).get().await?;
```

---

## 4. Sistem Relasi (Relationships)

Lumina ORM mendukung penulisan relasi yang **sangat manusiawi** dengan dua pilihan sintaks:
1. **Sintaks Deklaratif 1 Baris (`belongs_to!`, `has_many!`, `has_one!`, `belongs_to_many!`)** — *Sangat direkomendasikan karena 0 boilerplate!*
2. **Sintaks Fungsi Ringkas** menggunakan alias `&Db` dan `OrmResult<T>` (tanpa perlu menulis `sqlx::Error` dan `&DatabasePool`).

---

### A. One to Many (HasMany)
Contoh: 1 `User` memiliki banyak `Post`.

#### Opsi 1: Deklaratif 1 Baris (Paling Manusiawi)
```rust
impl User {
    // Otomatis foreign key "user_id"
    has_many!(posts: Post);
    
    // Atau jika nama foreign key custom:
    // has_many!(posts: Post, "author_id");
}
```

#### Opsi 2: Fungsi Ringkas (dengan `&Db` dan `OrmResult`)
```rust
impl User {
    pub async fn posts(&self, db: &Db) -> OrmResult<Vec<Post>> {
        self.has_many(db, "user_id").await
    }
}
```

**Cara Penggunaan di Controller:**
```rust
let user = User::find(req.db(), 1).await?.unwrap();
let posts = user.posts(req.db()).await?;
```

---

### B. Belongs To (BelongsTo)
Contoh: 1 `Post` dimiliki oleh 1 `User`.

#### Opsi 1: Deklaratif 1 Baris (Paling Manusiawi)
```rust
impl Post {
    // Belongs to User via field `self.user_id`
    belongs_to!(author: User, user_id);
}
```

#### Opsi 2: Fungsi Ringkas (dengan `&Db` dan `OrmResult`)
```rust
impl Post {
    pub async fn author(&self, db: &Db) -> OrmResult<User> {
        self.belongs_to(db, self.user_id).await
    }
}
```

**Cara Penggunaan di Controller:**
```rust
let post = Post::find(req.db(), 10).await?.unwrap();
let author = post.author(req.db()).await?;
```

---

### C. One to One (HasOne)
Contoh: 1 `User` memiliki 1 `Profile`.

```rust
impl User {
    // Deklaratif 1 baris
    has_one!(profile: Profile);
    
    // Atau manual ringkas:
    // pub async fn profile(&self, db: &Db) -> OrmResult<Profile> {
    //     self.has_one(db, "user_id").await
    // }
}
```

**Cara Penggunaan di Controller:**
```rust
let user = User::find(req.db(), 1).await?.unwrap();
let profile = user.profile(req.db()).await?;
```

---

### D. Many to Many (BelongsToMany)
Contoh: 1 `Post` memiliki banyak `Tag` via pivot table `post_tag`.

```rust
impl Post {
    // Deklaratif 1 baris via pivot table
    belongs_to_many!(tags: Tag, "post_tag");
    
    // Atau custom keys:
    // belongs_to_many!(tags: Tag, "post_tag", "post_id", "tag_id");
}
```

**Cara Penggunaan di Controller:**
```rust
let post = Post::find(req.db(), 10).await?.unwrap();
let tags = post.tags(req.db()).await?;
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

### G. Kombinasi Bersamaan: Menggabungkan Macro 1-Baris & Fungsi Kustom dalam 1 Model

Kedua sintaks di atas **100% kompatibel dan dapat digunakan secara bersamaan** dalam satu model yang sama:
- **Utamakan Macro 1-Baris** untuk relasi standar tanpa filter tambahan.
- **Gunakan Fungsi Manual (`&Db` & `OrmResult`)** jika membutuhkan filter kustom, pengurutan khusus, batas limit, atau manipulasi data bisnis.

```rust
use crate::app::prelude::*;

impl User {
    // ── 1. Relasi Standar (Prioritas Utama: 0 Boilerplate) ───────────
    has_many!(posts: Post, "user_id");
    has_one!(profile: Profile, "user_id");

    // ── 2. Relasi Kustom (Logika Spesifik dengan &Db & OrmResult) ────
    /// Mengambil hanya postingan yang sudah dipublikasikan (status = 'published')
    pub async fn published_posts(&self, db: &Db) -> OrmResult<Vec<Post>> {
        Post::query(db)
            .where_eq("user_id", self.id)
            .where_eq("status", "published")
            .latest()
            .get()
            .await
    }

    /// Mengambil postingan terpopuler dengan batas limit tertentu
    pub async fn top_posts(&self, db: &Db, limit: usize) -> OrmResult<Vec<Post>> {
        Post::query(db)
            .where_eq("user_id", self.id)
            .order_by("views", "DESC")
            .limit(limit)
            .get()
            .await
    }
}
```

**Penggunaan di Controller / Service:**
```rust
let user = User::find(req.db(), 1).await?.unwrap();

// Memanggil relasi standar macro 1-baris:
let all_posts = user.posts(req.db()).await?;

// Memanggil relasi kustom fungsi manual:
let published = user.published_posts(req.db()).await?;
let top_3 = user.top_posts(req.db(), 3).await?;
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
