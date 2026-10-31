# 🛡️ Modul 15: Enterprise Security & Zero-Panic Safety

Keamanan dan keandalan sistem adalah prioritas tertinggi dalam pengembangan aplikasi berskala enterprise, perbankan, dan pemerintahan. Lumina Framework menghadirkan arsitektur keamanan tingkat lanjut (*Enterprise Grade Security*) serta mekanisme **Zero-Panic Safety Boundary** guna memastikan server tidak akan pernah tumbang (*zero-crash*) dalam kondisi apa pun.

---

## 🏛️ 1. Zero-Panic Error Boundary (`catch_unwind`)

Di ekosistem server backend konvensional, terjadinya eksekusi `panic!` yang tidak tertangani pada sebuah thread request dapat merusak status worker atau bahkan mematikan proses server secara mendadak (*crash*).

Lumina mengimplementasikan **Zero-Panic Safety Boundary** di tingkat middleware global menggunakan `std::panic::AssertUnwindSafe`.

```
┌────────────────────────────────────────────────────────────────────────┐
│                   ZERO-PANIC ERROR BOUNDARY FLOW                       │
├────────────────────────────────────────────────────────────────────────┤
│                                                                        │
│   Incoming Request                                                     │
│         │                                                              │
│         ▼                                                              │
│   [ Zero-Panic Middleware ] ──► std::panic::catch_unwind               │
│         │                                                              │
│         ├──► [ Normal Execution ] ──► Kembalikan HTTP Response Normal │
│         │                                                              │
│         └──► [ Terjadi Panic! ] ────► Tangkap Panic secara Aman        │
│                                           │                            │
│                                           ├──► Log Panic Stack Trace   │
│                                           ├──► Catat ke Audit Logger   │
│                                           └──► Kembalikan HTTP 500     │
│                                                (Graceful Error Page)   │
│                                                                        │
│   * Worker Tokio tetap sehat & siap melayani request berikutnya! *    │
│                                                                        │
└────────────────────────────────────────────────────────────────────────┘
```

### Cara Kerja pada Kode:
Anda dapat mengaktifkan proteksi ini pada seluruh rute atau grup rute tertentu:

```rust
use lumina::prelude::*;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/checkout", post(OrderController::checkout))
        .panic_boundary() // Membungkus seluruh handler di dalamnya dengan Zero-Panic Net
}
```

Ketika handler mengalami panic tak terduga (misalnya dereferensi `None` atau kesalahan logika matematis), server tidak crash. Sebaliknya:
1. Panic dicegah merambat ke luar thread Tokio.
2. Pesan error dan stack trace dicatat secara rinci ke log server.
3. Klien menerima respons terstruktur HTTP 500 Internal Server Error yang aman tanpa membocorkan kredensial internal sistem.

---

## 🔐 2. Granular Role-Based Access Control (RBAC)

Lumina menyediakan sistem hak akses dan peran (*Role & Permission*) deklaratif yang terintegrasi secara harmonis antara Model, Controller, Router, dan View Blade.

### A. Trait `HasRolesAndPermissions`
Setiap model pengguna atau token API mengimplementasikan trait ini:

```rust
use lumina::core::auth::HasRolesAndPermissions;

// Memeriksa izin spesifik
if user.has_permission("users.create") {
    // Diizinkan membuat user
}

// Memeriksa peran
if user.has_role("admin") {
    // Memiliki wewenang admin
}
```

#### Fitur Bawaan RBAC:
1. **Wildcard Permission (`*`)**: Izin `users.*` secara otomatis mencakup `users.create`, `users.edit`, dan `users.delete`.
2. **Superadmin Bypass**: Peran `superadmin` atau `admin` secara otomatis memiliki akses ke seluruh izin tanpa perlu didaftarkan satu per satu.

### B. Proteksi Rute Deklaratif
Proteksi rute dapat ditentukan langsung saat pendefinisian router:

```rust
Router::new()
    .get("/admin/users", UserController::index)
    .can("users.view")          // Hanya user dengan izin 'users.view' yang boleh masuk
    .has_role("administrator"); // Wajib memiliki peran 'administrator'
```
Jika pengguna tidak memiliki wewenang, Lumina langsung menolak akses dengan respons **HTTP 403 Forbidden**.

### C. Otorisasi Kontekstual di Controller (`req.authorize`)
Gunakan shortcut otorisasi satu baris pada Unified `Request` bundle:

```rust
pub async fn destroy(req: Request) -> LuminaResult {
    // 1-baris otorisasi: otomatis return HTTP 403 jika user tidak berhak
    req.authorize("users.delete")?;

    let id = req.id_or_fail()?;
    User::delete(req.db(), id).await?;
    
    req.success(json!({ "deleted": true }), "Data pengguna berhasil dihapus")
}
```

