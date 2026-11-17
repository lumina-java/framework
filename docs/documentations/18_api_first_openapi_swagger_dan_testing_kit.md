# 🚀 Modul 18: API-First Suite, OpenAPI v3, Swagger UI & Fluent Testing Kit

> **Filosofi Desain Lumina:**  
> *"Look like Laravel, Run at Rust Speed. Dokumentasi RESTful API terstandarisasi otomatis tanpa boilerplate, dan pengalaman pengujian HTTP in-memory yang seindah Laravel."*

Lumina Framework v0.8.0 menghadirkan **API-First Suite & Testing Ergonomics** yang mengubah cara pengembang membangun dan menguji backend REST API di Rust:
1. **Route Metadata & OpenAPI v3 Auto-Generation**: Tambahkan metadata ekspresif langsung pada method chaining router Lumina (`.summary()`, `.description()`, `.tag()`, `.deprecated()`).
2. **Built-in Swagger UI Endpoint**: Cukup panggil `.swagger_ui("/docs", ...)` pada router, maka visualisasi interaktif Swagger UI siap digunakan di browser.
3. **CLI Exporter**: Perintah Artisan bawaan `lumina route:openapi` dan `lumina export:postman` untuk ekspor instan ke CI/CD maupun Postman/Insomnia.
4. **Fluent HTTP Testing Assertion Kit (`lumina::testing::*`)**: Engine pengujian in-memory tanpa overhead jaringan TCP yang menyediakan 20+ assertion fluen ala Laravel (`assert_status`, `assert_json_path`, `assert_database_has`, dll).

---

## 📖 1. Route Metadata & OpenAPI v3

Anda dapat mendekorasi route Lumina dengan metadata OpenAPI standar secara fluen:

```rust
use lumina::prelude::*;

pub fn api_routes() -> Router<AppState> {
    Router::new()
        .get("/api/v1/contacts", contacts::index)
        .summary("Daftar Kontak")
        .description("Mengambil semua kontak aktif dengan dukungan paginasi.")
        .tag("Contacts")
        .operation_id("getContacts")

        .post("/api/v1/contacts", contacts::store)
        .summary("Tambah Kontak Baru")
        .description("Menyimpan entitas kontak baru ke database.")
        .tag("Contacts")

        .get("/api/v1/contacts/:id", contacts::show)
        .summary("Detail Kontak")
        .tag("Contacts")

        .delete("/api/v1/contacts/:id", contacts::destroy)
        .summary("Hapus Kontak")
        .tag("Contacts")

        .group("/admin", |r| {
            r.get("/metrics", admin::metrics)
                .summary("Metrik Server")
                .tag("Admin")
                .deprecated() // Menandai endpoint sebagai usang
        })
}
```

> [!TIP]
> **Deteksi Parameter Otomatis**: Parameter path Axum seperti `:id` atau `*slug` secara cerdas otomatis dikonversi ke standar OpenAPI `{id}` dengan skema tipe data `integer` atau `string`.

---

## 🎨 2. Memasang Swagger UI Interaktif

Hanya dengan satu baris method chaining `.swagger_ui()`, Lumina akan secara otomatis mendaftarkan endpoint visual Swagger UI v5 dan spesifikasi mentah JSON:

```rust
let router = Router::new()
    .merge(api_routes())
    .swagger_ui("/docs", "Lumina Application API", "1.0.0");
```

Endpoint yang otomatis aktif:
- `GET http://localhost:3000/docs` atau `GET /docs/swagger` ➔ Halaman interaktif Swagger UI v5 yang cantik dan modern.
- `GET http://localhost:3000/docs/openapi.json` ➔ Spesifikasi OpenAPI v3.0.3 lengkap dalam format JSON.

---

## ⚡ 3. Lumina CLI Exporter (OpenAPI & Postman)

Bagi tim backend yang mengintegrasikan spesifikasi API ke pipeline CI/CD atau membagikannya ke tim Frontend / Mobile Developer:

### A. Ekspor OpenAPI Spec (`route:openapi`)
```bash
# Menghasilkan openapi.json (default)
lumina route:openapi

# Menghasilkan openapi.yaml
lumina route:openapi --format yaml --output openapi.yaml

# Kustomisasi nama API & versi
lumina route:openapi --title "POS API Service" --version "2.1.0"
```

### B. Ekspor Koleksi Postman v2.1 (`export:postman`)
```bash
# Menghasilkan koleksi Postman yang siap di-import 1-klik
lumina export:postman --output pos_postman.json --name "POS Service API"
```

