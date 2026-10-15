# 🚢 Deployment Guide

Panduan ini menjelaskan cara melakukan deployment aplikasi Lumina ke server produksi menggunakan Docker, Nginx, dan Supervisor.

## 1. Persiapan Server (VPS)
Pastikan server Anda sudah terinstall:
- Docker & Docker Compose
- Nginx (Optional, jika ingin reverse proxy di level host)

## 2. Menggunakan Docker (Rekomendasi)
Lumina menyediakan scaffolding Docker otomatis:
```bash
./lumina make:docker
```
Ini akan membuat `Dockerfile` (multi-stage) dan `docker-compose.yml`.

### Menjalankan Stack
```bash
docker-compose up -d
```
Stack ini mencakup:
- **Lumina App** (Port 8000)
- **PostgreSQL** (Database)
- **Redis** (Queue & Cache)
- **Nginx Container** (Port 80 & 443)

## 3. Konfigurasi Nginx (Reverse Proxy)
Jika Anda ingin mengatur Nginx secara manual:
```bash
./lumina make:nginx
```
Gunakan file `nginx/nginx.conf` yang dihasilkan sebagai referensi. Jangan lupa untuk menambahkan sertifikat SSL (misal: Let's Encrypt).

## 4. Manajemen Proses (Supervisor)
Jika Anda tidak menggunakan Docker, gunakan Supervisor untuk memastikan aplikasi tetap berjalan:
```bash
./lumina make:supervisor
```
Salin file `supervisor/lumina.conf` ke `/etc/supervisor/conf.d/` dan jalankan:
```bash
sudo supervisorctl reread
sudo supervisorctl update
sudo supervisorctl start lumina
```

## 5. Keamanan Produksi
Lumina secara otomatis menyertakan middleware `SecurityHeaders`. Pastikan Anda mengatur variabel environment di `.env`:
- `APP_ENV=production`
- `DATABASE_URL` (Gunakan password yang kuat)
- `JWT_SECRET` (Gunakan string acak panjang)

## 6. Health Monitoring
Gunakan endpoint `/health` untuk memantau status aplikasi via sistem eksternal.
```bash
curl http://your-domain.com/health
```
