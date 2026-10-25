# 🤖 Lumina Framework — Developer & AI Agent Guidelines

> **Visi Inti Lumina:**  
> *"Look like Laravel, Run at Rust Speed. Cuantik, elegan, tercepat, dan menghadirkan pengalaman pengembangan (Developer Experience) paling menyenangkan di dunia backend."*

Dokumen ini adalah pedoman mutlak (**Rules & Standards**) bagi seluruh AI Agent (Antigravity, Cursor, Copilot, dsb.) dan developer manusia yang menulis, memperbarui, atau melakukan porting kode pada proyek Lumina Framework.

---

## 🏛️ 1. Prinsip Utama Desain Kode

1. **Laravel Elegance**: Sintaks harus ekspresif, ringkas, bersih (*human-readable*), dan deklaratif. Hindari kode bertele-tele (*boilerplate*) yang menghambat kecepatan berpikir developer.
2. **Rust Raw Speed & Safety**: Menggunakan Tokio async runtime, Axum engine, dan zero-cost abstractions tanpa Garbage Collector.
3. **One Fast, Easy Development**: Menghadirkan *Batteries-Included Framework* di mana ORM, Blade view, autentikasi, router, WebSocket, scheduler, dan queue sudah terintegrasi secara harmonis.

---

## 🎮 2. Rules untuk Controller (`src/app/controllers/`)

### A. Wajib Menggunakan Unified `Request` Bundle
- **DILARANG KERAS** menggunakan puluhan extractor mentah Axum (`State(state)`, `Path(id)`, `Query(q)`, `Session(s)`, `CsrfToken`, dsb.) pada controller aplikasi.
- **WAJIB** menggunakan satu parameter tunggal: `req: Request` (dari `lumina::prelude::*`).

### B. Standardisasi Return Type
- **Controller Halaman Web (HTML View)**:
  - **WAJIB** mengembalikan `impl IntoResponse`.
  - **WAJIB** merender view menggunakan `View::make("nama.view").with("kunci", nilai)...render(req).await` (Bebas tanda ampersand `&` yang merepotkan).
  - Contoh:
    ```rust
    pub async fn index(req: Request) -> impl IntoResponse {
        let user = req.user.clone().expect("Unauthorized");
        let users = User::query(req.db()).with("posts").get().await.unwrap_or_default();

        // Bersih, elegan, tanpa tanda '&'
        View::make("dashboard.index")
            .with("title", "Dashboard — Lumina")
            .with("user", user)
            .with("users", users)
            .render(req)
            .await
    }
    ```

- **Controller RESTful API (JSON)**:
  - **WAJIB** mengembalikan `LuminaResult` (alias untuk `Result<Response, AppError>`).
  - **WAJIB** memanfaatkan shortcut helper response bawaan `Request`:
    - `req.ok(data)` ➔ HTTP 200 `{ "success": true, "data": ... }`
    - `req.success(data, "Pesan sukses")` ➔ HTTP 200 `{ "success": true, "message": "...", "data": ... }`
    - `req.created_with(data, "Pesan dibuat")` ➔ HTTP 201 `{ "success": true, "message": "...", "data": ... }`
    - `req.paginate(paginator)` ➔ HTTP 200 dengan metadata paginasi lengkap
    - `req.not_found("Pesan error")` ➔ HTTP 404
    - `req.bad_request("Pesan error")` ➔ HTTP 400
    - `req.unauthorized("Pesan error")` ➔ HTTP 401
  - Contoh:
    ```rust
    pub async fn api_index(req: Request) -> LuminaResult {
        let (page, per_page) = req.pagination();
        let paginator = Contact::paginate(req.db(), page, per_page).await?;
        req.paginate(paginator)
    }
    ```

### C. Validasi Input Otomatis
- Form Web: Gunakan `ValidatedForm<T>`. Jika validasi gagal, engine Lumina otomatis me-redirect balik dengan flash error dan input lama (*old input*).
- REST API: Gunakan `ValidatedJson<T>`. Jika validasi gagal, engine otomatis mengembalikan HTTP 422 Unprocessable Entity berformat JSON.

### D. Akses Data Kontekstual via `Request`
- Akses Database: `req.db()` atau `req.db_arc()`.
- Akses User Login: `req.user` atau `req.user()`.
- Akses Path ID: `let id = req.id_or_fail()?;`.
- Akses Query Param: `let search = req.query_val("search");`.
- Akses Paginasi: `let (page, per_page) = req.pagination();`.
- Akses Storage: `req.storage()`.
- Akses Mailer: `req.mail()`.
- Redirect Flluen: `req.redirect("/dashboard").with_success("Sukses!").go(&req).await`.

---

## 🗄️ 3. Rules untuk Model & ORM (`src/app/models/`)

1. **Macro Derivation**:
   Setiap entitas model **WAJIB** menambahkan derive:
   ```rust
   #[derive(Debug, Serialize, Deserialize, FromRow, Clone, Default, LuminaModel)]
   #[table("nama_tabel")]
   pub struct User {
       pub id: i64,
       pub name: String,
       pub email: String,
       pub created_at: Option<String>,
       pub updated_at: Option<String>,
       pub deleted_at: Option<String>,
   }
   ```
