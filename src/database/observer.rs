use super::connection::DatabasePool;
use async_trait::async_trait;

/// Trait untuk siklus hidup model (Model Lifecycle Events)
///
/// Memungkinkan pengeksekusian logika otomatis (side-effects) sebelum atau sesudah
/// data disimpan (INSERT/UPDATE) atau dihapus (DELETE).
///
/// # Contoh Penggunaan:
/// ```rust,ignore
/// #[async_trait::async_trait]
/// impl ModelLifecycle for Article {
///     async fn creating(&mut self, pool: &DatabasePool) -> Result<(), sqlx::Error> {
///         self.slug = AutoSlug::generate(pool, "articles", &self.title, None).await?;
///         Ok(())
///     }
/// }
/// ```
#[async_trait]
pub trait ModelLifecycle: Sized + Send + Sync {
    /// Berjalan sebelum data disimpan (baik operasi INSERT maupun UPDATE).
    async fn saving(&mut self, _pool: &DatabasePool) -> Result<(), sqlx::Error> {
        Ok(())
    }

    /// Berjalan setelah data berhasil disimpan ke database.
    async fn saved(&self, _pool: &DatabasePool) -> Result<(), sqlx::Error> {
        Ok(())
    }

    /// Berjalan sebelum data baru di-insert ke database (id == 0).
    async fn creating(&mut self, _pool: &DatabasePool) -> Result<(), sqlx::Error> {
        Ok(())
    }

    /// Berjalan setelah data baru berhasil di-insert ke database.
    async fn created(&self, _pool: &DatabasePool) -> Result<(), sqlx::Error> {
        Ok(())
    }

    /// Berjalan sebelum data lama diperbarui (id > 0).
    async fn updating(&mut self, _pool: &DatabasePool) -> Result<(), sqlx::Error> {
        Ok(())
    }

    /// Berjalan setelah data lama berhasil diperbarui di database.
    async fn updated(&self, _pool: &DatabasePool) -> Result<(), sqlx::Error> {
        Ok(())
    }

    /// Berjalan sebelum data dihapus (baik soft-delete maupun hard-delete).
    async fn deleting(&self, _pool: &DatabasePool) -> Result<(), sqlx::Error> {
        Ok(())
    }

    /// Berjalan setelah data berhasil dihapus dari database.
    async fn deleted(&self, _pool: &DatabasePool) -> Result<(), sqlx::Error> {
        Ok(())
    }
}

/// Trait untuk Observer mandiri yang mengamati model dari class terpisah (Separation of Concerns).
///
/// Mirip dengan `UserObserver` pada Laravel Eloquent.
#[async_trait]
pub trait Observer<T>: Send + Sync {
    async fn saving(&self, _model: &mut T, _pool: &DatabasePool) -> Result<(), sqlx::Error> {
        Ok(())
    }

    async fn saved(&self, _model: &T, _pool: &DatabasePool) -> Result<(), sqlx::Error> {
        Ok(())
    }

    async fn creating(&self, _model: &mut T, _pool: &DatabasePool) -> Result<(), sqlx::Error> {
        Ok(())
    }

    async fn created(&self, _model: &T, _pool: &DatabasePool) -> Result<(), sqlx::Error> {
        Ok(())
    }

    async fn updating(&self, _model: &mut T, _pool: &DatabasePool) -> Result<(), sqlx::Error> {
        Ok(())
    }

    async fn updated(&self, _model: &T, _pool: &DatabasePool) -> Result<(), sqlx::Error> {
        Ok(())
    }

    async fn deleting(&self, _model: &T, _pool: &DatabasePool) -> Result<(), sqlx::Error> {
        Ok(())
    }

    async fn deleted(&self, _model: &T, _pool: &DatabasePool) -> Result<(), sqlx::Error> {
        Ok(())
    }
}

// =========================================================================
// 🧰 Helper Bawaan: AutoSlug (SEO Friendly URL Slug Generator)
// =========================================================================

pub struct AutoSlug;

impl AutoSlug {
    /// Ubah teks biasa menjadi format slug URL (lowercase, hyphen-separated, bersih dari simbol)
    ///
    /// Contoh: `"Promo Diskon 50%! (Spesial 2026)"` ➔ `"promo-diskon-50-spesial-2026"`
    pub fn slugify(text: &str) -> String {
        let mut slug = String::with_capacity(text.len());
        let mut prev_is_dash = false;

        for c in text.chars() {
            if c.is_alphanumeric() {
                slug.push(c.to_ascii_lowercase());
                prev_is_dash = false;
            } else if (c == ' ' || c == '-' || c == '_' || c == '/') && !prev_is_dash && !slug.is_empty() {
                slug.push('-');
                prev_is_dash = true;
            }
        }

        // Hapus tanda strip di akhir jika ada
        if slug.ends_with('-') {
            slug.pop();
        }

        if slug.is_empty() {
            "n-a".to_string()
        } else {
            slug
        }
    }

    /// Hasilkan slug unik dengan memeriksa benturan data di database.
    /// Jika slug sudah dipakai oleh baris lain, tambahkan akhiran `-1`, `-2`, dst.
    pub async fn generate(
        pool: &DatabasePool,
        table: &str,
        source_text: &str,
        exclude_id: Option<i64>,
    ) -> Result<String, sqlx::Error> {
        let base_slug = Self::slugify(source_text);
        let mut candidate = base_slug.clone();
        let mut counter = 1;

        loop {
            let (query, count) = if let Some(id) = exclude_id {
                let sql = format!("SELECT COUNT(*) as count FROM {} WHERE slug = ? AND id != ?", table);
                let count: i64 = sqlx::query_scalar(&pool.prepare_sql(&sql))
                    .bind(&candidate)
                    .bind(id)
                    .fetch_one(&pool.pool)
                    .await?;
                (sql, count)
            } else {
                let sql = format!("SELECT COUNT(*) as count FROM {} WHERE slug = ?", table);
                let count: i64 = sqlx::query_scalar(&pool.prepare_sql(&sql))
                    .bind(&candidate)
                    .fetch_one(&pool.pool)
                    .await?;
                (sql, count)
            };
            let _ = query;

            if count == 0 {
                return Ok(candidate);
            }

            candidate = format!("{}-{}", base_slug, counter);
            counter += 1;
        }
    }
}

// =========================================================================
// 🔐 Helper Bawaan: AutoHash (Bcrypt Password Hashing)
// =========================================================================

pub struct AutoHash;

impl AutoHash {
    /// Cek apakah sebuah string sudah merupakan hash Bcrypt
    pub fn is_hashed(val: &str) -> bool {
        val.starts_with("$2a$") || val.starts_with("$2b$") || val.starts_with("$2y$")
    }

    /// Hash password jika belum berupa format Bcrypt hash
    pub fn hash(plain_or_hashed: &str) -> Result<String, bcrypt::BcryptError> {
        if Self::is_hashed(plain_or_hashed) {
            Ok(plain_or_hashed.to_string())
        } else {
            bcrypt::hash(plain_or_hashed, 10)
        }
    }
}
