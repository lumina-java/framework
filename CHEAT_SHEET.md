# ⚡ Lumina Beauty Code — Ultimate Cheat Sheet

Panduan komprehensif untuk koding efisien, bersih, dan manusiawi di Lumina Framework.

---

## 🏗️ 1. Controller & Dependency Injection (The `Request` Way)

Lumina menggunakan **Bundle Extractor** untuk merampingkan parameter fungsi Anda.

```rust
// ✅ REKOMENDASI (Satu parameter untuk semua)
pub async fn store(req: Request, ValidatedForm(form): ValidatedForm<MyForm>) -> impl IntoResponse {
    let db = req.state.db();    // Akses Database
    let session = &req.session; // Akses Session
    let token = req.token;      // Akses CSRF
    let user = req.user;        // Akses User (Option<AuthUser>)
    
    // ...
}
```

### Extractor Lainnya:
| Extractor | Kegunaan | Contoh |
| :--- | :--- | :--- |
| `Path(id)` | Ambil ID dari URL | `/users/:id` |
| `Query(p)` | Ambil Query String | `?page=1&search=abc` |
| `HeaderMap` | Ambil HTTP Headers | `req.headers().get("User-Agent")` |

---

## 📡 2. WebSocket & Event Broadcasting (Production Ready)

Fitur WebSocket bawaan untuk fitur real-time seperti obrolan dan notifikasi.

```rust
use lumina::core::echo::ShouldBroadcast;
use serde_json::json;

#[derive(Debug)]
pub struct OrderPlacedEvent {
    pub order_id: u64,
    pub user_id: u64,
}

impl ShouldBroadcast for OrderPlacedEvent {
    fn broadcast_on(&self) -> Vec<String> {
        vec![format!("private-user-{}", self.user_id)]
    }

    fn broadcast_as(&self) -> String {
        "OrderPlaced".to_string()
    }

    fn broadcast_with(&self) -> serde_json::Value {
        json!({ "order_id": self.order_id })
    }
}

// Dispatch Event (Otomatis ter-broadcast ke channel WebSocket)
state.events.dispatch(OrderPlacedEvent { order_id: 101, user_id: 1 }, state.clone()).await;
```

---

## 🏥 3. Health Check & Production Features

Lumina menyediakan endpoint pemantauan kesehatan serta keamanan produksi bawaan:

* **Health Check Endpoint:** `GET /up` atau `GET /health` (Memeriksa status server & database).
* **Graceful Shutdown:** Server menangani sinyal `SIGINT` / `SIGTERM` secara aman saat proses restart atau deployment.
* **Security Headers:** HTTP headers bawaan (`X-Frame-Options`, `X-Content-Type-Options`, `X-XSS-Protection`, `HSTS`, `Referrer-Policy`).

---

## 🔐 4. Autentikasi & Keamanan (Facade Auth)

Hindari path panjang, gunakan Facade `Auth`.

```rust
use crate::core::auth::Auth;

// Cek Password (Bcrypt)
if Auth::check("password_user", &hashed_password) { ... }

// Login (Simpan JWT & User ke Session)
Auth::login(&session, auth_user).await?;

// Logout (Hapus Session)
Auth::logout(&session).await;

// Buat Objek User (Helper)
let user = Auth::user(id, email, role);
```

---

## 🔗 5. ORM & Relasi Database

Gunakan Trait `Model` untuk interaksi database yang elegan.

```rust
// 🔎 Ambil Data
let user = User::find(db, 1).await?;
let user = User::find_by_email(db, "test@mail.com").await?;

// 🔗 Relasi (Laravel Style)
let posts = user.posts(db).await?; // Has Many
let author = post.user(db).await?; // Belongs To

// 💾 Simpan / Update
user.save(db).await?;

// ⚡ Eager Loading (Solusi N+1)
let users = User::query(db).with("posts").get().await?;
```

---

## 🚀 6. Redirect, Flash & CSRF

Sistem feedback user yang sangat ringkas.

```rust
// ✅ Redirect dengan Success Flash & Sinkronisasi CSRF
Redirect::to("/dashboard")
    .with_success("Selamat Datang!")
    .go(req.token, &req.session)
    .await

// ✅ Redirect Back dengan Old Input (Form tidak akan kosong saat error)
Redirect::back(&req_parts)
    .with_input(json!({"email": form.email}))
    .with_error("Terjadi kesalahan!")
    .go(req.token, &req.session)
    .await
```

---

## 🐞 7. Debugging Pro

Jangan biarkan bug bersembunyi.

```rust
// Dump and Die (Browser akan menampilkan data dengan cantik dan berhenti di situ)
crate::dd!(variabel_anda);

```

---

## ⌨️ 8. CLI Tools (Keyboardholic)

| Perintah | Fungsi |
| :--- | :--- |
| `./lumina watch` | **Hot Reload** (Browser refresh otomatis saat kode berubah). |
| `./lumina migrate` | Jalankan semua migrasi database yang belum selesai. |
| `./lumina make:controller [Name]` | Generate boilerplate controller baru. |
| `./lumina make:model [Name]` | Generate boilerplate model baru. |

---

> **Dokumentasi Lengkap:** [DOCUMENTATION.md](./DOCUMENTATION.md)
