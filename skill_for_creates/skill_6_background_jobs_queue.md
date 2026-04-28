# Skill 6: Implementasi Background Jobs / Task Queue

## Deskripsi / Tujuan
Untuk eksekusi operasi yang memakan waktu (seperti pengiriman *email*, pembuatan laporan ekspor Excel/PDF, integrasi API lambat), sangat penting untuk tidak memblokir respon HTTP. Membangun sistem *Background Job / Task Queue* lokal memungkinkan *developer* untuk melakukan `dispatch(SendEmailJob)` di *background*.

## Kebutuhan Teknis
- Buat sistem antrean sederhana. Untuk tahap MVP (*Minimum Viable Product*), *in-memory channel* (menggunakan `tokio::sync::mpsc`) sudah cukup. 
- *Next step* (bisa sebagai opsi lanjutan) adalah menyimpannya ke tabel database atau Redis.
- Terdapat fungsi untuk men-*dispatch* (mengirim ke antrean) dan sebuah *worker loop* terpisah yang bertugas memproses antrean secara asinkron.

## Langkah-langkah Implementasi
1. Buat modul `src/core/queue.rs`.
2. Definisikan trait `Job` yang mewajibkan implementasi metode `async fn handle(&self) -> Result<(), AppError>`.
3. Sediakan komponen `Queue` yang mengandalkan channel MPSC tokio. Satu *sender* bisa dibagikan ke `AppState` untuk bisa dipanggil dari *controllers*.
4. *Receiver* (Worker) di-spawn di *thread* baru (`tokio::spawn`) di `Application::serve()` dan jalan di *background*.
5. Buat fungsi helper `dispatch()` untuk melempar pekerjaan ke antrean dengan mudah.

## Instruksi Git & Pull Request
- Checkout branch baru: `git checkout -b feature/background-jobs-queue`.
- Tulis kodenya secara terstruktur, jangan menggabungkannya ke *core* routing HTTP.
- Buka Pull Request untuk disetujui secara manual.

## Panduan User Test Manual
1. Buat struct Dummy `TestJob` yang mengimplementasi interface antrean.
2. Di dalam method `handle()` milik `TestJob`, buat agar ia berhenti selama 5 detik (`tokio::time::sleep(Duration::from_secs(5)).await;`), lalu lakukan pencetakan (print) "Job Selesai Dikerjakan!".
3. Buat rute web `/test-queue`. Di dalam handler-nya, lakukan `dispatch(TestJob::new())` dan segera me-*return* response JSON `{"status": "Job dikirim ke antrean"}`.
4. Lakukan pemanggilan cURL/Postman ke `/test-queue`.
5. *Ekspektasi 1:* Response API harus *instant* (kurang dari 100ms) mereturn JSON. Tidak tertahan selama 5 detik.
6. *Ekspektasi 2:* Di terminal konsol server, setelah kurang lebih 5 detik pasca *request*, akan muncul tulisan "Job Selesai Dikerjakan!". Ini menandakan sistem *background processing* berjalan sukses.
