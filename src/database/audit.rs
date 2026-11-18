use super::connection::{DatabaseKind, DatabasePool};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Representasi baris log audit pada tabel `audit_logs`
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow, PartialEq)]
pub struct AuditLog {
    pub id: i64,
    pub auditable_type: String,
    pub auditable_id: i64,
    pub action: String,
    pub user_id: Option<i64>,
    pub old_values: Option<String>,
    pub new_values: Option<String>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub created_at: Option<String>,
}

/// Facade & Builder untuk Audit Trail Logging Lumina Framework
pub struct Audit;

impl Audit {
    /// Inisialisasi builder pencatatan log audit baru
    pub fn record(pool: &DatabasePool) -> AuditRecordBuilder<'_> {
        AuditRecordBuilder::new(pool)
    }

    /// Ambil seluruh riwayat mutasi dari suatu entitas tertentu
    pub async fn history(
        pool: &DatabasePool,
        auditable_type: &str,
        auditable_id: i64,
    ) -> Result<Vec<AuditLog>, sqlx::Error> {
        let is_pg = pool.kind == DatabaseKind::Postgres;
        let created_col = if is_pg {
            "to_char(created_at, 'YYYY-MM-DD HH24:MI:SS') as created_at"
        } else {
            "CAST(created_at AS TEXT) as created_at"
        };

        let sql = format!(
            "SELECT id, auditable_type, auditable_id, action, user_id, old_values, new_values, ip_address, user_agent, {} \
             FROM audit_logs \
             WHERE auditable_type = ? AND auditable_id = ? \
             ORDER BY id DESC",
            created_col
        );

        let sql = pool.prepare_sql(&sql);
        sqlx::query_as::<_, AuditLog>(&sql)
            .bind(auditable_type)
            .bind(auditable_id)
            .fetch_all(&pool.pool)
            .await
    }

    /// Ambil daftar log mutasi audit terbaru secara global (berguna untuk Admin / Dashboard)
    pub async fn latest(pool: &DatabasePool, limit: i64) -> Result<Vec<AuditLog>, sqlx::Error> {
        let is_pg = pool.kind == DatabaseKind::Postgres;
        let created_col = if is_pg {
            "to_char(created_at, 'YYYY-MM-DD HH24:MI:SS') as created_at"
        } else {
            "CAST(created_at AS TEXT) as created_at"
        };

        let sql = format!(
            "SELECT id, auditable_type, auditable_id, action, user_id, old_values, new_values, ip_address, user_agent, {} \
             FROM audit_logs \
             ORDER BY id DESC \
             LIMIT ?",
            created_col
        );

        let sql = pool.prepare_sql(&sql);
        sqlx::query_as::<_, AuditLog>(&sql)
            .bind(limit)
            .fetch_all(&pool.pool)
            .await
    }

    /// Buat tabel `audit_logs` secara otomatis jika belum ada di database
    pub async fn ensure_table(pool: &DatabasePool) -> Result<(), sqlx::Error> {
        let sql = match pool.kind {
            DatabaseKind::Postgres => {
                r#"
                CREATE TABLE IF NOT EXISTS audit_logs (
                    id BIGSERIAL PRIMARY KEY,
                    auditable_type VARCHAR(255) NOT NULL,
                    auditable_id BIGINT NOT NULL,
                    action VARCHAR(50) NOT NULL,
                    user_id BIGINT,
                    old_values TEXT,
                    new_values TEXT,
                    ip_address VARCHAR(45),
                    user_agent TEXT,
                    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
                );
                CREATE INDEX IF NOT EXISTS idx_audit_logs_auditable ON audit_logs(auditable_type, auditable_id);
                "#
            }
            DatabaseKind::MySql => {
                r#"
                CREATE TABLE IF NOT EXISTS audit_logs (
                    id BIGINT AUTO_INCREMENT PRIMARY KEY,
                    auditable_type VARCHAR(255) NOT NULL,
                    auditable_id BIGINT NOT NULL,
                    action VARCHAR(50) NOT NULL,
                    user_id BIGINT,
                    old_values TEXT,
                    new_values TEXT,
                    ip_address VARCHAR(45),
                    user_agent TEXT,
                    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                    INDEX idx_audit_logs_auditable (auditable_type, auditable_id)
                ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;
                "#
            }
            _ => {
                r#"
                CREATE TABLE IF NOT EXISTS audit_logs (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    auditable_type VARCHAR(255) NOT NULL,
                    auditable_id BIGINT NOT NULL,
                    action VARCHAR(50) NOT NULL,
                    user_id BIGINT,
                    old_values TEXT,
                    new_values TEXT,
                    ip_address VARCHAR(45),
                    user_agent TEXT,
                    created_at TEXT DEFAULT CURRENT_TIMESTAMP
                );
                CREATE INDEX IF NOT EXISTS idx_audit_logs_auditable ON audit_logs(auditable_type, auditable_id);
                "#
            }
        };

        sqlx::query(sql).execute(&pool.pool).await?;
        Ok(())
    }
}

