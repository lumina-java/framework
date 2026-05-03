use sqlx::{Pool, Any};
use crate::database::connection::DatabaseKind;
use std::{fs, path::Path};
use regex::Regex;

/// Jalankan semua file migrasi dari direktori `database/migrations/`
/// secara berurutan. Mendukung multi-database (SQLite, MySQL, Postgres).
pub async fn run_migrations(pool: &Pool<Any>, kind: DatabaseKind) -> Result<(), sqlx::Error> {
    // 1. Buat tabel tracker migrasi dengan syntax yang sesuai driver
    let create_table_sql = match kind {
        DatabaseKind::Sqlite => {
            "CREATE TABLE IF NOT EXISTS _migrations (
                id       INTEGER PRIMARY KEY AUTOINCREMENT,
                name     TEXT NOT NULL UNIQUE,
                run_at   DATETIME DEFAULT CURRENT_TIMESTAMP
            )"
        }
        DatabaseKind::MySql => {
            "CREATE TABLE IF NOT EXISTS _migrations (
                id       INT AUTO_INCREMENT PRIMARY KEY,
                name     VARCHAR(255) NOT NULL UNIQUE,
                run_at   DATETIME DEFAULT CURRENT_TIMESTAMP
            )"
        }
        DatabaseKind::Postgres => {
            "CREATE TABLE IF NOT EXISTS _migrations (
                id       SERIAL PRIMARY KEY,
                name     VARCHAR(255) NOT NULL UNIQUE,
                run_at   TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            )"
        }
    };

    sqlx::query(create_table_sql)
        .execute(pool)
        .await?;

    let migration_dir = Path::new("database/migrations");
    if !migration_dir.exists() {
        println!("  ⚠️  Migration folder tidak ditemukan, skip.");
        return Ok(());
    }

    let mut files: Vec<_> = fs::read_dir(migration_dir)
        .expect("Gagal baca direktori migrations")
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map(|x| x == "sql").unwrap_or(false))
        .collect();

    files.sort_by_key(|e| e.file_name());

    // Siapkan Regex untuk pembersihan spasi dan replacement
    // Case-insensitive replacement untuk AUTOINCREMENT
    let re_autoinc = Regex::new(r"(?i)INTEGER\s+PRIMARY\s+KEY\s+AUTOINCREMENT").unwrap();
    let re_datetime = Regex::new(r"(?i)DATETIME\s+DEFAULT\s+CURRENT_TIMESTAMP").unwrap();

    for entry in files {
        let name = entry.file_name().to_string_lossy().to_string();

        let count: i64 = sqlx::query_scalar("SELECT COUNT(1) FROM _migrations WHERE name = ?")
            .bind(&name)
            .fetch_one(pool)
            .await?;

        if count > 0 {
            continue;
        }

        let mut sql = fs::read_to_string(entry.path())
            .expect(&format!("Gagal baca file: {}", name));

        // Auto-fix syntax menggunakan Regex agar tidak sensitif terhadap spasi/tab
        match kind {
            DatabaseKind::MySql => {
                sql = re_autoinc.replace_all(&sql, "INT AUTO_INCREMENT PRIMARY KEY").to_string();
                sql = re_datetime.replace_all(&sql, "DATETIME DEFAULT CURRENT_TIMESTAMP").to_string();
                // Ganti TEXT ke VARCHAR(255) hanya jika baris tersebut berisi UNIQUE (biasanya email/username)
                // Ini pendekatan sederhana, untuk project besar disarankan migrasi terpisah.
                if sql.contains("UNIQUE") {
                    sql = sql.replace("TEXT", "VARCHAR(255)");
                }
            },
            DatabaseKind::Postgres => {
                sql = re_autoinc.replace_all(&sql, "SERIAL PRIMARY KEY").to_string();
                sql = re_datetime.replace_all(&sql, "TIMESTAMP DEFAULT CURRENT_TIMESTAMP").to_string();
                if sql.contains("UNIQUE") {
                    sql = sql.replace("TEXT", "VARCHAR(255)");
                }
            },
            _ => {}
        }

        sqlx::query(&sql).execute(pool).await?;

        sqlx::query("INSERT INTO _migrations (name) VALUES (?)")
            .bind(&name)
            .execute(pool)
            .await?;

        println!("  ✅ Migrated: {}", name);
    }

    Ok(())
}

pub async fn migrate_status(pool: &Pool<Any>) -> Result<(), sqlx::Error> {
    let applied_migrations: Vec<String> = match sqlx::query_scalar::<_, String>("SELECT name FROM _migrations ORDER BY id ASC")
        .fetch_all(pool)
        .await {
            Ok(v) => v,
            Err(_) => {
                println!("⚠️  Tabel _migrations tidak ditemukan atau belum diinisialisasi.");
                return Ok(());
            }
        };

    let migration_dir = Path::new("database/migrations");
    if !migration_dir.exists() {
        println!("⚠️  Migration folder tidak ditemukan.");
        return Ok(());
    }

    let mut files: Vec<_> = fs::read_dir(migration_dir)
        .expect("Gagal baca direktori migrations")
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map(|x| x == "sql").unwrap_or(false))
        .collect();

    files.sort_by_key(|e| e.file_name());

    println!("\n📊 Status Migrasi Database:");
    println!("+-----------------------------------------------+-----------+");
    println!("| {:<45} | {:<9} |", "Nama File Migrasi", "Status");
    println!("+-----------------------------------------------+-----------+");

    for entry in files {
        let name = entry.file_name().to_string_lossy().to_string();
        let status = if applied_migrations.contains(&name) {
            "Applied"
        } else {
            "Pending"
        };
        println!("| {:<45} | {:<9} |", name, status);
    }
    println!("+-----------------------------------------------+-----------+\n");

    Ok(())
}

pub async fn migrate_rollback(pool: &Pool<Any>) -> Result<(), sqlx::Error> {
    let last_migration: Option<String> = match sqlx::query_scalar::<_, String>("SELECT name FROM _migrations ORDER BY id DESC LIMIT 1")
        .fetch_optional(pool)
        .await {
            Ok(v) => v,
            Err(_) => {
                println!("⚠️  Tabel _migrations tidak ditemukan.");
                return Ok(());
            }
        };

    let name = match last_migration {
        Some(n) => n,
        None => {
            println!("ℹ️  Tidak ada migrasi yang bisa di-rollback.");
            return Ok(());
        }
    };

    println!("🔄 Memulai rollback untuk migrasi: {}", name);

    let migration_dir = Path::new("database/migrations");
    let file_path = migration_dir.join(&name);

    if !file_path.exists() {
        println!("❌ File migrasi {} tidak ditemukan di folder migrations.", name);
        return Ok(());
    }

    let sql = fs::read_to_string(file_path).expect("Gagal baca file migrasi");

    let re_create = Regex::new(r"(?i)CREATE\s+TABLE\s+(?:IF\s+NOT\s+EXISTS\s+)?([a-zA-Z0-9_]+)").unwrap();
    if let Some(caps) = re_create.captures(&sql) {
        let table_name = &caps[1];
        let drop_sql = format!("DROP TABLE IF EXISTS {}", table_name);
        println!("💥 Menghapus tabel: {}", table_name);
        sqlx::query(&drop_sql).execute(pool).await?;
    } else {
        println!("⚠️  Gagal mengekstrak nama tabel dari file migrasi.");
    }

    sqlx::query("DELETE FROM _migrations WHERE name = ?")
        .bind(&name)
        .execute(pool)
        .await?;

    println!("✅ Sukses rollback migrasi: {}", name);

    Ok(())
}
