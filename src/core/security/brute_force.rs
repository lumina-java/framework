use crate::core::error::AppError;
use moka::future::Cache;
use std::sync::LazyLock;
use std::time::{Duration, Instant};
use subtle::ConstantTimeEq;

/// Data status percobaan brute force untuk satu kunci (misal: IP atau Email).
#[derive(Clone, Debug)]
pub struct AttemptRecord {
    pub count: u32,
    pub last_attempt: Instant,
    pub locked_until: Option<Instant>,
}

/// Konfigurasi proteksi Brute-Force & Credential Stuffing.
#[derive(Clone, Debug)]
pub struct BruteForceConfig {
    /// Maksimum kegagalan sebelum akun/IP dikunci sementara. Default: 5 kali.
    pub max_attempts: u32,
    /// Durasi kunci dalam detik. Default: 900 detik (15 menit).
    pub lockout_duration: Duration,
    /// Durasi jendela waktu retensi percobaan. Default: 900 detik.
    pub window_duration: Duration,
}

impl Default for BruteForceConfig {
    fn default() -> Self {
        Self {
            max_attempts: 5,
            lockout_duration: Duration::from_secs(900), // 15 menit
            window_duration: Duration::from_secs(900),
        }
    }
}

static BRUTE_FORCE_CACHE: LazyLock<Cache<String, AttemptRecord>> = LazyLock::new(|| {
    Cache::builder()
        .time_to_idle(Duration::from_secs(1800)) // Hapus data idle setelah 30 menit
        .max_capacity(100_000)                   // Mampu menangani hingga 100.000 IP/user concurrent
        .build()
});

/// Guard untuk mencegah serangan Brute-Force & Credential Stuffing.
/// Berjalan secara asynchronous, in-memory, dan lock-free menggunakan Moka Cache.
pub struct BruteForceGuard;

impl BruteForceGuard {
    /// Periksa apakah kunci (IP atau Email) saat ini sedang terkunci.
    /// Mengembalikan `Ok(())` jika aman, atau `Err(AppError::Forbidden)` jika sedang terkunci.
    pub async fn check(key: &str) -> Result<(), AppError> {
        Self::check_with_config(key, &BruteForceConfig::default()).await
    }

    /// Periksa status kunci dengan konfigurasi kustom.
    pub async fn check_with_config(key: &str, _config: &BruteForceConfig) -> Result<(), AppError> {
        if let Some(record) = BRUTE_FORCE_CACHE.get(key).await {
            if let Some(locked_until) = record.locked_until {
                let now = Instant::now();
                if now < locked_until {
                    let remaining_secs = (locked_until - now).as_secs();
                    return Err(AppError::Forbidden(format!(
                        "Terlalu banyak percobaan gagal. Akun/IP dikunci sementara. Silakan coba lagi dalam {} detik.",
                        remaining_secs.max(1)
                    )));
                }
            }
        }
        Ok(())
    }

    /// Catat percobaan gagal baru.
    /// Mengembalikan jumlah total kegagalan yang tercatat saat ini.
    pub async fn hit(key: &str) -> u32 {
        Self::hit_with_config(key, &BruteForceConfig::default()).await
    }

    /// Catat percobaan gagal dengan konfigurasi kustom.
    pub async fn hit_with_config(key: &str, config: &BruteForceConfig) -> u32 {
        let now = Instant::now();
        let mut count = 1;
        let mut locked_until = None;

        if let Some(existing) = BRUTE_FORCE_CACHE.get(key).await {
            count = existing.count + 1;
            if count >= config.max_attempts {
                locked_until = Some(now + config.lockout_duration);
            }
        }

        BRUTE_FORCE_CACHE
            .insert(
                key.to_string(),
                AttemptRecord {
                    count,
                    last_attempt: now,
                    locked_until,
                },
            )
            .await;

        count
    }

    /// Reset status kunci (misal: saat login berhasil).
    pub async fn reset(key: &str) {
        BRUTE_FORCE_CACHE.invalidate(key).await;
    }

    /// Periksa apakah kunci sedang dalam status terkunci (Boolean).
    pub async fn is_locked(key: &str) -> bool {
        if let Some(record) = BRUTE_FORCE_CACHE.get(key).await {
            if let Some(locked_until) = record.locked_until {
                return Instant::now() < locked_until;
            }
        }
        false
    }

    /// Helper verifikasi string dengan perbandingan waktu konstan (Constant-Time Comparison).
    /// Menggagalkan serangan Timing Attacks pada password hash, token, atau secret keys.
    pub fn constant_time_eq(a: &str, b: &str) -> bool {
        a.as_bytes().ct_eq(b.as_bytes()).into()
    }
}