/// Fluent Builder untuk mencatat satu aktivitas audit mutasi
pub struct AuditRecordBuilder<'a> {
    pool: &'a DatabasePool,
    auditable_type: String,
    auditable_id: i64,
    action: String,
    user_id: Option<i64>,
    old_values: Option<String>,
    new_values: Option<String>,
    ip_address: Option<String>,
    user_agent: Option<String>,
}

impl<'a> AuditRecordBuilder<'a> {
    pub fn new(pool: &'a DatabasePool) -> Self {
        Self {
            pool,
            auditable_type: String::new(),
            auditable_id: 0,
            action: "UPDATE".to_string(),
            user_id: None,
            old_values: None,
            new_values: None,
            ip_address: None,
            user_agent: None,
        }
    }

    /// Tentukan tipe entitas (nama model/tabel) dan ID entitas
    pub fn on(mut self, auditable_type: &str, auditable_id: i64) -> Self {
        self.auditable_type = auditable_type.to_string();
        self.auditable_id = auditable_id;
        self
    }

    /// Tentukan aksi mutasi: CREATE, UPDATE, DELETE, RESTORE
    pub fn action(mut self, action: &str) -> Self {
        self.action = action.to_uppercase();
        self
    }

    /// Tentukan ID operator / pengguna yang melakukan aksi
    pub fn by_user(mut self, user_id: Option<i64>) -> Self {
        self.user_id = user_id;
        self
    }

    /// Simpan snapshot nilai sebelum perubahan
    pub fn old_values(mut self, old: Value) -> Self {
        self.old_values = Some(old.to_string());
        self
    }

    /// Simpan snapshot nilai setelah perubahan
    pub fn new_values(mut self, new: Value) -> Self {
        self.new_values = Some(new.to_string());
        self
    }

    /// Shortcut mencatat perbedaan (diff) antara nilai lama dan nilai baru
    pub fn diff(mut self, old: Value, new: Value) -> Self {
        self.old_values = Some(old.to_string());
        self.new_values = Some(new.to_string());
        self
    }

    /// Simpan alamat IP pemanggil
    pub fn ip(mut self, ip: Option<&str>) -> Self {
        self.ip_address = ip.map(|s| s.to_string());
        self
    }

    /// Simpan User Agent pemanggil
    pub fn user_agent(mut self, ua: Option<&str>) -> Self {
        self.user_agent = ua.map(|s| s.to_string());
        self
    }

    /// Simpan rekaman audit secara asinkron ke tabel `audit_logs`
    pub async fn save(self) -> Result<i64, sqlx::Error> {
        // Pastikan tabel sudah ada
        Audit::ensure_table(self.pool).await?;

        let is_pg = self.pool.kind == DatabaseKind::Postgres;
        if is_pg {
            let sql = "INSERT INTO audit_logs (auditable_type, auditable_id, action, user_id, old_values, new_values, ip_address, user_agent) \
                       VALUES (?, ?, ?, ?, ?, ?, ?, ?) RETURNING id";
            let sql = self.pool.prepare_sql(sql);
            let row: (i64,) = sqlx::query_as(&sql)
                .bind(&self.auditable_type)
                .bind(self.auditable_id)
                .bind(&self.action)
                .bind(self.user_id)
                .bind(&self.old_values)
                .bind(&self.new_values)
                .bind(&self.ip_address)
                .bind(&self.user_agent)
                .fetch_one(&self.pool.pool)
                .await?;
            Ok(row.0)
        } else {
            let sql = "INSERT INTO audit_logs (auditable_type, auditable_id, action, user_id, old_values, new_values, ip_address, user_agent) \
                       VALUES (?, ?, ?, ?, ?, ?, ?, ?)";
            let sql = self.pool.prepare_sql(sql);
            let res = sqlx::query(&sql)
                .bind(&self.auditable_type)
                .bind(self.auditable_id)
                .bind(&self.action)
                .bind(self.user_id)
                .bind(&self.old_values)
                .bind(&self.new_values)
                .bind(&self.ip_address)
                .bind(&self.user_agent)
                .execute(&self.pool.pool)
                .await?;

            let inserted_id = res.last_insert_id().unwrap_or(0);
            if inserted_id > 0 {
                Ok(inserted_id)
            } else {
                let row: (i64,) = sqlx::query_as("SELECT COALESCE(MAX(id), 0) FROM audit_logs")
                    .fetch_one(&self.pool.pool)
                    .await
                    .unwrap_or((0,));
                Ok(row.0)
            }
        }
    }
}
