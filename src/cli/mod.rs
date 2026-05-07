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
        let mod_file = "src/app/controllers/mod.rs";
        if let Ok(content) = fs::read_to_string(mod_file) {
            let mod_line = format!("pub mod {};", file_name);
            if !content.contains(&mod_line) {
                let mut f = fs::OpenOptions::new().append(true).open(mod_file).unwrap();
                use std::io::Write;
                let _ = writeln!(f, "pub mod {};", file_name);
                println!("✅ Controller auto-registered in controllers/mod.rs");
            }
        }
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
        let mod_file = "src/app/models/mod.rs";
        if let Ok(content) = fs::read_to_string(mod_file) {
            let mod_line = format!("pub mod {};", file_name);
            if !content.contains(&mod_line) {
                let mut f = fs::OpenOptions::new().append(true).open(mod_file).unwrap();
                use std::io::Write;
                let _ = writeln!(f, "pub mod {};", file_name);
                println!("✅ Model auto-registered in models/mod.rs");
            }
        }
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

pub async fn handle_make_request(name: &str) {
    let file_name = camel_to_snake(name);
    let path_str = format!("src/app/requests/{}.rs", file_name);
    let path = Path::new(&path_str);

    if path.exists() {
        println!("❌ Request {} sudah ada!", path_str);
        return;
    }

    let stub = include_str!("stubs/request.stub");
    let content = stub.replace("{{name}}", name);

    if let Err(e) = fs::write(path, content) {
        println!("❌ Gagal membuat request: {}", e);
    } else {
        println!("✅ Request berhasil dibuat: {}", path_str);
        let mod_file = "src/app/requests/mod.rs";
        if let Ok(content) = fs::read_to_string(mod_file) {
            let mod_line = format!("pub mod {};", file_name);
            if !content.contains(&mod_line) {
                let mut f = fs::OpenOptions::new().append(true).open(mod_file).unwrap();
                use std::io::Write;
                let _ = writeln!(f, "pub mod {};", file_name);
                println!("✅ Request auto-registered in requests/mod.rs");
            }
        }
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

    // 5. Auto Registration: Model
    let mod_file = "src/app/models/mod.rs";
    if let Ok(content) = fs::read_to_string(mod_file) {
        let mod_line = format!("pub mod {};", snake_name);
        if !content.contains(&mod_line) {
            let mut f = fs::OpenOptions::new().append(true).open(mod_file).unwrap();
            use std::io::Write;
            let _ = writeln!(f, "pub mod {};", snake_name);
            println!("✅ Model registered in models/mod.rs");
        }
    }

    // 6. Auto Registration: Controller
    let ctrl_mod_file = "src/app/controllers/mod.rs";
    if let Ok(content) = fs::read_to_string(ctrl_mod_file) {
        let mod_line = format!("pub mod {}_controller;", snake_name);
        if !content.contains(&mod_line) {
            let mut f = fs::OpenOptions::new().append(true).open(ctrl_mod_file).unwrap();
            use std::io::Write;
            let _ = writeln!(f, "pub mod {}_controller;", snake_name);
            println!("✅ Controller registered in controllers/mod.rs");
        }
    }

    // 7. Auto Registration: Web Routes
    let web_routes_file = "routes/web.rs";
    if let Ok(mut content) = fs::read_to_string(web_routes_file) {
        let import_line = format!("use crate::app::controllers::{}_controller::{}Controller;\n", snake_name, name);
        if !content.contains(&import_line) {
            content.insert_str(0, &import_line);
        }
        
        let routes = format!(
            "        // {} CRUD Routes\n\
             \x20       .get(\"/{books}\", {name}Controller::index)\n\
             \x20       .get(\"/{books}/create\", {name}Controller::create)\n\
             \x20       .post(\"/{books}\", {name}Controller::store)\n\
             \x20       .get(\"/{books}/:id/edit\", {name}Controller::edit)\n\
             \x20       .post(\"/{books}/:id\", {name}Controller::update)\n\
             \x20       .get(\"/{books}/:id/delete\", {name}Controller::delete)\n\x20\x20\x20\x20\x20\x20\x20\x20",
            name,
            name = name,
            books = table_name
        );

        if !content.contains(&format!("/{books}", books = table_name)) {
            if let Some(pos) = content.find("        // Debug") {
                content.insert_str(pos, &routes);
            } else if let Some(pos) = content.find("    // ─── Protected Routes") {
                content.insert_str(pos, &routes);
            }
        }

        let _ = fs::write(web_routes_file, content);
        println!("✅ Registered routes in routes/web.rs");
    }

    // 8. Auto Registration: Sidebar / Menu in layout.html
    let layout_file = "resources/views/layout.html";
    if let Ok(mut content) = fs::read_to_string(layout_file) {
        let menu_item = format!(
            "                    <li class=\"nav-item\"><a class=\"nav-link\" href=\"/{}\">{}</a></li>\n",
            table_name, name
        );
        if !content.contains(&format!("href=\"/{}\"", table_name)) {
            if let Some(pos) = content.find("<li class=\"nav-item\"><a class=\"nav-link\" href=\"/about\">About</a></li>") {
                content.insert_str(pos + "<li class=\"nav-item\"><a class=\"nav-link\" href=\"/about\">About</a></li>\n".len(), &menu_item);
            }
            let _ = fs::write(layout_file, content);
            println!("✅ Menu item added in layout.html");
        }
    }

    // 9. Auto Registration: Sidebar in dashboard.html
    let dashboard_file = "resources/views/dashboard.html";
    if let Ok(mut content) = fs::read_to_string(dashboard_file) {
        let sidebar_item = format!(
            "            <a href=\"/{books}\" class=\"flex items-center px-4 py-3 text-slate-300 hover:bg-slate-800 hover:text-white rounded-lg transition-colors\">\n\
             \x20               <svg class=\"w-5 h-5 mr-3\" fill=\"none\" stroke=\"currentColor\" viewBox=\"0 0 24 24\"><path stroke-linecap=\"round\" stroke-linejoin=\"round\" stroke-width=\"2\" d=\"M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2m-3 7h3m-3 4h3m-6-4h.01M9 16h.01\"></path></svg>\n\
             \x20               {name}s\n\
             \x20           </a>\n",
            books = table_name,
            name = name
        );
        if !content.contains(&format!("href=\"/{}\"", table_name)) {
            if let Some(pos) = content.find("</a>\n            <a href=\"/\"") {
                content.insert_str(pos + "</a>\n".len(), &sidebar_item);
            }
            let _ = fs::write(dashboard_file, content);
            println!("✅ Sidebar item added in dashboard.html");
        }
    }

    // 10. Auto Registration: Sidebar in dashboard/index.html
    let dashboard_idx_file = "resources/views/dashboard/index.html";
    if let Ok(mut content) = fs::read_to_string(dashboard_idx_file) {
        let sidebar_item = format!(
            "            <a href=\"/{books}\"\n\
             \x20               class=\"flex items-center px-4 py-3 text-slate-300 hover:bg-slate-800 hover:text-white rounded-lg transition-all duration-200\">\n\
             \x20               <svg class=\"w-5 h-5 mr-3\" fill=\"none\" stroke=\"currentColor\" viewBox=\"0 0 24 24\"><path stroke-linecap=\"round\" stroke-linejoin=\"round\" stroke-width=\"2\" d=\"M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2m-3 7h3m-3 4h3m-6-4h.01M9 16h.01\"></path></svg>\n\
             \x20               {name}s\n\
             \x20           </a>\n",
            books = table_name,
            name = name
        );
        if !content.contains(&format!("href=\"/{}\"", table_name)) {
            if let Some(pos) = content.find("</a>\n            <a href=\"/\"") {
                content.insert_str(pos + "</a>\n".len(), &sidebar_item);
            }
            let _ = fs::write(dashboard_idx_file, content);
            println!("✅ Sidebar item added in dashboard/index.html");
        }
    }

    println!("\n🚀 CRUD Blueprint Selesai!");
}

pub async fn handle_make_auth() {
    let path_str = "src/app/controllers/auth_controller.rs";
    let path = Path::new(&path_str);

    if path.exists() {
        println!("❌ AuthController {} sudah ada!", path_str);
    } else {
        let stub = include_str!("stubs/auth_controller.stub");
        if let Err(e) = fs::write(path, stub) {
            println!("❌ Gagal membuat AuthController: {}", e);
        } else {
            println!("✅ AuthController berhasil dibuat: {}", path_str);
        }
    }

    let view_dir = "resources/views/auth";
    let _ = fs::create_dir_all(&view_dir);

    let view_stubs = [
        ("login.html", include_str!("stubs/view_login.stub")),
        ("register.html", include_str!("stubs/view_register.stub")),
    ];

    for (file, stub) in view_stubs {
        let view_path = format!("{}/{}", view_dir, file);
        if !Path::new(&view_path).exists() {
            let _ = fs::write(&view_path, stub);
            println!("✅ Auth View berhasil dibuat: {}", view_path);
        }
    }

    println!("\n🚀 Auth Scaffolding Selesai!");
    println!("📌 Selesaikan langkah berikut:");
    println!("1. Daftarkan AuthController di src/app/controllers/mod.rs");
    println!("2. Daftarkan routes autentikasi di routes/web.rs");
}

pub async fn handle_migrate() {
    let config = crate::core::config::ConfigManager::new();
    let db_url = config.get_db_url();
    println!("🔄 Menjalankan migrasi database...");
    match crate::database::connection::DatabasePool::connect(&db_url).await {
        Ok(pool) => {
            match crate::database::migration::run_migrations(&pool.pool, pool.kind).await {
                Ok(_) => println!("✅ Semua migrasi berhasil dijalankan."),
                Err(e) => println!("❌ Gagal menjalankan migrasi: {}", e),
            }
        }
        Err(e) => println!("❌ Gagal connect ke database: {}", e),
    }
}

pub async fn handle_migrate_status() {
    let config = crate::core::config::ConfigManager::new();
    let db_url = config.get_db_url();
    match crate::database::connection::DatabasePool::connect(&db_url).await {
        Ok(pool) => {
            if let Err(e) = crate::database::migration::migrate_status(&pool.pool).await {
                println!("❌ Gagal cek status migrasi: {}", e);
            }
        }
        Err(e) => println!("❌ Gagal connect ke database: {}", e),
    }
}

pub async fn handle_migrate_rollback() {
    let config = crate::core::config::ConfigManager::new();
    let db_url = config.get_db_url();
    match crate::database::connection::DatabasePool::connect(&db_url).await {
        Ok(pool) => {
            if let Err(e) = crate::database::migration::migrate_rollback(&pool.pool).await {
                println!("❌ Gagal rollback migrasi: {}", e);
            }
        }
        Err(e) => println!("❌ Gagal connect ke database: {}", e),
    }
}

pub async fn handle_make_seeder(name: &str) {
    let file_name = camel_to_snake(name);
    let path_str = format!("database/seeders/{}.rs", file_name);
    let path = Path::new(&path_str);

    if path.exists() {
        println!("❌ Seeder {} sudah ada!", path_str);
        return;
    }

    let stub = include_str!("stubs/seeder.stub");
    let content = stub.replace("{{name}}", name);

    if let Err(e) = fs::write(path, content) {
        println!("❌ Gagal membuat seeder: {}", e);
    } else {
        println!("✅ Seeder berhasil dibuat: {}", path_str);
        println!("📌 Jangan lupa daftarkan di database/seeders/mod.rs");
    }
}

pub async fn handle_db_seed() {
    let config = crate::core::config::ConfigManager::new();
    let db_url = config.get_db_url();
    match crate::database::connection::DatabasePool::connect(&db_url).await {
        Ok(pool) => {
            if let Err(e) = crate::seeders::run(&pool).await {
                println!("❌ Gagal menjalankan seeder: {}", e);
            }
        }
        Err(e) => println!("❌ Gagal connect ke database: {}", e),
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
