use async_trait::async_trait;
use std::path::PathBuf;
use std::time::Duration;
use tokio::fs;
use super::StorageDriver;

/// Driver penyimpanan berkas lokal filesystem dengan proteksi directory traversal.
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

    /// Memastikan path yang diminta aman dan berada di dalam root directory (anti directory traversal).
    fn safe_path(&self, path: &str) -> Result<PathBuf, String> {
        let clean = path.trim_start_matches('/').trim_start_matches('\\');
        if clean.contains("..") {
            return Err("Invalid path: directory traversal attempt detected".to_string());
        }

        let full_path = self.root.join(clean);
        Ok(full_path)
    }
}

#[async_trait]
impl StorageDriver for LocalStorage {
    async fn put(&self, path: &str, contents: &[u8]) -> Result<(), String> {
        let full_path = self.safe_path(path)?;

        if let Some(parent) = full_path.parent() {
            fs::create_dir_all(parent)
                .await
                .map_err(|e| format!("Failed to create storage directory: {}", e))?;
        }

        fs::write(full_path, contents)
            .await
            .map_err(|e| format!("Failed to write storage file: {}", e))
    }

    async fn get(&self, path: &str) -> Result<Vec<u8>, String> {
        let full_path = self.safe_path(path)?;
        fs::read(full_path)
            .await
            .map_err(|e| format!("Failed to read storage file: {}", e))
    }

    async fn exists(&self, path: &str) -> bool {
        match self.safe_path(path) {
            Ok(p) => p.exists(),
            Err(_) => false,
        }
    }

    async fn delete(&self, path: &str) -> Result<(), String> {
        let full_path = self.safe_path(path)?;
        if full_path.exists() {
            fs::remove_file(full_path)
                .await
                .map_err(|e| format!("Failed to delete storage file: {}", e))?;
        }
        Ok(())
    }

    fn url(&self, path: &str) -> String {
        let clean = path.trim_start_matches('/').trim_start_matches('\\');
        format!(
            "{}/{}",
            self.base_url.trim_end_matches('/'),
            clean
        )
    }

    fn temporary_url(&self, path: &str, _ttl: Duration) -> Result<String, String> {
        // Untuk disk lokal, kembalikan URL standar atau route aman
        Ok(self.url(path))
    }
}