---

## 🧪 4. Fluent HTTP Testing Assertion Kit (`lumina::testing::*`)

Pengujian di Lumina berjalan **in-memory** menggunakan runtime Tokio dan Tower `oneshot`, memberikan eksekusi tes instan tanpa port collision di CI/CD.

### A. Dasar Pengujian dengan `TestApp`

```rust
use lumina::testing::TestApp;
use serde_json::json;

#[tokio::test]
async fn test_contact_creation_flow() {
    let mut app = TestApp::new().await;
    app.router = my_app_router();

    // 1. Uji Validasi Gagal (422)
    let res = app.post_req("/api/v1/contacts")
        .with_json(json!({ "name": "" }))
        .send().await;

    res.assert_unprocessable()
       .assert_json_path("success", false)
       .assert_json_has("errors.name");

    // 2. Uji Sukses (201 Created)
    let res_ok = app.post_req("/api/v1/contacts")
        .with_token("valid_jwt_token")
        .with_json(json!({
            "name": "Budi Santoso",
            "email": "budi@lumina.dev",
            "phone": "08123456789"
        }))
        .send().await;

    res_ok
       .assert_created()
       .assert_json_path("success", true)
       .assert_json_path("data.name", "Budi Santoso");
}
```

---

## 📋 5. Daftar Lengkap Assertion Methods

### A. HTTP Status Assertions
| Method | Deskripsi |
| :--- | :--- |
| `res.assert_status(200u16)` | Memastikan kode status HTTP cocok (`u16` atau `StatusCode`). |
| `res.assert_ok()` | Memastikan response berstatus HTTP 200 OK. |
| `res.assert_created()` | Memastikan response berstatus HTTP 201 Created. |
| `res.assert_no_content()` | Memastikan response berstatus HTTP 204 No Content. |
| `res.assert_bad_request()` | Memastikan response berstatus HTTP 400 Bad Request. |
| `res.assert_unauthorized()` | Memastikan response berstatus HTTP 401 Unauthorized. |
| `res.assert_forbidden()` | Memastikan response berstatus HTTP 403 Forbidden. |
| `res.assert_not_found()` | Memastikan response berstatus HTTP 404 Not Found. |
| `res.assert_unprocessable()` | Memastikan response berstatus HTTP 422 Unprocessable Entity. |
| `res.assert_server_error()` | Memastikan response berstatus HTTP 500 Internal Server Error. |

### B. JSON Structure & Content Assertions
| Method | Deskripsi |
| :--- | :--- |
| `res.assert_json_path("data.id", 42)` | Memastikan nilai pada dot notation path sama persis. |
| `res.assert_json_has("data.token")` | Memastikan path tertentu ada di dalam JSON. |
| `res.assert_json_missing("data.secret")` | Memastikan path tertentu TIDAK ada di dalam JSON. |
| `res.assert_json_count("data.items", 5)` | Memastikan array atau object pada path memiliki panjang tertentu. |

### C. Text, Header & Redirection Assertions
| Method | Deskripsi |
| :--- | :--- |
| `res.assert_see("Selamat Datang")` | Memastikan teks tertentu terkandung dalam response body. |
| `res.assert_dont_see("Error")` | Memastikan teks tertentu TIDAK muncul dalam response body. |
| `res.assert_header("X-Custom", "Val")` | Memastikan header HTTP tertentu memiliki nilai yang sesuai. |
| `res.assert_redirect("/dashboard")` | Memastikan response adalah redirect (3xx) menuju target path. |

---

## 🗄️ 6. Database Testing Assertions

`TestApp` dilengkapi dengan helper pengujian database yang langsung memvalidasi baris data di SQLite in-memory:

```rust
#[tokio::test]
async fn test_database_state() {
    let app = TestApp::new().await;

    // Memastikan baris data tersimpan di tabel
    app.assert_database_has("users", json!({
        "email": "budi@lumina.dev",
        "is_active": 1
    })).await;

    // Memastikan data tertentu TIDAK ada di tabel
    app.assert_database_missing("users", json!({
        "email": "hacker@evil.com"
    })).await;

    // Memastikan total baris tabel sesuai ekspektasi
    app.assert_database_count("users", 1).await;
}
```

---

### Navigasi Modul:
- **Sebelumnya**: [Modul 17: Universal Storage & Redis Suite](./17_universal_storage_dan_redis_suite.md)
- **Kembali ke**: [Daftar Isi Dokumentasi](./README.md)

