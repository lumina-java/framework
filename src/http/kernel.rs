use axum::middleware::from_fn;
use crate::core::application::AppState;
use crate::core::router::Router;

pub struct HttpKernel;

impl HttpKernel {
    pub fn new() -> Self {
        Self
    }

    /// Middleware Global yang akan dijalankan di SEMUA request.
    pub fn global_middleware(&self, router: Router<AppState>) -> Router<AppState> {
        router
            .layer(from_fn(crate::http::middleware::security_headers))
    }

    /// Middleware Groups (seperti 'web' atau 'api' di Laravel).
    pub fn middleware_groups(&self, router: Router<AppState>, group: &str) -> Router<AppState> {
        match group {
            "web" => {
                router
                    // .layer(from_fn(crate::http::middleware::session))
                    // .layer(from_fn(crate::http::middleware::csrf))
            },
            "api" => {
                router
                    // .layer(from_fn(crate::http::middleware::api_auth))
            },
            _ => router
        }
    }
}
