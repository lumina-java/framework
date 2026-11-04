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

Lumina memiliki migration runner otomatis yang mendukung **SQLite, PostgreSQL, MySQL, dan Microsoft SQL Server (MSSQL)**:

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

## 8. Dukungan Multi-Database & Microsoft SQL Server (MSSQL)

Lumina Framework dirancang untuk bekerja secara seragam di berbagai engine database modern:
- **SQLite**: Pilihan ideal untuk pengembangan lokal (*Zero-Config*).
- **PostgreSQL**: Direkomendasikan untuk aplikasi skala besar dengan query kompleks dan konkurensi tinggi.
- **MySQL / MariaDB**: Standar industri aplikasi web modern.
- **Microsoft SQL Server (MSSQL)**: Solusi database enterprise dengan protokol biner TDS pure-Rust (**Tiberius + bb8 pool**).

### A. Fitur Driver Microsoft SQL Server (MSSQL)
Lumina mengintegrasikan driver TDS murni tanpa memerlukan instalasi driver C/ODBC tambahan:
1. **Auto-Translation Dialek T-SQL**:
   - Primary Key: `INTEGER PRIMARY KEY AUTOINCREMENT` $
ightarrow$ `INT IDENTITY(1,1) PRIMARY KEY`.
   - Tipe Waktu: `DATETIME` $
ightarrow$ `DATETIME2 DEFAULT CURRENT_TIMESTAMP`.
   - Teks Panjang: `TEXT` $
ightarrow$ `NVARCHAR(MAX)`.
   - Boolean: `BOOLEAN` $
ightarrow$ `BIT`.
   - DDL Bersyarat: `CREATE TABLE IF NOT EXISTS` $
ightarrow$ `IF NOT EXISTS (SELECT * FROM sys.tables WHERE name = '...') CREATE TABLE (...)`.
2. **Paginasi ANSI/T-SQL Otomatis**:
   Method `.paginate(page, per_page)` dan `.limit().offset()` otomatis dikompilasi ke sintaks:
   ```sql
   ORDER BY id OFFSET 20 ROWS FETCH NEXT 10 ROWS ONLY
   ```
3. **Eksekusi ORM Fluen**:
   Seluruh operasi model berjalan identik dengan database lainnya:
   ```rust
   // Query model di MSSQL sama persis dan sepenuhnya teroptimasi
   let users = User::query(req.db())
       .where_eq("role", "admin")
       .order_by("created_at", "DESC")
       .get()
       .await?;
   ```
4. **Native Direct Access**:
   Jika Anda membutuhkan akses langsung ke client TDS Tiberius untuk query raw atau *stored procedure*:
   ```rust
   if let Some(pool) = req.db().mssql() {
       let mut conn = pool.get().await?;
       let stream = conn.query("EXEC sp_my_procedure @P1", &[&user_id]).await?;
   }
   ```


---

## 9. Transaksi Database ACID Fluen (Zero-Ampersand, Zero-Pipes, Zero-Generics)

Lumina menghadirkan sistem transaksi database ACID yang dirancang khusus untuk kenyamanan developer:
- ❌ **Tanpa Tanda Ampersand `&tx`**: Tidak perlu meminjam (*borrowing*) transaksi secara rumit antar pemanggilan.
- ❌ **Tanpa Tanda Pipa `|`**: Bebas dari pengetikan simbol pipa yang sulit dijangkau jari pada keyboard.
- ❌ **Tanpa Anotasi Turbofish `<>`**: Ekstraksi baris data berlangsung mulus dan transparan ke format JSON/nilai tanpa generic rumit yang membingungkan developer baru.

Lumina menyediakan dua gaya penulisan transaksi yang sama-sama elegan dan bebas Anda pilih:

### A. Gaya 1: Direct Transaction Handle (`tx.table(...)`)
Sangat cocok untuk transaksi langsung di dalam fungsi, service layer, atau controller:
```rust
// 1. Mulai transaksi dari Request atau DatabasePool
let tx = req.begin().await?;

// 2. Lakukan operasi secara berantai tanpa tanda ampersand '&'
tx.table("accounts")
    .where_eq("id", 1)
    .decrement("balance", 50000)
    .await?;

tx.table("accounts")
    .where_eq("id", 2)
    .increment("balance", 50000)
    .await?;

// 3. Simpan perubahan secara permanen
tx.commit().await?;
```

