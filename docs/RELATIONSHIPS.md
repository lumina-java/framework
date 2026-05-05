# 🔗 Database Relationships (ORM)

Lumina menyediakan sistem relasi yang elegan dan efisien, mendukung *Eager Loading* untuk menyelesaikan masalah N+1 query.

---

## 📑 Daftar Isi
1. [One To One (HasOne)](#one-to-one-hasone)
2. [One To Many (HasMany)](#one-to-many-hasmany)
3. [Belongs To](#belongs-to)
4. [Many To Many (BelongsToMany)](#many-to-many-belongstomany)
5. [Eager Loading (.with)](#eager-loading-with)

---

## One To One (HasOne)

Digunakan jika satu model memiliki tepat satu model lain.
Contoh: `User` has one `Profile`.

```rust
// Di dalam impl User
pub async fn profile(&self, pool: &DatabasePool) -> Result<Profile, sqlx::Error> {
    Self::has_one::<Profile>(pool, "user_id", self.id).await
}
```

---

## One To Many (HasMany)

Digunakan jika satu model memiliki banyak model lain.
Contoh: `User` has many `Post`.

```rust
// Di dalam impl User
pub async fn posts(&self, pool: &DatabasePool) -> Result<Vec<Post>, sqlx::Error> {
    Self::has_many::<Post>(pool, "user_id", self.id).await
}
```

---

## Belongs To

Kebalikan dari HasMany/HasOne.
Contoh: `Post` belongs to `User`.

```rust
// Di dalam impl Post
pub async fn author(&self, pool: &DatabasePool) -> Result<User, sqlx::Error> {
    Self::belongs_to::<User>(pool, self.user_id).await
}
```

---

## Many To Many (BelongsToMany)

Digunakan untuk relasi banyak-ke-banyak melalui tabel *pivot*.
Contoh: `Post` has many `Tag`, dan `Tag` has many `Post`.

**Tabel Pivot:** `post_tag` (post_id, tag_id)

```rust
// Di dalam impl Post
pub async fn tags(&self, pool: &DatabasePool) -> Result<Vec<Tag>, sqlx::Error> {
    Self::belongs_to_many::<Tag>(
        pool, 
        "post_tag", // Nama tabel pivot
        "post_id",  // FK di tabel pivot untuk model ini
        "tag_id",   // FK di tabel pivot untuk model relasi
        self.id
    ).await
}
```

---

## Eager Loading (.with)

Eager loading sangat penting untuk performa. Gunakan `.with("relasi")` untuk mengambil data relasi dalam sekecil mungkin query.

```rust
// Mengambil semua post beserta Author dan Tags-nya
// Hanya butuh 3 query total (1 posts, 1 authors, 1 tags)
let posts = Post::query(req.db())
    .with("author")
    .with("tags")
    .get()
    .await?;

for post in posts {
    println!("Judul: {}", post.title);
    println!("Penulis: {}", post.author.unwrap().name);
    for tag in post.tags.unwrap() {
        println!("Tag: {}", tag.name);
    }
}
```

### Implementasi Eager Load di Model

Agar `.with()` bekerja, Anda harus meng-override method `eager_load` di model Anda:

```rust
#[async_trait]
impl Model for Post {
    // ... find, all, save ...

    async fn eager_load(relation: &str, items: &mut [Self], pool: &DatabasePool) -> Result<(), sqlx::Error> {
        if relation == "author" {
            // Logika ambil User berdasarkan post.user_id
        } else if relation == "tags" {
            // Logika ambil Tags melalui tabel pivot post_tag
        }
        Ok(())
    }
}
```

---

> [!TIP]
> Selalu gunakan Eager Loading jika Anda akan melakukan iterasi (loop) pada data yang memiliki relasi untuk menghindari ribuan query ke database yang memperlambat aplikasi.
