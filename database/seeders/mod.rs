use crate::database::{connection::DatabasePool, seeder::DatabaseSeeder};

/// Registrasi semua seeder yang akan dijalankan.
pub async fn run(db: &DatabasePool) -> Result<(), sqlx::Error> {
    let seeder = DatabaseSeeder::new();
    
    // Daftarkan seeder Anda di sini:
    // seeder.call(UserSeeder);
    
    seeder.run_all(db).await
}
