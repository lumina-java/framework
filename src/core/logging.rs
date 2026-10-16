use std::path::Path;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

/// Setup logger Lumina dengan dukungan:
/// - File logging rolling harian ke `storage/logs/`
/// - Filter level via env `LOG_LEVEL` (default: `info`)
/// - Dual output: stdout (dev) + file (production)
///
/// Mengembalikan `WorkerGuard` yang HARUS di-hold sepanjang lifetime aplikasi.
/// Jika guard ini di-drop, log file writer akan berhenti.
///
/// # Contoh
/// ```rust
/// // Di main.rs:
/// let _log_guard = lumina::core::logging::init();
/// ```
pub fn init() -> WorkerGuard {
    // Buat direktori logs jika belum ada
    let log_dir = Path::new("storage/logs");
    if !log_dir.exists() {
        std::fs::create_dir_all(log_dir).expect("Gagal membuat direktori storage/logs");
    }

    // Rolling file appender: satu file baru per hari
    let file_appender = tracing_appender::rolling::daily("storage/logs", "lumina.log");
    let (non_blocking_writer, guard) = tracing_appender::non_blocking(file_appender);

    // Level filter dari env: LOG_LEVEL=debug,info,warn,error
    let log_level = std::env::var("LOG_LEVEL").unwrap_or_else(|_| "info".to_string());
    let env_filter = EnvFilter::try_new(&log_level).unwrap_or_else(|_| EnvFilter::new("info"));

    // Layer 1: stdout (colorful, untuk development)
    let stdout_layer = fmt::layer()
        .with_target(true)
        .with_thread_ids(false)
        .with_ansi(true);

    // Layer 2: file (tanpa ANSI color, format compact untuk production)
    let file_layer = fmt::layer()
        .with_writer(non_blocking_writer)
        .with_target(true)
        .with_ansi(false)
        .with_thread_ids(true)
        .compact();

    // Gabungkan kedua layer dengan filter global
    tracing_subscriber::registry()
        .with(env_filter)
        .with(stdout_layer)
        .with(file_layer)
        .init();

    tracing::info!("📝 Lumina Logger aktif. Log disimpan ke: storage/logs/");

    guard
}
