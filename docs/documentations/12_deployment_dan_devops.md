# 📘 Modul 12: Deployment & DevOps

Lumina dirancang agar sangat mudah dideploy ke lingkungan produksi (VPS, Docker, Cloud Container) dengan ukuran biner tunggal yang sangat kecil, konsumsi memori minimal (< 30 MB RAM), dan tanpa ketergantungan runtime eksternal.

---

## 1. Kompilasi Produksi (Release Build)

Jalankan kompilasi teroptimasi:

```bash
cargo build --release
```
Biner hasil rilis berada di:
`target/release/<project_name>-server` (Linux/macOS) atau `.exe` (Windows).

Konfigurasi optimasi bawaan di `Cargo.toml`:
```toml
[profile.release]
opt-level = "z"       # Optimasi ukuran biner minimal
lto = true            # Link-Time Optimization antar crate
codegen-units = 1     # Pengurangan overhead duplikasi kode
panic = "abort"       # Membuang stack unwinding untuk binary lebih kecil
strip = true          # Otomatis menghapus debug symbols
```

---

## 2. Docker & Containerization (`lumina make:docker`)

Lumina menyediakan generator konfigurasi Dockerfile multi-stage production-ready:

```bash
lumina make:docker
```
Perintah ini akan membuat:
- `Dockerfile`: Multi-stage build menggunakan image Rust builder dan image Alpine/Debian slim untuk runtime akhir (ukuran container akhir biasanya hanya ~25-45 MB).
- `docker-compose.yml`: Sudah menyertakan konfigurasi container aplikasi Lumina dan database (PostgreSQL / MySQL).

### Menjalankan Container:
```bash
docker compose up -d --build
```

---

## 3. Reverse Proxy Nginx (`lumina make:nginx`)

Generate konfigurasi reverse proxy Nginx yang sudah dilengkapi dengan proxy header dan dukungan WebSocket:

```bash
lumina make:nginx
```
File akan dibuat di `deploy/nginx.conf`:

```nginx
server {
    listen 80;
    server_name mydomain.com;

    location / {
        proxy_pass http://127.0.0.1:8000;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;

        # Dukungan WebSocket Lumina Echo
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection "upgrade";
    }

    # Static Assets Cache
    location /static/ {
        alias /var/www/my-app/public/;
        expires 30d;
        add_header Cache-Control "public, no-transform";
    }
}
```

---

## 4. Process Manager Supervisor (`lumina make:supervisor`)

Pastikan aplikasi web dan worker background job selalu berjalan dan restart otomatis jika terjadi kegagalan sistem:

```bash
lumina make:supervisor
```
File akan dibuat di `deploy/supervisor.conf`:

```ini
[program:lumina-app]
command=/var/www/my-app/target/release/my-app-server
directory=/var/www/my-app
autostart=true
autorestart=true
user=www-data
redirect_stderr=true
stdout_logfile=/var/log/lumina-app.log
environment=APP_ENV="production"
```

---

## 5. Backup Database & Storage (`lumina backup:run`)

Lumina dilengkapi dengan toolchain backup mandiri:

```bash
lumina backup:run
```
Perintah ini akan membuat snapshot cadangan database SQL dan arsip file dari `storage/app/public/` ke dalam folder `storage/backups/<timestamp>_backup.zip`.

---

### Navigasi Modul:
- **Sebelumnya**: [Modul 11: Observabilitas & Telescope](./11_observabilitas_dan_telescope.md) 
- **Kembali ke**: [Daftar Isi Dokumentasi](./README.md)
