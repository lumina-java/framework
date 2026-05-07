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

## 🔐 2. Autentikasi & Keamanan (Facade Auth)

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

## 🔗 3. ORM & Relasi Database

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

## 🚀 4. Redirect, Flash & CSRF

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

## 🐞 5. Debugging Pro

Jangan biarkan bug bersembunyi.

```rust
// Dump and Die (Browser akan menampilkan data dengan cantik dan berhenti di situ)
crate::dd!(variabel_anda);

// 🚀 HTMX (Zero-Mouse UI)
// Lumina otomatis mendukung partial rendering di layout.blade.rs
// Cukup gunakan hx-boost="true" pada tag body.
```

---

## ⌨️ 6. CLI Tools (Keyboardholic)

| Perintah | Fungsi |
| :--- | :--- |
| `./lumina watch` | **Hot Reload** (Browser refresh otomatis saat kode berubah). |
| `./lumina migrate` | Jalankan semua migrasi database yang belum selesai. |
| `./lumina make:controller [Name]` | Generate boilerplate controller baru. |
| `./lumina make:model [Name]` | Generate boilerplate model baru. |

---

## 💡 Saran Pengembangan Berikutnya (Next Skills)

1.  **Validation Macros**: Deklarasi validasi non-teknis `required|email|unique`.
2.  **Global Middleware Groups**: Kemudahan registrasi middleware dalam grup (web, api, auth).
3.  **Lumina Blueprint**: CLI yang lebih pintar untuk generate seluruh CRUD sekaligus.
4.  **Task Scheduling**: Menjalankan cron jobs bergaya Laravel.

> **Dokumentasi Lengkap:** [DOCUMENTATION.md](./DOCUMENTATION.md)
