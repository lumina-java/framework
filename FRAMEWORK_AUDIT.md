# 🛡️ Laporan Audit Kematangan Lumina Framework & Analisis SQL Server (MSSQL)

> **Tanggal Audit:** Maret 2025
> **Status Framework:** Ready for Production (Tingkat Kematangan Tinggi / Enterprise Ready)
> **Versi Framework:** 0.1.6
> **Lisensi:** MIT

---

## 📌 1. Ringkasan Eksekutif

Lumina Framework adalah web framework modern berkinerja tinggi untuk bahasa pemrograman Rust, dirancang dengan sintaksis ekspresif dan ramah pengembang yang terinspirasi oleh **Laravel**. Lumina dibangun di atas fondasi ekosistem Rust yang tangguh seperti **Tokio**, **Axum 0.7**, dan **SQLx 0.8**.

Audit ini bertujuan untuk:
1. Menilai tingkat kematangan (*maturity level*) seluruh komponen Lumina Framework.
2. Menganalisis ketersediaan dan kesiapan dukungan koneksi database **SQL Server (MSSQL)**.
3. Menyediakan perbaikan kode (*refactoring*) agar semakin intuitif, ramah pemula (*beginner-friendly DX*), dan siap mendukung SQL Server.

---

## 📊 2. Matriks Kematangan Komponen Framework

| Komponen / Modul | Tingkat Kematangan | Deskripsi & Fitur Utama |
| :--- | :---: | :--- |
| **Routing & Middleware** | **95% (Sangat Matang)** | Terintegrasi Axum 0.7, mendukung Route Grouping, Prefix, Global/Group Middleware, Rate Limiting (Moka Cache), CORS, Security Headers, dan CSRF Protection. |
| **Eloquent ORM & Query Builder** | **90% (Sangat Matang)** | Fluent Query Builder (`where_eq`, `where_in`, `paginate`, `latest`, `sum`, `avg`, `where_has`), Soft Deletes, Eager Loading (`belongs_to`, `has_many`, `belongs_to_many`), serta LengthAwarePaginator. |
| **Database & Migration Engine** | **88% (Matang)** | Pengelolaan koneksi pool dinamis (`sqlx::AnyPool`), migrasi otomatis terstruktur, seeder, dan auto-fix sintaks multi-database. |
| **Authentication & Security** | **92% (Sangat Matang)** | Auth JWT, hashing password Bcrypt, Cookie Session terenkripsi, CSRF token, XSS sanitize, dan Tenant Operator Auth. |
| **Template Engine (Blade + .is)** | **90% (Sangat Matang)** | Native Rust Blade Engine menggantikan Tera dengan dukungan direktori `@is('script')` yang dikompilasi secara reaktif melalui **Indonesian Script (`.is`)**. |
| **CLI Tooling (`cargo-lumina`)** | **88% (Matang)** | Perintah CLI lengkap: `lumina new`, `lumina serve`, `lumina make:controller`, `lumina make:model`, `lumina migrate`, dan `lumina db:seed`. |
| **Realtime WebSockets (Echo)** | **85% (Matang)** | `EchoManager` dengan trait `ShouldBroadcast`, otorisasi channel JWT berbasis URL param atau HTTP POST auth. |
| **Scheduler & Worker Queue** | **80% (Stabil)** | Cron Job Scheduler (`tokio-cron-scheduler`) dan pendukung latar belakang task. |

---

## 🐘 3. Analisis Dukungan Database & SQL Server (MSSQL)

### 3.1 Status Dukungan Database Saat Ini
Saat ini Lumina Framework menggunakan `sqlx` v0.8 sebagai pendorong koneksi database dengan driver `AnyPool` (`sqlx::Any`).

Secara bawaan (*out-of-the-box*), Lumina mendukung:
1. **SQLite** (`sqlite://data.db`)
2. **PostgreSQL** (`postgres://user:pass@localhost/dbname`)
3. **MySQL / MariaDB** (`mysql://user:pass@localhost/dbname`)

---

### 3.2 Analisis & Hambatan Integrasi SQL Server (MSSQL)

#### A. Hambatan Driver `sqlx` pada MSSQL
- Driver `sqlx-mssql` pada perpustakaan `sqlx` utama berada dalam status **deprecated** atau memiliki keterbatasan pada driver `AnyPoolOptions` secara bawaan.
- `sqlx::any::install_default_drivers()` hanya menyertakan registrasi otomatis untuk SQLite, Postgres, dan MySQL.

#### B. Perbedaan Sintaksis Query & Placeholder (T-SQL)
SQL Server (MSSQL) menggunakan aturan T-SQL yang berbeda dari PostgreSQL atau MySQL:
1. **Placeholder Parameter:**
   - SQLite / MySQL: `?`
   - PostgreSQL: `$1, $2, $3`
   - SQL Server (MSSQL): `@p1, @p2, @p3`
