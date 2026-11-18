use crate::database::connection::DatabasePool;
use async_trait::async_trait;
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

/// Type alias untuk fungsi eksekusi seeder asinkron
pub type SeederFn = Arc<
    dyn Fn(&DatabasePool) -> Pin<Box<dyn Future<Output = Result<(), sqlx::Error>> + Send>>
        + Send
        + Sync,
>;

/// Trait yang diimplementasikan oleh setiap Seeder class
///
/// # Contoh:
/// ```rust,ignore
/// pub struct UserSeeder;
///
/// #[async_trait::async_trait]
/// impl Seeder for UserSeeder {
///     async fn run(&self, db: &DatabasePool) -> Result<(), sqlx::Error> {
///         User::factory().count(50).create(db).await?;
///         Ok(())
///     }
/// }
/// ```
#[async_trait]
pub trait Seeder: Send + Sync {
    /// Jalankan proses pengisian data dummy atau awal ke database
    async fn run(&self, db: &DatabasePool) -> Result<(), sqlx::Error>;
}

/// Helper function untuk mengeksekusi seeder anak secara langsung dari dalam seeder lain.
///
/// Mirip dengan `$this->call(UserSeeder::class)` di Laravel:
/// ```rust,ignore
/// call_seeder::<UserSeeder>(db).await?;
/// ```
pub async fn call_seeder<S: Seeder + Default + 'static>(db: &DatabasePool) -> Result<(), sqlx::Error> {
    let seeder = S::default();
    seeder.run(db).await
}

/// Registry terpusat untuk mendaftarkan dan mengeksekusi Seeder berdasarkan nama class (CLI support).
#[derive(Clone, Default)]
pub struct SeederRegistry {
    entries: HashMap<String, SeederFn>,
}

impl SeederRegistry {
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }

    /// Daftarkan seeder berdasarkan nama class
    pub fn register<S>(&mut self, name: &str)
    where
        S: Seeder + Default + 'static,
    {
        let name_owned = name.to_string();
        self.entries.insert(
            name_owned,
            Arc::new(|db: &DatabasePool| {
                let pool = db.clone();
                Box::pin(async move {
                    let seeder = S::default();
                    seeder.run(&pool).await
                })
            }),
        );
    }

    /// Jalankan seeder berdasarkan nama class tertentu
    pub async fn run_class(&self, name: &str, db: &DatabasePool) -> Result<bool, sqlx::Error> {
        if let Some(seeder_fn) = self.entries.get(name) {
            println!("🌱 Menjalankan Seeder: {}", name);
            seeder_fn(db).await?;
            println!("✅ Seeder [{}] selesai dijalankan.", name);
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Jalankan seluruh seeder yang terdaftar secara berurutan
    pub async fn run_all(&self, db: &DatabasePool) -> Result<(), sqlx::Error> {
        println!("🌱 Menjalankan seluruh seeder yang terdaftar ({} seeder)...", self.entries.len());
        for (name, seeder_fn) in &self.entries {
            println!("  ➔ Menjalankan {}", name);
            seeder_fn(db).await?;
        }
        println!("✅ Seluruh seeder berhasil dieksekusi.");
        Ok(())
    }
}

/// Orchestrator eksekusi seeder klasik (Backward-Compatible)
pub struct DatabaseSeeder {
    seeders: Vec<Box<dyn Seeder>>,
}

impl DatabaseSeeder {
    pub fn new() -> Self {
        Self {
            seeders: Vec::new(),
        }
    }

    /// Tambahkan seeder ke daftar eksekusi
    pub fn call<S: Seeder + 'static>(&mut self, seeder: S) -> &mut Self {
        self.seeders.push(Box::new(seeder));
        self
    }

    /// Jalankan semua seeder yang sudah terdaftar
    pub async fn run_all(&self, db: &DatabasePool) -> Result<(), sqlx::Error> {
        println!("🌱 Menjalankan DatabaseSeeder...");
        for seeder in &self.seeders {
            seeder.run(db).await?;
        }
        println!("✅ Seeding selesai.");
        Ok(())
    }
}

impl Default for DatabaseSeeder {
    fn default() -> Self {
        Self::new()
    }
}
