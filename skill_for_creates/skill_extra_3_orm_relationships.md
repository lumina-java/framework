# Skill Extra 3: ORM Relationships (BelongsTo & HasMany)

## 🎯 Objektif
Mengubah Lumina ORM dari sekadar *Query Builder* sederhana menjadi sebuah sistem relasional yang mempermudah pengambilan data terkait (Relationship), layaknya Eloquent di Laravel.

## 📋 Deskripsi Tugas
Saat ini, jika kita memiliki tabel `users` dan `posts`, mengambil `posts` milik seorang `user` membutuhkan query manual baru:
```rust
let posts = Post::query(db.clone())
    .filter("user_id", "=", user.id)
    .get().await?;
```
Sintaks yang kita harapkan:
```rust
let posts = user.posts(db.clone()).await?;
```
Kita butuh memperluas trait `Model` atau membuat trait spesifik (`HasMany`, `BelongsTo`) yang memungkinkan kita untuk mendefinisikan hubungan antar tabel secara elegan.

## 🛠️ Langkah Eksekusi (Untuk Junior Programmer / AI)

### Langkah 1: Buat Modul Relasi ORM
Di dalam `src/database/orm.rs` atau `src/database/relations.rs`, definisikan dua struktur/fungsi baru: `HasMany` dan `BelongsTo`.

### Langkah 2: Implementasi `HasMany`
1. Definisikan metode helper (misal lewat trait `HasMany<T>`) yang akan otomatis menginstansiasi `QueryBuilder` dari model target (`T`).
2. Metode ini harus otomatis melakukan filter `.filter("foreign_key", "=", self.id)`.
3. Dalam penerapannya, model `User` akan menambahkan method:
   ```rust
   impl User {
       pub async fn posts(&self, db: sqlx::MySqlPool) -> Result<Vec<Post>, sqlx::Error> {
           Post::query(db)
               .filter("user_id", "=", self.id)
               .get().await
       }
   }
   ```
   *Catatan: Eksekusi pertama bisa difokuskan pada manual helper method di dalam implementasi Model untuk mempercepat development, sebelum diarahkan ke Macro murni.*

### Langkah 3: Implementasi `BelongsTo`
1. Kebalikan dari `HasMany`, `BelongsTo` akan mengambil 1 (satu) entri dari model induk.
2. Contoh di model `Post`:
   ```rust
   impl Post {
       pub async fn user(&self, db: sqlx::MySqlPool) -> Result<User, sqlx::Error> {
           User::query(db)
               .filter("id", "=", self.user_id)
               .first().await
       }
   }
   ```

### Langkah 4: Eager Loading (Advance - Opsional)
Jika eksekusi langkah 2 dan 3 sudah berjalan, buat fitur `with("relation")` pada `QueryBuilder` untuk mengatasi masalah *N+1 Query*. Fitur ini akan mengambil id induk dan menjalankan *satu query* `WHERE IN (ids)` ke tabel relasi, lalu memetakan hasilnya ke setiap objek induk di memori.

## ✅ Kriteria Penerimaan
- Minimal 2 relasi bisa dibuat pada model apapun dan terbukti berfungsi.
- Relasi tersebut dapat dipanggil dengan gaya metode elegan: `model.relasi(db).await?`.
- Mampu mereduksi penulisan *Query Builder* di layer Controller saat berurusan dengan data berelasi.
