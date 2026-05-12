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

    /// Jadwalkan tugas menggunakan ekspresi cron secara langsung.
    pub async fn cron<F, Fut>(&self, cron_expr: &str, task_fn: F) -> Result<(), String>
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

    /// Entry point untuk membuat tugas dengan API yang fluent.
    pub fn call<F, Fut>(&self, task_fn: F) -> ScheduledTask<'_, F, Fut> 
    where
        F: Fn() -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = ()> + Send + 'static,
    {
        ScheduledTask {
            scheduler: self,
            task_fn,
            _phantom: std::marker::PhantomData,
        }
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

/// Helper untuk API penjadwalan yang lebih manusiawi.
pub struct ScheduledTask<'a, F, Fut> {
    scheduler: &'a Scheduler,
    task_fn: F,
    _phantom: std::marker::PhantomData<Fut>,
}

impl<'a, F, Fut> ScheduledTask<'a, F, Fut>
where
    F: Fn() -> Fut + Send + Sync + 'static,
    Fut: std::future::Future<Output = ()> + Send + 'static,
{
    pub async fn every_second(self) -> Result<(), String> {
        self.scheduler.cron("* * * * * *", self.task_fn).await
    }

    pub async fn every_minute(self) -> Result<(), String> {
        self.scheduler.cron("0 * * * * *", self.task_fn).await
    }

    pub async fn every_five_minutes(self) -> Result<(), String> {
        self.scheduler.cron("0 */5 * * * *", self.task_fn).await
    }

    pub async fn hourly(self) -> Result<(), String> {
        self.scheduler.cron("0 0 * * * *", self.task_fn).await
    }

    pub async fn daily(self) -> Result<(), String> {
        self.scheduler.cron("0 0 0 * * *", self.task_fn).await
    }

    pub async fn daily_at(self, time: &str) -> Result<(), String> {
        // time format: "HH:MM"
        let parts: Vec<&str> = time.split(':').collect();
        if parts.len() != 2 {
            return Err("Format waktu harus HH:MM".to_string());
        }
        let cron = format!("0 {} {} * * *", parts[1], parts[0]);
        self.scheduler.cron(&cron, self.task_fn).await
    }

    pub async fn weekly(self) -> Result<(), String> {
        self.scheduler.cron("0 0 0 * * 0", self.task_fn).await
    }

    pub async fn monthly(self) -> Result<(), String> {
        self.scheduler.cron("0 0 0 1 * *", self.task_fn).await
    }
}
