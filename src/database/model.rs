use super::connection::DatabasePool;
use super::orm::QueryBuilder;
use async_trait::async_trait;

/// Trait yang wajib diimplementasikan oleh setiap Model.
///
/// Menyediakan interface CRUD async yang seragam.
/// Implementasi detail query ada di masing-masing struct model.
///
/// # Contoh
/// ```rust,ignore
/// let user = User::find(&pool, 1).await?;
/// let all  = User::all(&pool).await?;
/// ```
#[async_trait]
pub trait Model:
    Sized + Send + Sync + for<'r> sqlx::FromRow<'r, sqlx::any::AnyRow> + Unpin
{
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

    /// Shortcut query WHERE column = value
    fn where_eq<'a, V>(pool: &'a DatabasePool, column: &'a str, value: V) -> QueryBuilder<'a, Self>
    where
        V: 'a + Send + sqlx::Encode<'a, sqlx::Any> + sqlx::Type<sqlx::Any>,
    {
        Self::query(pool).where_eq(column, value)
    }

    /// Shortcut query WHERE column LIKE pattern
    fn where_like<'a, V>(pool: &'a DatabasePool, column: &'a str, pattern: V) -> QueryBuilder<'a, Self>
    where
        V: 'a + Send + sqlx::Encode<'a, sqlx::Any> + sqlx::Type<sqlx::Any>,
    {
        Self::query(pool).where_like(column, pattern)
    }

    /// Shortcut query WHERE column IS NULL
    fn where_null<'a>(pool: &'a DatabasePool, column: &'a str) -> QueryBuilder<'a, Self> {
        Self::query(pool).where_null(column)
    }

    /// Shortcut query WHERE column IS NOT NULL
    fn where_not_null<'a>(pool: &'a DatabasePool, column: &'a str) -> QueryBuilder<'a, Self> {
        Self::query(pool).where_not_null(column)
    }

    /// Shortcut ORDER BY created_at DESC
    fn latest<'a>(pool: &'a DatabasePool) -> QueryBuilder<'a, Self> {
        Self::query(pool).latest()
    }

    /// Shortcut ORDER BY created_at ASC
    fn oldest<'a>(pool: &'a DatabasePool) -> QueryBuilder<'a, Self> {
        Self::query(pool).oldest()
    }

    /// Shortcut query WHERE EXISTS relasi (Laravel: Model::whereHas)
    fn where_has<'a>(
        pool: &'a DatabasePool,
        relation_table: &'a str,
        foreign_key: &'a str,
        local_key: &'a str,
    ) -> QueryBuilder<'a, Self> {
        Self::query(pool).where_has(relation_table, foreign_key, local_key)
    }

    /// Shortcut query WHERE NOT EXISTS relasi (Laravel: Model::whereDoesntHave)
    fn where_doesnt_have<'a>(
        pool: &'a DatabasePool,
        relation_table: &'a str,
        foreign_key: &'a str,
        local_key: &'a str,
    ) -> QueryBuilder<'a, Self> {
        Self::query(pool).where_doesnt_have(relation_table, foreign_key, local_key)
    }

    /// Hitung total record aktif di database
    async fn count(pool: &DatabasePool) -> Result<i64, sqlx::Error> {
        Self::query(pool).count().await
    }

    /// Ambil data terpaginasi langsung dari Model
    async fn paginate(
        pool: &DatabasePool,
        page: usize,
        per_page: usize,
    ) -> Result<super::orm::LengthAwarePaginator<Self>, sqlx::Error> {
        Self::query(pool).paginate(page, per_page).await
    }

    /// Hapus record berdasarkan ID (alias delete ala Laravel Destroy)
    async fn destroy(pool: &DatabasePool, id: i64) -> Result<bool, sqlx::Error> {
        Self::delete(pool, id).await
    }

    /// Cek apakah terdapat record yang memenuhi kriteria
    async fn exists(pool: &DatabasePool) -> Result<bool, sqlx::Error> {
        Self::query(pool).exists().await
    }

    /// Ambil record berdasarkan ID atau return Error.
    async fn find_or_fail(pool: &DatabasePool, id: i64) -> Result<Self, sqlx::Error> {
        Self::find(pool, id).await
    }

    /// Cari record berdasarkan kriteria, atau buat baru jika tidak ditemukan.
    async fn first_or_create<K, V>(
        pool: &DatabasePool,
        attributes: impl IntoIterator<Item = (K, V)> + Send,
        values: impl IntoIterator<Item = (K, V)> + Send,
    ) -> Result<Self, sqlx::Error>
    where
        K: AsRef<str>,
        V: ToString,
    {
        let attr_vec: Vec<(String, String)> = attributes
            .into_iter()
            .map(|(k, v)| (k.as_ref().to_string(), v.to_string()))
            .collect();

        let val_vec: Vec<(String, String)> = values
            .into_iter()
            .map(|(k, v)| (k.as_ref().to_string(), v.to_string()))
            .collect();

        let mut builder = Self::query(pool);
        for (k, v) in &attr_vec {
            builder = builder.where_eq_str(k, v);
        }

        if let Some(item) = builder.first().await? {
            return Ok(item);
        }

        let mut create_map: Vec<(String, String)> = attr_vec;
        for (key, val) in val_vec {
            if !create_map.iter().any(|(existing_k, _)| existing_k == &key) {
                create_map.push((key, val));
            }
        }

        Self::create(pool, create_map).await
    }

    /// Update record jika ditemukan berdasarkan atribut, atau buat baru jika tidak ada (Laravel: updateOrCreate).
    async fn update_or_create<K, V>(
        pool: &DatabasePool,
        attributes: impl IntoIterator<Item = (K, V)> + Send,
        values: impl IntoIterator<Item = (K, V)> + Send,
    ) -> Result<Self, sqlx::Error>
    where
        K: AsRef<str>,
        V: ToString,
    {
        let attr_vec: Vec<(String, String)> = attributes
            .into_iter()
            .map(|(k, v)| (k.as_ref().to_string(), v.to_string()))
            .collect();

        let val_vec: Vec<(String, String)> = values
            .into_iter()
            .map(|(k, v)| (k.as_ref().to_string(), v.to_string()))
            .collect();

        let mut builder = Self::query(pool);
        for (k, v) in &attr_vec {
            builder = builder.where_eq_str(k, v);
        }

        if let Some(_item) = builder.first().await? {
            if !val_vec.is_empty() {
                let mut update_builder = Self::query(pool);
                for (k, v) in &attr_vec {
                    update_builder = update_builder.where_eq_str(k, v);
                }
                update_builder.update(val_vec).await?;
            }

            let mut refetch = Self::query(pool);
            for (k, v) in &attr_vec {
                refetch = refetch.where_eq_str(k, v);
            }
            return refetch.first_or_fail().await;
        }

        let mut create_map: Vec<(String, String)> = attr_vec;
        for (key, val) in val_vec {
            if !create_map.iter().any(|(existing_k, _)| existing_k == &key) {
                create_map.push((key, val));
            }
        }

        Self::create(pool, create_map).await
    }

    /// Ambil sekumpulan record berdasarkan array of primary key IDs (Laravel: findMany).
    async fn find_many(pool: &DatabasePool, ids: &[i64]) -> Result<Vec<Self>, sqlx::Error> {
        if ids.is_empty() {
            return Ok(Vec::new());
        }
        Self::query(pool)
            .where_in("id", ids.to_vec())
            .get()
            .await
    }

    /// Pulihkan record yang telah di-soft-delete (Laravel: restore)
    async fn restore(pool: &DatabasePool, id: i64) -> Result<bool, sqlx::Error> {
        let sql = format!("UPDATE {} SET deleted_at = NULL WHERE id = ?", Self::TABLE);
        let sql = pool.prepare_sql(&sql);
        let res = sqlx::query(&sql).bind(id).execute(&pool.pool).await?;
        Ok(res.rows_affected() > 0)
    }

    /// Hapus record secara permanen (Hard Delete) dari database (Laravel: forceDelete)
    async fn force_delete(pool: &DatabasePool, id: i64) -> Result<bool, sqlx::Error> {
        let sql = format!("DELETE FROM {} WHERE id = ?", Self::TABLE);
        let sql = pool.prepare_sql(&sql);
        let res = sqlx::query(&sql).bind(id).execute(&pool.pool).await?;
        Ok(res.rows_affected() > 0)
    }

    /// Kosongkan seluruh data tabel model (Laravel: truncate)
    async fn truncate(pool: &DatabasePool) -> Result<(), sqlx::Error> {
        let sql = format!("DELETE FROM {}", Self::TABLE);
        sqlx::query(&sql).execute(&pool.pool).await?;
        Ok(())
    }

    /// Shortcut query yang menyertakan data yang sudah di-soft-delete (Laravel: withTrashed)
    fn with_trashed<'a>(pool: &'a DatabasePool) -> QueryBuilder<'a, Self> {
        Self::query(pool).with_trashed()
    }

    /// Shortcut query yang HANYA mengambil data yang telah di-soft-delete (Laravel: onlyTrashed)
    fn only_trashed<'a>(pool: &'a DatabasePool) -> QueryBuilder<'a, Self> {
        Self::query(pool).only_trashed()
    }

    /// Buat record baru dari key-value pairs secara dinamis.
    async fn create<K, V>(
        pool: &DatabasePool,
        data: impl IntoIterator<Item = (K, V)> + Send,
    ) -> Result<Self, sqlx::Error>
    where
        K: AsRef<str>,
        V: ToString,
    {
        let pairs: Vec<(String, String)> = data
            .into_iter()
            .map(|(k, v)| (k.as_ref().to_string(), v.to_string()))
            .collect();

        if pairs.is_empty() {
            return Err(sqlx::Error::Decode("No fields provided for create".into()));
        }

        let keys: Vec<String> = pairs.iter().map(|p| p.0.clone()).collect();
        let placeholders = vec!["?"; keys.len()].join(", ");
        let sql = format!(
            "INSERT INTO {} ({}) VALUES ({})",
            Self::TABLE,
            keys.join(", "),
            placeholders
        );

        let mut query = sqlx::query(&sql);
        for pair in &pairs {
            query = query.bind(&pair.1);
        }

        let res = query.execute(&pool.pool).await?;
        let inserted_id = res.last_insert_id().unwrap_or(0);

        if inserted_id != 0 {
            if let Ok(item) = Self::find(pool, inserted_id as i64).await {
                return Ok(item);
            }
        }

        // Fallback jika driver tidak mengembalikan last_insert_id (misal pada beberapa mode sqlite connection/any)
        let last_item = Self::query(pool).order_by("id", "DESC").first_or_fail().await?;
        Ok(last_item)
    }

    async fn has_many<R>(
        pool: &DatabasePool,
        foreign_key: &str,
        local_id: i64,
    ) -> Result<Vec<R>, sqlx::Error>
    where
        R: Model,
    {
        R::query(pool)
            .filter(foreign_key, "=", local_id)
            .get()
            .await
    }

    /// Definisi relasi Has-One.
    /// Contoh: `user.has_one::<Profile>(db, "user_id", user.id).await`
    async fn has_one<R>(
        pool: &DatabasePool,
        foreign_key: &str,
        local_id: i64,
    ) -> Result<R, sqlx::Error>
    where
        R: Model,
    {
        R::query(pool)
            .filter(foreign_key, "=", local_id)
            .first_or_fail()
            .await
    }

    async fn belongs_to<R>(pool: &DatabasePool, foreign_key_id: i64) -> Result<R, sqlx::Error>
    where
        R: Model,
    {
        R::find(pool, foreign_key_id).await
    }

    /// Definisi relasi Many-To-Many.
    /// Contoh: `post.belongs_to_many::<Tag>(db, "post_tag", "post_id", "tag_id", post.id).await`
    async fn belongs_to_many<R>(
        pool: &DatabasePool,
        pivot_table: &str,
        foreign_key: &str,
        related_key: &str,
        local_id: i64,
    ) -> Result<Vec<R>, sqlx::Error>
    where
        R: Model,
    {
        // SELECT tags.* FROM tags JOIN post_tag p ON tags.id = p.tag_id WHERE p.post_id = ?
        R::query(pool)
            .select(&format!("{}.*", R::TABLE))
            .join(&format!(
                "JOIN {} p ON {}.id = p.{}",
                pivot_table,
                R::TABLE,
                related_key
            ))
            .filter(&format!("p.{}", foreign_key), "=", local_id)
            .get()
            .await
    }

    /// Eager Load relasi untuk sekumpulan model (N+1 Solution).
    /// Di-override oleh model untuk mendukung .with("relation")
    async fn eager_load(
        _relation: &str,
        _items: &mut [Self],
        _pool: &DatabasePool,
    ) -> Result<(), sqlx::Error> {
        Ok(())
    }
}

