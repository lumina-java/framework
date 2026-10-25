use crate::core::error::AppError;
use crate::database::connection::DatabasePool;
use crate::database::model::Model;
use async_trait::async_trait;
use std::marker::PhantomData;

/// Trait Factory — Pondasi pembuatan data mock/dummy untuk pengujian dan seeding ala Laravel.
#[async_trait]
pub trait Factory<M: Model + Send + Sync + 'static>: Send + Sync + Sized {
    /// Definisikan nilai atribut default untuk sebuah instance model
    fn definition(&self) -> M;

    /// Buat 1 instance model in-memory tanpa menyimpannya ke database
    fn make_one(&self) -> M {
        self.definition()
    }

    /// Menentukan jumlah record yang akan dibuat (fluent method chaining)
    fn count(self, count: usize) -> FactoryBuilder<M, Self> {
        FactoryBuilder::new(self, count)
    }

    /// Menambahkan state modifier untuk mengubah nilai field tertentu
    fn state<F>(self, modifier: F) -> FactoryBuilder<M, Self>
    where
        F: Fn(&mut M) + Send + Sync + 'static,
    {
        let mut builder = FactoryBuilder::new(self, 1);
        builder.add_state(modifier);
        builder
    }

    /// Menyimpan 1 instance model langsung ke database dan memuat ulang record dari DB
    async fn create_one(&self, db: &DatabasePool) -> Result<M, AppError> {
        let item = self.definition();
        let id = item.save(db).await.map_err(AppError::DatabaseError)?;
        let reloaded = M::find(db, id).await.map_err(AppError::DatabaseError)?;
        Ok(reloaded)
    }

    /// Menyimpan sejumlah record langsung ke database
    async fn create_many(&self, db: &DatabasePool, count: usize) -> Result<Vec<M>, AppError> {
        let mut results = Vec::with_capacity(count);
        for _ in 0..count {
            results.push(self.create_one(db).await?);
        }
        Ok(results)
    }
}

/// FactoryBuilder — Mengelola modifier state, perulangan, dan eksekusi penyimpanan ke database.
pub struct FactoryBuilder<M, F> {
    factory: F,
    count: usize,
    states: Vec<Box<dyn Fn(&mut M) + Send + Sync>>,
    _phantom: PhantomData<M>,
}

impl<M: Model + Send + Sync + 'static, F: Factory<M>> FactoryBuilder<M, F> {
    pub fn new(factory: F, count: usize) -> Self {
        Self {
            factory,
            count: count.max(1),
            states: Vec::new(),
            _phantom: PhantomData,
        }
    }

    /// Mengubah jumlah record yang ingin dihasilkan
    pub fn count(mut self, count: usize) -> Self {
        self.count = count.max(1);
        self
    }

    /// Menambahkan modifikasi state (misal: role = "admin", is_active = true)
    pub fn state<S>(mut self, modifier: S) -> Self
    where
        S: Fn(&mut M) + Send + Sync + 'static,
    {
        self.states.push(Box::new(modifier));
        self
    }

    pub fn add_state<S>(&mut self, modifier: S)
    where
        S: Fn(&mut M) + Send + Sync + 'static,
    {
        self.states.push(Box::new(modifier));
    }

    /// Menghasilkan 1 objek model in-memory dengan menerapkan seluruh state modifier
    pub fn make_one(&self) -> M {
        let mut item = self.factory.definition();
        for state in &self.states {
            state(&mut item);
        }
        item
    }

    /// Menghasilkan kumpulan objek model in-memory (tanpa menyentuh database)
    pub fn make(&self) -> Vec<M> {
        (0..self.count)
            .map(|_| self.make_one())
            .collect()
    }

    /// Menyimpan 1 record ke database dan mengembalikan model yang sudah memiliki ID
    pub async fn create_one(&self, db: &DatabasePool) -> Result<M, AppError> {
        let item = self.make_one();
        let id = item.save(db).await.map_err(AppError::DatabaseError)?;
        let reloaded = M::find(db, id).await.map_err(AppError::DatabaseError)?;
        Ok(reloaded)
    }

    /// Menyimpan seluruh record ke database secara asinkron
    pub async fn create(&self, db: &DatabasePool) -> Result<Vec<M>, AppError> {
        let mut results = Vec::with_capacity(self.count);
        for _ in 0..self.count {
            results.push(self.create_one(db).await?);
        }
        Ok(results)
    }
}

/// Trait opsional agar Model dapat langsung memanggil method `.factory()`
pub trait HasFactory<F>: Sized {
    fn factory() -> F;
}