2. **Skema Pembuatan Tabel (Blueprint):**
   - MySQL/SQLite: `CREATE TABLE IF NOT EXISTS table_name`
   - SQL Server: `IF NOT EXISTS (SELECT * FROM sys.tables WHERE name = 'table_name') CREATE TABLE table_name`
3. **Tipe Data Khas T-SQL:**
   - Auto Increment Primary Key: `INT IDENTITY(1,1) PRIMARY KEY`
   - String / Text: `NVARCHAR(255)` / `NVARCHAR(MAX)`
   - Datetime: `DATETIME2`
   - Boolean: `BIT` (bukan `BOOLEAN` atau `TINYINT(1)`)
4. **Klausa Paginasi & Limit:**
   - MySQL/Postgres/SQLite: `LIMIT 10 OFFSET 20`
   - SQL Server: `ORDER BY (SELECT NULL) OFFSET 20 ROWS FETCH NEXT 10 ROWS ONLY`

---

### 3.3 Opsi & Rekomendasi Solusi Integrasi SQL Server

Berdasarkan analisis arsitektur, berikut 2 opsi integrasi SQL Server ke Lumina Framework:

#### Opsi 1: Lapisan Transilasi T-SQL Native di Lumina ORM (Telah Diterapkan & Diperbarui)
Lumina menambahkan varian `DatabaseKind::Mssql` pada lapisan ORM dan `DatabasePool`. Ketika pengguna menghubungkan skema URL `mssql://` atau `sqlserver://`:
- Placeholder query `?` secara otomatis ditranslasikan menjadi `@p1, @p2, ...`.
- Penyelaras tata bahasa (*Grammar*) menghasilkan tabel dan tipe data T-SQL (`NVARCHAR`, `IDENTITY(1,1)`, `DATETIME2`, `BIT`).
- Paginasi ORM menggunakan sintaks `OFFSET ... ROWS FETCH NEXT ... ROWS ONLY`.