// ─────────────────────────────────────────────────────────────────────────────
//  HUMAN ELOQUENT RELATIONSHIP MACROS (Laravel-Style Declarations)
// ─────────────────────────────────────────────────────────────────────────────

/// Macro deklarasi relasi Belongs To (N to 1) bergaya Laravel.
///
/// # Contoh Penggunaan:
/// ```rust,ignore
/// impl Post {
///     // Sintaks 1 baris manusiawi:
///     belongs_to!(author: User, user_id);
/// }
/// ```
#[macro_export]
macro_rules! belongs_to {
    ($fn_name:ident : $related:ident, $fk:ident) => {
        pub async fn $fn_name(&self, db: &$crate::database::connection::DatabasePool) -> $crate::database::orm::OrmResult<$related> {
            self.belongs_to(db, self.$fk).await
        }
    };
    ($fn_name:ident : $related:ident) => {
        pub async fn $fn_name(&self, db: &$crate::database::connection::DatabasePool) -> $crate::database::orm::OrmResult<$related> {
            self.belongs_to(db, self.user_id).await
        }
    };
}

/// Macro deklarasi relasi Has Many (1 to N) bergaya Laravel.
///
/// # Contoh Penggunaan:
/// ```rust,ignore
/// impl User {
///     // Sintaks 1 baris manusiawi:
///     has_many!(posts: Post, "user_id");
///     // Atau otomatis foreign key "user_id":
///     has_many!(posts: Post);
/// }
/// ```
#[macro_export]
macro_rules! has_many {
    ($fn_name:ident : $related:ident, $fk:expr) => {
        pub async fn $fn_name(&self, db: &$crate::database::connection::DatabasePool) -> $crate::database::orm::OrmResult<Vec<$related>> {
            self.has_many(db, $fk).await
        }
    };
    ($fn_name:ident : $related:ident) => {
        pub async fn $fn_name(&self, db: &$crate::database::connection::DatabasePool) -> $crate::database::orm::OrmResult<Vec<$related>> {
            let fk = format!("{}_id", Self::TABLE.trim_end_matches('s'));
            self.has_many(db, &fk).await
        }
    };
}

