use async_trait::async_trait;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::fs;

#[async_trait]
pub trait StorageDriver: Send + Sync {
    /// Simpan data ke file.
    async fn put(&self, path: &str, contents: &[u8]) -> Result<(), String>;

    /// Ambil data dari file.
    async fn get(&self, path: &str) -> Result<Vec<u8>, String>;

    /// Cek apakah file ada.
    async fn exists(&self, path: &str) -> bool;

    /// Hapus file.
    async fn delete(&self, path: &str) -> Result<(), String>;

    /// Dapatkan URL publik untuk file.
    fn url(&self, path: &str) -> String;
}

pub struct LocalStorage {
    root: PathBuf,
    base_url: String,
}

impl LocalStorage {
    pub fn new(root: &str, base_url: &str) -> Self {
        Self {
            root: PathBuf::from(root),
            base_url: base_url.to_string(),
        }
    }
}

#[async_trait]
impl StorageDriver for LocalStorage {
    async fn put(&self, path: &str, contents: &[u8]) -> Result<(), String> {
        let full_path = self.root.join(path);

        if let Some(parent) = full_path.parent() {
            fs::create_dir_all(parent)
                .await
                .map_err(|e| e.to_string())?;
        }

        fs::write(full_path, contents)
            .await
            .map_err(|e| e.to_string())
    }

    async fn get(&self, path: &str) -> Result<Vec<u8>, String> {
        let full_path = self.root.join(path);
        fs::read(full_path).await.map_err(|e| e.to_string())
    }

    async fn exists(&self, path: &str) -> bool {
        self.root.join(path).exists()
    }

    async fn delete(&self, path: &str) -> Result<(), String> {
        let full_path = self.root.join(path);
        fs::remove_file(full_path).await.map_err(|e| e.to_string())
    }

    fn url(&self, path: &str) -> String {
        format!(
            "{}/{}",
            self.base_url.trim_end_matches('/'),
            path.trim_start_matches('/')
        )
    }
}

/// Driver placeholder untuk S3 (Gunakan crate 'aws-sdk-s3' untuk implementasi penuh)
pub struct S3Storage {
    pub bucket: String,
    pub region: String,
}

#[async_trait]
impl StorageDriver for S3Storage {
    async fn put(&self, _path: &str, _contents: &[u8]) -> Result<(), String> {
        Err("S3 Driver belum diimplementasikan sepenuhnya. Silakan instal aws-sdk-s3.".to_string())
    }

    async fn get(&self, _path: &str) -> Result<Vec<u8>, String> {
        Err("S3 Driver belum diimplementasikan sepenuhnya.".to_string())
    }

    async fn exists(&self, _path: &str) -> bool {
        false
    }

    async fn delete(&self, _path: &str) -> Result<(), String> {
        Err("S3 Driver belum diimplementasikan sepenuhnya.".to_string())
    }

    fn url(&self, path: &str) -> String {
        format!(
            "https://{}.s3.{}.amazonaws.com/{}",
            self.bucket, self.region, path
        )
    }
}

/// Storage Manager (Facade) untuk mengelola berbagai disk.
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

    /// Daftarkan disk baru.
    pub fn add_disk(&mut self, name: &str, driver: Arc<dyn StorageDriver>) {
        self.disks.insert(name.to_string(), driver);
    }

    /// Ambil disk spesifik.
    pub fn disk(&self, name: &str) -> Arc<dyn StorageDriver> {
        self.disks
            .get(name)
            .cloned()
            .expect(&format!("Storage disk '{}' tidak ditemukan", name))
    }

    /// Ambil disk default.
    pub fn default(&self) -> Arc<dyn StorageDriver> {
        self.disk(&self.default_disk)
    }

    // Proxy methods untuk disk default
    pub async fn put(&self, path: &str, contents: &[u8]) -> Result<(), String> {
        self.default().put(path, contents).await
    }

    pub async fn get(&self, path: &str) -> Result<Vec<u8>, String> {
        self.default().get(path).await
    }

    pub fn url(&self, path: &str) -> String {
        self.default().url(path)
    }
}
