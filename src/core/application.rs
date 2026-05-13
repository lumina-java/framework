use axum::{
    Router as AxumRouter,
    middleware::{from_fn, from_fn_with_state},
    extract::State,
    response::{IntoResponse, Response},
};
use tower_http::catch_panic::CatchPanicLayer;
use tower_http::services::ServeDir;
use crate::core::router::Router;
use crate::http::server::Server;
use crate::http::middleware::logger;
use crate::core::container::Container;
use crate::database::{connection::DatabasePool, migration};
use crate::core::view::ViewEngine;
use crate::app::controllers::error_controller::ErrorController;
use std::sync::Arc;
use tower_sessions::{SessionManagerLayer, Expiry, MemoryStore};
use crate::core::session::store::LuminaSessionStore;
use tower_sessions_sqlx_store::{MySqlStore, SqliteStore, PostgresStore};
use axum_csrf::CsrfLayer;
use crate::core::security::csrf;
use time::Duration;

use axum::extract::FromRef;
use axum_csrf::CsrfConfig;

/// State global yang dibagikan ke semua handler.
#[derive(Clone)]
pub struct AppState {
    pub db: Option<Arc<DatabasePool>>,
    pub view: ViewEngine,
    pub auth_service: Option<Arc<crate::app::services::auth_service::AuthService>>,
    /// Pesan error database yang akan ditampilkan di halaman whoops.
    pub db_error: Option<String>,
    pub csrf_config: CsrfConfig,
    pub storage: Arc<crate::core::storage::Storage>,
    pub config: crate::core::config::Config,
    pub queue: Arc<crate::core::queue::QueueManager>,
    pub cache: Arc<crate::core::cache::CacheManager>,
    pub mail: Arc<crate::core::mail::Mail>,
    pub registry: Arc<crate::core::queue::JobRegistry>,
    pub events: Arc<crate::core::event::EventDispatcher>,
    pub telescope: Arc<crate::core::telescope::TelescopeManager>,
    pub lang: Arc<crate::core::i18n::LangManager>,
    pub echo: Arc<crate::core::echo::EchoManager>,
    pub socialite: Arc<crate::core::auth::socialite::SocialiteManager>,
}

impl AppState {
    /// Akses cepat ke Database Pool.
    /// Panik jika database tidak terhubung (seharusnya ditangani oleh db_guard).
    pub fn db(&self) -> &DatabasePool {
        self.db.as_ref().expect("Database connection is not available")
    }

    /// Akses cepat ke CacheManager (in-memory).
    pub fn cache(&self) -> &crate::core::cache::CacheManager {
        &self.cache
    }
}

impl FromRef<AppState> for CsrfConfig {
    fn from_ref(state: &AppState) -> Self {
        state.csrf_config.clone()
    }
}

#[allow(dead_code)]
pub struct Application {
    container: Container,
}

impl Application {
    pub fn new() -> Self {
        Self {
            container: Container::new(),
        }
    }

    fn build_router(&self, state: AppState, session_store: LuminaSessionStore) -> AxumRouter {
        let kernel = crate::http::kernel::HttpKernel::new();
        
        let session_layer = SessionManagerLayer::new(session_store)
            .with_secure(false) // Set to true in production with HTTPS
            .with_expiry(Expiry::OnInactivity(Duration::minutes(5)));

        let csrf_layer = CsrfLayer::new(csrf::config());

        let mut web = crate::web_routes::register(&state.config);
        web = kernel.middleware_groups(web, "web");

        let mut api = crate::api_routes::register(&state.config);
        api = kernel.middleware_groups(api, "api");

        let router = Router::new()
            .merge(web)
            .nest("/api", api);

        // Terapkan Global Middleware dari Kernel
        let router = kernel.global_middleware(router);

        router.into_axum()
            .route("/lumina/echo", axum::routing::get(crate::core::echo::handler::echo_handler))
            .nest_service("/js", ServeDir::new("resources/js"))
            .nest_service("/images", ServeDir::new("resources/images"))
            .nest_service("/storage", tower_http::services::ServeDir::new("storage/app/public"))
            .fallback(ErrorController::not_found)
            .with_state(state.clone())
            // ── Middleware dasar (urutan: dari luar ke dalam) ──────────────
            .layer(from_fn_with_state(state.clone(), crate::core::rate_limit::rate_limit_middleware))
            .layer(from_fn_with_state(state.clone(), crate::core::i18n::localization_middleware))
            .layer(from_fn_with_state(state.clone(), crate::core::telescope::telescope_middleware))
            .layer(from_fn(logger))
            .layer(CatchPanicLayer::custom(crate::support::debug::handle_panic))
            .layer(from_fn_with_state(state.clone(), db_guard))
            .layer(csrf_layer)
            .layer(session_layer)
    }

