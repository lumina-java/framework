# Skill 9: Implementasi Event System (Events & Listeners)

## Deskripsi / Tujuan
Pola desain Observer (Event-Driven) sangat penting untuk *decoupling* kode. Daripada menjalankan 5 tugas berbeda (misal: log, kirim email, beri notif, update statistik) secara berurutan di Controller registrasi, kita cukup men-*dispatch* `UserRegisteredEvent`. Berbagai listener akan bereaksi secara independen.

## Kebutuhan Teknis
- Buat abstraksi `Event` dan `Listener` di `src/core/event/mod.rs`.
- Buat `EventManager` (dispatcher) yang mengelola *subscription* listener ke *event* tertentu.
- Sinkronisasi dengan **Queue System** (Skill 6) agar listener yang berat bisa otomatis dilempar ke background job (`ShouldQueue`).

## Langkah-langkah Implementasi
1. Buat modul inti `src/core/event/mod.rs`.
2. Implementasikan *pub/sub* sederhana (in-memory) atau *Channel* berbasis MPSC.
3. Tambahkan `events: Arc<EventManager>` ke `AppState`.
4. Buat perintah CLI `./lumina make:event` dan `./lumina make:listener`.
5. Siapkan demonstrasi: `UserRegisteredEvent` dengan `SendWelcomeEmailListener`.

## Instruksi Git & Pull Request
- Checkout branch baru: `git checkout -b feature/event-system`.
- Buka Pull Request dengan judul "feat(events): Implement Event System (Skill 9)".
