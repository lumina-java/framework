use axum::{serve, Router};
use tokio::net::TcpListener;
use tokio::signal;

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    pub async fn start(&self, router: Router) {
        let listener = TcpListener::bind(&self.addr)
            .await
            .expect(&format!("❌ Gagal bind ke {}", self.addr));

        println!("🔥 HTTP Server running on http://{}", self.addr);

        serve(listener, router)
            .with_graceful_shutdown(shutdown_signal())
            .await
            .expect("❌ Server error");
    }
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    println!("🛑 Graceful shutdown signal received, shutting down server gracefully...");
}
