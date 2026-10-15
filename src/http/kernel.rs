<<<<<<< HEAD
use crate::core::application::AppState;
use crate::core::router::Router;
use axum::middleware::from_fn;
=======
use axum::middleware::from_fn;
use crate::core::application::AppState;
use crate::core::router::Router;
>>>>>>> dev/master

pub struct HttpKernel;

impl HttpKernel {
    pub fn new() -> Self {
        Self
    }

    /// Middleware Global yang akan dijalankan di SEMUA request.
    pub fn global_middleware(&self, router: Router<AppState>) -> Router<AppState> {
<<<<<<< HEAD
        router.layer(from_fn(crate::http::middleware::security_headers))
=======
        router
            .layer(from_fn(crate::http::middleware::security_headers))
>>>>>>> dev/master
    }

    /// Middleware Groups (seperti 'web' atau 'api' di Laravel).
    pub fn middleware_groups(&self, router: Router<AppState>, group: &str) -> Router<AppState> {
        match group {
            "web" => {
                router
<<<<<<< HEAD
                // .layer(from_fn(crate::http::middleware::session))
                // .layer(from_fn(crate::http::middleware::csrf))
            }
            "api" => {
                router
                // .layer(from_fn(crate::http::middleware::api_auth))
            }
            _ => router,
=======
                    // .layer(from_fn(crate::http::middleware::session))
                    // .layer(from_fn(crate::http::middleware::csrf))
            },
            "api" => {
                router
                    // .layer(from_fn(crate::http::middleware::api_auth))
            },
            _ => router
>>>>>>> dev/master
        }
    }
}
