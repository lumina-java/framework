use async_trait::async_trait;
use super::connection::DatabasePool;
use super::orm::QueryBuilder;

/// Trait yang wajib diimplementasikan oleh setiap Model.
///
/// Menyediakan interface CRUD async yang seragam.
/// Implementasi detail query ada di masing-masing struct model.
///
/// # Contoh
/// ```rust
/// let user = User::find(&pool, 1).await?;
/// let all  = User::all(&pool).await?;
/// ```
#[async_trait]
pub trait Model: Sized + Send + Sync + for<'r> sqlx::FromRow<'r, sqlx::any::AnyRow> + Unpin {
    /// Nama tabel di database (konstanta compile-time)
    const TABLE: &'static str;

    /// Ambil satu record berdasarkan primary key.
    /// Mengembalikan error jika tidak ditemukan.
    async fn find(pool: &DatabasePool, id: i64) -> Result<Self, sqlx::Error>;

    /// Ambil semua record yang aktif (bukan soft-deleted).
    async fn all(pool: &DatabasePool) -> Result<Vec<Self>, sqlx::Error>;

    /// Simpan record ini ke database.
    /// Mengembalikan `last_insert_rowid` jika sukses.
    async fn save(&self, pool: &DatabasePool) -> Result<i64, sqlx::Error>;

    /// Soft delete — set `deleted_at = NOW()` tanpa menghapus baris.
    async fn delete(pool: &DatabasePool, id: i64) -> Result<bool, sqlx::Error>;

    /// Inisialisasi QueryBuilder untuk model ini.
    fn query(pool: &DatabasePool) -> QueryBuilder<'_, Self> {
        QueryBuilder::new(pool, Self::TABLE)
    }

    async fn has_many<R>(pool: &DatabasePool, foreign_key: &str, local_id: i64) -> Result<Vec<R>, sqlx::Error> 
    where R: Model 
    {
        R::query(pool)
            .filter(foreign_key, "=", local_id)
            .get()
            .await
    }

    /// Definisi relasi Has-One.
    /// Contoh: `user.has_one::<Profile>(db, "user_id", user.id).await`
    async fn has_one<R>(pool: &DatabasePool, foreign_key: &str, local_id: i64) -> Result<R, sqlx::Error>
    where R: Model
    {
        R::query(pool)
            .filter(foreign_key, "=", local_id)
            .first()
            .await
    }

    async fn belongs_to<R>(pool: &DatabasePool, foreign_key_id: i64) -> Result<R, sqlx::Error>
    where R: Model
    {
        R::find(pool, foreign_key_id).await
    }

    /// Definisi relasi Many-To-Many.
    /// Contoh: `post.belongs_to_many::<Tag>(db, "post_tag", "post_id", "tag_id", post.id).await`
    async fn belongs_to_many<R>(pool: &DatabasePool, pivot_table: &str, foreign_key: &str, related_key: &str, local_id: i64) -> Result<Vec<R>, sqlx::Error>
    where R: Model
    {
        // SELECT tags.* FROM tags JOIN post_tag p ON tags.id = p.tag_id WHERE p.post_id = ?
        R::query(pool)
            .select(&format!("{}.*", R::TABLE))
            .join(&format!("JOIN {} p ON {}.id = p.{}", pivot_table, R::TABLE, related_key))
            .filter(&format!("p.{}", foreign_key), "=", local_id)
            .get()
            .await
    }

    /// Eager Load relasi untuk sekumpulan model (N+1 Solution).
    /// Di-override oleh model untuk mendukung .with("relation")
    async fn eager_load(_relation: &str, _items: &mut [Self], _pool: &DatabasePool) -> Result<(), sqlx::Error> {
        Ok(())
    }
}
