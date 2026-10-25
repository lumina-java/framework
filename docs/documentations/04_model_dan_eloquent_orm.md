# 📘 Modul 04: Model & Eloquent ORM

Lumina menghadirkan **Human ORM** murni bergaya Laravel Eloquent di atas Rust. Menggabungkan kemudahan sintaks *Active Record* dengan kecepatan *zero-cost abstraction* dan *type-safety* Rust.

---

## 1. Membuat Model & Migrasi (`lumina make:model`)

```bash
lumina make:model Product
```
Perintah ini akan secara otomatis:
1. Membuat file model di `src/app/models/product.rs`.
2. Membuat file migrasi SQL baru di `database/migrations/<timestamp>_create_products_table.sql`.
3. Mendaftarkan modul ke `src/app/models/mod.rs`.

---

## 2. Menjalankan Migrasi Database

Lumina memiliki migration runner otomatis yang mendukung SQLite, PostgreSQL, dan MySQL:

```bash
# Menjalankan seluruh migrasi yang tertunda
lumina migrate

# Memeriksa status migrasi yang telah diterapkan
lumina migrate:status

# Membatalkan (rollback) migrasi terakhir
lumina migrate:rollback
```

Contoh file migrasi SQL (`database/migrations/20261025_create_products_table.sql`):
```sql
CREATE TABLE IF NOT EXISTS products (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name VARCHAR(255) NOT NULL,
    price REAL NOT NULL,
    stock INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
```

---

## 3. Definisi Model dengan Makro `LuminaModel`

Model di Lumina didefinisikan menggunakan derive macro `#[derive(LuminaModel)]`:

```rust
use serde::{Deserialize, Serialize};
use lumina::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, LuminaModel)]
#[lumina(table = "products")]
pub struct Product {
    #[lumina(primary_key)]
    pub id: i64,
    pub name: String,
    pub price: f64,
    pub stock: i32,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}
```

---

## 4. Operasi CRUD (Create, Read, Update, Delete)

### A. Create (Menambah Data Baru)
```rust
let new_product = Product {
    id: 0,
    name: "Laptop Gaming Ultra".to_string(),
    price: 15_000_000.0,
    stock: 10,
    created_at: None,
    updated_at: None,
};

let saved = new_product.save(&state.db).await?;
println!("Produk berhasil dibuat dengan ID: {}", saved.id);
```

### B. Read (Mencari Data Berdasarkan ID)
```rust
// Mengambil data berdasarkan Primary Key
let product = Product::find(&state.db, 1).await?;
if let Some(p) = product {
    println!("Ditemukan: {} - Rp {}", p.name, p.price);
}
```

### C. Update (Memperbarui Data)
```rust
if let Some(mut product) = Product::find(&state.db, 1).await? {
    product.price = 14_500_000.0;
    product.stock -= 1;
    product.save(&state.db).await?;
}
```

### D. Delete (Menghapus Data)
```rust
Product::delete(&state.db, 1).await?;
```

---

## 5. Eloquent Fluent Query Builder

Untuk query yang lebih kompleks, gunakan query builder berantai (*chainable*):

```rust
// Ambil semua produk dengan harga > 5 juta, urutkan termurah, ambil 10 teratas
let products = Product::query()
    .where_gt("price", 5_000_000.0)
    .where_eq("stock", 10)
    .order_by("price", "ASC")
    .limit(10)
    .get(&state.db)
    .await?;

// Pagination otomatis (Laravel Style)
let page = 1;
let per_page = 15;
let paginator = Product::query()
    .where_gt("stock", 0)
    .paginate(&state.db, page, per_page)
    .await?;

println!("Total Halaman: {}", paginator.last_page);
println!("Total Data   : {}", paginator.total);
for item in paginator.items {
    println!("- {}", item.name);
}
```

---

## 6. Relasi Model (Relationships)

Lumina menyediakan makro relasi 1-baris ekspresif:

```rust
// 1. One to Many: User -> Posts
pub fn posts(&self, db: &DbPool) -> HasMany<Post> {
    has_many!(self, db, Post, "user_id")
}

// 2. Many to One: Post -> User
pub fn user(&self, db: &DbPool) -> BelongsTo<User> {
    belongs_to!(self, db, User, "user_id")
}

// 3. One to One: User -> Profile
pub fn profile(&self, db: &DbPool) -> HasOne<Profile> {
    has_one!(self, db, Profile, "user_id")
}

// 4. Many to Many: User -> Roles (via pivot role_user)
pub fn roles(&self, db: &DbPool) -> BelongsToMany<Role> {
    belongs_to_many!(self, db, Role, "role_user", "user_id", "role_id")
}
```

---

## 7. Model Factory & Faker Engine (`lumina make:factory`)

Lumina menyediakan sistem pembuatan data tiruan (*mock / seeding*) otomatis berkecepatan tinggi ala Laravel Factory dengan lokalisasi data Indonesia:

### A. Membuat Factory Baru via CLI
```bash
lumina make:factory UserFactory
```
File `database/factories/user_factory.rs` akan dibuat secara otomatis:

```rust
use lumina::prelude::*;
use crate::app::models::user::User;

pub struct UserFactory;

impl UserFactory {
    pub fn new() -> Self {
        Self
    }
}

impl Factory<User> for UserFactory {
    fn definition(&self) -> User {
        User {
            name: Faker::name(),
            email: Faker::unique_email(),
            phone: Faker::phone(),
            created_at: Some(Faker::datetime_recent()),
            updated_at: Some(Faker::datetime_recent()),
            ..Default::default()
        }
    }
}

impl HasFactory<UserFactory> for User {
    fn factory() -> UserFactory {
        UserFactory::new()
    }
}
```

### B. Penggunaan di Seeder atau Unit Test
```rust
// 1. Buat 100 User baru langsung masuk ke database
let users = User::factory().count(100).create(req.db()).await?;

// 2. Modifikasi atribut dengan State Modifier
let admins = User::factory()
    .count(5)
    .state(|u| {
        u.role = "admin".to_string();
    })
    .create(req.db())
    .await?;

// 3. Buat objek in-memory tanpa menyentuh database
let mock_user = User::factory().make_one();
```

---

### Navigasi Modul:
- **Sebelumnya**: [Modul 03: Controller & Request](./03_controller_dan_request.md) 
- **Berikutnya**: [Modul 05: Blade Template Engine](./05_blade_template_engine.md) ➔
- **Kembali ke**: [Daftar Isi Dokumentasi](./README.md)
