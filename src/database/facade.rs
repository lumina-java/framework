use crate::database::connection::DatabasePool;
use crate::database::transaction::Transaction;

/// Facade `DB` bergaya Laravel untuk akses database dan transaksi cepat tanpa repot.
pub struct DB;

impl DB {
    /// Mulai transaksi database baru (ACID Transaction) menggunakan pool yang diberikan.
    /// Contoh: `let tx = DB::begin(req.db()).await?;`
    pub async fn begin(pool: &DatabasePool) -> Result<Transaction, sqlx::Error> {
        pool.begin().await
    }

    /// Alias untuk `begin()`
    pub async fn begin_transaction(pool: &DatabasePool) -> Result<Transaction, sqlx::Error> {
        pool.begin_transaction().await
    }

    /// Jalankan blok transaksi otomatis (Commit jika Ok, Rollback otomatis jika Err).
    pub async fn transaction<F, Fut, R, E>(pool: &DatabasePool, callback: F) -> Result<R, E>
    where
        F: FnOnce(Transaction) -> Fut,
        Fut: std::future::Future<Output = Result<R, E>>,
        E: From<sqlx::Error>,
    {
        let tx = pool.begin().await?;
        let res = callback(tx.clone()).await;
        match res {
            Ok(val) => {
                tx.commit().await?;
                Ok(val)
            }
            Err(err) => {
                let _ = tx.rollback().await;
                Err(err)
            }
        }
    }
}

