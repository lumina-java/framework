use moka::future::Cache;
use std::sync::OnceLock;
use std::time::Duration;

static BLACKLIST: OnceLock<Cache<String, bool>> = OnceLock::new();

/// TokenBlacklist — In-memory TTL revocation cache berbasis Moka.
/// Mencegah penggunaan kembali JWT token yang sudah logout sebelum masa berlakunya berakhir.
pub struct TokenBlacklist;

impl TokenBlacklist {
    fn cache() -> &'static Cache<String, bool> {
        BLACKLIST.get_or_init(|| {
            Cache::builder()
                .max_capacity(100_000)
                .time_to_live(Duration::from_secs(86400 * 7)) // 7 hari auto-eviction
                .build()
        })
    }

    /// Cabut / batalkan (revoke) sebuah token secara instan.
    pub async fn revoke(token: &str) {
        Self::cache().insert(token.to_string(), true).await;
    }

    /// Periksa apakah token berada dalam daftar pembatalan.
    pub async fn is_revoked(token: &str) -> bool {
        Self::cache().get(token).await.unwrap_or(false)
    }

    /// Bersihkan seluruh cache pembatalan (misal untuk keperluan testing).
    pub fn clear() {
        if let Some(c) = BLACKLIST.get() {
            c.invalidate_all();
        }
    }
}
