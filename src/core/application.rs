use axum::{Router as AxumRouter, middleware::from_fn};
use crate::http::server::Server;
use crate::http::middleware::logger;
use crate::core::container::Container;
use crate::database::{connection::DatabasePool, migration};
use crate::support::env;
use crate::core::view::ViewEngine;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<DatabasePool>,
    pub view: ViewEngine,
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
        let web = crate::web_routes::register().into_axum();
        let api = crate::api_routes::register().into_axum();

        AxumRouter::new()
            .merge(web)
            .nest("/api", api)
            .with_state(state)
            .layer(from_fn(logger))
    }

    pub async fn serve(self, addr: &str) {
        // 1. Inisialisasi koneksi database
        let db_url = env("DATABASE_URL", "sqlite:./lumina.db");
        let pool = DatabasePool::connect(&db_url)
            .await
            .expect("❌ Gagal connect ke database");

        // 2. Inisialisasi View Engine (Tera)
        let view = ViewEngine::new();

        // 3. Jalankan migrasi otomatis
        println!("🔄 Running migrations...");
        migration::run_migrations(&pool.pool)
            .await
            .expect("❌ Migration failed");
        println!("✅ Migrations done.");

        // 4. Inisialisasi AppState
        let state = Arc::new(AppState {
            db: Arc::new(pool),
            view,
        });

        // 5. Serve HTTP
        println!("🌐 Listening on http://{}", addr);
        let router = self.build_router(state);
        let server = Server::new(addr.to_string());
        server.start(router).await;
    }
}
