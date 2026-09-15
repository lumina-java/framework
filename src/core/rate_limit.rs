use axum::{
    body::Body,
    extract::State,
    http::{HeaderValue, Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};

use crate::core::application::AppState;
use serde::Serialize;

/// Konfigurasi Rate Limiter yang bisa diatur via .env
#[derive(Clone, Debug)]
pub struct RateLimitConfig {
    /// Jumlah maksimum request per window
    pub max_requests: u32,
    /// Durasi window dalam detik
    pub window_seconds: u64,
}

impl RateLimitConfig {
    pub fn new(max_requests: u32, window_seconds: u64) -> Self {
        Self {
            max_requests,
            window_seconds,
        }
    }

    /// Baca konfigurasi dari environment variables:
    /// - `RATE_LIMIT_MAX` (default: 60)
    /// - `RATE_LIMIT_WINDOW` (default: 60 detik)
    pub fn from_env() -> Self {
        let max = std::env::var("RATE_LIMIT_MAX")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(60u32);
        let window = std::env::var("RATE_LIMIT_WINDOW")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(60u64);
        Self::new(max, window)
    }

    /// Preset ketat untuk endpoint sensitif seperti login, register, forgot password
    pub fn strict() -> Self {
        Self::new(10, 60) // 10 request per menit
    }

    /// Preset longgar untuk API umum
    pub fn api() -> Self {
        Self::new(120, 60) // 120 request per menit
    }
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self::from_env()
    }
}

/// Data yang disimpan per-IP di CacheManager
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
struct RateLimitEntry {
    count: u32,
    window_start_secs: u64, // Unix timestamp saat window dimulai
}

#[derive(Serialize)]
struct RateLimitError {
    error: &'static str,
    message: String,
    retry_after: u64,
}

/// Middleware Rate Limiter berbasis IP menggunakan CacheManager (moka) yang sudah ada.
///
/// Cara penggunaan sebagai global middleware:
/// ```rust,ignore
/// // Di application.rs atau route builder:
/// use lumina::core::rate_limit::{RateLimitConfig, rate_limit_middleware};
/// use axum::middleware;
///
/// let config = RateLimitConfig::from_env();
/// router.layer(middleware::from_fn_with_state(state, rate_limit_middleware))
/// ```
pub async fn rate_limit_middleware(
    State(state): State<AppState>,
    req: Request<Body>,
    next: Next,
) -> Response {
    let config = RateLimitConfig::from_env();
    rate_limit_with_config(state, config, req, next).await
}

/// Middleware Rate Limiter dengan konfigurasi kustom (berguna untuk per-route).
pub async fn rate_limit_with_config(
    state: AppState,
    config: RateLimitConfig,
    req: Request<Body>,
    next: Next,
) -> Response {
    // Ekstrak IP dari header X-Forwarded-For atau X-Real-IP (untuk behind proxy)
    // Fallback ke "unknown" jika tidak ada
    let client_ip = req
        .headers()
        .get("X-Forwarded-For")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.split(',').next())
        .or_else(|| req.headers().get("X-Real-IP").and_then(|v| v.to_str().ok()))
        .unwrap_or("unknown")
        .trim()
        .to_string();

    let cache_key = format!("rate_limit:{}", client_ip);
    let now_secs = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    let cache = state.cache();

    // Ambil entry yang ada dari cache
    let mut entry: RateLimitEntry =
        cache
            .get::<RateLimitEntry>(&cache_key)
            .await
            .unwrap_or(RateLimitEntry {
                count: 0,
                window_start_secs: now_secs,
            });

    // Cek apakah window sudah expired (reset counter)
    let window_expired = now_secs.saturating_sub(entry.window_start_secs) >= config.window_seconds;
    if window_expired {
        entry = RateLimitEntry {
            count: 0,
            window_start_secs: now_secs,
        };
    }

    // Hitung sisa waktu window (untuk header Retry-After)
    let window_remaining = config
        .window_seconds
        .saturating_sub(now_secs.saturating_sub(entry.window_start_secs));
    let requests_remaining = config.max_requests.saturating_sub(entry.count);

    // Cek apakah limit terlampaui
    if entry.count >= config.max_requests {
        tracing::warn!(
            ip = %client_ip,
            limit = config.max_requests,
            window_secs = config.window_seconds,
            "Rate limit exceeded"
        );

        let error_body = serde_json::json!(RateLimitError {
            error: "Too Many Requests",
            message: format!(
                "Terlalu banyak request. Silakan coba lagi dalam {} detik.",
                window_remaining
            ),
            retry_after: window_remaining,
        });

        let mut response = (StatusCode::TOO_MANY_REQUESTS, axum::Json(error_body)).into_response();

        // Tambahkan rate limit headers
        let headers = response.headers_mut();
        headers.insert(
            "X-RateLimit-Limit",
            HeaderValue::from_str(&config.max_requests.to_string()).unwrap(),
        );
        headers.insert("X-RateLimit-Remaining", HeaderValue::from_static("0"));
        headers.insert(
            "Retry-After",
            HeaderValue::from_str(&window_remaining.to_string()).unwrap(),
        );

        return response;
    }

    // Increment counter dan simpan kembali
    entry.count += 1;
    cache
        .put(&cache_key, &entry, config.window_seconds + 5)
        .await;

    // Lanjutkan ke handler berikutnya
    let mut response = next.run(req).await;

    // Inject X-RateLimit-* headers ke response
    let headers = response.headers_mut();
    headers.insert(
        "X-RateLimit-Limit",
        HeaderValue::from_str(&config.max_requests.to_string())
            .unwrap_or(HeaderValue::from_static("60")),
    );
    headers.insert(
        "X-RateLimit-Remaining",
        HeaderValue::from_str(&requests_remaining.saturating_sub(1).to_string())
            .unwrap_or(HeaderValue::from_static("0")),
    );

    response
}
