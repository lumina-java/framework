use sqlx::{Pool, Sqlite};
use std::{fs, path::Path};

/// Jalankan semua file migrasi dari direktori `database/migrations/`
/// secara berurutan berdasarkan nama file (0001_, 0002_, dst.).
///
/// Migration yang sudah pernah dijalankan akan di-skip otomatis
/// menggunakan tabel `_migrations` sebagai tracker.
pub async fn run_migrations(pool: &Pool<Sqlite>) -> Result<(), sqlx::Error> {
    // Buat tabel tracker migrasi jika belum ada
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS _migrations (
            id       INTEGER  PRIMARY KEY AUTOINCREMENT,
            name     TEXT     NOT NULL UNIQUE,
            run_at   DATETIME DEFAULT CURRENT_TIMESTAMP
        )"
    )
    .execute(pool)
    .await?;

    let migration_dir = Path::new("database/migrations");
    if !migration_dir.exists() {
        println!("  ⚠️  Migration folder tidak ditemukan, skip.");
        return Ok(());
    }

    // Kumpulkan semua file .sql
    let mut files: Vec<_> = fs::read_dir(migration_dir)
        .expect("Gagal baca direktori migrations")
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map(|x| x == "sql").unwrap_or(false))
        .collect();

    // Urutkan berdasarkan nama file
    files.sort_by_key(|e| e.file_name());

    for entry in files {
        let name = entry.file_name().to_string_lossy().to_string();

        // Cek apakah sudah pernah dijalankan
        let exists: bool = sqlx::query_scalar(
            "SELECT EXISTS(SELECT 1 FROM _migrations WHERE name = ?)"
        )
        .bind(&name)
        .fetch_one(pool)
        .await?;

        if exists {
            continue; // Skip — sudah pernah dijalankan
        }

        // Baca dan eksekusi SQL
        let sql = fs::read_to_string(entry.path())
            .expect(&format!("Gagal baca file: {}", name));

        sqlx::raw_sql(&sql).execute(pool).await?;

        // Catat ke tabel tracker
        sqlx::query("INSERT INTO _migrations (name) VALUES (?)")
            .bind(&name)
            .execute(pool)
            .await?;

        println!("  ✅ Migrated: {}", name);
    }

    Ok(())
}
