use colored::*;
use sqlx::any::AnyPoolOptions;
use sqlx::{Pool, Any, Row, Executor};

pub type AnyPool = Pool<Any>;
use std::fs;
use std::path::Path;
use std::time::Instant;

/// Helper untuk menyelesaikan URL database dari .env
pub fn resolve_database_url() -> Result<(String, String), String> {
    dotenv::dotenv().ok();

    if let Ok(url) = std::env::var("DATABASE_URL") {
        if !url.trim().is_empty() {
            let driver = if url.starts_with("postgres") || url.starts_with("postgresql") {
                "postgresql".to_string()
            } else if url.starts_with("mysql") {
                "mysql".to_string()
            } else {
                "sqlite".to_string()
            };
            return Ok((url, driver));
        }
    }

    let conn = std::env::var("DB_CONNECTION").unwrap_or_else(|_| "sqlite".to_string());
    let host = std::env::var("DB_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = std::env::var("DB_PORT").unwrap_or_default();
    let db = std::env::var("DB_DATABASE").unwrap_or_else(|_| "./database.sqlite".to_string());
    let user = std::env::var("DB_USERNAME").unwrap_or_default();
    let pass = std::env::var("DB_PASSWORD").unwrap_or_default();

    match conn.to_lowercase().as_str() {
        "postgres" | "postgresql" | "pg" => {
            let p = if port.is_empty() { "5432" } else { &port };
            let auth = if pass.is_empty() { user } else { format!("{}:{}", user, pass) };
            Ok((format!("postgres://{}@{}:{}/{}", auth, host, p, db), "postgresql".to_string()))
        }
        "mysql" | "mariadb" => {
            let p = if port.is_empty() { "3306" } else { &port };
            let auth = if pass.is_empty() { user } else { format!("{}:{}", user, pass) };
            Ok((format!("mysql://{}@{}:{}/{}", auth, host, p, db), "mysql".to_string()))
        }
        _ => {
            let path = if db.starts_with("./") || db.starts_with("/") || db.contains(":") {
                db
            } else {
                format!("./{}", db)
            };
            Ok((format!("sqlite:{}", path), "sqlite".to_string()))
        }
    }
}

/// Menghubungkan ke database dengan AnyPool
pub async fn connect_db(url: &str) -> Result<AnyPool, String> {
    sqlx::any::install_default_drivers();
    AnyPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(std::time::Duration::from_secs(10))
        .connect(url)
        .await
        .map_err(|e| format!("Gagal terhubung ke database di '{}': {}", mask_credentials(url), e))
}

fn mask_credentials(url: &str) -> String {
    if let Some(at_idx) = url.find('@') {
        if let Some(proto_idx) = url.find("://") {
            return format!("{}://***@{}", &url[..proto_idx], &url[at_idx + 1..]);
        }
    }
    url.to_string()
}

/// Memastikan tabel tracking migrasi (_lumina_migrations) tersedia
pub async fn ensure_migrations_table(pool: &AnyPool, driver: &str) -> Result<(), String> {
    let ddl = match driver {
        "postgresql" => {
            r#"
            CREATE TABLE IF NOT EXISTS _lumina_migrations (
                id SERIAL PRIMARY KEY,
                migration_name VARCHAR(255) UNIQUE NOT NULL,
                batch INTEGER NOT NULL DEFAULT 1,
                applied_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
            );
            "#
        }
        "mysql" => {
            r#"
            CREATE TABLE IF NOT EXISTS _lumina_migrations (
                id INT AUTO_INCREMENT PRIMARY KEY,
                migration_name VARCHAR(255) UNIQUE NOT NULL,
                batch INT NOT NULL DEFAULT 1,
                applied_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            );
            "#
        }
        _ => {
            r#"
            CREATE TABLE IF NOT EXISTS _lumina_migrations (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                migration_name TEXT UNIQUE NOT NULL,
                batch INTEGER NOT NULL DEFAULT 1,
                applied_at DATETIME DEFAULT CURRENT_TIMESTAMP
            );
            "#
        }
    };

    pool.execute(ddl)
        .await
        .map_err(|e| format!("Gagal menginisialisasi tabel tracking migrasi: {}", e))?;

    Ok(())
}

/// Mengambil daftar nama migrasi yang sudah pernah diaplikasikan
pub async fn get_applied_migrations(pool: &AnyPool) -> Result<Vec<(String, i32, String)>, String> {
    let mut results = Vec::new();

    // 1. Baca dari _lumina_migrations
    let rows = sqlx::query("SELECT migration_name, batch, CAST(applied_at AS TEXT) as applied_at FROM _lumina_migrations ORDER BY id ASC")
        .fetch_all(pool)
        .await;

    if let Ok(rows) = rows {
        for row in rows {
            let name: String = row.try_get("migration_name").unwrap_or_default();
            let batch: i32 = row.try_get("batch").unwrap_or(1);
            let applied_at: String = row.try_get("applied_at").unwrap_or_default();
            results.push((name, batch, applied_at));
        }
    }

    // 2. Cek kompatibilitas balik dengan _app_migrations jika ada
    let legacy_rows = sqlx::query("SELECT migration_name, CAST(applied_at AS TEXT) as applied_at FROM _app_migrations ORDER BY id ASC")
        .fetch_all(pool)
        .await;

    if let Ok(legacy_rows) = legacy_rows {
        for row in legacy_rows {
            let name: String = row.try_get("migration_name").unwrap_or_default();
            let applied_at: String = row.try_get("applied_at").unwrap_or_default();
            if !results.iter().any(|(n, _, _)| n == &name) {
                results.push((name, 1, applied_at));
            }
        }
    }

    Ok(results)
}

/// Eksekusi Perintah: lumina migrate
pub async fn run_migrate_async() {
    println!("{}", "🗄  LUMINA DATABASE MIGRATION ENGINE".bright_purple().bold());
    println!("─────────────────────────────────────────────────────────────────");

    let (url, driver) = match resolve_database_url() {
        Ok(res) => res,
        Err(e) => {
            println!("  {} {}", "✖ [ERROR]".red().bold(), e.bright_red());
            return;
        }
    };

    println!("  Target Database : {}", mask_credentials(&url).bright_cyan());
    println!("  Driver Engine   : {}", driver.bright_yellow());

    let pool = match connect_db(&url).await {
        Ok(p) => p,
        Err(e) => {
            println!("  {} {}", "✖ [CONN ERROR]".red().bold(), e.bright_red());
            return;
        }
    };

    if let Err(e) = ensure_migrations_table(&pool, &driver).await {
        println!("  {} {}", "✖ [ERROR]".red().bold(), e.bright_red());
        return;
    }

    let migrations_dir = Path::new("database/migrations");
    if !migrations_dir.exists() {
        println!("  {} Direktori database/migrations tidak ditemukan.", "ℹ [INFO]".blue());
        return;
    }

    let applied_list = get_applied_migrations(&pool).await.unwrap_or_default();
    let applied_names: std::collections::HashSet<String> = applied_list.iter().map(|(n, _, _)| n.clone()).collect();

    // Hitung batch berikutnya
    let max_batch = applied_list.iter().map(|(_, b, _)| *b).max().unwrap_or(0);
    let next_batch = max_batch + 1;

    let mut entries: Vec<_> = fs::read_dir(migrations_dir)
        .unwrap()
        .filter_map(|e| e.ok())
        .filter(|e| {
            let path = e.path();
            path.extension().map_or(false, |ext| ext == "sql") 
                && !path.file_name().unwrap().to_string_lossy().ends_with(".down.sql")
        })
        .collect();

    entries.sort_by_key(|e| e.path());

    let mut applied_count = 0;
    println!("\n[+] Menjalankan Migrasi Pending:");

    for entry in entries {
        let file_name = entry.file_name().to_string_lossy().to_string();

        if applied_names.contains(&file_name) {
            println!("  {} {}", "✔ [SKIPPED]".dimmed(), file_name.dimmed());
            continue;
        }

        let start = Instant::now();
        let sql = match fs::read_to_string(entry.path()) {
            Ok(s) => s,
            Err(e) => {
                println!("  {} Gagal membaca file {}: {}", "✖ [ERROR]".red().bold(), file_name, e);
                return;
            }
        };

        // Eksekusi SQL transaksional
        let mut tx = match pool.begin().await {
            Ok(t) => t,
            Err(e) => {
                println!("  {} Gagal memulai transaksi: {}", "✖ [ERROR]".red().bold(), e);
                return;
            }
        };

        if let Err(e) = tx.execute(sql.as_str()).await {
            println!("  {} Gagal mengeksekusi {}: {}", "✖ [FAILED]".red().bold(), file_name.bright_red(), e);
            let _ = tx.rollback().await;
            return;
        }

        // Catat ke _lumina_migrations
        let insert_query = if driver == "postgresql" {
            "INSERT INTO _lumina_migrations (migration_name, batch) VALUES ($1, $2)"
        } else {
            "INSERT INTO _lumina_migrations (migration_name, batch) VALUES (?, ?)"
        };
        if let Err(e) = sqlx::query(insert_query)
            .bind(&file_name)
            .bind(next_batch)
            .execute(&mut *tx)
            .await
        {
            println!("  {} Gagal mencatat status migrasi {}: {}", "✖ [FAILED]".red().bold(), file_name, e);
            let _ = tx.rollback().await;
            return;
        }

        if let Err(e) = tx.commit().await {
            println!("  {} Gagal melakukan commit transaksi: {}", "✖ [ERROR]".red().bold(), e);
            return;
        }

        let duration = start.elapsed();
        println!(
            "  {} {} (Batch #{} - {:.2?})",
            "✔ [MIGRATED]".green().bold(),
            file_name.bright_white(),
            next_batch.to_string().bright_yellow(),
            duration
        );
        applied_count += 1;
    }

    println!("─────────────────────────────────────────────────────────────────");
    if applied_count == 0 {
        println!("✨ Database sudah mutakhir. Tidak ada migrasi pending.");
    } else {
        println!("🎯 Sukses: {} berkas migrasi berhasil dieksekusi (Batch #{}).", applied_count.to_string().bright_green().bold(), next_batch);
    }
}

/// Eksekusi Perintah: lumina migrate:rollback
pub async fn run_rollback_async() {
    println!("{}", "🔄  LUMINA DATABASE ROLLBACK ENGINE".bright_purple().bold());
    println!("─────────────────────────────────────────────────────────────────");

    let (url, driver) = match resolve_database_url() {
        Ok(res) => res,
        Err(e) => {
            println!("  {} {}", "✖ [ERROR]".red().bold(), e.bright_red());
            return;
        }
    };

    let pool = match connect_db(&url).await {
        Ok(p) => p,
        Err(e) => {
            println!("  {} {}", "✖ [CONN ERROR]".red().bold(), e.bright_red());
            return;
        }
    };

    let applied_list = get_applied_migrations(&pool).await.unwrap_or_default();
    let max_batch = applied_list.iter().map(|(_, b, _)| *b).max().unwrap_or(0);

    if max_batch == 0 {
        println!("  {} Tidak ada migrasi yang dapat di-rollback.", "ℹ [INFO]".blue());
        return;
    }

    let target_migrations: Vec<_> = applied_list.into_iter().filter(|(_, b, _)| *b == max_batch).collect();
    println!("  Membatalkan migrasi Batch #{} ({} berkas):\n", max_batch.to_string().bright_yellow(), target_migrations.len());

    let migrations_dir = Path::new("database/migrations");

    for (file_name, _, _) in target_migrations.into_iter().rev() {
        let down_file = format!("{}.down.sql", file_name.trim_end_matches(".sql"));
        let down_path = migrations_dir.join(&down_file);

        let mut tx = match pool.begin().await {
            Ok(t) => t,
            Err(e) => {
                println!("  {} Gagal memulai transaksi: {}", "✖ [ERROR]".red().bold(), e);
                return;
            }
        };

        if down_path.exists() {
            let sql = fs::read_to_string(&down_path).unwrap_or_default();
            if let Err(e) = tx.execute(sql.as_str()).await {
                println!("  {} Gagal mengeksekusi {}: {}", "✖ [FAILED]".red().bold(), down_file, e);
                let _ = tx.rollback().await;
                return;
            }
        }

        // Hapus dari _lumina_migrations
        let del_query = if driver == "postgresql" {
            "DELETE FROM _lumina_migrations WHERE migration_name = $1"
        } else {
            "DELETE FROM _lumina_migrations WHERE migration_name = ?"
        };
        let _ = sqlx::query(del_query).bind(&file_name).execute(&mut *tx).await;

        let _ = tx.commit().await;
        println!("  {} {}", "✔ [ROLLED BACK]".yellow().bold(), file_name.bright_white());
    }

    println!("─────────────────────────────────────────────────────────────────");
    println!("🎯 Rollback Batch #{} selesai.", max_batch);
}

/// Eksekusi Perintah: lumina migrate:status
pub async fn run_status_async() {
    println!("{}", "📊  LUMINA DATABASE MIGRATION STATUS".bright_purple().bold());
    println!("─────────────────────────────────────────────────────────────────");

    let (url, _) = match resolve_database_url() {
        Ok(res) => res,
        Err(e) => {
            println!("  {} {}", "✖ [ERROR]".red().bold(), e.bright_red());
            return;
        }
    };

    let pool = match connect_db(&url).await {
        Ok(p) => p,
        Err(e) => {
            println!("  {} {}", "✖ [CONN ERROR]".red().bold(), e.bright_red());
            return;
        }
    };

    let applied_list = get_applied_migrations(&pool).await.unwrap_or_default();
    let applied_map: std::collections::HashMap<String, (i32, String)> = applied_list
        .into_iter()
        .map(|(name, batch, applied_at)| (name, (batch, applied_at)))
        .collect();

    let migrations_dir = Path::new("database/migrations");
    if !migrations_dir.exists() {
        println!("  Direktori database/migrations tidak ditemukan.");
        return;
    }

    let mut entries: Vec<_> = fs::read_dir(migrations_dir)
        .unwrap()
        .filter_map(|e| e.ok())
        .filter(|e| {
            let p = e.path();
            p.extension().map_or(false, |ext| ext == "sql") && !p.file_name().unwrap().to_string_lossy().ends_with(".down.sql")
        })
        .collect();

    entries.sort_by_key(|e| e.path());

    println!("{:<6} {:<45} {:<8} {:<20}", "STATUS", "MIGRATION FILE", "BATCH", "APPLIED AT");
    println!("{:-<6} {:-<45} {:-<8} {:-<20}", "", "", "", "");

    for entry in entries {
        let name = entry.file_name().to_string_lossy().to_string();
        if let Some((batch, applied_at)) = applied_map.get(&name) {
            println!(
                "{:<6} {:<45} {:<8} {:<20}",
                "Ran".green().bold(),
                name,
                batch.to_string().yellow(),
                applied_at
            );
        } else {
            println!(
                "{:<6} {:<45} {:<8} {:<20}",
                "Pending".red().bold(),
                name.bright_white(),
                "-",
                "-"
            );
        }
    }
    println!("─────────────────────────────────────────────────────────────────");
}

/// Eksekusi Perintah: lumina db:seed
pub async fn run_seed_async() {
    println!("{}", "🌱  LUMINA DATABASE SEEDER ENGINE".bright_purple().bold());
    println!("─────────────────────────────────────────────────────────────────");

    let (url, driver) = match resolve_database_url() {
        Ok(res) => res,
        Err(e) => {
            println!("  {} {}", "✖ [ERROR]".red().bold(), e.bright_red());
            return;
        }
    };

    println!("  Target Database : {}", mask_credentials(&url).bright_cyan());
    println!("  Driver Engine   : {}", driver.bright_yellow());

    let pool = match connect_db(&url).await {
        Ok(p) => p,
        Err(e) => {
            println!("  {} {}", "✖ [CONN ERROR]".red().bold(), e.bright_red());
            return;
        }
    };

    let seeders_dir = Path::new("database/seeders");
    if !seeders_dir.exists() {
        println!("  {} Direktori database/seeders tidak ditemukan.", "ℹ [INFO]".blue());
        return;
    }

    let mut entries: Vec<_> = fs::read_dir(seeders_dir)
        .unwrap()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map_or(false, |ext| ext == "sql"))
        .collect();

    entries.sort_by_key(|e| e.path());

    if entries.is_empty() {
        println!("  {} Tidak ada berkas .sql ditemukan di database/seeders/.", "ℹ [INFO]".blue());
        return;
    }

    let mut count = 0;
    println!("\n[+] Menjalankan Seed Data:");
    for entry in entries {
        let file_name = entry.file_name().to_string_lossy().to_string();
        let start = Instant::now();
        let sql = match fs::read_to_string(entry.path()) {
            Ok(s) => s,
            Err(e) => {
                println!("  {} Gagal membaca berkas {}: {}", "✖ [ERROR]".red().bold(), file_name, e);
                continue;
            }
        };

        let mut tx = match pool.begin().await {
            Ok(t) => t,
            Err(e) => {
                println!("  {} Gagal memulai transaksi untuk {}: {}", "✖ [ERROR]".red().bold(), file_name, e);
                continue;
            }
        };

        if let Err(e) = tx.execute(sql.as_str()).await {
            println!("  {} Gagal mengeksekusi {}: {}", "✖ [FAILED]".red().bold(), file_name.bright_red(), e);
            let _ = tx.rollback().await;
            continue;
        }

        if let Err(e) = tx.commit().await {
            println!("  {} Gagal commit transaksi {}: {}", "✖ [ERROR]".red().bold(), file_name, e);
            continue;
        }

        let duration = start.elapsed();
        println!("  {} {} ({:.2?})", "✔ [SEEDED]".green().bold(), file_name.bright_white(), duration);
        count += 1;
    }

    println!("─────────────────────────────────────────────────────────────────");
    println!("🎯 Sukses: {} berkas seeder berhasil dieksekusi.", count.to_string().bright_green().bold());
}

