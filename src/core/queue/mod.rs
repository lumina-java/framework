<<<<<<< HEAD
use crate::core::application::AppState;
use async_trait::async_trait;
use serde_json::Value;
use sqlx::{Any, Pool};
use std::collections::HashMap;
use std::sync::Arc;
=======
use async_trait::async_trait;
use std::sync::Arc;
use crate::core::application::AppState;
use sqlx::{Pool, Any};
use serde_json::Value;
use std::collections::HashMap;
>>>>>>> dev/master
use tokio::time::{sleep, Duration};

/// Trait yang harus diimplementasikan oleh setiap Background Job.
#[async_trait]
pub trait Job: Send + Sync {
    /// Nama unik job (digunakan untuk identifikasi di database).
    fn name(&self) -> &'static str;
<<<<<<< HEAD

=======

>>>>>>> dev/master
    /// Jalankan logika job.
    async fn handle(&self, state: Arc<AppState>, payload: Value) -> Result<(), String>;
}

/// Registry untuk mendaftarkan semua job yang tersedia.
pub struct JobRegistry {
    jobs: HashMap<String, Box<dyn Job>>,
}

impl JobRegistry {
    pub fn new() -> Self {
<<<<<<< HEAD
        Self {
            jobs: HashMap::new(),
        }
=======
        Self { jobs: HashMap::new() }
>>>>>>> dev/master
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
}

impl QueueManager {
    pub fn new(db: Pool<Any>) -> Self {
        Self { db }
    }

    /// Mengirim job ke antrean database.
    pub async fn dispatch(&self, name: &str, payload: Value) -> Result<(), String> {
        let full_payload = serde_json::json!({
            "job": name,
            "data": payload
        });

        sqlx::query("INSERT INTO jobs (queue, payload) VALUES ('default', ?)")
            .bind(full_payload.to_string())
            .execute(&self.db)
            .await
            .map_err(|e| format!("Gagal simpan job ke DB: {}", e))?;
<<<<<<< HEAD

=======

>>>>>>> dev/master
        Ok(())
    }
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
<<<<<<< HEAD

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

=======

        // Cek apakah tabel `jobs` ada sebelum mulai polling.
        // Jika belum ada, log warning SEKALI dan hentikan worker.
        let db = &state.db().pool;
        loop {
            let table_exists = sqlx::query("SELECT 1 FROM jobs LIMIT 1")
                .execute(db)
                .await;

            if let Err(e) = table_exists {
                let err_str = e.to_string();
                if err_str.contains("doesn't exist") || err_str.contains("no such table") {
                    tracing::warn!(
                        "⚠️  Tabel 'jobs' belum ada. Menunggu migrasi... (retry dalam 30 detik)"
                    );
                    sleep(Duration::from_secs(30)).await;
                } else {
                    // Jika error lain (misal koneksi terputus), tetap tunggu dan coba lagi
                    tracing::error!("❌ Error saat mengecek tabel jobs: {}. Retry dalam 30 detik...", e);
                    sleep(Duration::from_secs(30)).await;
                }
            } else {
                break; // Tabel sudah ada, lanjut ke polling loop
            }
        }

>>>>>>> dev/master
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
        let db = &state.db().pool;
<<<<<<< HEAD

=======

>>>>>>> dev/master
        // 1. Ambil job yang tersedia
        let row: Option<(i64, String)> = sqlx::query_as::<_, (i64, String)>(
            "SELECT id, payload FROM jobs 
             WHERE reserved_at IS NULL 
             AND available_at <= CURRENT_TIMESTAMP 
<<<<<<< HEAD
             ORDER BY available_at ASC LIMIT 1",
=======
             ORDER BY available_at ASC LIMIT 1"
>>>>>>> dev/master
        )
        .fetch_optional(db)
        .await
        .map_err(|e| e.to_string())?;

        let (id, payload_str) = match row {
            Some(r) => r,
            None => return Ok(false),
        };

        // 2. Tandai sebagai reserved
<<<<<<< HEAD
        sqlx::query(
            "UPDATE jobs SET reserved_at = CURRENT_TIMESTAMP, attempts = attempts + 1 WHERE id = ?",
        )
        .bind(id)
        .execute(db)
        .await
        .map_err(|e| e.to_string())?;
=======
        sqlx::query("UPDATE jobs SET reserved_at = CURRENT_TIMESTAMP, attempts = attempts + 1 WHERE id = ?")
            .bind(id)
            .execute(db)
            .await
            .map_err(|e| e.to_string())?;
>>>>>>> dev/master

        // 3. Parse payload
        let payload: Value = serde_json::from_str(&payload_str).map_err(|e| e.to_string())?;
        let job_name = payload["job"].as_str().unwrap_or("unknown");
        let job_data = payload["data"].clone();

        tracing::info!("📦 Processing job [{}] ID: {}", job_name, id);

        // 4. Cari handler
        if let Some(handler) = self.registry.get(job_name) {
            match handler.handle(state.clone(), job_data).await {
                Ok(_) => {
<<<<<<< HEAD
                    sqlx::query("DELETE FROM jobs WHERE id = ?")
                        .bind(id)
                        .execute(db)
                        .await
                        .ok();
                    tracing::info!("✅ Job [{}] ID: {} selesai.", job_name, id);
                }
                Err(e) => {
                    sqlx::query("UPDATE jobs SET reserved_at = NULL WHERE id = ?")
                        .bind(id)
                        .execute(db)
                        .await
                        .ok();
=======
                    sqlx::query("DELETE FROM jobs WHERE id = ?").bind(id).execute(db).await.ok();
                    tracing::info!("✅ Job [{}] ID: {} selesai.", job_name, id);
                }
                Err(e) => {
                    sqlx::query("UPDATE jobs SET reserved_at = NULL WHERE id = ?").bind(id).execute(db).await.ok();
>>>>>>> dev/master
                    tracing::error!("❌ Job [{}] ID: {} gagal: {}", job_name, id, e);
                }
            }
        } else {
            tracing::error!("⚠️  Tidak ada handler untuk job: {}", job_name);
<<<<<<< HEAD
            sqlx::query("DELETE FROM jobs WHERE id = ?")
                .bind(id)
                .execute(db)
                .await
                .ok();
=======
            sqlx::query("DELETE FROM jobs WHERE id = ?").bind(id).execute(db).await.ok();
>>>>>>> dev/master
        }

        Ok(true)
    }
}
