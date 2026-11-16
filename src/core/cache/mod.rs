pub mod driver;

pub use driver::{CacheDriver, MokaCacheDriver};
#[cfg(feature = "redis")]
pub use driver::RedisCacheDriver;

use serde::{de::DeserializeOwned, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, OnceLock};

static GLOBAL_CACHE: OnceLock<Arc<CacheManager>> = OnceLock::new();

/// Cache Manager yang mengelola berbagai driver cache (Moka in-memory, Redis, dll).
pub struct CacheManager {
    drivers: HashMap<String, Arc<dyn CacheDriver>>,
    default_driver: String,
}

impl Default for CacheManager {
    fn default() -> Self {
        Self::new()
    }
}

impl CacheManager {
    /// Membuat CacheManager baru dengan driver default Moka in-memory.
    pub fn new() -> Self {
        Self::with_driver("moka")
    }

    /// Membuat CacheManager baru dengan menentukan nama driver default.
    pub fn with_driver(default_driver: &str) -> Self {
        let mut mgr = Self {
            drivers: HashMap::new(),
            default_driver: default_driver.to_string(),
        };

        // Selalu sertakan driver Moka (In-Memory)
        mgr.add_driver("moka", Arc::new(MokaCacheDriver::new(10_000)));
        mgr
    }

    /// Membuat instance CacheManager berdasarkan konfigurasi environment (.env).
    pub fn from_env() -> Self {
        let default_driver = std::env::var("CACHE_DRIVER").unwrap_or_else(|_| "moka".to_string());
        #[allow(unused_mut)]
        let mut mgr = Self::with_driver(&default_driver);

        #[cfg(feature = "redis")]
        if let Ok(redis_url) = std::env::var("REDIS_URL") {
            match RedisCacheDriver::new(&redis_url) {
                Ok(redis_driver) => {
                    mgr.add_driver("redis", Arc::new(redis_driver));
                }
                Err(e) => {
                    eprintln!("⚠️  [Lumina Cache] Inisialisasi Redis gagal: {}. Fallback ke Moka.", e);
                }
            }
        }

        mgr
    }

    /// Daftarkan instance global untuk facade.
    pub fn set_global(cache: Arc<CacheManager>) {
        let _ = GLOBAL_CACHE.set(cache);
    }

    /// Dapatkan instance global CacheManager.
    pub fn global() -> Option<Arc<CacheManager>> {
        GLOBAL_CACHE.get().cloned()
    }

    /// Daftarkan driver cache baru.
    pub fn add_driver(&mut self, name: &str, driver: Arc<dyn CacheDriver>) {
        self.drivers.insert(name.to_string(), driver);
    }

    /// Ambil driver spesifik.
    pub fn driver(&self, name: &str) -> Arc<dyn CacheDriver> {
        self.drivers
            .get(name)
            .cloned()
            .unwrap_or_else(|| {
                // Fallback ke moka jika driver tidak ditemukan
                self.drivers
                    .get("moka")
                    .cloned()
                    .expect("Moka cache driver must be registered")
            })
    }

    /// Ambil driver default.
    pub fn default(&self) -> Arc<dyn CacheDriver> {
        self.driver(&self.default_driver)
    }

    // ── Proxy Methods Berbasis Tipe Generik ─────────────────────────────

    /// Simpan data ke cache default dengan TTL (Time To Live dalam detik).
    pub async fn put<T: Serialize>(&self, key: &str, value: T, ttl_seconds: u64) {
        if let Ok(serialized) = serde_json::to_string(&value) {
            self.default().put(key, &serialized, ttl_seconds).await;
        }
    }

    /// Ambil data dari cache default.
    pub async fn get<T: DeserializeOwned>(&self, key: &str) -> Option<T> {
        if let Some(serialized) = self.default().get(key).await {
            return serde_json::from_str(&serialized).ok();
        }
        None
    }

    /// Hapus data dari cache default.
    pub async fn forget(&self, key: &str) {
        self.default().forget(key).await;
    }

    /// Bersihkan seluruh data pada cache default.
    pub async fn flush(&self) {
        self.default().flush().await;
    }

    /// Ambil data dari cache default, atau eksekusi closure dan simpan jika belum ada.
    pub async fn remember<T, F, Fut>(&self, key: &str, ttl_seconds: u64, f: F) -> Option<T>
    where
        T: Serialize + DeserializeOwned,
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = T>,
    {
        if let Some(val) = self.get::<T>(key).await {
            return Some(val);
        }

        let val = f().await;
        self.put(key, &val, ttl_seconds).await;
        Some(val)
    }
}

/// Global Facade untuk ekosistem Cache Lumina.
pub struct Cache;

impl Cache {
    /// Akses driver spesifik secara statis (misal: `Cache::driver("redis")`).
    pub fn driver(name: &str) -> Arc<dyn CacheDriver> {
        let mgr = GLOBAL_CACHE
            .get()
            .expect("Global Cache belum diinisialisasi. Pastikan Application::build() telah dijalankan.");
        mgr.driver(name)
    }

    /// Simpan data ke cache secara statis.
    pub async fn put<T: Serialize>(key: &str, value: T, ttl_seconds: u64) {
        if let Some(mgr) = GLOBAL_CACHE.get() {
            mgr.put(key, value, ttl_seconds).await;
        }
    }

    /// Ambil data dari cache secara statis.
    pub async fn get<T: DeserializeOwned>(key: &str) -> Option<T> {
        if let Some(mgr) = GLOBAL_CACHE.get() {
            mgr.get::<T>(key).await
        } else {
            None
        }
    }

    /// Hapus data dari cache secara statis.
    pub async fn forget(key: &str) {
        if let Some(mgr) = GLOBAL_CACHE.get() {
            mgr.forget(key).await;
        }
    }

    /// Bersihkan seluruh isi cache secara statis.
    pub async fn flush() {
        if let Some(mgr) = GLOBAL_CACHE.get() {
            mgr.flush().await;
        }
    }

    /// Helper remember statis.
    pub async fn remember<T, F, Fut>(key: &str, ttl_seconds: u64, f: F) -> Option<T>
    where
        T: Serialize + DeserializeOwned,
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = T>,
    {
        if let Some(mgr) = GLOBAL_CACHE.get() {
            mgr.remember(key, ttl_seconds, f).await
        } else {
            let val = f().await;
            Some(val)
        }
    }
}
