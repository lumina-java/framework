use axum::{Router, routing::get};
use crate::app::controllers::home_controller::HomeController;
use crate::http::server::Server;
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

    /// Bangun router Axum dari route yang sudah didefinisikan
    fn build_router(&self) -> Router {
        Router::new()
            .route("/", get(HomeController::index))
            .route("/about", get(HomeController::about))
    }

    pub async fn serve(self, addr: &str) {
        println!("🌐 Listening on http://{}", addr);

        let router = self.build_router();
        let server = Server::new(addr.to_string());
        server.start(router).await;
    }
}
