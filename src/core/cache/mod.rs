use moka::future::Cache;
<<<<<<< HEAD
use serde::{de::DeserializeOwned, Serialize};
use serde_json;
use std::time::Duration;

#[derive(Clone)]
pub struct CacheManager {
    inner: Cache<String, String>,
=======
use moka::Expiry;
use std::time::Duration;
use serde::{Serialize, de::DeserializeOwned};
use serde_json;

/// CacheEntry menyimpan data dan durasi TTL untuk digunakan oleh Expiry policy moka.
#[derive(Clone)]
struct CacheEntry {
    value: String,
    ttl: Duration,
}

/// Expiry policy khusus Lumina untuk mendukung per-entry TTL secara native di moka.
struct LuminaExpiry;

impl Expiry<String, CacheEntry> for LuminaExpiry {
    fn expire_after_create(
        &self,
        _key: &String,
        entry: &CacheEntry,
        _current_time: std::time::Instant,
    ) -> Option<Duration> {
        Some(entry.ttl)
    }
}

#[derive(Clone)]
pub struct CacheManager {
    inner: Cache<String, CacheEntry>,
>>>>>>> dev/master
}

impl CacheManager {
    /// Membuat instance baru CacheManager (In-Memory).
    pub fn new() -> Self {
        let inner = Cache::builder()
            .max_capacity(10_000) // Batas maksimal 10.000 entry
<<<<<<< HEAD
            .build();

=======
            .expire_after(LuminaExpiry)
            .build();

>>>>>>> dev/master
        Self { inner }
    }

    /// Menyimpan data ke cache dengan TTL (Time To Live).
    pub async fn put<T: Serialize>(&self, key: &str, value: T, ttl_seconds: u64) {
        if let Ok(serialized) = serde_json::to_string(&value) {
<<<<<<< HEAD
            self.inner.insert(key.to_string(), serialized).await;

            // Note: moka handles expiry via policy, but for simple MVP
            // we could use different cache instances or per-entry expiry if supported.
            // In moka v0.12+, we use entry-based expiry via policy or builder.
            // However, to keep it simple and Laravel-like:
            tokio::spawn({
                let cache = self.inner.clone();
                let k = key.to_string();
                async move {
                    tokio::time::sleep(Duration::from_secs(ttl_seconds)).await;
                    cache.invalidate(&k).await;
                }
            });
=======
            let entry = CacheEntry {
                value: serialized,
                ttl: Duration::from_secs(ttl_seconds),
            };
            self.inner
                .insert(key.to_string(), entry)
                .await;
>>>>>>> dev/master
        }
    }

    /// Mengambil data dari cache.
    pub async fn get<T: DeserializeOwned>(&self, key: &str) -> Option<T> {
<<<<<<< HEAD
        if let Some(serialized) = self.inner.get(&key.to_string()).await {
            return serde_json::from_str(&serialized).ok();
=======
        if let Some(entry) = self.inner.get(&key.to_string()).await {
            return serde_json::from_str(&entry.value).ok();
>>>>>>> dev/master
        }
        None
    }

    /// Menghapus data dari cache.
    pub async fn forget(&self, key: &str) {
        self.inner.invalidate(&key.to_string()).await;
    }

    /// Mengambil data dari cache, atau eksekusi closure dan simpan jika tidak ada.
<<<<<<< HEAD
    pub async fn remember<T, F, Fut>(&self, key: &str, ttl_seconds: u64, f: F) -> Option<T>
    where
=======
    pub async fn remember<T, F, Fut>(&self, key: &str, ttl_seconds: u64, f: F) -> Option<T>
    where
>>>>>>> dev/master
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

    /// Membersihkan semua isi cache.
    pub async fn flush(&self) {
        self.inner.invalidate_all();
    }
}
