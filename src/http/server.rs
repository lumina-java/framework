use axum::{serve, Router};
use tokio::net::TcpListener;

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

        serve(listener, router).await.expect("❌ Server error");
    }
}
