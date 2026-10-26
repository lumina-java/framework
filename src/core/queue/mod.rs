use crate::core::application::AppState;
use async_trait::async_trait;
use serde_json::Value;
use sqlx::{Any, Pool};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::time::{sleep, Duration};

/// Trait yang harus diimplementasikan oleh setiap Background Job.
#[async_trait]
pub trait Job: Send + Sync {
    /// Nama unik job (digunakan untuk identifikasi di database).
    fn name(&self) -> &'static str;

    /// Jalankan logika job.
    async fn handle(&self, state: Arc<AppState>, payload: Value) -> Result<(), String>;
}

/// Trait yang memungkinkan pemanggilan job secara fluen: `MyJob { ... }.dispatch(&req).await?`
#[async_trait]
pub trait Dispatchable: serde::Serialize + Send + Sync {
    /// Nama pengenal job untuk pendaftaran di JobRegistry
    fn job_name() -> &'static str;

    /// Kirim job ke antrean latar belakang via Request bundle
    async fn dispatch(&self, req: &crate::core::request::Request) -> Result<(), String> {
        let payload = serde_json::to_value(self).map_err(|e| e.to_string())?;
        req.queue().dispatch(Self::job_name(), payload).await
    }

    /// Kirim job ke antrean latar belakang menggunakan QueueManager langsung
    async fn dispatch_queue(&self, queue: &QueueManager) -> Result<(), String> {
        let payload = serde_json::to_value(self).map_err(|e| e.to_string())?;
        queue.dispatch(Self::job_name(), payload).await
    }
}

/// Registry untuk mendaftarkan semua job yang tersedia.
pub struct JobRegistry {
    jobs: HashMap<String, Box<dyn Job>>,
}

impl JobRegistry {
    pub fn new() -> Self {
        Self {
            jobs: HashMap::new(),
        }
    }

    pub fn register<J: Job + 'static>(&mut self, job: J) {
        self.jobs.insert(job.name().to_string(), Box::new(job));
    }

    pub fn get(&self, name: &str) -> Option<&Box<dyn Job>> {
        self.jobs.get(name)
    }
}

/// QueueManager mengelola pengiriman job ke database.
pub struct QueueManager {
    db: Pool<Any>,
    pool_ref: Option<Arc<crate::database::connection::DatabasePool>>,
}

impl QueueManager {
    pub fn new(db: Pool<Any>) -> Self {
        Self {
            db,
            pool_ref: None,
        }
    }

    pub fn new_with_pool(pool: Arc<crate::database::connection::DatabasePool>) -> Self {
        Self {
            db: pool.pool.clone(),
            pool_ref: Some(pool),
        }
    }

    /// Mengirim job ke antrean database dengan multi-db parameter translation ($1 vs ?).
    pub async fn dispatch(&self, name: &str, payload: Value) -> Result<(), String> {
        let full_payload = serde_json::json!({
            "job": name,
            "data": payload
        });

        let raw_sql = "INSERT INTO jobs (queue, payload) VALUES ('default', ?)";
        let sql = if let Some(ref p) = self.pool_ref {
            p.prepare_sql(raw_sql)
        } else {
            raw_sql.to_string()
        };

        sqlx::query(&sql)
            .bind(full_payload.to_string())
            .execute(&self.db)
            .await
            .map_err(|e| format!("Gagal simpan job ke DB: {}", e))?;

        Ok(())
    }
}

