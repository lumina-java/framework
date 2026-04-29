# Background Job System (Task Queue)

Lumina menyediakan sistem antrean asinkron (Queue) yang terintegrasi untuk menjalankan tugas berat di latar belakang.

## 1. Membuat Job Baru
Gunakan perintah CLI Lumina untuk membuat class Job baru:
```bash
./lumina make:job SendEmailJob
```
File akan dibuat di `src/app/jobs/send_email_job.rs`. Jangan lupa daftarkan modulnya di `src/app/jobs/mod.rs`.

## 2. Struktur Job
Setiap job harus mengimplementasikan trait `Job`. Contoh sederhana:

```rust
#[async_trait]
impl Job for SendEmailJob {
    async fn handle(&self, state: Arc<AppState>) -> Result<(), String> {
        // Logika pekerjaan Anda (misal: kirim email)
        tracing::info!("Mengirim email...");
        Ok(())
    }
}
```

## 3. Dispatching Job
Anda bisa mengirim job ke antrean dari Controller atau Route mana pun yang memiliki akses ke `AppState`.

```rust
pub async fn store(State(state): State<AppState>) -> impl IntoResponse {
    let job = SendEmailJob::new("user@example.com");
    
    // Kirim ke antrean
    state.queue.dispatch(job).await.unwrap();
    
    ApiResponse::success("Job berhasil dikirim").into_response()
}
```

## 4. Keuntungan
- **Non-blocking:** Respon HTTP dikirim seketika tanpa menunggu job selesai.
- **Worker Auto-start:** Worker berjalan otomatis saat server startup.
- **AppState Access:** Job memiliki akses penuh ke Database, Storage, dan Config melalui `state`.