### B. Gaya 2: Contextual Request (`req.table(...)`)
Sangat intuitif bagi developer Laravel, di mana context request secara otomatis memelihara transaksi aktif:
```rust
// 1. Buka transaksi pada level Request
req.begin().await?;

// 2. Operasi tabel langsung melalui req
req.table("accounts").await?.where_eq("id", 1).decrement("balance", 50000).await?;
req.table("accounts").await?.where_eq("id", 2).increment("balance", 50000).await?;

// 3. Selesaikan transaksi
req.commit().await?;
```

### C. Facade `DB`
Anda juga dapat menggunakan facade `DB` yang familiar:
```rust
use lumina::prelude::*;

// Menggunakan DB::begin
let tx = DB::begin(req.db()).await?;
tx.table("wallets").where_eq("id", 1).decrement("balance", 10000).await?;
tx.commit().await?;

// Atau blok eksekusi otomatis (Auto-Commit saat Ok, Auto-Rollback saat Err)
DB::transaction(req.db(), |tx| async move {
    tx.table("wallets").where_eq("id", 1).decrement("balance", 10000).await?;
    tx.table("wallets").where_eq("id", 2).increment("balance", 10000).await?;
    Ok::<(), sqlx::Error>(())
}).await?;
```

### D. Fitur Transaksi Lanjutan:
1. **Auto-Rollback on Drop**: Jika sebuah transaksi keluar dari scope fungsi tanpa pernah memanggil `.commit().await?` (misalnya karena `?` return error lebih awal), Lumina secara otomatis membatalkan (*rollback*) seluruh perubahan di background secara non-blocking!
2. **Savepoint & Partial Rollback**:
   ```rust
   let tx = req.begin().await?;
   tx.table("orders").insert([("status", "pending")]).await?;
   
   // Tandai titik simpan
   tx.savepoint("step_1").await?;
   
   // Lakukan operasi eksperimental
   tx.table("logs").insert([("action", "test")]).await?;
   
   // Batalkan hanya perubahan setelah savepoint
   tx.rollback_to("step_1").await?;
   
   tx.commit().await?;
   ```

---

## 10. Smart Declarative Soft Deletes

Lumina menyematkan dukungan **Soft Deletes** cerdas yang terintegrasi langsung dengan derive macro `#[derive(LuminaModel)]`:

### A. Deklarasi Model
Cukup tambahkan field `deleted_at: Option<String>` pada struct Model Anda:
```rust
#[derive(Debug, Serialize, Deserialize, FromRow, Clone, Default, LuminaModel)]
#[table("products")]
pub struct Product {
    pub id: i64,
    pub name: String,
    pub price: f64,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub deleted_at: Option<String>, // ✨ Otomatis mengaktifkan Soft Deletes!
}
```
> [!TIP]
> **Anti-Crash Safety**: Model yang *tidak* memiliki field `deleted_at` secara cerdas tidak akan menyisipkan klausa `WHERE deleted_at IS NULL` yang invalid, sehingga mencegah query SQL error pada tabel warisan (*legacy*).

### B. Querying dengan Filter Otomatis
Secara default, query Model atau TableQueryBuilder hanya mengambil data aktif:
```rust
// Hanya mengambil produk yang belum terhapus (deleted_at IS NULL)
let active_products = Product::query(req.db()).get().await?;

// Sertakan data yang terhapus sementara
let all_products = Product::query(req.db()).with_trashed().get().await?;

// Ambil HANYA data yang terhapus (Trash Box)
let trashed = Product::query(req.db()).only_trashed().get().await?;
```

### C. Operasi Soft Delete, Restore, dan Force Delete
```rust
// 1. Soft Delete (Mengisi deleted_at dengan waktu saat ini)
Product::query(req.db()).where_eq("id", 1).delete().await?;

// 2. Restore (Mengembalikan deleted_at menjadi NULL)
Product::query(req.db()).where_eq("id", 1).restore().await?;

// 3. Force Delete (Menghapus baris fisik secara permanen dari tabel)
Product::query(req.db()).where_eq("id", 1).force_delete().await?;
```

---

### Navigasi Modul:
- **Sebelumnya**: [Modul 03: Controller & Request](./03_controller_dan_request.md) 
- **Berikutnya**: [Modul 05: Blade Template Engine](./05_blade_template_engine.md) ➔
- **Kembali ke**: [Daftar Isi Dokumentasi](./README.md)