/// Memastikan tabel antrean jobs tersedia di database aktif
pub async fn ensure_jobs_table(pool: &crate::database::connection::DatabasePool) -> Result<(), sqlx::Error> {
    let ddl = match pool.kind {
        crate::database::connection::DatabaseKind::Postgres => {
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
        crate::database::connection::DatabaseKind::MySql => {
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

    sqlx::query(ddl).execute(&pool.pool).await?;
    Ok(())
}

/// Worker yang memantau database dan mengeksekusi job.
pub struct QueueWorker {
    registry: Arc<JobRegistry>,
}

impl QueueWorker {
    pub fn new(registry: Arc<JobRegistry>) -> Self {
        Self { registry }
    }

    pub async fn run(self, state: Arc<AppState>) {
        tracing::info!("🚀 Persistent Queue Worker started...");

        // Cek apakah tabel `jobs` ada sebelum mulai polling.
        // Jika belum ada, log warning SEKALI dan hentikan worker.
        let db = &state.db().pool;
        let table_exists = sqlx::query("SELECT 1 FROM jobs LIMIT 1").execute(db).await;

        if let Err(e) = table_exists {
            let err_str = e.to_string();
            if err_str.contains("doesn't exist") || err_str.contains("no such table") {
                tracing::warn!(
                    "⚠️  Tabel 'jobs' belum ada. Queue worker dinonaktifkan. \
                     Jalankan: ./lumina migrate untuk membuat tabel."
                );
                return; // Hentikan worker, tidak perlu loop
            }
        }

        loop {
            match self.process_next_job(state.clone()).await {
                Ok(true) => { /* Ada job yang diproses, lanjut cek lagi */ }
                Ok(false) => {
                    sleep(Duration::from_secs(3)).await;
                }
                Err(e) => {
                    tracing::error!("❌ Error saat polling queue: {}", e);
                    sleep(Duration::from_secs(5)).await;
                }
            }
        }
    }

    async fn process_next_job(&self, state: Arc<AppState>) -> Result<bool, String> {
        let pool = state.db();
        let db = &pool.pool;

        // 1. Ambil job yang tersedia
        let fetch_sql = pool.prepare_sql(
            "SELECT id, payload FROM jobs 
             WHERE reserved_at IS NULL 
             AND available_at <= CURRENT_TIMESTAMP 
             ORDER BY available_at ASC LIMIT 1"
        );
        let row: Option<(i64, String)> = sqlx::query_as::<_, (i64, String)>(&fetch_sql)
            .fetch_optional(db)
            .await
            .map_err(|e| e.to_string())?;

        let (id, payload_str) = match row {
            Some(r) => r,
            None => return Ok(false),
        };

        // 2. Tandai sebagai reserved
        let reserve_sql = pool.prepare_sql(
            "UPDATE jobs SET reserved_at = CURRENT_TIMESTAMP, attempts = attempts + 1 WHERE id = ?"
        );
        sqlx::query(&reserve_sql)
            .bind(id)
            .execute(db)
            .await
            .map_err(|e| e.to_string())?;

        // 3. Parse payload
        let payload: Value = serde_json::from_str(&payload_str).map_err(|e| e.to_string())?;
        let job_name = payload["job"].as_str().unwrap_or("unknown");
        let job_data = payload["data"].clone();

        tracing::info!("📦 Processing job [{}] ID: {}", job_name, id);

        // 4. Cari handler
        let delete_sql = pool.prepare_sql("DELETE FROM jobs WHERE id = ?");
        let reset_sql = pool.prepare_sql("UPDATE jobs SET reserved_at = NULL WHERE id = ?");

        if let Some(handler) = self.registry.get(job_name) {
            match handler.handle(state.clone(), job_data).await {
                Ok(_) => {
                    sqlx::query(&delete_sql)
                        .bind(id)
                        .execute(db)
                        .await
                        .ok();
                    tracing::info!("✅ Job [{}] ID: {} selesai.", job_name, id);
                }
                Err(e) => {
                    sqlx::query(&reset_sql)
                        .bind(id)
                        .execute(db)
                        .await
                        .ok();
                    tracing::error!("❌ Job [{}] ID: {} gagal: {}", job_name, id, e);
                }
            }
        } else {
            tracing::error!("⚠️  Tidak ada handler untuk job: {}", job_name);
            sqlx::query(&delete_sql)
                .bind(id)
                .execute(db)
                .await
                .ok();
        }

        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::database::connection::DatabasePool;
    use serde::{Deserialize, Serialize};
    use std::sync::atomic::{AtomicBool, Ordering};

    #[derive(Serialize, Deserialize, Clone)]
    struct DummyEmailJob {
        pub to: String,
    }

    static JOB_EXECUTED: AtomicBool = AtomicBool::new(false);

    #[async_trait]
    impl Job for DummyEmailJob {
        fn name(&self) -> &'static str {
            "DummyEmailJob"
        }

        async fn handle(&self, _state: Arc<AppState>, payload: Value) -> Result<(), String> {
            let email = payload["data"]["to"].as_str()
                .or_else(|| payload["to"].as_str())
                .unwrap_or_default();
            if email == "test@lumina.dev" {
                JOB_EXECUTED.store(true, Ordering::SeqCst);
                Ok(())
            } else {
                Err("Invalid email".to_string())
            }
        }
    }

    impl Dispatchable for DummyEmailJob {
        fn job_name() -> &'static str {
            "DummyEmailJob"
        }
    }

    #[tokio::test]
    async fn test_queue_dispatch_and_worker_processing() {
        let db_url = format!("sqlite:file:queue_mem_{}?mode=memory&cache=shared", uuid::Uuid::new_v4());
        let pool = Arc::new(
            DatabasePool::connect(&db_url)
                .await
                .expect("Failed to connect SQLite in-memory"),
        );

        // 1. Ensure jobs table exists
        ensure_jobs_table(&pool).await.expect("Failed to create jobs table");

        // 2. QueueManager
        let queue = QueueManager::new_with_pool(pool.clone());

        // 3. Dispatch Job via Dispatchable trait
        let job = DummyEmailJob {
            to: "test@lumina.dev".to_string(),
        };
        job.dispatch_queue(&queue).await.expect("Failed to dispatch job");

        // 4. Verify job in DB
        let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM jobs")
            .fetch_one(&pool.pool)
            .await
            .expect("Failed to query jobs table");
        assert_eq!(count.0, 1, "Job should be in database queue");

        // 5. Worker processing
        let mut registry = JobRegistry::new();
        registry.register(DummyEmailJob {
            to: "".to_string(),
        });
        let registry = Arc::new(registry);
        let worker = QueueWorker::new(registry.clone());

        let config = Arc::new(crate::core::config::ConfigManager::new());
        let lang = Arc::new(crate::core::i18n::LangManager::new("en".to_string()));
        let view = crate::core::view::ViewEngine::new(Some(lang.clone()));
        let cache = Arc::new(crate::core::cache::CacheManager::new());
        let events = Arc::new(crate::core::event::EventDispatcher::new());
        let telescope = Arc::new(crate::core::telescope::TelescopeManager::new());
        let echo = Arc::new(crate::core::echo::EchoManager::new());
        let storage = Arc::new(crate::core::storage::Storage::new("local"));
        let mail = Arc::new(crate::core::mail::Mail::new(
            Arc::new(crate::core::mail::LogDriver),
            view.clone(),
        ));
        let socialite = Arc::new(crate::core::auth::socialite::SocialiteManager::new());

        let state = Arc::new(AppState {
            db: Some(pool.clone()),
            view,
            db_error: None,
            csrf_config: crate::core::security::csrf::config(),
            storage,
            config,
            queue: Arc::new(queue),
            cache,
            mail,
            registry,
            events,
            telescope,
            lang,
            echo,
            socialite,
            sandbox: Arc::new(crate::core::sandbox::LuminaPluginManager::new()),
        });

        let processed = worker.process_next_job(state).await.expect("Worker error");
        assert!(processed, "Job should have been processed");
        assert!(JOB_EXECUTED.load(Ordering::SeqCst), "Job handler should have run");

        // 6. Verify job removed from DB
        let remaining: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM jobs")
            .fetch_one(&pool.pool)
            .await
            .expect("Failed to query remaining jobs");
        assert_eq!(remaining.0, 0, "Processed job should be deleted from queue");
    }
}