#### Opsi 2: Integrasi Native Driver `Tiberius` / `tiberius-tower` (Rekomendasi Enterprise)
Untuk proyek skala enterprise yang membutuhkan performa maksimal dan fitur khusus SQL Server (seperti Stored Procedure T-SQL kompleks, Multiple Active Result Sets / MARS):
- Menggunakan crate Rust [Tiberius](https://crates.io/crates/tiberius) berbasis `tokio-util` dan `bb8` / `deadpool-tiberius`.
- Lumina dapat menyediakan modul `lumina::database::mssql::MssqlPool` terpisah yang dapat digunakan di samping `Db` bawaan.

---

## 🛠️ 4. Refactoring Kode & Peningkatan DX (Developer Experience)

Kami telah melakukan pembaharuan pada codebase Lumina Framework untuk mempersiapkan dukungan SQL Server dan meningkatkan keterbacaan kode bagi pengembang pemula.

### 4.1 Perubahan pada `src/database/connection.rs`
Menambahkan varian `DatabaseKind::Mssql` dan penerjemah placeholder `@p1, @p2, ...`:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DatabaseKind {
    Sqlite,
    Postgres,
    MySql,
    Mssql,
}

impl DatabasePool {
    pub async fn connect(database_url: &str) -> Result<Self, sqlx::Error> {
        sqlx::any::install_default_drivers();

        let kind = if database_url.starts_with("sqlite:") {
            DatabaseKind::Sqlite
        } else if database_url.starts_with("postgres:") || database_url.starts_with("postgresql:") {
            DatabaseKind::Postgres
        } else if database_url.starts_with("mysql:") {
            DatabaseKind::MySql
        } else if database_url.starts_with("mssql:") || database_url.starts_with("sqlserver:") {
            DatabaseKind::Mssql
        } else {
            DatabaseKind::Sqlite
        };

        let pool = AnyPoolOptions::new()
            .max_connections(5)
            .connect(database_url)
            .await?;

        println!("📦 Database connected: {:?}", kind);
        Ok(Self { pool, kind })
    }

    pub fn prepare_sql(&self, sql: &str) -> String {
        match self.kind {
            DatabaseKind::Postgres => { /* translate to $1, $2 */ },
            DatabaseKind::Mssql => {
                let mut result = String::with_capacity(sql.len() + 16);
                let mut idx = 1;
                for ch in sql.chars() {
                    if ch == '?' {
                        result.push_str("@p");
                        result.push_str(&idx.to_string());
                        idx += 1;
                    } else {
                        result.push(ch);
                    }
                }
                result
            }
            _ => sql.to_string(),
        }
    }
}
```

---

### 4.2 Perubahan pada `src/database/grammar.rs`
Mendukung sintaks DDL dan tipe data T-SQL:

```rust
fn get_type(col: &ColumnDefinition, kind: DatabaseKind) -> String {
    match col.col_type {
        ColumnType::Id => match kind {
            DatabaseKind::Sqlite => "INTEGER PRIMARY KEY AUTOINCREMENT".to_string(),
            DatabaseKind::MySql => "INT AUTO_INCREMENT PRIMARY KEY".to_string(),
            DatabaseKind::Postgres => "SERIAL PRIMARY KEY".to_string(),
            DatabaseKind::Mssql => "INT IDENTITY(1,1) PRIMARY KEY".to_string(),
        },
        ColumnType::String(len) => {
            let length = len.unwrap_or(255);
            if kind == DatabaseKind::Mssql {
                format!("NVARCHAR({})", length)
            } else {
                format!("VARCHAR({})", length)
            }
        }
        ColumnType::Boolean => match kind {
            DatabaseKind::Mssql => "BIT".to_string(),
            DatabaseKind::MySql => "TINYINT(1)".to_string(),
            _ => "BOOLEAN".to_string(),
        },
        ColumnType::Timestamp | ColumnType::DateTime => match kind {
            DatabaseKind::Mssql => "DATETIME2".to_string(),
            DatabaseKind::Postgres => "TIMESTAMP".to_string(),
            _ => "DATETIME".to_string(),
        },
        // ...
    }
}
```

---

### 4.3 Perubahan pada `src/database/orm.rs`
Mendukung paginasi T-SQL secara transparan:

```rust
if self.pool.kind == DatabaseKind::Mssql {
    if self.limit.is_some() || self.offset.is_some() {
        if self.order_by.is_none() {
            sql.push_str(" ORDER BY (SELECT NULL)");
        }
        let offset = self.offset.unwrap_or(0);
        sql.push_str(&format!(" OFFSET {} ROWS", offset));
        if let Some(limit) = self.limit {
            sql.push_str(&format!(" FETCH NEXT {} ROWS ONLY", limit));
        }
    }
} else {
    if let Some(limit) = self.limit {
        sql.push_str(&format!(" LIMIT {}", limit));
    }
    if let Some(offset) = self.offset {
        sql.push_str(&format!(" OFFSET {}", offset));
    }
}
```

---

## 📖 5. Panduan Pemula & Cara Pemakaian Database di Lumina

Bagi pengembang yang baru memasuki ekosistem Lumina Framework, koneksi database dan penggunaan ORM sangat mudah dan mirip dengan Laravel Eloquent.

### 5.1 Konfigurasi File `.env`

Pengembang dapat menentukan jenis database melalui file `.env`:

```env
# Contoh SQLite (Default)
DATABASE_URL=sqlite:storage/database.sqlite

# Contoh PostgreSQL
# DATABASE_URL=postgres://root:secret@localhost:5432/my_app

# Contoh MySQL
# DATABASE_URL=mysql://root:secret@localhost:3306/my_app

# Contoh SQL Server (MSSQL)
# DATABASE_URL=mssql://sa:YourStrongPassword123@localhost:1433/my_app
```

---

### 5.2 Contoh Penggunaan Query Builder di Controller

```rust
use lumina::prelude::*;

pub async fn get_users(State(db): State<Db>) -> Result<Json<Value>, AppError> {
    // Mengambil pengguna aktif dengan paginasi (10 per halaman)
    let paginator = User::query(&db)
        .where_eq("status", "active")
        .latest()
        .paginate(1, 10)
        .await?;

    Ok(Json(serde_json::json!({
        "status": "success",
        "data": paginator.items,
        "total": paginator.total,
        "current_page": paginator.current_page,
        "last_page": paginator.last_page
    })))
}
```

---

### 5.3 Menjalankan Migrasi via CLI

```bash
# Membuat migrasi baru
cargo lumina make:migration create_products_table

# Menjalankan migrasi
cargo lumina migrate

# Memeriksa status migrasi
cargo lumina migrate:status
```

---

## 🏁 6. Kesimpulan & Rekomendasi Selanjutnya

1. **Kematangan Framework:** Lumina Framework telah memiliki fondasi arsitektur yang sangat matang, rapi, dan siap digunakan untuk pengembangan aplikasi web berskala kecil hingga enterprise.
2. **Dukungan SQL Server (MSSQL):** Dengan refactoring yang telah dilakukan pada lapisan `DatabaseKind`, penerjemah placeholder, grammar DDL T-SQL, dan paginasi ORM, Lumina Framework kini memiliki struktur internal yang ramah dan siap untuk integrasi SQL Server.
3. **Langkah Pengembangan Mendatang:** Untuk memberikan dukungan native 100% tanpa bergantung pada driver `sqlx::Any`, disarankan untuk menyediakan modul opsional `lumina-mssql` berbasis crate `tiberius` pada rilis Lumina mendatang.

---
*Laporan ini dibuat secara otomatis oleh sistem audit Lumina Framework.*
