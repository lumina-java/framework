use std::sync::Arc;
use crate::core::application::AppState;
use crate::core::schedule::Scheduler;

/// Console Kernel — tempat mendefinisikan jadwal tugas (Schedule).
/// Anda bisa menambahkan tugas baru di fungsi `schedule`.
pub async fn schedule(sched: &Scheduler) {
    let state = sched.state();
    
    // CONTOH: Jalankan tugas setiap menit menggunakan API Fluent
    let _ = sched.call(|| {
        async move {
            println!("📅 [SCHEDULE] Minute task running via fluent API...");
        }
    }).every_minute().await;

    // CONTOH: Jalankan tugas setiap hari pada jam 12:00
    let _ = sched.call(|| {
        async move {
            println!("🕛 [SCHEDULE] Running daily maintenance at 12:00...");
        }
    }).daily_at("12:00").await;
}

/// Entry point untuk menjalankan scheduler dari application bootstrap.
pub async fn run(state: Arc<AppState>) {
    let sched = Scheduler::new(state).await;
    schedule(&sched).await;
    sched.run().await;
}
