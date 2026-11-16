# 📦 Modul 17: Universal Storage & Redis Multi-Driver Suite

> *"Penyimpanan berkas universal dan ekosistem terdistribusi yang membebaskan aplikasi Lumina dari batasan single-server menuju skala cloud enterprise."*

---

## 🌟 1. Pengantar

Ketika aplikasi berpindah dari server tunggal menuju lingkungan multi-pod (Docker Swarm, Kubernetes, atau AWS ECS), dua tantangan utama muncul:
1. **Penyimpanan Berkas**: Disk lokal bersifat ephemeral (sementara). Berkas yang diunggah harus disimpan ke Object Storage (AWS S3, Cloudflare R2, MinIO, atau Google Cloud Storage).
2. **Sinkronisasi Antar-Pod**: Cache memori lokal, antrean database tunggal, dan WebSocket in-memory tidak tersinkronisasi antar-server. **Redis** hadir sebagai message broker dan shared store performa tinggi berlatensi sub-milidetik.

Lumina menghadirkan **Universal Storage Facade** dan **Redis Multi-Driver Suite** dengan sintaks yang sangat ergonomis ala Laravel.

---

## 🗄️ 2. Universal Storage Facade (`Storage`)

### A. Konfigurasi Lingkungan (`.env`)
```ini
# Pilihan default disk: local | public | s3
FILESYSTEM_DISK=s3
UPLOAD_MAX_SIZE_MB=10

# Konfigurasi S3 / MinIO / Cloudflare R2
AWS_ACCESS_KEY_ID=your_access_key
AWS_SECRET_ACCESS_KEY=your_secret_key
AWS_DEFAULT_REGION=us-east-1
AWS_BUCKET=hospital-records

# Opsional untuk MinIO atau Cloudflare R2:
# AWS_ENDPOINT=http://127.0.0.1:9000
# AWS_USE_PATH_STYLE_ENDPOINT=true # true untuk MinIO lokal
# AWS_URL=https://cdn.example.com  # Custom CDN Domain
```

### B. Penggunaan Melalui Global Facade
```rust
use lumina::prelude::*;

// 1. Simpan berkas ke disk default
Storage::put_file("documents/report.pdf", &pdf_bytes).await?;

// 2. Simpan berkas ke disk spesifik ("s3")
Storage::disk_named("s3").put("avatars/user_1.jpg", &image_bytes).await?;

// 3. Ambil isi berkas
let bytes = Storage::disk_named("s3").get("documents/report.pdf").await?;

// 4. Periksa keberadaan berkas
if Storage::disk_named("s3").exists("avatars/user_1.jpg").await {
    println!("Berkas ditemukan!");
}

// 5. Hapus berkas
Storage::disk_named("s3").delete("temp/cache.tmp").await?;
```

### C. Presigned Temporary URLs (AWS SigV4)
Untuk berkas privat (seperti hasil scan rekam medis, invoice tagihan, atau laporan rahasia), Lumina menyediakan pembuatan **Presigned URL** berbatas waktu (*Time-To-Live*) secara instan dengan algoritma AWS Signature Version 4 (SigV4):

```rust
use std::time::Duration;

// URL download privat yang hanya valid selama 1 jam (3600 detik)
let secure_url = Storage::disk_named("s3")
    .temporary_url("patient_records/scan_001.pdf", Duration::from_secs(3600))?;

println!("Akses aman: {}", secure_url);
```

### D. Penggunaan di Controller via `Request`
```rust
pub async fn upload_avatar(req: Request) -> LuminaResult {
    let file = req.file("avatar")?;
    
    // Simpan langsung ke S3 melalui helper storage
    let target = format!("avatars/{}_{}", Uuid::new_v4(), file.filename);
    req.storage().disk("s3").put(&target, &file.bytes).await?;
    
    let public_url = req.storage().disk("s3").url(&target);
    req.success(json!({ "url": public_url }), "Foto berhasil diunggah ke S3")
}
```

---

## ⚡ 3. Distributed Redis Cache

### A. Konfigurasi `.env`
```ini
CACHE_DRIVER=redis # Pilihan: moka (in-memory) | redis
REDIS_URL="redis://127.0.0.1:6379/0"
```

### B. Penggunaan Facade `Cache`
```rust
use lumina::prelude::*;

// 1. Simpan ke cache (TTL 3600 detik)
Cache::put("user:profile:101", &user_data, 3600).await;

// 2. Ambil dari cache
let profile: Option<User> = Cache::get("user:profile:101").await;

// 3. Pattern Remember (Cache-Aside Pattern)
// Jika ada di cache, langsung kembalikan. Jika belum, jalankan query lalu simpan otomatis.
let top_products = Cache::remember("dashboard:top_products", 600, || async {
    Product::query(db).order_by("sales", "DESC").limit(5).get().await.unwrap_or_default()
}).await;

// 4. Hapus cache spesifik & flush seluruh cache
Cache::forget("user:profile:101").await;
Cache::flush().await;
```

---

## 🚀 4. High-Speed Redis Queue Driver

### A. Konfigurasi `.env`
```ini
QUEUE_CONNECTION=redis # Pilihan: database (SQL) | redis
REDIS_URL="redis://127.0.0.1:6379/0"
```

### B. Dispatching Job
```rust
// Kirim job latar belakang ke Redis Queue
SendWelcomeEmail { email: "user@example.com".into() }.dispatch(&req).await?;
```

### C. Menjalankan Worker Daemon
Eksekusi worker daemon untuk memproses antrean dari Redis:
```bash
lumina queue:work
```
Worker akan secara otomatis mendeteksi konfigurasi `QUEUE_CONNECTION=redis` dan mengeksekusi job dengan latensi mikrodetik.

---

## 📡 5. Lumina Echo Redis Pub/Sub Broadcaster

Di lingkungan Kubernetes multi-pod, browser pengguna dapat terhubung ke Pod A sementara event broadcast dipicu dari Pod B. Dengan mengaktifkan `BROADCAST_DRIVER=redis`, seluruh pod Lumina saling terhubung via Redis Pub/Sub:

```ini
BROADCAST_DRIVER=redis
REDIS_URL="redis://127.0.0.1:6379/0"
```

Event yang dikirim dari controller:
```rust
req.broadcast("orders", "OrderPlaced", json!({ "order_id": 42 })).await;
```
Akan otomatis diteruskan ke seluruh pod Lumina dan diterima oleh client browser manapun di pod manapun secara real-time!

