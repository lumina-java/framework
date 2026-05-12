use std::fs;
use std::path::Path;
use chrono::Local;

pub mod tinker;

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

pub async fn handle_make_service(name: &str) {
    let file_name = camel_to_snake(name);
    let path_str = format!("src/app/services/{}.rs", file_name);
    let path = Path::new(&path_str);

    if path.exists() {
        println!("❌ Service {} sudah ada!", path_str);
        return;
    }

    // Ekstrak nama model dari nama service (misal: UserService -> User)
    let model_camel = name.replace("Service", "");
    let model_snake = camel_to_snake(&model_camel);
    let table_name = format!("{}s", model_snake);

    let stub = include_str!("stubs/service.stub");
    let content = stub.replace("{{name}}", &model_camel)
                      .replace("{{snake_name}}", &model_snake)
                      .replace("{{table_name}}", &table_name);

    if let Err(e) = fs::write(path, content) {
        println!("❌ Gagal membuat service: {}", e);
    } else {
        println!("✅ Service berhasil dibuat: {}", path_str);
        let mod_file = "src/app/services/mod.rs";
        if let Ok(content) = fs::read_to_string(mod_file) {
            let mod_line = format!("pub mod {};", file_name);
            if !content.contains(&mod_line) {
                let mut f = fs::OpenOptions::new().append(true).open(mod_file).unwrap();
                use std::io::Write;
                let _ = writeln!(f, "pub mod {};", file_name);
                println!("✅ Service auto-registered in services/mod.rs");
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

    // 3. Generate Service
    let service_name = format!("{}Service", name);
    handle_make_service(&service_name).await;

    // 4. Generate Request
    let request_name = format!("{}Request", name);
    let req_file_name = camel_to_snake(&request_name);
    let req_path_str = format!("src/app/requests/{}.rs", req_file_name);
    let req_path = Path::new(&req_path_str);
    if !req_path.exists() {
        let stub = include_str!("stubs/crud_request.stub");
        let content = stub.replace("{{name}}", name);
        let _ = fs::write(req_path, content);
        println!("✅ CRUD Request berhasil dibuat: {}", req_path_str);
        
        let mod_file = "src/app/requests/mod.rs";
        if let Ok(content) = fs::read_to_string(mod_file) {
            let mod_line = format!("pub mod {};", req_file_name);
            if !content.contains(&mod_line) {
                let mut f = fs::OpenOptions::new().append(true).open(mod_file).unwrap();
                use std::io::Write;
                let _ = writeln!(f, "pub mod {};", req_file_name);
            }
        }
    }

    // 5. Generate Controller (using specialized CRUD stub)
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

    // 6. Generate Views
    let view_dir = format!("resources/views/{}", snake_name);
    let _ = fs::create_dir_all(&view_dir);

    let view_stubs = [
        ("index.blade.rs", include_str!("stubs/view_index.stub")),
        ("create.blade.rs", include_str!("stubs/view_create.stub")),
        ("edit.blade.rs", include_str!("stubs/view_edit.stub")),
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

    // 7. Auto Registration: Model, Controller, Routes, etc. (tetap sama)
    // ... (logic registration di bawah ini tetap dipertahankan) ...
    
    // (Melanjutkan dari pendaftaran mod model)
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

    // Menu registration ...
    let layout_file = "resources/views/layout.blade.rs";
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
            println!("✅ Menu item added in layout.blade.rs");
        }
    }

    // Auto Registration: Sidebar in dashboard.html
    let dashboard_file = "resources/views/dashboard.blade.rs";
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
            println!("✅ Sidebar item added in dashboard.blade.rs");
        }
    }

    // Auto Registration: Sidebar in dashboard/index.html
    let dashboard_idx_file = "resources/views/dashboard/index.blade.rs";
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
            println!("✅ Sidebar item added in dashboard/index.blade.rs");
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
        ("login.blade.rs", include_str!("stubs/view_login.stub")),
        ("register.blade.rs", include_str!("stubs/view_register.stub")),
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

pub async fn handle_make_factory(name: &str) {
    let file_name = camel_to_snake(name);
    let path_str = format!("database/factories/{}.rs", file_name);
    let path = Path::new(&path_str);

    if path.exists() {
        println!("❌ Factory {} sudah ada!", path_str);
        return;
    }

    // Ekstrak nama model dari nama factory (misal: UserFactory -> User)
    let model_camel = name.replace("Factory", "");
    let model_snake = camel_to_snake(&model_camel);

    let stub = include_str!("stubs/factory.stub");
    let content = stub.replace("{{name}}", name)
                      .replace("{{model_camel}}", &model_camel)
                      .replace("{{model_snake}}", &model_snake);

    if let Err(e) = fs::write(path, content) {
        println!("❌ Gagal membuat factory: {}", e);
    } else {
        println!("✅ Factory berhasil dibuat: {}", path_str);
        println!("📌 Jangan lupa daftarkan di database/factories/mod.rs");
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

pub async fn handle_tinker() {
    let config = crate::core::config::ConfigManager::new();
    let db_url = config.get_db_url();
    match crate::database::connection::DatabasePool::connect(&db_url).await {
        Ok(pool) => {
            if let Err(e) = tinker::run(&pool).await {
                println!("❌ Tinker error: {}", e);
            }
        }
        Err(e) => println!("❌ Gagal connect ke database: {}", e),
    }
}

pub async fn handle_make_docker() {
    let dockerfile_path = "Dockerfile";
    let docker_compose_path = "docker-compose.yml";

    let dockerfile_content = include_str!("stubs/dockerfile.stub");
    let docker_compose_content = include_str!("stubs/docker-compose.stub");

    let _ = fs::write(dockerfile_path, dockerfile_content);
    let _ = fs::write(docker_compose_path, docker_compose_content);

    println!("✅ Dockerfile & docker-compose.yml berhasil dibuat.");
    handle_make_deployment_guide().await;
}

pub async fn handle_make_nginx() {
    let nginx_dir = "nginx";
    let _ = fs::create_dir_all(nginx_dir);
    let nginx_conf_path = format!("{}/nginx.conf", nginx_dir);

    let content = include_str!("stubs/nginx.stub");
    let _ = fs::write(nginx_conf_path, content);

    println!("✅ Konfigurasi Nginx berhasil dibuat di: {}", nginx_conf_path);
    handle_make_deployment_guide().await;
}

pub async fn handle_make_supervisor() {
    let supervisor_dir = "supervisor";
    let _ = fs::create_dir_all(supervisor_dir);
    let supervisor_conf_path = format!("{}/lumina.conf", supervisor_dir);

    let content = include_str!("stubs/supervisor.stub");
    let _ = fs::write(supervisor_conf_path, content);

    println!("✅ Konfigurasi Supervisor berhasil dibuat di: {}", supervisor_conf_path);
    handle_make_deployment_guide().await;
}

async fn handle_make_deployment_guide() {
    let guide_path = "docs/DEPLOYMENT.md";
    if Path::new(guide_path).exists() { return; }
    
    let _ = fs::create_dir_all("docs");
    let content = "# Deployment Guide\n\nPanduan langkah-demi-langkah untuk deploy Lumina Framework...\n\n(Isi panduan akan segera dilengkapi di modul dokumentasi)";
    let _ = fs::write(guide_path, content);
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
