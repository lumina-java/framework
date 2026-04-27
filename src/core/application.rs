use axum::{Router as AxumRouter, middleware::from_fn};
use crate::http::server::Server;
use crate::http::middleware::logger;
use crate::core::container::Container;

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

    /// Bangun axum::Router dari route files yang terdaftar,
    /// lalu terapkan logger middleware ke semua route.
    fn build_router(&self) -> AxumRouter {
        // Web routes: GET /, /about, /users, /users/:id
        let web = crate::web_routes::register().into_axum();

        // API routes: akan diprefix /api → /api/users, /api/users/:id
        let api = crate::api_routes::register().into_axum();

        AxumRouter::new()
            .merge(web)
            .nest("/api", api)
            .layer(from_fn(logger))  // Logger middleware aktif di semua route
    }

    pub async fn serve(self, addr: &str) {
        println!("🌐 Listening on http://{}", addr);

        let router = self.build_router();
        let server = Server::new(addr.to_string());
        server.start(router).await;
    }
}