/// Macro deklarasi relasi Has One (1 to 1) bergaya Laravel.
///
/// # Contoh Penggunaan:
/// ```rust,ignore
/// impl User {
///     has_one!(profile: Profile, "user_id");
///     // Atau otomatis:
///     has_one!(profile: Profile);
/// }
/// ```
#[macro_export]
macro_rules! has_one {
    ($fn_name:ident : $related:ident, $fk:expr) => {
        pub async fn $fn_name(&self, db: &$crate::database::connection::DatabasePool) -> $crate::database::orm::OrmResult<$related> {
            self.has_one(db, $fk).await
        }
    };
    ($fn_name:ident : $related:ident) => {
        pub async fn $fn_name(&self, db: &$crate::database::connection::DatabasePool) -> $crate::database::orm::OrmResult<$related> {
            let fk = format!("{}_id", Self::TABLE.trim_end_matches('s'));
            self.has_one(db, &fk).await
        }
    };
}

/// Macro deklarasi relasi Belongs To Many (Many to Many / Pivot Table) bergaya Laravel.
///
/// # Contoh Penggunaan:
/// ```rust,ignore
/// impl Post {
///     belongs_to_many!(tags: Tag, "post_tag", "post_id", "tag_id");
///     // Atau otomatis foreign key & related key:
///     belongs_to_many!(tags: Tag, "post_tag");
/// }
/// ```
#[macro_export]
macro_rules! belongs_to_many {
    ($fn_name:ident : $related:ident, $pivot:expr, $fk:expr, $rk:expr) => {
        pub async fn $fn_name(&self, db: &$crate::database::connection::DatabasePool) -> $crate::database::orm::OrmResult<Vec<$related>> {
            self.belongs_to_many(db, $pivot, $fk, $rk).await
        }
    };
    ($fn_name:ident : $related:ident, $pivot:expr) => {
        pub async fn $fn_name(&self, db: &$crate::database::connection::DatabasePool) -> $crate::database::orm::OrmResult<Vec<$related>> {
            let fk = format!("{}_id", Self::TABLE.trim_end_matches('s'));
            let rk = format!("{}_id", <$related as $crate::database::model::Model>::TABLE.trim_end_matches('s'));
            self.belongs_to_many(db, $pivot, &fk, &rk).await
        }
    };
}
