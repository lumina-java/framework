# Skill 10: Implementasi Task Scheduling (Cron Jobs)

## Deskripsi / Tujuan
Menjadwalkan tugas secara terpusat dari kode sumber (mirip `php artisan schedule:run` di Laravel) tanpa harus repot menyetel ulang `crontab` OS pada setiap deployment. Berguna untuk membersihkan data lama, mengirim ringkasan email harian, dsb.

## Kebutuhan Teknis
- Menggunakan library penjadwalan berulang (misal `tokio-cron-scheduler`).
- Menyediakan *builder pattern* untuk mendefinisikan interval: `.daily()`, `.every_hour()`, `.every_minute()`.
- Menjadwalkan eksekusi *Job* atau *Closure* yang sudah ada.

## Langkah-langkah Implementasi
1. Tambahkan dependensi `tokio-cron-scheduler` di `Cargo.toml`.
2. Buat `SchedulerManager` di `src/core/schedule/mod.rs`.
3. Jalankan *scheduler loop* di background (menggunakan `tokio::spawn`) bersamaan dengan start server di `Application::serve`.
4. Buat file khusus `src/app/console/kernel.rs` untuk mendaftarkan jadwal-jadwal (mirip dengan Laravel `Console\Kernel::schedule()`).

## Instruksi Git & Pull Request
- Checkout branch baru: `git checkout -b feature/task-scheduling`.
- Buka Pull Request dengan judul "feat(schedule): Implement Task Scheduling (Skill 10)".
