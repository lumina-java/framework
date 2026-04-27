use sqlx::{Pool, Sqlite, sqlite::SqlitePoolOptions};

/// Lumina DatabasePool — wrapper di atas `sqlx::Pool<Sqlite>`.
///
/// Mendukung SQLite untuk development, PostgreSQL untuk production
/// (cukup ganti DATABASE_URL di .env).
pub struct DatabasePool {
    pub pool: Pool<Sqlite>,
}

impl DatabasePool {
    /// Buat koneksi pool baru dari `database_url`.
    ///
    /// Contoh: `"sqlite:./lumina.db"` atau `"postgres://user:pass@localhost/lumina"`
    pub async fn connect(database_url: &str) -> Result<Self, sqlx::Error> {
        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect(database_url)
            .await?;

        println!("📦 Database connected: {}", database_url);
        Ok(Self { pool })
    }
}