    pub async fn serve(self, addr: &str) {
        // ── 1. Inisialisasi Config & View Engine ───────────────────────────
        let config = Arc::new(crate::core::config::ConfigManager::new());
        let lang = Arc::new(crate::core::i18n::LangManager::new(config.get_or("APP_LOCALE", "en")));
        let view = ViewEngine::new(Some(lang.clone()));
        let cache = Arc::new(crate::core::cache::CacheManager::new());
        let events = Arc::new(crate::core::event::EventDispatcher::new());
        crate::app::providers::event_service_provider::register_events(&events);
        let telescope = Arc::new(crate::core::telescope::TelescopeManager::new());
        let echo = Arc::new(crate::core::echo::EchoManager::new());
        
        let mut socialite = crate::core::auth::socialite::SocialiteManager::new();
        if let (Some(id), Some(secret), Some(url)) = (
            config.get("GOOGLE_CLIENT_ID"),
            config.get("GOOGLE_CLIENT_SECRET"),
            config.get("GOOGLE_REDIRECT_URL"),
        ) {
            socialite.register(Arc::new(crate::core::auth::socialite::GoogleProvider::new(id, secret, url)));
        }
        let socialite = Arc::new(socialite);

        // ── 2. Inisialisasi Job Registry ──────────────────────────────────
        let registry = crate::core::queue::JobRegistry::new();
        // Daftarkan job global di sini jika ada
        let registry = Arc::new(registry);

        // ── 3. Coba connect ke database ───────────────────────────────────
        let db_url = config.get_db_url();

        let (db_pool, auth_service, db_error) = match DatabasePool::connect(&db_url).await {
            Ok(pool) => {
                // ── 3a. Koneksi sukses → jalankan migrasi ─────────────────
                println!("🔄 Running migrations...");
                match migration::run_migrations(&pool.pool, pool.kind).await {
                    Ok(_) => println!("✅ Migrations done."),
                    Err(e) => {
                        eprintln!("⚠️  Migration gagal: {}. Server tetap lanjut...", e);
                    }
                }

                let pool_arc = Arc::new(pool);
                let svc = Arc::new(
                    crate::app::services::auth_service::AuthService::new(pool_arc.clone())
                );
                (Some(pool_arc), Some(svc), None)
            }
            Err(e) => {
                // ── 3b. Koneksi gagal → server tetap lanjut, catat error ──
                let msg = format!("{}", e);
                eprintln!();
                eprintln!("╔══════════════════════════════════════════════════════╗");
                eprintln!("║  ⚠️  DATABASE TIDAK TERSEDIA                         ║");
                eprintln!("╠══════════════════════════════════════════════════════╣");
                let truncated = if msg.len() > 52 { format!("{}...", &msg[..49]) } else { msg.clone() };
                eprintln!("║  {:<52}  ║", truncated);
                eprintln!("╠══════════════════════════════════════════════════════╣");
                eprintln!("║  Server tetap berjalan — buka browser untuk detail.  ║");
                eprintln!("╚══════════════════════════════════════════════════════╝");
                eprintln!();
                (None, None, Some(msg))
            }
        };

        // ── 4. Bangun AppState ─────────────────────────────────────────────
        let queue_manager = match db_pool.as_ref() {
            Some(pool) => Arc::new(crate::core::queue::QueueManager::new(pool.pool.clone())),
            None => {
                // Fallback jika DB tidak ada (tidak bisa persistent)
                // Ini butuh penanganan lebih lanjut jika ingin benar-benar fallback ke memory
                Arc::new(crate::core::queue::QueueManager::new(sqlx::AnyPool::connect("sqlite::memory:").await.unwrap()))
            }
        };

        let worker = crate::core::queue::QueueWorker::new(registry.clone());

        // ── 4. Bangun Storage Manager ──────────────────────────────────────
        let default_disk = std::env::var("STORAGE_DISK").unwrap_or_else(|_| "local".to_string());
        let mut storage_manager = crate::core::storage::Storage::new(&default_disk);
        
        // Disk Lokal
        storage_manager.add_disk("local", Arc::new(
            crate::core::storage::LocalStorage::new("storage/app/public", "/storage")
        ));

        // Disk S3 (Placeholder config)
        if let Ok(bucket) = std::env::var("S3_BUCKET") {
            storage_manager.add_disk("s3", Arc::new(
                crate::core::storage::S3Storage {
                    bucket,
                    region: std::env::var("S3_REGION").unwrap_or_else(|_| "us-east-1".to_string()),
                }
            ));
        }

        // ── 5. Bangun Mailer ──────────────────────────────────────────────
        let mail_driver_str = std::env::var("MAIL_DRIVER").unwrap_or_else(|_| "log".to_string());
        let mail_driver: Arc<dyn crate::core::mail::MailDriver> = if mail_driver_str == "smtp" {
            Arc::new(crate::core::mail::SmtpDriver)
        } else {
            Arc::new(crate::core::mail::LogDriver)
        };
        let mail_manager = Arc::new(crate::core::mail::Mail::new(mail_driver, view.clone()));

        let state = AppState {
            db: db_pool.clone(),
            view,
            auth_service,
            db_error,
            csrf_config: csrf::config(),
            storage: Arc::new(storage_manager),
            config: config.clone(),
            queue: queue_manager,
            cache,
            mail: mail_manager,
            registry: registry.clone(),
            events: events.clone(),
            telescope,
            lang,
            echo,
            socialite,
        };

        let state_arc = Arc::new(state);

        // ── 5. Inisialisasi Session Store (Persistent) ──────────────────────
        let session_db_url = config.get_db_url();
        let mut session_store = LuminaSessionStore::Memory(MemoryStore::default());

        if db_pool.is_some() {
            if session_db_url.starts_with("mysql:") {
                if let Ok(pool) = sqlx::MySqlPool::connect(&session_db_url).await {
                    let store = MySqlStore::new(pool);
                    let _ = store.migrate().await;
                    session_store = LuminaSessionStore::MySql(store);
                    println!("💾 Session Store: Persistent (MySQL)");
                }
            } else if session_db_url.starts_with("postgres:") || session_db_url.starts_with("postgresql:") {
                if let Ok(pool) = sqlx::PgPool::connect(&session_db_url).await {
                    let store = PostgresStore::new(pool);
                    let _ = store.migrate().await;
                    session_store = LuminaSessionStore::Postgres(store);
                    println!("💾 Session Store: Persistent (PostgreSQL)");
                }
            } else if session_db_url.starts_with("sqlite:") {
                if let Ok(pool) = sqlx::SqlitePool::connect(&session_db_url).await {
                    let store = SqliteStore::new(pool);
                    let _ = store.migrate().await;
                    session_store = LuminaSessionStore::Sqlite(store);
                    println!("💾 Session Store: Persistent (SQLite)");
                }
            }
        }

        // ── 6. Jalankan Background Worker ──────────────────────────────────
        tokio::spawn(worker.run(state_arc.clone()));

        // ── 6a. Jalankan Task Scheduler ────────────────────────────────────
        crate::app::console::kernel::run(state_arc.clone()).await;

        // ── 7. Serve HTTP ──────────────────────────────────────────────────
        println!("🌐 Listening on http://{}", addr);
        let router = self.build_router((*state_arc).clone(), session_store);
        let server = Server::new(addr.to_string());
        server.start(router).await;
    }
}

// ─── Middleware: db_guard ─────────────────────────────────────────────────────

/// Middleware yang mengecek ketersediaan database sebelum meneruskan request.
pub async fn db_guard(
    State(state): State<AppState>,
    req: axum::extract::Request,
    next: axum::middleware::Next,
) -> Response {
    if state.db.is_some() {
        return next.run(req).await;
    }

    // DB tidak tersedia — render halaman Whoops (HTTP 503) via ErrorController
    let error_msg = state
        .db_error
        .clone()
        .unwrap_or_else(|| "Unknown database error".to_string());

    let is_api = ErrorController::is_api_request(&req);
    ErrorController::render_db_error(&state, &error_msg, is_api).into_response()
}
