use axum::{
    Router as AxumRouter,
    middleware::{from_fn, from_fn_with_state},
    extract::State,
    response::{IntoResponse, Response},
};
use tower_http::catch_panic::CatchPanicLayer;
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
        let session_layer = SessionManagerLayer::new(session_store)
            .with_secure(false) // Set to true in production with HTTPS
            .with_expiry(Expiry::OnInactivity(Duration::minutes(5)));

        let csrf_layer = CsrfLayer::new(csrf::config());

        let web = crate::web_routes::register(&state.config).into_axum();
        let api = crate::api_routes::register(&state.config).into_axum();

        AxumRouter::new()
            .merge(web)
            .nest("/api", api)
            // ── Serve Storage public folder ─────────────────────────────────
            .nest_service("/storage", tower_http::services::ServeDir::new("storage/app/public"))
            // ── Fallback 404 ────────────────────────────────────────────────
            .fallback(ErrorController::not_found)
            .with_state(state.clone())
            // ── Middleware stack (urutan: dari luar ke dalam) ──────────────
            .layer(from_fn(logger))
            // Catch Panic Layer — menangkap panic dan mengembalikan 500 / Halaman DD
            .layer(CatchPanicLayer::custom(crate::support::debug::handle_panic))
            // db_guard — intercept semua request jika DB tidak tersedia
            .layer(from_fn_with_state(state.clone(), db_guard))
            // CSRF Layer
            .layer(csrf_layer)
            // Session Layer
            .layer(session_layer)
    }

    pub async fn serve(self, addr: &str) {
        // ── 1. Inisialisasi Config & View Engine ───────────────────────────
        let config = Arc::new(crate::core::config::ConfigManager::new());
        let view = ViewEngine::new();
        let cache = Arc::new(crate::core::cache::CacheManager::new());

        // ── 2. Inisialisasi Queue System ──────────────────────────────────
        let (tx, rx) = tokio::sync::mpsc::channel(100);
        let queue_manager = Arc::new(crate::core::queue::QueueManager::new(tx));
        let worker = crate::core::queue::QueueWorker::new(rx);

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
        let state = AppState {
            db: db_pool.clone(),
            view,
            auth_service,
            db_error,
            csrf_config: csrf::config(),
            storage: Arc::new(crate::core::storage::Storage::new_local("storage/app/public", "/storage")),
            config: config.clone(),
            queue: queue_manager,
            cache,
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
