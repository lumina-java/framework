use sqlx::{any::AnyPoolOptions, Any, Pool};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DatabaseKind {
    Sqlite,
    Postgres,
    MySql,
    Mssql,
}

/// Lumina DatabasePool — wrapper di atas `sqlx::Pool<Any>` dan driver native enterprise (termasuk MSSQL Tiberius).
#[derive(Clone)]
pub struct DatabasePool {
    pub pool: Pool<Any>,
    pub kind: DatabaseKind,
    #[cfg(feature = "postgres")]
    pub pg_pool: Option<sqlx::PgPool>,
    #[cfg(feature = "sqlite")]
    pub sqlite_pool: Option<sqlx::SqlitePool>,
    #[cfg(feature = "mysql")]
    pub mysql_pool: Option<sqlx::MySqlPool>,
    #[cfg(feature = "mssql")]
    pub mssql_pool: Option<bb8::Pool<bb8_tiberius::ConnectionManager>>,
}

/// Alias ringkas & manusiawi untuk DatabasePool
pub type Db = DatabasePool;

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
        } else if database_url.starts_with("mssql:")
            || database_url.starts_with("sqlserver:")
            || database_url.starts_with("jdbc:sqlserver:")
            || database_url.to_ascii_lowercase().starts_with("server=")
        {
            DatabaseKind::Mssql
        } else {
            // Default atau fallback (misal SQLite)
            DatabaseKind::Sqlite
        };

        // Inisialisasi pool sqlx::Any (SQLite, Postgres, MySQL)
        let pool = if kind == DatabaseKind::Mssql {
            // sqlx::any tidak menyediakan driver internal untuk MSSQL di v0.8+.
            // Sediakan in-memory SQLite fallback agar field `pool` tetap valid dan aman.
            AnyPoolOptions::new()
                .max_connections(1)
                .connect("sqlite::memory:")
                .await
                .unwrap_or_else(|_| AnyPoolOptions::new().connect_lazy("sqlite::memory:").unwrap())
        } else {
            AnyPoolOptions::new()
                .max_connections(5)
                .connect(database_url)
                .await?
        };

        #[cfg(feature = "postgres")]
        let pg_pool = if kind == DatabaseKind::Postgres {
            sqlx::PgPool::connect(database_url).await.ok()
        } else {
            None
        };

        #[cfg(feature = "sqlite")]
        let sqlite_pool = if kind == DatabaseKind::Sqlite {
            sqlx::SqlitePool::connect(database_url).await.ok()
        } else {
            None
        };

        #[cfg(feature = "mysql")]
        let mysql_pool = if kind == DatabaseKind::MySql {
            sqlx::MySqlPool::connect(database_url).await.ok()
        } else {
            None
        };

        #[cfg(feature = "mssql")]
        let mssql_pool = if kind == DatabaseKind::Mssql {
            match Self::connect_mssql(database_url).await {
                Ok(p) => {
                    println!("  🚀 MSSQL Connection Pool (Tiberius TDS) connected successfully!");
                    Some(p)
                }
                Err(e) => {
                    eprintln!("  ⚠️ Gagal menghubungkan MSSQL Pool: {}", e);
                    None
                }
            }
        } else {
            None
        };

        println!("📦 Database connected: {:?}", kind);
        Ok(Self {
            pool,
            kind,
            #[cfg(feature = "postgres")]
            pg_pool,
            #[cfg(feature = "sqlite")]
            sqlite_pool,
            #[cfg(feature = "mysql")]
            mysql_pool,
            #[cfg(feature = "mssql")]
            mssql_pool,
        })
    }

    /// Mulai transaksi database baru (ACID Transaction) yang aman, otomatis rollback jika gagal.
    pub async fn begin(&self) -> Result<crate::database::transaction::Transaction, sqlx::Error> {
        self.begin_transaction().await
    }

    /// Mulai transaksi database baru (ACID Transaction) dengan gaya imperatif tanpa closure.
    pub async fn begin_transaction(&self) -> Result<crate::database::transaction::Transaction, sqlx::Error> {
        #[cfg(feature = "mssql")]
        if self.kind == DatabaseKind::Mssql {
            if let Some(pool) = &self.mssql_pool {
                let mut conn = pool.get_owned().await.map_err(|e| {
                    sqlx::Error::Configuration(format!("Gagal mengambil koneksi MSSQL untuk transaksi: {}", e).into())
                })?;
                conn.simple_query("BEGIN TRANSACTION").await.map_err(|e| {
                    sqlx::Error::Configuration(format!("Gagal memulai transaksi MSSQL: {}", e).into())
                })?;
                return Ok(crate::database::transaction::Transaction::from_mssql(conn));
            } else {
                return Err(sqlx::Error::Configuration("MSSQL pool belum siap".into()));
            }
        }

        let tx = self.pool.begin().await?;
        Ok(crate::database::transaction::Transaction::from_sqlx(self.kind, tx))
    }

    /// Eksekusi SQL raw langsung pada database pool
    pub async fn execute_raw(&self, sql: &str) -> Result<u64, sqlx::Error> {
        let res = sqlx::query(sql).execute(&self.pool).await?;
        Ok(res.rows_affected())
    }

    /// Akses direct native PgPool (hanya tersedia jika fitur `postgres` aktif)
    #[cfg(feature = "postgres")]
    pub fn pg(&self) -> Option<&sqlx::PgPool> {
        self.pg_pool.as_ref()
    }

    /// Akses direct native SqlitePool (hanya tersedia jika fitur `sqlite` aktif)
    #[cfg(feature = "sqlite")]
    pub fn sqlite(&self) -> Option<&sqlx::SqlitePool> {
        self.sqlite_pool.as_ref()
    }

    /// Akses direct native MySqlPool (hanya tersedia jika fitur `mysql` aktif)
    #[cfg(feature = "mysql")]
    pub fn mysql(&self) -> Option<&sqlx::MySqlPool> {
        self.mysql_pool.as_ref()
    }

    /// Akses direct native MSSQL Pool (hanya tersedia jika fitur `mssql` aktif)
    #[cfg(feature = "mssql")]
    pub fn mssql(&self) -> Option<&bb8::Pool<bb8_tiberius::ConnectionManager>> {
        self.mssql_pool.as_ref()
    }

    /// Eksekusi raw query sederhana pada MSSQL
    #[cfg(feature = "mssql")]
    pub async fn query_mssql_simple(&self, sql: &str) -> Result<Vec<tiberius::Row>, Box<dyn std::error::Error + Send + Sync>> {
        if let Some(pool) = &self.mssql_pool {
            let mut conn = pool.get().await?;
            let stream = conn.simple_query(sql).await?;
            let rows = stream.into_first_result().await?;
            Ok(rows)
        } else {
            Err("MSSQL Pool belum diinisialisasi atau fitur mssql tidak aktif".into())
        }
    }

    /// Helper internal untuk inisialisasi koneksi MSSQL menggunakan Tiberius & bb8
    #[cfg(feature = "mssql")]
    pub async fn connect_mssql(database_url: &str) -> Result<bb8::Pool<bb8_tiberius::ConnectionManager>, Box<dyn std::error::Error + Send + Sync>> {
        let config = if database_url.starts_with("sqlserver://")
            || database_url.starts_with("mssql://")
            || database_url.starts_with("jdbc:sqlserver://")
        {
            let jdbc_str = if let Some(stripped) = database_url.strip_prefix("mssql://") {
                format!("jdbc:sqlserver://{}", stripped)
            } else if let Some(stripped) = database_url.strip_prefix("sqlserver://") {
                format!("jdbc:sqlserver://{}", stripped)
            } else {
                database_url.to_string()
            };
            tiberius::Config::from_jdbc_string(&jdbc_str)
                .or_else(|_| tiberius::Config::from_ado_string(database_url))?
        } else {
            tiberius::Config::from_ado_string(database_url)?
        };

        let mgr = bb8_tiberius::ConnectionManager::new(config);
        let pool = bb8::Pool::builder()
            .max_size(10)
            .build(mgr)
            .await?;
        Ok(pool)
    }

    /// Helper untuk mendapatkan referensi ke pool asli
    pub fn get_pool(&self) -> &Pool<Any> {
        &self.pool
    }

    /// Menyesuaikan placeholder query secara otomatis berdasarkan driver database aktif.
    /// SQLite/MySQL: tetap menggunakan `?`.
    /// PostgreSQL: mentranslasikan `?` menjadi `$1, $2, $3, ...`.
    /// MSSQL: mentranslasikan `?` menjadi `@p1, @p2, @p3, ...`.
    pub fn prepare_sql(&self, sql: &str) -> String {
        match self.kind {
            DatabaseKind::Postgres => {
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
            }
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
