# Skill 8: Implementasi Mail System (Mailable)

## Deskripsi / Tujuan
Sistem Mail sangat krusial untuk web modern (notifikasi, reset password, OTP). Tujuannya adalah membuat abstraksi mirip `Mail::to()->send()` di Laravel agar pengiriman email via SMTP menjadi elegan dan terpusat.

## Kebutuhan Teknis
- Buat modul `src/core/mail/mod.rs`.
- Gunakan *crate* `lettre` untuk integrasi pengiriman email via SMTP.
- Buat abstraksi `Mailable` trait yang mendefinisikan *subject*, *view template* (menggunakan Tera), dan *data*.
- Konfigurasi kredensial SMTP diambil dari `.env` (MAIL_HOST, MAIL_PORT, MAIL_USERNAME, MAIL_PASSWORD).
- Metode pengiriman harus mendukung *async* agar tidak memblokir thread.

## Langkah-langkah Implementasi
1. Tambahkan dependensi `lettre` di `Cargo.toml`.
2. Tambahkan konfigurasi default SMTP di `.env.example`.
3. Buat `Mailer` manager di `src/core/mail`.
4. Tambahkan `mailer` ke dalam `AppState`.
5. Buat perintah CLI `./lumina make:mail` untuk men-generate file mailable.

## Instruksi Git & Pull Request
- Checkout branch baru: `git checkout -b feature/mail-system`.
- Buka Pull Request dengan judul "feat(mail): Implement Mail System (Skill 8)".
