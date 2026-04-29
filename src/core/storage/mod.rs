use std::path::PathBuf;
use tokio::fs;
use async_trait::async_trait;

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
        
        // Pastikan direktori tujuan ada
        if let Some(parent) = full_path.parent() {
            fs::create_dir_all(parent).await.map_err(|e| e.to_string())?;
        }

        fs::write(full_path, contents).await.map_err(|e| e.to_string())
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
        format!("{}/{}", self.base_url.trim_end_matches('/'), path.trim_start_matches('/'))
    }
}

/// Storage Manager yang menyimpan berbagai disk.
pub struct Storage {
    pub disk: Box<dyn StorageDriver>,
}

impl Storage {
    pub fn new_local(root: &str, base_url: &str) -> Self {
        Self {
            disk: Box::new(LocalStorage::new(root, base_url)),
        }
    }
}
