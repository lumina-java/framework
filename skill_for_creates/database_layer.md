# 🗄️ Lumina Framework — Database Layer Skill

## 🎯 Goal
Implementasi **Database Layer** pada Lumina Framework menggunakan `sqlx` sebagai query executor async yang type-safe. Arsitektur terinspirasi Laravel Eloquent namun tetap idiomatik Rust.

**Fitur yang dibangun:**
- `DatabasePool` — connection pool wrapper (SQLite dev / PostgreSQL prod)
- `Model` trait — abstraksi query per-table
- Migration runner — eksekusi file `.sql` secara berurutan
- Integrasi dengan `Application` melalui Service Container
- Contoh model `User` sebagai referensi implementasi

---

## 📁 Perubahan File

```
Cargo.toml                              ← tambah sqlx dependency

src/database/
├── mod.rs                              ← MODIFY: export modules baru
├── connection.rs                       ← MODIFY: DatabasePool (sqlx::Pool)
├── model.rs                            ← NEW: Model trait
└── migration.rs                        ← NEW: Migration runner

src/app/models/
├── mod.rs                              ← MODIFY: export User model
└── user.rs                             ← NEW: contoh implementasi Model

database/
└── migrations/
    └── 0001_create_users_table.sql     ← NEW: schema SQL pertama

src/core/application.rs                 ← MODIFY: init DB pool saat startup
.env.example                            ← MODIFY: tambah DATABASE_URL
```

---

## 📝 File Implementations

### 1. `Cargo.toml` — Tambah Dependencies
```toml
sqlx = { version = "0.8", features = [
    "runtime-tokio",
    "sqlite",
    "postgres",
    "macros",
] }
```

---

### 2. `src/database/connection.rs` — DatabasePool
```rust
use sqlx::{Pool, Sqlite, sqlite::SqlitePoolOptions};

/// Lumina DatabasePool — wrapper di atas sqlx::Pool.
/// Mendukung SQLite untuk development, PostgreSQL untuk production.
pub struct DatabasePool {
    pub pool: Pool<Sqlite>,
}

impl DatabasePool {
    /// Buat koneksi pool baru dari DATABASE_URL di .env
    pub async fn connect(database_url: &str) -> Result<Self, sqlx::Error> {
        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect(database_url)
            .await?;

        println!("📦 Database connected: {}", database_url);
        Ok(Self { pool })
    }
}
```

---

### 3. `src/database/model.rs` — Model Trait
```rust
use async_trait::async_trait;
use super::connection::DatabasePool;

/// Trait yang harus diimplementasikan oleh setiap Model.
/// Menyediakan method dasar CRUD yang async.
#[async_trait]
pub trait Model: Sized + Send + Sync {
    /// Nama tabel di database
    const TABLE: &'static str;

    /// Ambil satu record by primary key
    async fn find(pool: &DatabasePool, id: i64) -> Result<Self, sqlx::Error>;

    /// Ambil semua record
    async fn all(pool: &DatabasePool) -> Result<Vec<Self>, sqlx::Error>;

    /// Simpan record baru
    async fn save(&self, pool: &DatabasePool) -> Result<i64, sqlx::Error>;

    /// Hapus record by primary key
    async fn delete(pool: &DatabasePool, id: i64) -> Result<bool, sqlx::Error>;
}
```

---

### 4. `src/database/migration.rs` — Migration Runner
```rust
use sqlx::{Pool, Sqlite};
use std::fs;
use std::path::Path;

/// Jalankan semua file migrasi dari direktori `database/migrations/`
/// secara berurutan berdasarkan nama file.
pub async fn run_migrations(pool: &Pool<Sqlite>) -> Result<(), sqlx::Error> {
    // Buat tabel migrasi jika belum ada
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS _migrations (
            id        INTEGER PRIMARY KEY AUTOINCREMENT,
            name      TEXT NOT NULL UNIQUE,
            run_at    DATETIME DEFAULT CURRENT_TIMESTAMP
        )"
    ).execute(pool).await?;

    let migration_dir = Path::new("database/migrations");
    if !migration_dir.exists() { return Ok(()); }

    let mut files: Vec<_> = fs::read_dir(migration_dir)
        .unwrap()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map(|x| x == "sql").unwrap_or(false))
        .collect();

    // Urutkan berdasarkan nama file (0001_, 0002_, dst.)
    files.sort_by_key(|e| e.file_name());

    for entry in files {
        let name = entry.file_name().to_string_lossy().to_string();

        // Skip jika sudah pernah dijalankan
        let exists: bool = sqlx::query_scalar(
            "SELECT EXISTS(SELECT 1 FROM _migrations WHERE name = ?)"
        )
        .bind(&name)
        .fetch_one(pool)
        .await?;

        if exists { continue; }

        let sql = fs::read_to_string(entry.path()).unwrap();
        sqlx::raw_sql(&sql).execute(pool).await?;

        sqlx::query("INSERT INTO _migrations (name) VALUES (?)")
            .bind(&name)
            .execute(pool)
            .await?;

        println!("  ✅ Migrated: {}", name);
    }

    Ok(())
}
```

