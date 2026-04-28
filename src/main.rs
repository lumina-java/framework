use lumina::prelude::*;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    
    // Inisialisasi tracing-subscriber agar tracing::info dsb tercetak di console
    tracing_subscriber::fmt::init();

    let app = Application::new();
    
    println!("✨ Lumina Framework");
    println!("🚀 Server starting at http://127.0.0.1:8000");
    
    app.serve("127.0.0.1:8000").await;
}
