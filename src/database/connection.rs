use sqlx::{any::AnyPoolOptions, Any, Pool};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DatabaseKind {
    Sqlite,
    Postgres,
    MySql,
}

/// Lumina DatabasePool — wrapper di atas `sqlx::Pool<Any>`.
#[derive(Clone)]
pub struct DatabasePool {
    pub pool: Pool<Any>,
    pub kind: DatabaseKind,
}

impl DatabasePool {
    /// Buat koneksi pool baru dari `database_url`.
    pub async fn connect(database_url: &str) -> Result<Self, sqlx::Error> {
        sqlx::any::install_default_drivers();

        // Tentukan jenis database dari URL
        let kind = if database_url.starts_with("sqlite:") {
            DatabaseKind::Sqlite
        } else if database_url.starts_with("postgres:") || database_url.starts_with("postgresql:") {
            DatabaseKind::Postgres
        } else if database_url.starts_with("mysql:") {
            DatabaseKind::MySql
        } else {
            // Default atau fallback (misal SQLite)
            DatabaseKind::Sqlite
        };

        let pool = AnyPoolOptions::new()
            .max_connections(5)
            .connect(database_url)
            .await?;

        println!("📦 Database connected: {:?}", kind);
        Ok(Self { pool, kind })
    }

    /// Helper untuk mendapatkan referensi ke pool asli
    pub fn get_pool(&self) -> &Pool<Any> {
        &self.pool
    }

    /// Menyesuaikan placeholder query secara otomatis berdasarkan driver database aktif.
    /// SQLite/MySQL: tetap menggunakan `?`.
    /// PostgreSQL: mentranslasikan `?` menjadi `$1, $2, $3, ...`.
    pub fn prepare_sql(&self, sql: &str) -> String {
        if self.kind == DatabaseKind::Postgres {
            let mut result = String::with_capacity(sql.len() + 16);
            let mut idx = 1;
            for ch in sql.chars() {
                if ch == '?' {
                    result.push('$');
                    result.push_str(&idx.to_string());
                    idx += 1;
                } else {
                    result.push(ch);
                }
            }
            result
        } else {
            sql.to_string()
        }
    }
}

