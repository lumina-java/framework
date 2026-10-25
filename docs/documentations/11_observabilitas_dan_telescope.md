# 📘 Modul 11: Observabilitas & Lumina Telescope

**Lumina Telescope** adalah dasbor debug dan observabilitas bawaan yang terinspirasi oleh Laravel Telescope. Dashboard ini memberikan visibilitas penuh terhadap semua request HTTP, query database, background job, event siaran, log error, dan konsumsi memori sandbox WebAssembly secara real-time.

---

## 1. Mengakses Dashboard Telescope

Dashboard Telescope dapat diakses langsung melalui browser Anda di rute:

👉 **`http://localhost:8000/lumina/telescope`**

Tampilan dasbor dibuat sangat modern dengan dark mode, indikator status berwarna, dan integrasi HTMX untuk pembaruan data instan tanpa me-refresh halaman.

---

## 2. Kategori Pemantauan

Telescope mengelompokkan data ke dalam beberapa tab inspeksi:

| Tab | Deskripsi |
| :--- | :--- |
| **Requests** | Melacak method HTTP, URL path, status code (200, 404, 500), durasi eksekusi dalam milidetik, dan IP pengunjung. |
| **Queries** | Menampilkan query SQL yang dieksekusi, parameter query, serta waktu eksekusi database. |
| **Jobs** | Memantau antrean background jobs (status: `pending`, `running`, `completed`, `failed`). |
| **Events** | Daftar event real-time yang disiarkan melalui Lumina Echo. |
| **Logs** | Log level info, warning, dan error yang dihasilkan oleh sistem. |
| **Sandbox** | Tab baru untuk memantau performa dan konsumsi memori plugin WebAssembly & Indonesian Script. |

---

## 3. Tab Sandbox & Metrics

Dengan kehadiran arsitektur WASM Native Hybrid di Lumina v0.2.0, Telescope menyertakan panel khusus **Sandbox Monitor**:

- **Loaded WASM Plugins**: Daftar biner `.wasm` aktif yang dimuat ke dalam memori.
- **Cached Indonesian Scripts**: Skrip `.is` yang terkompilasi dalam AST runner cache.
- **Memory Allocation**: Halaman memori virtual WebAssembly yang dialokasikan per fungsi guest.
- **Execution Latency**: Durasi eksekusi transformasi string dan evaluasi skrip dalam skala mikrodetik (`μs`).

Data ini otomatis disuntikkan oleh `LuminaPluginManager::get_metrics()` dan dapat diinspeksi secara detail per request.

---

## 4. Health Checks Endpoint

Lumina juga menyediakan endpoint pemeriksaan kesehatan (*health check*) standar untuk Kubernetes, Docker, dan load balancer:

- **`GET /up`**: Mengembalikan status HTTP 200 jika server aktif dan siap menerima lalu lintas.
- **`GET /health`**: Mengembalikan payload JSON diagnostik (status database pool, penggunaan memori, dan uptime server).

---

### Navigasi Modul:
- **Sebelumnya**: [Modul 10: Queue, Jobs & Scheduler](./10_queue_jobs_dan_scheduler.md) 
- **Berikutnya**: [Modul 12: Deployment & DevOps](./12_deployment_dan_devops.md) ➔
- **Kembali ke**: [Daftar Isi Dokumentasi](./README.md)
