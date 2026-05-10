use async_trait::async_trait;
use crate::database::{connection::DatabasePool, seeder::Seeder, factory::Factory};
use crate::factories::user_factory::UserFactory;

pub struct UserSeeder;

#[async_trait]
impl Seeder for UserSeeder {
    async fn run(&self, db: &DatabasePool) -> Result<(), sqlx::Error> {
        // Buat 10 user menggunakan factory
        let factory = UserFactory;
        if let Err(e) = factory.create_many(db, 10).await {
            eprintln!("❌ Gagal men-generate user: {}", e);
        } else {
            println!("✅ Berhasil men-generate 10 user dummy.");
        }
        
        Ok(())
    }
}
