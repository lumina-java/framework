use crate::database::connection::DatabasePool;
use async_trait::async_trait;

/// Trait yang harus diimplementasikan oleh setiap seeder.
#[async_trait]
pub trait Seeder: Send + Sync {
    /// Jalankan proses pengisian data.
    async fn run(&self, db: &DatabasePool) -> Result<(), sqlx::Error>;
}

/// Orchestrator utama untuk menjalankan semua seeder.
pub struct DatabaseSeeder {
    seeders: Vec<Box<dyn Seeder>>,
}

impl DatabaseSeeder {
    pub fn new() -> Self {
        Self {
            seeders: Vec::new(),
        }
    }

    /// Tambahkan seeder ke daftar eksekusi.
    pub fn call<S: Seeder + 'static>(&mut self, seeder: S) {
        self.seeders.push(Box::new(seeder));
    }

    /// Jalankan semua seeder yang sudah terdaftar.
    pub async fn run_all(&self, db: &DatabasePool) -> Result<(), sqlx::Error> {
        println!("🌱 Seeding database...");
        for seeder in &self.seeders {
            seeder.run(db).await?;
        }
        println!("✅ Seeding completed.");
        Ok(())
    }
}
