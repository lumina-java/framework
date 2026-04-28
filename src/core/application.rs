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
use crate::support::env;
use crate::core::view::ViewEngine;
use crate::app::controllers::error_controller::ErrorController;
use std::sync::Arc;
use tower_sessions::{SessionManagerLayer, Expiry, MemoryStore};
use time::Duration;

// ─── AppState ────────────────────────────────────────────────────────────────

/// State global yang dibagikan ke semua handler.
#[derive(Clone)]
pub struct AppState {
    pub db: Option<Arc<DatabasePool>>,
    pub view: ViewEngine,
    pub auth_service: Option<Arc<crate::app::services::auth_service::AuthService>>,
    /// Pesan error database yang akan ditampilkan di halaman whoops.
    pub db_error: Option<String>,
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

    fn build_router(&self, state: Arc<AppState>) -> AxumRouter {
        // Initialize Session Store (MemoryStore for now)
        let session_store = MemoryStore::default();
        let session_layer = SessionManagerLayer::new(session_store)
            .with_secure(false) // Set to true in production with HTTPS
            .with_expiry(Expiry::OnInactivity(Duration::days(1)));

        let web = crate::web_routes::register().into_axum();
        let api = crate::api_routes::register().into_axum();

        AxumRouter::new()
            .merge(web)
            .nest("/api", api)
            // ── Fallback 404 — menangkap semua route yang tidak terdaftar ──
            .fallback(ErrorController::not_found)
            .with_state(state.clone())
            // ── Middleware stack (urutan: dari luar ke dalam) ──────────────
            .layer(from_fn(logger))
            // Catch Panic Layer — menangkap panic dan mengembalikan 500
            .layer(CatchPanicLayer::new())
            // db_guard — intercept semua request jika DB tidak tersedia
            .layer(from_fn_with_state(state.clone(), db_guard))
            // Session Layer
            .layer(session_layer)
    }

    pub async fn serve(self, addr: &str) {
        // ── 1. Inisialisasi View Engine terlebih dahulu (tidak butuh DB) ──
        let view = ViewEngine::new();

        // ── 2. Coba connect ke database ───────────────────────────────────
        let db_url = env("DATABASE_URL", "sqlite:./lumina.db");

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
        let state = Arc::new(AppState {
            db: db_pool,
            view,
            auth_service,
            db_error,
        });

        // ── 5. Serve HTTP ──────────────────────────────────────────────────
        println!("🌐 Listening on http://{}", addr);
        let router = self.build_router(state);
        let server = Server::new(addr.to_string());
        server.start(router).await;
    }
}

// ─── Middleware: db_guard ─────────────────────────────────────────────────────

/// Middleware yang mengecek ketersediaan database sebelum meneruskan request.
pub async fn db_guard(
    State(state): State<Arc<AppState>>,
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
