# 📘 Modul 10: Queue, Jobs & Scheduler

Lumina Framework menyediakan sistem **Background Jobs** dan **Task Scheduler** untuk memproses tugas-tugas berat di latar belakang (seperti pengiriman email massal, ekspor laporan, dan sinkronisasi data) tanpa memblokir respons HTTP pengguna.

---

## 1. Membuat Background Job (`lumina make:job`)

Gunakan CLI Lumina untuk membuat job baru:

```bash
lumina make:job SendWelcomeEmailJob
```
File akan dibuat di `src/app/jobs/send_welcome_email_job.rs`:

```rust
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use lumina::core::queue::Job;
use lumina::core::application::AppState;

#[derive(Serialize, Deserialize, Clone)]
pub struct SendWelcomeEmailJob {
    pub email: String,
    pub user_name: String,
}

#[async_trait]
impl Job for SendWelcomeEmailJob {
    async fn handle(&self, state: &AppState) -> Result<(), String> {
        println!("Mengirimkan email selamat datang ke: {} ({})", self.user_name, self.email);
        
        // Simulasikan pengiriman email atau panggil Mail Service
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

        println!("Email sukses terkirim!");
        Ok(())
    }
}
```

---

## 2. Memasukkan Tugas ke Antrean (Dispatching)

Panggil method `req.state.queue.dispatch(...)` dari controller:

```rust
use lumina::prelude::*;
use axum::response::IntoResponse;

pub async fn register_success(req: Request) -> impl IntoResponse {
    let job = SendWelcomeEmailJob {
        email: "ahmad@example.com".to_string(),
        user_name: "Ahmad".to_string(),
    };

    // Masukkan ke antrean queue tanpa menunggu proses email selesai
    let _ = req.state.queue.dispatch(job).await;

    View::make("auth.registered_success")
        .with("title", "Registrasi Berhasil")
        .with("message", "Email sambutan sedang dikirimkan di latar belakang!")
        .render(&req)
        .await
}
```

---

## 3. Worker Antrean Database (Persistent Queue)

Tugas-tugas antrean disimpan secara persisten di database (tabel `jobs`). Jika server mati mendadak, antrean tidak akan hilang dan akan otomatis dilanjutkan saat server kembali berjalan (*crash-resilient*).

---

## 4. Penjadwalan Tugas Otomatis (Task Scheduler)

Lumina menyediakan scheduler bawaan bergaya Cron. Daftarkan tugas berulang di `src/main.rs`:

```rust
use lumina::core::schedule::Scheduler;

let mut scheduler = Scheduler::new();

// Jalankan pembersihan token setiap hari jam 02:00
scheduler.daily_at("02:00", || async {
    println!("Menjalankan pembersihan expired tokens...");
});

// Jalankan pencadangan database setiap minggu
scheduler.weekly(|| async {
    println!("Menjalankan auto-backup database...");
});
```

---

### Navigasi Modul:
- **Sebelumnya**: [Modul 09: Broadcasting & WebSocket](./09_broadcasting_dan_websocket.md) 
- **Berikutnya**: [Modul 11: Observabilitas & Telescope](./11_observabilitas_dan_telescope.md) ➔
- **Kembali ke**: [Daftar Isi Dokumentasi](./README.md)
