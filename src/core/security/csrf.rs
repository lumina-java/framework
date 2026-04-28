use axum_csrf::CsrfConfig;
use time::Duration;

/// Konfigurasi CSRF untuk Lumina.
pub fn config() -> CsrfConfig {
    CsrfConfig::default()
        .with_lifetime(Duration::hours(1))
        .with_cookie_name("lumina_csrf")
}