2. **Fluent Query Builder**:
   Gunakan query builder yang lancar ala Laravel:
   ```rust
   let active_users = User::query(req.db())
       .where_eq("is_active", true)
       .where_gt("age", 18)
       .order_by("name", "ASC")
       .limit(10)
       .get()
       .await?;
   ```
3. **One-Line Relationships**:
   Definisikan relasi antar model secara deklaratif dalam 1 baris:
   ```rust
   // Di dalam model User
   has_many!(Post, "user_id");
   has_one!(Profile, "user_id");
   belongs_to_many!(Role, "user_roles", "user_id", "role_id");

   // Di dalam model Post
   belongs_to!(User, "user_id");
   ```
4. **Cegah Masalah N+1 Query**:
   Selalu gunakan eager loading `.with("relation_name")` saat mengambil data relasi untuk koleksi:
   ```rust
   let posts = Post::query(req.db()).with("user").get().await?;
   ```
5. **Model Factory & Faker (Testing & Seeding)**:
   Gunakan Model Factory (`lumina make:factory <Model>Factory`) untuk keperluan seeding data dan unit testing. Manfaatkan generator `Faker` untuk membangkitkan data realistis Indonesia (nama, telepon, email, alamat, rupiah) dengan method chaining yang fluen:
   ```rust
   User::factory().count(50).state(|u| u.is_active = true).create(req.db()).await?;
   ```

---

## 🎨 4. Rules untuk View (Blade Engine — `resources/views/`)

1. **Pure Native Blade**:
   - File template bertipe `.blade.rs` dan disimpan di bawah folder `resources/views/`.
   - Hindari logika bisnis berat di dalam view. View hanya bertugas untuk presentasi data.
2. **Pewarisan Layout**:
   - Selalu gunakan `@extends('layouts.app')`, `@section('content')`, dan `@yield('content')`.
3. **Looping & Kontrol Alur**:
   - Gunakan `@foreach($items as $item)` dan manfaatkan objek `$loop` (`$loop.index`, `$loop.first`, `$loop.last`).
   - Gunakan `@if`, `@elseif`, `@else`, `@endif`.
   - Gunakan `@auth` / `@guest` untuk kontrol tampilan berbasis status login.
4. **Pipeline Filter**:
   - Manfaatkan operator pipa `|` untuk formatting: `{{ $nominal | rupiah }}`, `{{ $title | slug | uppercase }}`.
5. **Sandbox & Script Integration**:
   - Delegasikan pemrosesan plugin pihak ketiga dengan `@wasm('plugin', 'func', $arg)`.
   - Delegasikan logika skrip dinamis dengan `@eval_is('skrip.is', 'func', $arg)`.
6. **Form Security**:
   - Setiap form HTML ber-method `POST` **WAJIB** menyertakan direktif `@csrf`.
   - Gunakan `@method('PUT')` atau `@method('DELETE')` untuk spoofing HTTP method.

---

## ⚙️ 5. Rules untuk Service Layer (`src/app/services/`)

1. **Pemisahan Logika Bisnis**:
   - Tempatkan proses kalkulasi kompleks, pemanggilan API eksternal pihak ketiga, orkestrasi multi-model, atau transaksi bisnis di dalam struct Service.
2. **Desain Service**:
   - Service harus berorientasi domain, asinkron (`async fn`), dan mengembalikan `Result<T, AppError>`.
   - Contoh:
     ```rust
     pub struct OrderService;

     impl OrderService {
         pub async fn checkout(db: &DatabasePool, user_id: i64, cart: Cart) -> Result<Order, AppError> {
             // 1. Validasi stok barang
             // 2. Buat transaksi pesanan
             // 3. Kurangi stok
             // 4. Return Order
         }
     }
     ```

---

## 📦 6. Rules untuk Repository Layer (`src/app/repositories/`)

1. **Abstraksi Data Terstruktur**:
   - Gunakan Repository ketika sebuah modul membutuhkan query SQL kustom yang rumit, agregasi data analitik berskala besar, atau caching bertingkat.
2. **Koneksi Cache Cerdas**:
   - Integrasikan Moka Cache di level repository untuk query-query berat yang sering dibaca namun jarang berubah.

---

## 🚫 7. Larangan Keras (Strict Prohibitions)

1. ❌ **DILARANG** menggunakan `unwrap()` atau `expect()` secara sembarangan di handler yang dapat diakses publik — selalu tangani error dengan operator `?` atau konversikan ke `AppError`.
2. ❌ **DILARANG** menulis query mentah SQL yang rentan SQL Injection — selalu gunakan query builder bawaan atau prepared statement parametrik (`$1`, `$2` / `?`).
3. ❌ **DILARANG** mengembalikan boilerplate Axum mentah di Controller pengguna baru. Selalu tunjukkan pola ergonomis `Request` dan `IntoResponse` / `LuminaResult`.
4. ❌ **DILARANG** memasukkan file sementara, log, catatan kerja internal, atau laporan scratch ke git publik — letakkan hanya di `docs/history_pekerjaan/` yang terabaikan oleh `.gitignore`.

---

*Lumina Framework: Keringat performa Rust di balik keindahan dan kemudahan Laravel.*