/// Eksekusi Perintah: lumina queue:work
pub async fn run_queue_worker_async(queue_name: &str) {
    println!("{}", "⚡  LUMINA PERSISTENT QUEUE WORKER".bright_purple().bold());
    println!("─────────────────────────────────────────────────────────────────");

    let (url, driver) = match resolve_database_url() {
        Ok(res) => res,
        Err(e) => {
            println!("  {} {}", "✖ [ERROR]".red().bold(), e.bright_red());
            return;
        }
    };

    println!("  Target Database : {}", mask_credentials(&url).bright_cyan());
    println!("  Driver Engine   : {}", driver.bright_yellow());
    println!("  Queue Channel   : {}", queue_name.bright_green());

    let pool = match connect_db(&url).await {
        Ok(p) => p,
        Err(e) => {
            println!("  {} {}", "✖ [CONN ERROR]".red().bold(), e.bright_red());
            return;
        }
    };

    // Pastikan tabel jobs tersedia
    let ddl = match driver.as_str() {
        "postgresql" => {
            r#"
            CREATE TABLE IF NOT EXISTS jobs (
                id SERIAL PRIMARY KEY,
                queue VARCHAR(255) NOT NULL DEFAULT 'default',
                payload TEXT NOT NULL,
                attempts INT NOT NULL DEFAULT 0,
                reserved_at TIMESTAMPTZ,
                available_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
            );
            "#
        }
        "mysql" => {
            r#"
            CREATE TABLE IF NOT EXISTS jobs (
                id BIGINT AUTO_INCREMENT PRIMARY KEY,
                queue VARCHAR(255) NOT NULL DEFAULT 'default',
                payload LONGTEXT NOT NULL,
                attempts INT NOT NULL DEFAULT 0,
                reserved_at TIMESTAMP NULL,
                available_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
                created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
            );
            "#
        }
        _ => {
            r#"
            CREATE TABLE IF NOT EXISTS jobs (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                queue TEXT NOT NULL DEFAULT 'default',
                payload TEXT NOT NULL,
                attempts INTEGER NOT NULL DEFAULT 0,
                reserved_at DATETIME,
                available_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
                created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
            );
            "#
        }
    };

    if let Err(e) = pool.execute(ddl).await {
        println!("  {} Gagal memastikan tabel jobs: {}", "✖ [ERROR]".red(), e);
        return;
    }

    println!("\n[+] Worker aktif dan memantau antrean. Tekan Ctrl+C untuk berhenti.\n");

    loop {
        let is_pg = driver == "postgresql";
        let fetch_sql = if is_pg {
            "SELECT id, payload FROM jobs WHERE reserved_at IS NULL AND available_at <= NOW() AND queue = $1 ORDER BY available_at ASC LIMIT 1"
        } else {
            "SELECT id, payload FROM jobs WHERE reserved_at IS NULL AND available_at <= CURRENT_TIMESTAMP AND queue = ? ORDER BY available_at ASC LIMIT 1"
        };

        let row: Option<(i64, String)> = sqlx::query_as(fetch_sql)
            .bind(queue_name)
            .fetch_optional(&pool)
            .await
            .unwrap_or(None);

        match row {
            Some((id, payload_str)) => {
                let start = Instant::now();
                let payload: serde_json::Value = serde_json::from_str(&payload_str)
                    .unwrap_or_else(|_| serde_json::json!({"job": "unknown"}));
                let job_name = payload["job"].as_str().unwrap_or("unknown");

                println!("  {} Job [{}] ID: {}", "▶ [PROCESSING]".cyan().bold(), job_name.bright_white(), id);

                // Reserve
                let reserve_sql = if is_pg {
                    "UPDATE jobs SET reserved_at = NOW(), attempts = attempts + 1 WHERE id = $1"
                } else {
                    "UPDATE jobs SET reserved_at = CURRENT_TIMESTAMP, attempts = attempts + 1 WHERE id = ?"
                };
                let _ = sqlx::query(reserve_sql).bind(id).execute(&pool).await;

                // Delete completed
                let del_sql = if is_pg {
                    "DELETE FROM jobs WHERE id = $1"
                } else {
                    "DELETE FROM jobs WHERE id = ?"
                };
                let _ = sqlx::query(del_sql).bind(id).execute(&pool).await;

                println!("  {} Job [{}] ID: {} ({:.2?})\n", "✔ [PROCESSED]".green().bold(), job_name.bright_white(), id, start.elapsed());
            }
            None => {
                tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
            }
        }
    }
}
