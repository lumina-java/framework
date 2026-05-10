use std::sync::Arc;
use crate::core::application::AppState;
use crate::core::schedule::Scheduler;

/// Console Kernel — tempat mendefinisikan jadwal tugas (Schedule).
/// Anda bisa menambahkan tugas baru di fungsi `schedule`.
pub async fn schedule(sched: &Scheduler) {
    let state = sched.state();
    
    // CONTOH: Jalankan tugas setiap 30 detik
    let _ = sched.call("1/30 * * * * *", move || {
        let _s = state.clone();
        async move {
            println!("🔔 [SCHEDULE] Periodic task running every 30 seconds...");
            // Di sini Anda bisa memanggil Service, Job, atau query Database
            // let count = User::all(&s.db()).await.unwrap_or_default().len();
            // println!("📊 Current users in DB: {}", count);
        }
    }).await;

    // CONTOH: Jalankan tugas setiap menit
    let _ = sched.call("0 * * * * *", || {
        async move {
            println!("📅 [SCHEDULE] Minute task running...");
        }
    }).await;
}

/// Entry point untuk menjalankan scheduler dari application bootstrap.
pub async fn run(state: Arc<AppState>) {
    let sched = Scheduler::new(state).await;
    schedule(&sched).await;
    sched.run().await;
}
