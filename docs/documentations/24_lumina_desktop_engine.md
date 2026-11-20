# 🖥️ Modul 24: Lumina Desktop Compilation Engine

> *"Kompilasi Aplikasi Lumina Menjadi Standalone Single Executable Native Desktop App (.exe di Windows, Linux binary, macOS .app) dengan Performa Native Zero-Overhead."*

---

## 🏛️ 1. Mengapa Lumina Desktop?

Berbeda dengan Electron yang mengemas Chromium raksasa (ukuran > 150MB dan boros RAM), **Lumina Desktop Engine**:
- Menggunakan engine WebView native sistem operasi (WebView2 di Windows, WebKitGTK di Linux).
- Ukuran binary sangat ramping (~10-15MB).
- Konsumsi memori sangat hemat (< 60MB RAM).
- Database SQLite dan backend async Axum/Tokio berjalan secara embedded dalam 1 proses mandiri tanpa butuh web server terpisah.

---

## 🚀 2. Inisialisasi Mode Desktop (`lumina desktop:init`)

Jalankan perintah ini di root proyek Lumina Anda:
```bash
lumina desktop:init --title="Kasir POS Pintar" --width=1366 --height=768
```
Perintah ini menghasilkan:
1. `desktop.toml`: Konfigurasi dimensi window, judul aplikasi, dan port lokal.
2. `src/bin/desktop.rs`: Entrypoint native desktop runner.

---

## 🎮 3. Menjalankan Mode Desktop (`lumina desktop:run`)

Untuk menjalankan aplikasi dalam window GUI native selama pengembangan:
```bash
lumina desktop:run
```
Lumina akan otomatis mengalokasikan port loopback dinamis dan membuka window native yang terhubung ke server lokal secara instan.

---

## 📦 4. Kompilasi Executable Mandiri (`lumina desktop:build`)

Untuk membuat binary tunggal siap rilis ke pengguna akhir:
```bash
lumina desktop:build --release
```
Hasil file kompilasi:
- **Windows**: `target/release/desktop.exe`
- **Linux**: `target/release/desktop`
- **macOS**: `target/release/desktop`

Pengguna cukup melakukan double click pada file `.exe` untuk menjalankan aplikasi secara penuh tanpa instalasi dependency apa pun!
