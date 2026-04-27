use async_trait::async_trait;
use super::connection::DatabasePool;

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
pub trait Model: Sized + Send + Sync {
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
}
