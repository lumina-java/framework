use lumina::prelude::*;

#[tokio::main]
async fn main() {
    std::panic::set_hook(Box::new(|info| {
        let msg = if let Some(s) = info.payload().downcast_ref::<String>() {
            s.clone()
        } else if let Some(s) = info.payload().downcast_ref::<&str>() {
            s.to_string()
        } else {
            "Unknown panic".to_string()
        };

        let loc = info.location()
            .map(|l| format!("{}:{}", l.file(), l.line()))
            .unwrap_or_else(|| "Unknown location".to_string());

        lumina::support::debug::PANIC_INFO.with(|p| {
            *p.borrow_mut() = Some((msg, loc));
        });
    }));

    if !std::path::Path::new(".env").exists() {
        println!("\x1b[1;33m⚠️  [WARNING] File .env tidak ditemukan!\x1b[0m");
        println!("\x1b[33mSilakan copy dari .env.example untuk konfigurasi dasar:\x1b[0m");
        println!("  \x1b[1mcp .env.example .env\x1b[0m\n");
    } else {
        dotenv::dotenv().ok();
        let db_url = std::env::var("DATABASE_URL").unwrap_or_default();
        let db_conn = std::env::var("DB_CONNECTION").unwrap_or_default();
        
        if db_url.is_empty() && db_conn.is_empty() {
            println!("\x1b[1;31m⚠️  [WARNING] Konfigurasi Database di .env tidak ditemukan!\x1b[0m");
            println!("\x1b[31mPastikan untuk mengisi DATABASE_URL atau DB_CONNECTION di file .env Anda, contoh:\x1b[0m");
            println!("  \x1b[1mDATABASE_URL=mysql://root:password@127.0.0.1:3306/db_name\x1b[0m");
            println!("  atau");
            println!("  \x1b[1mDB_CONNECTION=mysql\x1b[0m");
            println!("  \x1b[1mDB_HOST=127.0.0.1\x1b[0m\n");
        }
    }
    
    // Inisialisasi tracing-subscriber agar tracing::info dsb tercetak di console
    tracing_subscriber::fmt::init();

    let app = Application::new();
    
    println!("✨ Lumina Framework");
    println!("🚀 Server starting at http://127.0.0.1:8000");
    
    app.serve("127.0.0.1:8000").await;
}
