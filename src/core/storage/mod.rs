pub mod local;
pub mod s3;

pub use local::LocalStorage;
pub use s3::{S3Config, S3Storage};

use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::{Arc, OnceLock};
use std::time::Duration;

static GLOBAL_STORAGE: OnceLock<Arc<Storage>> = OnceLock::new();

/// Trait universal untuk seluruh driver storage (Lokal, S3, R2, MinIO, dll).
#[async_trait]
pub trait StorageDriver: Send + Sync {
    /// Simpan data ke berkas.
    async fn put(&self, path: &str, contents: &[u8]) -> Result<(), String>;

    /// Ambil isi berkas dalam bentuk byte buffer.
    async fn get(&self, path: &str) -> Result<Vec<u8>, String>;

    /// Cek apakah berkas ada.
    async fn exists(&self, path: &str) -> bool;

    /// Hapus berkas.
    async fn delete(&self, path: &str) -> Result<(), String>;

    /// Dapatkan URL publik untuk berkas.
    fn url(&self, path: &str) -> String;

    /// Dapatkan URL sementara bertanda tangan (Presigned URL) dengan batas waktu kedaluwarsa (TTL).
    fn temporary_url(&self, path: &str, ttl: Duration) -> Result<String, String>;
}

/// Storage Manager (Facade) untuk mengelola berbagai disk penyimpanan.
pub struct Storage {
    disks: HashMap<String, Arc<dyn StorageDriver>>,
    default_disk: String,
}

impl Storage {
    pub fn new(default_disk: &str) -> Self {
        Self {
            disks: HashMap::new(),
            default_disk: default_disk.to_string(),
        }
    }

    /// Registrasi instance global Storage Facade.
    pub fn set_global(storage: Arc<Storage>) {
        let _ = GLOBAL_STORAGE.set(storage);
    }

    /// Dapatkan instance global Storage Facade.
    pub fn global() -> Option<Arc<Storage>> {
        GLOBAL_STORAGE.get().cloned()
    }

    /// Daftarkan disk baru.
    pub fn add_disk(&mut self, name: &str, driver: Arc<dyn StorageDriver>) {
        self.disks.insert(name.to_string(), driver);
    }

    /// Ambil disk spesifik dari instance.
    pub fn disk(&self, name: &str) -> Arc<dyn StorageDriver> {
        self.disks
            .get(name)
            .cloned()
            .unwrap_or_else(|| panic!("Storage disk '{}' tidak ditemukan. Pastikan disk sudah dikonfigurasi.", name))
    }

    /// Ambil disk default dari instance.
    pub fn default(&self) -> Arc<dyn StorageDriver> {
        self.disk(&self.default_disk)
    }

    // ── Static Facade Methods ──────────────────────────────────────────

    /// Akses disk spesifik melalui Static Facade (misal: `Storage::disk_named("s3")`).
    pub fn disk_named(name: &str) -> Arc<dyn StorageDriver> {
        let storage = GLOBAL_STORAGE
            .get()
            .expect("Global Storage belum diinisialisasi. Pastikan Application::build() telah dijalankan.");
        storage.disk(name)
    }

    /// Simpan data ke disk default secara statis.
    pub async fn put_file(path: &str, contents: &[u8]) -> Result<(), String> {
        let storage = GLOBAL_STORAGE
            .get()
            .expect("Global Storage belum diinisialisasi.");
        storage.default().put(path, contents).await
    }

    /// Ambil data dari disk default secara statis.
    pub async fn get_file(path: &str) -> Result<Vec<u8>, String> {
        let storage = GLOBAL_STORAGE
            .get()
            .expect("Global Storage belum diinisialisasi.");
        storage.default().get(path).await
    }

    /// Dapatkan URL berkas dari disk default secara statis.
    pub fn file_url(path: &str) -> String {
        let storage = GLOBAL_STORAGE
            .get()
            .expect("Global Storage belum diinisialisasi.");
        storage.default().url(path)
    }

    // Proxy methods untuk disk default pada instance
    pub async fn put(&self, path: &str, contents: &[u8]) -> Result<(), String> {
        self.default().put(path, contents).await
    }

    pub async fn get(&self, path: &str) -> Result<Vec<u8>, String> {
        self.default().get(path).await
    }

    pub async fn exists(&self, path: &str) -> bool {
        self.default().exists(path).await
    }

    pub async fn delete(&self, path: &str) -> Result<(), String> {
        self.default().delete(path).await
    }

    pub fn url(&self, path: &str) -> String {
        self.default().url(path)
    }

    pub fn temporary_url(&self, path: &str, ttl: Duration) -> Result<String, String> {
        self.default().temporary_url(path, ttl)
    }
}
