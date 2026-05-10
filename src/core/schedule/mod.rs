use std::sync::Arc;
use tokio_cron_scheduler::{Job, JobScheduler};
use crate::core::application::AppState;

/// Lumina Scheduler — wrapper untuk tokio-cron-scheduler.
/// Memungkinkan penjadwalan tugas (cron jobs) langsung dari aplikasi.
pub struct Scheduler {
    sched: JobScheduler,
    state: Arc<AppState>,
}

impl Scheduler {
    /// Buat instance scheduler baru.
    pub async fn new(state: Arc<AppState>) -> Self {
        let sched = JobScheduler::new().await.expect("Gagal menginisialisasi Scheduler");
        Self { sched, state }
    }

    /// Jadwalkan tugas menggunakan ekspresi cron.
    /// Contoh: `* * * * * *` (setiap detik)
    pub async fn call<F, Fut>(&self, cron_expr: &str, task_fn: F) -> Result<(), String>
    where
        F: Fn() -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = ()> + Send + 'static,
    {
        let job = Job::new_async(cron_expr, move |_uuid, _l| {
            let fut = task_fn();
            Box::pin(async move {
                fut.await;
            })
        }).map_err(|e| format!("Cron error: {}", e))?;

        self.sched.add(job).await.map_err(|e| format!("Scheduler error: {}", e))?;
        Ok(())
    }

    /// Mulai menjalankan scheduler di background.
    pub async fn run(&self) {
        if let Err(e) = self.sched.start().await {
            eprintln!("❌ Gagal menjalankan Scheduler: {}", e);
        } else {
            println!("⏰ Task Scheduler started.");
        }
    }

    /// Akses ke AppState dari dalam job.
    pub fn state(&self) -> Arc<AppState> {
        self.state.clone()
    }
}