### D. Direktif Blade Terintegrasi (`@can`, `@cannot`, `@role`)
Kontrol tampilan UI secara instan di dalam template Blade:

```blade
@can('laporan.download')
    <a href="/laporan/export" class="btn btn-success">Unduh Laporan Keuangan</a>
@endcan

@cannot('produk.hapus')
    <span class="text-muted">Anda tidak memiliki hak menghapus produk ini</span>
@endcannot

@role('superadmin')
    <div class="alert alert-warning">Mode Super Admin Aktif</div>
@endrole
```

---

## 🛑 3. Anti Brute-Force & Credential Stuffing Guard

Serangan tebakan kata sandi beruntun (*Brute-Force Attacks*) dan pencurian kredensial (*Credential Stuffing*) dicegah menggunakan `BruteForceGuard`.

* **Lock-Free Cache Berkecepatan Tinggi**: Menggunakan *Moka Cache* in-memory dengan latensi nanodetik.
* **Mekanisme Exponential Backoff**:
  - Gagal 1 - 4 kali: Percobaan dicatat dengan toleransi normal.
  - Gagal 5 kali: Akun dan IP dikunci secara otomatis selama **15 menit (900 detik)**.
  - Reset Otomatis: Setiap kali pengguna berhasil login dengan benar, penghitung kegagalan langsung di-reset ke nol.

```rust
// Contoh penggunaan pada handler autentikasi
let client_ip = req.client_ip().unwrap_or("unknown");
let lockout_key = format!("{}:{}", client_ip, email);

if state.brute_force.is_locked(&lockout_key).await {
    let remaining = state.brute_force.lockout_remaining_seconds(&lockout_key).await;
    return Err(AppError::Forbidden(
        format!("Akun sementara dikunci karena terlalu banyak percobaan gagal. Coba lagi dalam {} detik.", remaining)
    ));
}
```

---

## ⏱️ 4. Constant-Time String Comparison (Anti Timing Attacks)

Penyerang dapat menebak secret key, API token, atau kata sandi dengan mengukur selisih nanodetik waktu respons perbandingan string (*Side-Channel Timing Attacks*).

Lumina mengamankan seluruh verifikasi sensitif dengan fungsi perbandingan konstan berbasis library `subtle`:

```rust
use subtle::ConstantTimeEq;

pub fn verify_signature(secret: &str, input: &str) -> bool {
    if secret.len() != input.len() {
        return false;
    }
    secret.as_bytes().ct_eq(input.as_bytes()).unwrap_u8() == 1
}
```
Instruksi ini memastikan waktu komparasi bit selalu bernilai konstan, menggagalkan teknik pengukuran waktu oleh penyerang.

---

## 🌐 5. Comprehensive Security Headers (OWASP Standard)

Lumina secara otomatis menyematkan header keamanan esensial sesuai panduan OWASP:

```rust
Router::new()
    .merge(api_routes)
    .security_headers() // Menambahkan header OWASP ke seluruh response
```

Header keamanan yang diinjeksi secara otomatis:
* `X-Frame-Options: DENY`: Melindungi aplikasi dari serangan pembajakan klik (*Clickjacking*).
* `X-Content-Type-Options: nosniff`: Mencegah browser menebak MIME-type berkas (*MIME Confusion Attacks*).
* `X-XSS-Protection: 1; mode=block`: Mengaktifkan filter XSS bawaan browser.
* `Referrer-Policy: strict-origin-when-cross-origin`: Menjaga kerahasiaan URL internal.
* `Strict-Transport-Security (HSTS)`: Memaksa koneksi HTTPS aman pada mode produksi.

---

## 📝 6. Enterprise Audit Trail Logger (SOC 2 / ISO 27001)

Pencatatan rekam jejak aktivitas (*Audit Trail*) bersifat asinkron, non-blocking, dan terstruktur untuk mematuhi regulasi perbankan dan privasi data (GDPR / ISO 27001).

```rust
// Mencatat aktivitas penting dari controller
req.audit_log(
    "ORDER_REFUND_APPROVED",
    "orders",
    order_id.to_string(),
    json!({
        "approved_by": req.user_id(),
        "amount": 500000,
        "reason": "Barang cacat produksi"
    })
);
```

### Format Log Terstandarisasi:
Audit log dicatat dalam format JSON terstruktur yang siap dikirimkan ke SIEM (*Elasticsearch, Datadog, atau Splunk*):
```json
{
  "timestamp": "2026-10-31T04:30:00Z",
  "action": "ORDER_REFUND_APPROVED",
  "table": "orders",
  "record_id": "90214",
  "user_id": 42,
  "client_ip": "103.28.12.5",
  "user_agent": "Mozilla/5.0 ...",
  "metadata": {
    "amount": 500000,
    "reason": "Barang cacat produksi"
  }
}
```

---

*Lumina Enterprise Security: Perisai baja tangguh untuk aplikasi misi kritis Anda.*

