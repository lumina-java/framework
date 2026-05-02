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

pub async fn handle_make_crud(name: &str) {
    let snake_name = camel_to_snake(name);
    let table_name = format!("{}s", snake_name);

    // 1. Generate Model
    handle_make_model(name).await;

    // 2. Generate Migration
    let migration_name = format!("create_{}_table", table_name);
    handle_make_migration(&migration_name).await;

    // 3. Generate Controller (using specialized CRUD stub)
    let controller_path_str = format!("src/app/controllers/{}_controller.rs", snake_name);
    let controller_path = Path::new(&controller_path_str);
    if !controller_path.exists() {
        let stub = include_str!("stubs/crud_controller.stub");
        let content = stub.replace("{{name}}", name)
                          .replace("{{snake_name}}", &snake_name)
                          .replace("{{table_name}}", &table_name);
        let _ = fs::write(controller_path, content);
        println!("✅ CRUD Controller berhasil dibuat: {}", controller_path_str);
    }

    // 4. Generate Views
    let view_dir = format!("resources/views/{}", snake_name);
    let _ = fs::create_dir_all(&view_dir);

    let view_stubs = [
        ("index.html", include_str!("stubs/view_index.stub")),
        ("create.html", include_str!("stubs/view_create.stub")),
        ("edit.html", include_str!("stubs/view_edit.stub")),
    ];

    for (file, stub) in view_stubs {
        let view_path = format!("{}/{}", view_dir, file);
        if !Path::new(&view_path).exists() {
            let content = stub.replace("{{ name }}", name)
                              .replace("{{ snake_name }}", &snake_name);
            let _ = fs::write(&view_path, content);
            println!("✅ View berhasil dibuat: {}", view_path);
        }
    }

    println!("\n🚀 CRUD Blueprint Selesai!");
    println!("📌 Selesaikan langkah berikut:");
    println!("1. Daftarkan model di src/app/models/mod.rs");
    println!("2. Daftarkan controller di src/app/controllers/mod.rs");
    println!("3. Daftarkan route di src/web_routes.rs");
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
