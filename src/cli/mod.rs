use std::fs;
use std::path::Path;
use chrono::Local;

pub async fn handle_make_controller(name: &str) {
    let file_name = camel_to_snake(name);
    let path_str = format!("src/app/controllers/{}.rs", file_name);
    let path = Path::new(&path_str);

    if path.exists() {
        println!("❌ Controller {} sudah ada!", path_str);
        return;
    }

    let stub = include_str!("stubs/controller.stub");
    let content = stub.replace("{{name}}", name);

    if let Err(e) = fs::write(path, content) {
        println!("❌ Gagal membuat controller: {}", e);
    } else {
        println!("✅ Controller berhasil dibuat: {}", path_str);
        println!("📌 Jangan lupa daftarkan di src/app/controllers/mod.rs");
    }
}

pub async fn handle_make_model(name: &str) {
    let file_name = camel_to_snake(name);
    let path_str = format!("src/app/models/{}.rs", file_name);
    let path = Path::new(&path_str);

    if path.exists() {
        println!("❌ Model {} sudah ada!", path_str);
        return;
    }

    let stub = include_str!("stubs/model.stub");
    let content = stub.replace("{{name}}", name)
                      .replace("{{table_name}}", &format!("{}s", file_name));

    if let Err(e) = fs::write(path, content) {
        println!("❌ Gagal membuat model: {}", e);
    } else {
        println!("✅ Model berhasil dibuat: {}", path_str);
        println!("📌 Jangan lupa daftarkan di src/app/models/mod.rs");
    }
}

pub async fn handle_make_migration(name: &str) {
    let timestamp = Local::now().format("%Y%m%d%H%M%S");
    let path_str = format!("database/migrations/{}_{}.sql", timestamp, name);
    let path = Path::new(&path_str);

    if let Err(e) = fs::write(path, "-- Migration file created by Lumina CLI\n") {
        println!("❌ Gagal membuat migration: {}", e);
    } else {
        println!("✅ Migration berhasil dibuat: {}", path_str);
    }
}

pub async fn handle_make_job(name: &str) {
    let file_name = camel_to_snake(name);
    let path_str = format!("src/app/jobs/{}.rs", file_name);
    let path = Path::new(&path_str);

    if path.exists() {
        println!("❌ Job {} sudah ada!", path_str);
        return;
    }

    let stub = include_str!("stubs/job.stub");
    let content = stub.replace("{{name}}", name);

    if let Err(e) = fs::write(path, content) {
        println!("❌ Gagal membuat job: {}", e);
    } else {
        println!("✅ Job berhasil dibuat: {}", path_str);
        println!("📌 Jangan lupa daftarkan di src/app/jobs/mod.rs");
    }
}

fn camel_to_snake(s: &str) -> String {
    let mut snake = String::new();
    for (i, c) in s.chars().enumerate() {
        if c.is_uppercase() && i > 0 {
            snake.push('_');
        }
        snake.push(c.to_lowercase().next().unwrap());
    }
    snake
}
