use crate::database::connection::DatabasePool;
use async_trait::async_trait;

/// Trait dasar untuk semua Factory di Lumina.
/// Factory digunakan untuk men-generate data dummy untuk testing atau seeding.
#[async_trait]
pub trait Factory: Send + Sync {
    type Model: Send + Sync;

    /// Definisikan bagaimana model harus diisi dengan data dummy.
    fn definition(&self) -> Self::Model;

    /// Buat instance model tanpa menyimpannya ke database.
    fn make(&self) -> Self::Model {
        self.definition()
    }

    /// Buat banyak instance model tanpa menyimpannya ke database.
    fn make_many(&self, count: usize) -> Vec<Self::Model> {
        (0..count).map(|_| self.make()).collect()
    }

    /// Buat instance model dan simpan ke database.
    async fn create(&self, db: &DatabasePool) -> Result<Self::Model, String>;

    /// Buat banyak instance model dan simpan ke database.
    async fn create_many(
        &self,
        db: &DatabasePool,
        count: usize,
    ) -> Result<Vec<Self::Model>, String> {
        let mut models = Vec::new();
        for _ in 0..count {
            models.push(self.create(db).await?);
        }
        Ok(models)
    }
}