---

### 5. `database/migrations/0001_create_users_table.sql`
```sql
CREATE TABLE IF NOT EXISTS users (
    id         INTEGER PRIMARY KEY AUTOINCREMENT,
    name       TEXT    NOT NULL,
    email      TEXT    NOT NULL UNIQUE,
    password   TEXT    NOT NULL,
    role       TEXT    NOT NULL DEFAULT 'user',
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    deleted_at DATETIME
);
```

---

### 6. `src/app/models/user.rs` — User Model
```rust
use async_trait::async_trait;
use serde::{Serialize, Deserialize};
use sqlx::FromRow;
use crate::database::{connection::DatabasePool, model::Model};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id:         i64,
    pub name:       String,
    pub email:      String,
    pub password:   String,
    pub role:       String,
}

#[async_trait]
impl Model for User {
    const TABLE: &'static str = "users";

    async fn find(pool: &DatabasePool, id: i64) -> Result<Self, sqlx::Error> {
        sqlx::query_as!(User,
            "SELECT id, name, email, password, role FROM users WHERE id = ? AND deleted_at IS NULL",
            id
        )
        .fetch_one(&pool.pool)
        .await
    }

    async fn all(pool: &DatabasePool) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as!(User,
            "SELECT id, name, email, password, role FROM users WHERE deleted_at IS NULL"
        )
        .fetch_all(&pool.pool)
        .await
    }

    async fn save(&self, pool: &DatabasePool) -> Result<i64, sqlx::Error> {
        let result = sqlx::query!(
            "INSERT INTO users (name, email, password, role) VALUES (?, ?, ?, ?)",
            self.name, self.email, self.password, self.role
        )
        .execute(&pool.pool)
        .await?;
        Ok(result.last_insert_rowid())
    }

    async fn delete(pool: &DatabasePool, id: i64) -> Result<bool, sqlx::Error> {
        // Soft delete — set deleted_at
        sqlx::query!("UPDATE users SET deleted_at = CURRENT_TIMESTAMP WHERE id = ?", id)
            .execute(&pool.pool)
            .await?;
        Ok(true)
    }
}
```

---

### 7. `src/core/application.rs` — Init DB saat Startup
```rust
// Tambahkan di Application::serve():
pub async fn serve(self, addr: &str) {
    // Init database
    let db_url = crate::support::env("DATABASE_URL", "sqlite:./lumina.db");
    let pool = DatabasePool::connect(&db_url).await
        .expect("❌ Gagal connect ke database");

    // Jalankan migrasi otomatis
    migration::run_migrations(&pool.pool).await
        .expect("❌ Migration failed");

    // ... lanjut serve HTTP
}
```

---

### 8. `.env.example` — Tambah DATABASE_URL
```env
# Database
DATABASE_URL=sqlite:./lumina.db
# Untuk PostgreSQL: DATABASE_URL=postgres://user:pass@localhost/lumina
```

---

## ✅ Validation Checklist

```bash
# Build
cargo build    # 0 errors

# Run (auto-migrate saat startup)
cargo run
# Expected output:
# 📦 Database connected: sqlite:./lumina.db
#   ✅ Migrated: 0001_create_users_table.sql
# 🌐 Listening on http://127.0.0.1:8000
```

### Endpoint baru setelah implementasi:
| Method | URL | Deskripsi |
|--------|-----|-----------|
| GET | `/api/users` | Ambil dari DB (bukan hardcode) |
| GET | `/api/users/:id` | Query DB by ID |
| POST | `/api/users` | Insert ke DB |

---

## 📌 Next Steps
Setelah Database Layer: **Auth Middleware** (Issue #3) — proteksi route API dengan JWT.

---
**End of Skill** 🚀
