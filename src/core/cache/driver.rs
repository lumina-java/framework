use async_trait::async_trait;
use moka::future::Cache as MokaCache;
use std::time::Duration;

#[async_trait]
pub trait CacheDriver: Send + Sync {
    /// Mengambil nilai mentah string dari cache.
    async fn get(&self, key: &str) -> Option<String>;

    /// Menyimpan nilai string ke cache dengan batas waktu (TTL dalam detik).
    async fn put(&self, key: &str, value: &str, ttl_seconds: u64);

    /// Menghapus entri cache berdasarkan kunci.
    async fn forget(&self, key: &str);

    /// Menghapus seluruh isi cache pada driver ini.
    async fn flush(&self);
}

/// Driver Cache In-Memory berbasis Moka (Sangat cepat, default tanpa setup eksternal).
pub struct MokaCacheDriver {
    inner: MokaCache<String, String>,
}

impl MokaCacheDriver {
    pub fn new(capacity: u64) -> Self {
        let inner = MokaCache::builder()
            .max_capacity(capacity)
            .build();
        Self { inner }
    }
}

#[async_trait]
impl CacheDriver for MokaCacheDriver {
    async fn get(&self, key: &str) -> Option<String> {
        self.inner.get(&key.to_string()).await
    }

    async fn put(&self, key: &str, value: &str, ttl_seconds: u64) {
        self.inner.insert(key.to_string(), value.to_string()).await;

        let cache = self.inner.clone();
        let k = key.to_string();
        tokio::spawn(async move {
            tokio::time::sleep(Duration::from_secs(ttl_seconds)).await;
            cache.invalidate(&k).await;
        });
    }

    async fn forget(&self, key: &str) {
        self.inner.invalidate(&key.to_string()).await;
    }

    async fn flush(&self) {
        self.inner.invalidate_all();
    }
}

/// Driver Cache Terdistribusi berbasis Redis (Ideal untuk Multi-Pod Kubernetes & Docker Cluster).
#[cfg(feature = "redis")]
pub struct RedisCacheDriver {
    client: redis::Client,
}

#[cfg(feature = "redis")]
impl RedisCacheDriver {
    pub fn new(redis_url: &str) -> Result<Self, String> {
        let client = redis::Client::open(redis_url)
            .map_err(|e| format!("Gagal inisialisasi Redis client: {}", e))?;
        Ok(Self { client })
    }
}

#[cfg(feature = "redis")]
#[async_trait]
impl CacheDriver for RedisCacheDriver {
    async fn get(&self, key: &str) -> Option<String> {
        let mut conn = match self.client.get_multiplexed_async_connection().await {
            Ok(c) => c,
            Err(e) => {
                eprintln!("⚠️  Redis Cache error: {}", e);
                return None;
            }
        };

        match redis::cmd("GET").arg(key).query_async(&mut conn).await {
            Ok(val) => val,
            Err(_) => None,
        }
    }

    async fn put(&self, key: &str, value: &str, ttl_seconds: u64) {
        let mut conn = match self.client.get_multiplexed_async_connection().await {
            Ok(c) => c,
            Err(e) => {
                eprintln!("⚠️  Redis Cache error: {}", e);
                return;
            }
        };

        let _: Result<(), redis::RedisError> = redis::cmd("SETEX")
            .arg(key)
            .arg(ttl_seconds)
            .arg(value)
            .query_async(&mut conn)
            .await;
    }

    async fn forget(&self, key: &str) {
        let mut conn = match self.client.get_multiplexed_async_connection().await {
            Ok(c) => c,
            Err(e) => {
                eprintln!("⚠️  Redis Cache error: {}", e);
                return;
            }
        };

        let _: Result<(), redis::RedisError> = redis::cmd("DEL")
            .arg(key)
            .query_async(&mut conn)
            .await;
    }

    async fn flush(&self) {
        let mut conn = match self.client.get_multiplexed_async_connection().await {
            Ok(c) => c,
            Err(e) => {
                eprintln!("⚠️  Redis Cache error: {}", e);
                return;
            }
        };

        let _: Result<(), redis::RedisError> = redis::cmd("FLUSHDB")
            .query_async(&mut conn)
            .await;
    }
}

