use chrono::Local;
use std::fs;
use std::path::Path;

pub mod tinker;
pub mod skeleton;

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
    let content = stub
        .replace("{{name}}", name)
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

pub async fn handle_make_live(name: &str) {
    let file_name = camel_to_snake(name);
    let comp_path_str = format!("src/app/live/{}.rs", file_name);
    let view_path_str = format!("resources/views/live/{}.blade.rs", file_name);

    let _ = fs::create_dir_all("src/app/live");
    let _ = fs::create_dir_all("resources/views/live");

    if Path::new(&comp_path_str).exists() {
        println!("❌ Live Component {} sudah ada!", comp_path_str);
        return;
    }

    let comp_content = format!(
r#"use async_trait::async_trait;
use lumina::prelude::*;
use serde::{{Deserialize, Serialize}};
use serde_json::Value;

/// Komponen Reaktif Lumina Live: {name}
/// Mendukung Tri-Dialect: `kilat:*` (Indonesia), `live:*` (Universal), `wire:*` (Laravel Legacy).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct {name} {{
    pub count: i32,
    pub title: String,
}}

impl Default for {name} {{
    fn default() -> Self {{
        Self {{
            count: 0,
            title: "{name}".to_string(),
        }}
    }}
}}

#[async_trait]
impl LiveComponent for {name} {{
    fn name(&self) -> &'static str {{
        "{file_name}"
    }}

    fn view(&self) -> &'static str {{
        "live.{file_name}"
    }}

    async fn call_action(&mut self, action: &str, _params: Vec<Value>) -> Result<(), AppError> {{
        match action {{
            "tambah" | "increment" => self.count += 1,
            "kurang" | "decrement" => if self.count > 0 {{ self.count -= 1; }},
            "reset" => self.count = 0,
            _ => return Err(AppError::BadRequest(format!("Aksi tidak dikenal: {{}}", action))),
        }}
        Ok(())
    }}
}}
"#);

    let view_content = format!(
r#"<div class="p-6 bg-white rounded-2xl shadow-xl border border-slate-100 max-w-md mx-auto my-6 text-center">
    <div class="mb-4">
        <span class="inline-flex items-center px-3 py-1 rounded-full text-xs font-semibold bg-emerald-100 text-emerald-800">
            ⚡ Lumina Live ({name})
        </span>
    </div>

    <h2 class="text-xl font-bold text-slate-700">{{{{ $title }}}}</h2>
    <div class="my-4">
        <span class="text-5xl font-black text-slate-800">{{{{ $count }}}}</span>
    </div>

    <!-- Tombol Interaksi (Mendukung kilat:klik, live:click, wire:click) -->
    <div class="flex items-center justify-center space-x-3">
        <button kilat:klik="kurang" class="w-10 h-10 flex items-center justify-center bg-rose-500 hover:bg-rose-600 text-white font-bold rounded-lg transition">-</button>
        <button live:click="reset" kilat:yakin="Yakin ingin mereset?" class="px-3 h-10 bg-slate-100 hover:bg-slate-200 text-slate-700 text-sm font-semibold rounded-lg transition">Reset</button>
        <button kilat:klik="tambah" class="w-10 h-10 flex items-center justify-center bg-emerald-500 hover:bg-emerald-600 text-white font-bold rounded-lg transition">+</button>
    </div>

    <div class="mt-4">
        <span kilat:tunggu class="text-xs text-slate-400 animate-pulse">Menyinkronkan...</span>
    </div>
</div>
"#);

    let _ = fs::write(&comp_path_str, comp_content);
    let _ = fs::write(&view_path_str, view_content);

    let mod_file = "src/app/live/mod.rs";
    let mod_line = format!("pub mod {};\npub use {}::{};\n", file_name, file_name, name);
    if Path::new(mod_file).exists() {
        let existing = fs::read_to_string(mod_file).unwrap_or_default();
        if !existing.contains(&format!("pub mod {};", file_name)) {
            use std::io::Write;
            if let Ok(mut f) = fs::OpenOptions::new().append(true).open(mod_file) {
                let _ = writeln!(f, "pub mod {};\npub use {}::{};", file_name, file_name, name);
            }
        }
    } else {
        let _ = fs::write(mod_file, format!("// Lumina Live Components Registry\n{}", mod_line));
    }

    println!("✅ Live Component berhasil dibuat: {}", comp_path_str);
    println!("✅ Live View template berhasil dibuat: {}", view_path_str);
    println!("💡 Pasang di template Blade: @kilat('{}') atau @live('{}')", file_name, file_name);
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
        let mod_dir = "src/app/requests";
        let _ = fs::create_dir_all(mod_dir);
        let mod_file = "src/app/requests/mod.rs";
        if !Path::new(mod_file).exists() {
            let _ = fs::write(mod_file, "");
        }
        if let Ok(content) = fs::read_to_string(mod_file) {
            let mod_line = format!("pub mod {};", file_name);
            if !content.contains(&mod_line) {
                let mut f = fs::OpenOptions::new().append(true).open(mod_file).unwrap();
                use std::io::Write;
                let _ = writeln!(f, "pub mod {};", file_name);
                println!("✅ Request auto-registered in requests/mod.rs");
            }
        }
        // Register in app/mod.rs
        let app_mod_file = "src/app/mod.rs";
        if let Ok(content) = fs::read_to_string(app_mod_file) {
            if !content.contains("pub mod requests;") {
                let mut f = fs::OpenOptions::new().append(true).open(app_mod_file).unwrap();
                use std::io::Write;
                let _ = writeln!(f, "pub mod requests;");
            }
        }
    }
}

pub async fn handle_new(name: &str, db: &str) {
    let db_normalized = db.to_lowercase();
    println!("✨ Memulai pembuatan project Lumina baru: {} (Database: {})...", name, db_normalized);

    let path = Path::new(name);
    if path.exists() {
        println!("❌ Error: Direktori {} sudah ada!", name);
        return;
    }

    println!("🚀 Generating Lumina Skeleton...");
    skeleton::create_full_skeleton(path, name, &db_normalized);

    // Initialize fresh git repository
    let _ = std::process::Command::new("git")
        .arg("init")
        .current_dir(path)
        .status();
    println!("✅ Initialize fresh git repository.");

    println!("\n🎉 Project {} siap digunakan!", name);
    println!("👉 Database terkonfigurasi: {}", db_normalized);
    println!("👉 Jalankan perintah berikut:");
    println!("   cd {}", name);
    println!("   cargo run\n");
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
    let content = stub
        .replace("{{name}}", &model_camel)
        .replace("{{snake_name}}", &model_snake)
        .replace("{{table_name}}", &table_name);

    if let Err(e) = fs::write(path, content) {
        println!("❌ Gagal membuat service: {}", e);
    } else {
        println!("✅ Service berhasil dibuat: {}", path_str);
        let mod_dir = "src/app/services";
        let _ = fs::create_dir_all(mod_dir);
        let mod_file = "src/app/services/mod.rs";
        if !Path::new(mod_file).exists() {
            let _ = fs::write(mod_file, "");
        }
        if let Ok(content) = fs::read_to_string(mod_file) {
            let mod_line = format!("pub mod {};", file_name);
            if !content.contains(&mod_line) {
                let mut f = fs::OpenOptions::new().append(true).open(mod_file).unwrap();
                use std::io::Write;
                let _ = writeln!(f, "pub mod {};", file_name);
                println!("✅ Service auto-registered in services/mod.rs");
            }
        }
        // Register in app/mod.rs
        let app_mod_file = "src/app/mod.rs";
        if let Ok(content) = fs::read_to_string(app_mod_file) {
            if !content.contains("pub mod services;") {
                let mut f = fs::OpenOptions::new().append(true).open(app_mod_file).unwrap();
                use std::io::Write;
                let _ = writeln!(f, "pub mod services;");
            }
        }
    }
}

pub async fn handle_make_crud(name: &str, field_args: Vec<String>) {
    let snake_name = camel_to_snake(name);
    let table_name = format!("{}s", snake_name);

    // Parse fields
    let fields = parse_fields(field_args);

    // 1. Generate Model
    handle_make_model_with_fields(name, &fields).await;

    // 2. Generate Migration
    let migration_name = format!("create_{}_table", table_name);
    handle_make_migration_with_fields(&migration_name, &table_name, &fields).await;

    // 3. Generate Service
    let service_name = format!("{}Service", name);
    handle_make_service_with_fields(&service_name, &fields).await;

    // 4. Generate Request
    let request_name = format!("{}Request", name);
    let req_file_name = camel_to_snake(&request_name);
    let req_path_str = format!("src/app/requests/{}.rs", req_file_name);
    let req_path = Path::new(&req_path_str);
    if !req_path.exists() {
        let stub = include_str!("stubs/crud_request.stub");

        let mut field_code = String::new();
        for field in &fields {
            let rules = if field.rules.is_empty() {
                ""
            } else {
                &format!("#[rule(\"{}\")]\n    ", field.rules)
            };
            field_code.push_str(&format!(
                "{}pub {}: {},\n    ",
                rules,
                field.name,
                field.rust_type()
            ));
        }

        let content = stub
            .replace("{{name}}", name)
            .replace("{{fields}}", &field_code);
        let _ = fs::write(req_path, content);
        println!("✅ CRUD Request berhasil dibuat: {}", req_path_str);

        let mod_dir = "src/app/requests";
        let _ = fs::create_dir_all(mod_dir);
        let mod_file = "src/app/requests/mod.rs";
        if !Path::new(mod_file).exists() {
            let _ = fs::write(mod_file, "");
        }
        if let Ok(content) = fs::read_to_string(mod_file) {
            let mod_line = format!("pub mod {};", req_file_name);
            if !content.contains(&mod_line) {
                let mut f = fs::OpenOptions::new().append(true).open(mod_file).unwrap();
                use std::io::Write;
                let _ = writeln!(f, "pub mod {};", req_file_name);
            }
        }
        // Register in app/mod.rs
        let app_mod_file = "src/app/mod.rs";
        if let Ok(content) = fs::read_to_string(app_mod_file) {
            if !content.contains("pub mod requests;") {
                let mut f = fs::OpenOptions::new().append(true).open(app_mod_file).unwrap();
                use std::io::Write;
                let _ = writeln!(f, "pub mod requests;");
            }
        }
    }

    // 5. Generate Controller (using specialized CRUD stub)
    let controller_path_str = format!("src/app/controllers/{}_controller.rs", snake_name);
    let controller_path = Path::new(&controller_path_str);
    if !controller_path.exists() {
        let stub = include_str!("stubs/crud_controller.stub");

        let mut store_logic = String::new();
        let mut update_logic = String::new();
        let mut input_map = String::new();

        for field in &fields {
            store_logic.push_str(&format!(
                "item.{} = form.{}.clone();\n        ",
                field.name, field.name
            ));
            update_logic.push_str(&format!(
                "item.{} = form.{}.clone();\n        ",
                field.name, field.name
            ));
            input_map.push_str(&format!("\"{}\": form.{}, ", field.name, field.name));
        }

        let content = stub
            .replace("{{name}}", name)
            .replace("{{snake_name}}", &snake_name)
            .replace("{{table_name}}", &table_name)
            .replace("{{store_logic}}", &store_logic)
            .replace("{{update_logic}}", &update_logic)
            .replace("{{input_map}}", &input_map);

        let _ = fs::write(controller_path, content);
        println!(
            "✅ CRUD Controller berhasil dibuat: {}",
            controller_path_str
        );
    }

    // 6. Generate Views
    let view_dir = format!("resources/views/{}", snake_name);
    let _ = fs::create_dir_all(&view_dir);

    // Index View
    let mut th_cols = String::new();
    let mut td_cols = String::new();
    for field in &fields {
        th_cols.push_str(&format!(
            "<th class=\"px-3 py-2 text-slate-400 font-bold text-[10px]\">{}</th>\n                    ",
            field.name.to_uppercase()
        ));
        td_cols.push_str(&format!(
            "<td class=\"px-3 py-2 text-slate-300 text-[11px]\">{{{{ item.{} }}}}</td>\n                    ",
            field.name
        ));
    }

    // Create/Edit Form Fields
    let mut form_fields = String::new();
    for field in &fields {
        let input_type = match field.field_type.as_str() {
            "integer" => "number",
            "boolean" => "checkbox",
            _ => "text",
        };

        if field.field_type == "text" {
            form_fields.push_str(&format!(
                "<div>\n\
                 \x20   <label class=\"block font-semibold text-slate-300 mb-1 text-[11px]\">{}</label>\n\
                 \x20   <textarea name=\"{}\" class=\"w-full bg-darkBg border border-borderBg rounded px-2.5 py-1.5 text-white focus:outline-none focus:border-primary transition text-[11px]\" rows=\"3\">{{{{ item.{} | default(value=\"\") }}}}</textarea>\n\
                 </div>\n",
                field.name.to_uppercase(), field.name, field.name
            ));
        } else {
            form_fields.push_str(&format!(
                "<div>\n\
                 \x20   <label class=\"block font-semibold text-slate-300 mb-1 text-[11px]\">{}</label>\n\
                 \x20   <input type=\"{}\" name=\"{}\" value=\"{{{{ item.{} | default(value=\"\") }}}}\" class=\"w-full bg-darkBg border border-borderBg rounded px-2.5 py-1.5 text-white focus:outline-none focus:border-primary transition text-[11px]\">\n\
                 </div>\n",
                field.name.to_uppercase(), input_type, field.name, field.name
            ));
        }
    }

    let view_stubs = [
        (
            "index.blade.rs",
            include_str!("stubs/view_index.stub"),
            vec![("{{th_cols}}", th_cols), ("{{td_cols}}", td_cols)],
        ),
        (
            "create.blade.rs",
            include_str!("stubs/view_create.stub"),
            vec![("{{form_fields}}", form_fields.clone())],
        ),
        (
            "edit.blade.rs",
            include_str!("stubs/view_edit.stub"),
            vec![("{{form_fields}}", form_fields)],
        ),
    ];

    for (file, stub, replacements) in view_stubs {
        let view_path = format!("{}/{}", view_dir, file);
        if !Path::new(&view_path).exists() {
            let mut content = stub
                .replace("{{ name }}", name)
                .replace("{{ snake_name }}", &snake_name);
            for (from, to) in replacements {
                content = content.replace(from, &to);
            }
            let _ = fs::write(&view_path, content);
            println!("✅ View berhasil dibuat: {}", view_path);
        }
    }

    // 7. Auto Registration: Model, Controller, Routes, etc.
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
            let mut f = fs::OpenOptions::new()
                .append(true)
                .open(ctrl_mod_file)
                .unwrap();
            use std::io::Write;
            let _ = writeln!(f, "pub mod {}_controller;", snake_name);
            println!("✅ Controller registered in controllers/mod.rs");
        }
    }

    let web_routes_file = "routes/web.rs";
    if let Ok(mut content) = fs::read_to_string(web_routes_file) {
        let import_line = format!(
            "use crate::app::controllers::{}_controller::{}Controller;\n",
            snake_name, name
        );
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
            if let Some(pos) = content
                .find("<li class=\"nav-item\"><a class=\"nav-link\" href=\"/about\">About</a></li>")
            {
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
        (
            "register.blade.rs",
            include_str!("stubs/view_register.stub"),
        ),
    ];

    for (file, stub) in view_stubs {
        let view_path = format!("{}/{}", view_dir, file);
        if !Path::new(&view_path).exists() {
            let _ = fs::write(&view_path, stub);
            println!("✅ Auth View berhasil dibuat: {}", view_path);
        }
    }

    // 1. Ensure User model exists and has find_by_email & role
    let user_model_path = if Path::new("src/app/models/user.rs").exists() {
        "src/app/models/user.rs"
    } else {
        "app/models/user.rs"
    };

    if !Path::new(user_model_path).exists() {
        let user_model_content = r#"#![allow(dead_code)]
use async_trait::async_trait;
use serde::{Serialize, Deserialize};
use sqlx::FromRow;
use lumina::database::{connection::DatabasePool, model::Model};

#[derive(Debug, Serialize, Deserialize, FromRow, Clone, Default)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub email: String,
    pub password: String,
    pub role: String,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub deleted_at: Option<String>,
}

impl User {
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn find_by_email(pool: &DatabasePool, email: &str) -> Result<Self, sqlx::Error> {
        Self::query(pool).where_eq("email", email).first().await
    }
}

#[async_trait]
impl Model for User {
    const TABLE: &'static str = "users";

    async fn find(pool: &DatabasePool, id: i64) -> Result<Self, sqlx::Error> {
        Self::query(pool).where_eq("id", id).first().await
    }

    async fn all(pool: &DatabasePool) -> Result<Vec<Self>, sqlx::Error> {
        Self::query(pool).get().await
    }

    async fn save(&self, pool: &DatabasePool) -> Result<i64, sqlx::Error> {
        if self.id > 0 {
            sqlx::query("UPDATE users SET name = ?, email = ?, password = ?, role = ? WHERE id = ?")
                .bind(&self.name)
                .bind(&self.email)
                .bind(&self.password)
                .bind(&self.role)
                .bind(self.id)
                .execute(&pool.pool)
                .await?;
            Ok(self.id)
        } else {
            let result = sqlx::query(
                "INSERT INTO users (name, email, password, role) VALUES (?, ?, ?, ?)"
            )
            .bind(&self.name)
            .bind(&self.email)
            .bind(&self.password)
            .bind(&self.role)
            .execute(&pool.pool)
            .await?;

            Ok(result.last_insert_id().unwrap_or(0))
        }
    }

    async fn delete(pool: &DatabasePool, id: i64) -> Result<bool, sqlx::Error> {
        sqlx::query("UPDATE users SET deleted_at = CURRENT_TIMESTAMP WHERE id = ?")
            .bind(id)
            .execute(&pool.pool)
            .await?;

        Ok(true)
    }
}
"#;
        let _ = fs::write(user_model_path, user_model_content);
        println!("✅ Model User berhasil dibuat: {}", user_model_path);

        let mod_file = if Path::new("src/app/models/mod.rs").exists() {
            "src/app/models/mod.rs"
        } else {
            "app/models/mod.rs"
        };

        if let Ok(content) = fs::read_to_string(mod_file) {
            let mod_line = "pub mod user;";
            if !content.contains(mod_line) {
                let mut f = fs::OpenOptions::new().append(true).open(mod_file).unwrap();
                use std::io::Write;
                let _ = writeln!(f, "pub mod user;");
                println!("✅ Model auto-registered in models/mod.rs");
            }
        }
    } else {
        if let Ok(mut content) = fs::read_to_string(user_model_path) {
            let mut modified = false;
            if !content.contains("find_by_email") {
                if let Some(pos) = content.find("impl User {") {
                    let insert_pos = pos + "impl User {".len();
                    let inject_code = r#"
    pub async fn find_by_email(pool: &DatabasePool, email: &str) -> Result<Self, sqlx::Error> {
        Self::query(pool).where_eq("email", email).first().await
    }
"#;
                    content.insert_str(insert_pos, inject_code);
                    modified = true;
                }
            }
            if !content.contains("pub role:") && !content.contains("role:") {
                if let Some(pos) = content.find("pub password: String,") {
                    let insert_pos = pos + "pub password: String,".len();
                    content.insert_str(insert_pos, "\n    pub role: String,");
                    modified = true;
                }
            }
            if modified {
                let _ = fs::write(user_model_path, content);
                println!("✅ Model User berhasil diperbarui dengan find_by_email dan field role.");
            }
        }
    }

    // 2. Register AuthController in controllers/mod.rs
    let ctrl_mod_file = if Path::new("src/app/controllers/mod.rs").exists() {
        "src/app/controllers/mod.rs"
    } else {
        "app/controllers/mod.rs"
    };

    if let Ok(content) = fs::read_to_string(ctrl_mod_file) {
        let mod_line = "pub mod auth_controller;";
        if !content.contains(mod_line) {
            let mut f = fs::OpenOptions::new()
                .append(true)
                .open(ctrl_mod_file)
                .unwrap();
            use std::io::Write;
            let _ = writeln!(f, "pub mod auth_controller;");
            println!("✅ AuthController registered in controllers/mod.rs");
        }
    }

    // 3. Register routes in routes/web.rs
    let web_routes_file = if Path::new("src/routes/web.rs").exists() {
        "src/routes/web.rs"
    } else {
        "routes/web.rs"
    };

    if let Ok(mut content) = fs::read_to_string(web_routes_file) {
        let import_line = "use crate::app::controllers::auth_controller::AuthController;\n";
        if !content.contains(&import_line) {
            content.insert_str(0, &import_line);
        }

        let mut routes = String::new();
        routes.push_str("        // Authentication Routes\n");
        routes.push_str("        .get(\"/login\",          AuthController::show_login)\n");
        routes.push_str("        .post(\"/login\",         AuthController::login)\n");
        routes.push_str("        .get(\"/register\",       AuthController::show_register)\n");
        routes.push_str("        .post(\"/register\",      AuthController::register)\n");
        routes.push_str("        .get(\"/logout\",         AuthController::logout)\n");
        routes.push_str("        .get(\"/auth/login\",     AuthController::show_login)\n");
        routes.push_str("        .post(\"/auth/login\",    AuthController::login)\n");
        routes.push_str("        .get(\"/auth/register\",  AuthController::show_register)\n");
        routes.push_str("        .post(\"/auth/register\", AuthController::register)\n");
        routes.push_str("        .get(\"/auth/logout\",    AuthController::logout)\n        ");

        if !content.contains("AuthController::show_login") {
            // Remove the old /login and /register routes from welcome controller if they are present (checking all whitespace options)
            content = content.replace(".get(\"/login\", WelcomeController::login)", "");
            content = content.replace(".get(\"/register\", WelcomeController::register)", "");
            content = content.replace(".get(\"/login\",WelcomeController::login)", "");
            content = content.replace(".get(\"/register\",WelcomeController::register)", "");
            content = content.replace(".get(\"/login\",       WelcomeController::login)", "");
            content = content.replace(".get(\"/register\",    WelcomeController::register)", "");
            content = content.replace(".get(\"/login\",            WelcomeController::login)", "");
            content = content.replace(".get(\"/register\",         WelcomeController::register)", "");

            if let Some(pos) = content.find("        .get(\"/dashboard\"") {
                content.insert_str(pos, &routes);
            } else if let Some(pos) = content.find("        // Debug") {
                content.insert_str(pos, &routes);
            } else if let Some(pos) = content.find("    // ─── Protected Routes") {
                content.insert_str(pos, &routes);
            } else if let Some(_) = content.find("}") {
                let last_brace = content.rfind("}").unwrap();
                content.insert_str(last_brace, &routes);
            }
            let _ = fs::write(web_routes_file, content);
            println!("✅ Registered authentication routes in {}", web_routes_file);
        }
    }

    println!("\n🚀 Auth Scaffolding Selesai dengan Sempurna!");
}

pub async fn handle_migrate() {
    let config = crate::core::config::ConfigManager::new();
    let db_url = config.get_db_url();
    println!("🔄 Menjalankan migrasi database...");
    match crate::database::connection::DatabasePool::connect(&db_url).await {
        Ok(pool) => match crate::database::migration::run_migrations(&pool.pool, pool.kind).await {
            Ok(_) => println!("✅ Semua migrasi berhasil dijalankan."),
            Err(e) => println!("❌ Gagal menjalankan migrasi: {}", e),
        },
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
    let content = stub
        .replace("{{name}}", name)
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
        Ok(_) => {
            println!("🌱 Please run seeding from your application main entry.");
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
    let _ = fs::write(&nginx_conf_path, content);

    println!(
        "✅ Konfigurasi Nginx berhasil dibuat di: {}",
        nginx_conf_path
    );
    handle_make_deployment_guide().await;
}

pub async fn handle_make_supervisor() {
    let supervisor_dir = "supervisor";
    let _ = fs::create_dir_all(supervisor_dir);
    let supervisor_conf_path = format!("{}/lumina.conf", supervisor_dir);

    let content = include_str!("stubs/supervisor.stub");
    let _ = fs::write(&supervisor_conf_path, content);

    println!(
        "✅ Konfigurasi Supervisor berhasil dibuat di: {}",
        supervisor_conf_path
    );
    handle_make_deployment_guide().await;
}

async fn handle_make_deployment_guide() {
    let guide_path = "docs/DEPLOYMENT.md";
    let _ = fs::create_dir_all("docs");

    let content = r#"# 🚀 Lumina Framework Deployment Guide

Panduan ini menjelaskan langkah-langkah untuk mendeploy aplikasi Lumina Framework ke lingkungan produksi dengan standar industri.

---

## 1. 🐳 Deployment dengan Docker (Direkomendasikan)

Lumina dilengkapi dengan konfigurasi Docker yang siap pakai (production-ready).

### Persiapan
1. Pastikan Docker dan Docker Compose terinstall di server.
2. Jalankan perintah scaffolding docker:
   ```bash
   ./lumina make:docker
   ```

### Menjalankan Aplikasi
Cukup jalankan satu perintah:
```bash
docker-compose up -d --build
```
Aplikasi akan berjalan di port `8000`, Nginx di port `80/443`, PostgreSQL di port `5432`, dan Redis di port `6379`.

---

## 2. 🛠️ Deployment Manual (Ubuntu/Debian)

Jika Anda ingin mengontrol setiap komponen secara manual di VPS.

### A. Install Dependencies
```bash
sudo apt update && sudo apt install -y pkg-config libssl-dev build-essential postgresql redis-server nginx supervisor
```

### B. Build Aplikasi
1. Clone repository ke server.
2. Build binary release:
   ```bash
   cargo build --release
   ```
3. Binary akan berada di `target/release/lumina-server`.

### C. Konfigurasi Database & Environment
1. Salin `.env.example` menjadi `.env`.
2. Update `DATABASE_URL` dan `APP_ENV=production`.
3. Jalankan migrasi:
   ```bash
   ./target/release/lumina migrate
   ```

---

## 3. 🛡️ Konfigurasi Nginx (Reverse Proxy)

Gunakan Nginx untuk menangani traffic HTTP/HTTPS dan static files.

1. Jalankan `./lumina make:nginx`.
2. Salin isi `nginx/nginx.conf` ke `/etc/nginx/sites-available/lumina`.
3. Aktifkan site dan restart Nginx:
   ```bash
   sudo ln -s /etc/nginx/sites-available/lumina /etc/nginx/sites-enabled/
   sudo nginx -t
   sudo systemctl restart nginx
   ```

---

## 4. 🔄 Process Management (Supervisor)

Gunakan Supervisor untuk memastikan aplikasi tetap berjalan (auto-restart) jika terjadi crash.

1. Jalankan `./lumina make:supervisor`.
2. Salin konfigurasi ke folder Supervisor:
   ```bash
   sudo cp supervisor/lumina.conf /etc/supervisor/conf.d/
   sudo supervisorctl reread
   sudo supervisorctl update
   sudo supervisorctl start all
   ```

---

## 5. 📝 Production Checklist

- [ ] **Security**: Ganti `APP_KEY` dan pastikan password database kuat.
- [ ] **SSL**: Gunakan Certbot untuk mendapatkan sertifikat SSL gratis.
- [ ] **Logging**: Cek log di `storage/logs/` secara berkala.
- [ ] **Backups**: Jalankan `./lumina backup:run` secara terjadwal menggunakan Cron.

---

Lumina Framework - *Advanced Agentic Coding*
"#;

    let _ = fs::write(guide_path, content);
}

pub async fn handle_backup() {
    println!("🔄 Memulai proses backup...");
    let config = crate::core::config::ConfigManager::new();
    let db_url = config.get_db_url();
    let db_conn = std::env::var("DB_CONNECTION").unwrap_or_else(|_| "sqlite".to_string());

    let manager = crate::core::backup::BackupManager::new();
    match manager.run_full_backup(&db_conn, &db_url).await {
        Ok(path) => println!("✅ Backup berhasil dibuat: {}", path),
        Err(e) => println!("❌ Backup gagal: {}", e),
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

struct Field {
    name: String,
    field_type: String,
    rules: String,
}

impl Field {
    fn rust_type(&self) -> &str {
        match self.field_type.as_str() {
            "integer" => "i64",
            "boolean" => "bool",
            "date" => "chrono::NaiveDate",
            _ => "String",
        }
    }

    fn sql_type(&self) -> &str {
        match self.field_type.as_str() {
            "integer" => "INTEGER",
            "boolean" => "BOOLEAN",
            "date" => "DATE",
            "text" => "TEXT",
            _ => "VARCHAR(255)",
        }
    }
}

fn parse_fields(args: Vec<String>) -> Vec<Field> {
    let mut fields = Vec::new();
    for arg in args {
        let parts: Vec<&str> = arg.split(':').collect();
        if parts.len() >= 2 {
            fields.push(Field {
                name: parts[0].to_string(),
                field_type: parts[1].to_string(),
                rules: if parts.len() >= 3 {
                    parts[2..].join("|")
                } else {
                    String::new()
                },
            });
        }
    }

    // Add default name if no fields provided
    if fields.is_empty() {
        fields.push(Field {
            name: "name".to_string(),
            field_type: "string".to_string(),
            rules: "required|min:3".to_string(),
        });
    }

    fields
}

async fn handle_make_model_with_fields(name: &str, fields: &[Field]) {
    let file_name = camel_to_snake(name);
    let path_str = format!("src/app/models/{}.rs", file_name);
    let path = Path::new(&path_str);

    if path.exists() {
        println!("❌ Model {} sudah ada!", path_str);
        return;
    }

    let stub = include_str!("stubs/model.stub");

    let mut field_code = String::new();
    let mut sql_fields = String::new();
    let mut sql_placeholders = String::new();
    let mut sql_update = String::new();
    let mut bind_fields = String::new();

    for field in fields {
        field_code.push_str(&format!("pub {}: {},\n    ", field.name, field.rust_type()));
        sql_fields.push_str(&format!("{}, ", field.name));
        sql_placeholders.push_str("?, ");
        sql_update.push_str(&format!("{} = ?, ", field.name));
        bind_fields.push_str(&format!(".bind(&self.{})", field.name));
    }

    let content = stub
        .replace("{{name}}", name)
        .replace("{{table_name}}", &format!("{}s", file_name))
        .replace("{{fields}}", &field_code)
        .replace("{{sql_fields}}", sql_fields.trim_end_matches(", "))
        .replace(
            "{{sql_placeholders}}",
            sql_placeholders.trim_end_matches(", "),
        )
        .replace("{{sql_update}}", sql_update.trim_end_matches(", "))
        .replace("{{bind_fields}}", &bind_fields);

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
            }
        }
    }
}

async fn handle_make_migration_with_fields(name: &str, table_name: &str, fields: &[Field]) {
    let timestamp = Local::now().format("%Y%m%d%H%M%S");
    let path_str = format!("database/migrations/{}_{}.sql", timestamp, name);
    let path = Path::new(&path_str);

    let mut sql = format!(
        "CREATE TABLE {} (\n    id INTEGER PRIMARY KEY AUTOINCREMENT,\n",
        table_name
    );
    for field in fields {
        sql.push_str(&format!("    {} {},\n", field.name, field.sql_type()));
    }
    sql.push_str("    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,\n");
    sql.push_str("    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP\n);");

    if let Err(e) = fs::write(path, sql) {
        println!("❌ Gagal membuat migration: {}", e);
    } else {
        println!("✅ Migration berhasil dibuat: {}", path_str);
    }
}

async fn handle_make_service_with_fields(name: &str, _fields: &[Field]) {
    let file_name = camel_to_snake(name);
    let path_str = format!("src/app/services/{}.rs", file_name);
    let path = Path::new(&path_str);

    if path.exists() {
        println!("❌ Service {} sudah ada!", path_str);
        return;
    }

    let model_camel = name.replace("Service", "");
    let model_snake = camel_to_snake(&model_camel);
    let table_name = format!("{}s", model_snake);

    let stub = include_str!("stubs/service.stub");

    let content = stub
        .replace("{{name}}", &model_camel)
        .replace("{{snake_name}}", &model_snake)
        .replace("{{table_name}}", &table_name);

    if let Err(e) = fs::write(path, content) {
        println!("❌ Gagal membuat service: {}", e);
    } else {
        println!("✅ Service berhasil dibuat: {}", path_str);
        let mod_dir = "src/app/services";
        let _ = fs::create_dir_all(mod_dir);
        let mod_file = "src/app/services/mod.rs";
        if !Path::new(mod_file).exists() {
            let _ = fs::write(mod_file, "");
        }
        if let Ok(content) = fs::read_to_string(mod_file) {
            let mod_line = format!("pub mod {};", file_name);
            if !content.contains(&mod_line) {
                let mut f = fs::OpenOptions::new().append(true).open(mod_file).unwrap();
                use std::io::Write;
                let _ = writeln!(f, "pub mod {};", file_name);
            }
        }
        // Register in app/mod.rs
        let app_mod_file = "src/app/mod.rs";
        if let Ok(content) = fs::read_to_string(app_mod_file) {
            if !content.contains("pub mod services;") {
                let mut f = fs::OpenOptions::new().append(true).open(app_mod_file).unwrap();
                use std::io::Write;
                let _ = writeln!(f, "pub mod services;");
            }
        }
    }
}

pub async fn handle_plugin_new(name: &str) {
    let clean_name = name.trim().to_lowercase().replace("-", "_");
    if clean_name.is_empty() {
        println!("❌ Nama plugin tidak boleh kosong!");
        return;
    }

    let plugin_dir = Path::new("plugins").join(&clean_name);
    if plugin_dir.exists() {
        println!("❌ Plugin '{}' sudah ada di folder {:?}", clean_name, plugin_dir);
        return;
    }

    let src_dir = plugin_dir.join("src");
    if let Err(e) = fs::create_dir_all(&src_dir) {
        println!("❌ Gagal membuat direktori plugin: {}", e);
        return;
    }

    // 1. Cargo.toml
    let cargo_toml = format!(
        r#"[package]
name = "{}"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
"#,
        clean_name
    );
    let _ = fs::write(plugin_dir.join("Cargo.toml"), cargo_toml);

    // 2. src/lib.rs
    let lib_rs = format!(
        r#"use std::mem;

/// Alokasi buffer memori heap guest untuk transfer data dari host Lumina.
#[no_mangle]
pub extern "C" fn alloc(len: i32) -> i32 {{
    let mut buf = Vec::with_capacity(len as usize);
    let ptr = buf.as_mut_ptr();
    mem::forget(buf);
    ptr as i32
}}

/// Dealokasi buffer memori heap guest (opsional).
#[no_mangle]
pub extern "C" fn dealloc(ptr: i32, len: i32) {{
    unsafe {{
        let _ = Vec::from_raw_parts(ptr as *mut u8, len as usize, len as usize);
    }}
}}

/// Fungsi filter transformasi untuk dipanggil dari Blade:
/// {{{{ $data | {} }}}} atau @wasm('{}', 'transform', $data)
/// Return: (out_ptr << 32) | (out_len & 0xFFFF_FFFF)
#[no_mangle]
pub extern "C" fn transform(ptr: i32, len: i32) -> i64 {{
    let input = unsafe {{
        let slice = std::slice::from_raw_parts(ptr as *const u8, len as usize);
        std::str::from_utf8(slice).unwrap_or("")
    }};

    // Logika transformasi default: bungkus dengan tanda kurung siku (silakan modifikasi sesuai kebutuhan)
    let output = format!("[{{}}]", input);
    let out_bytes = output.into_bytes();
    let out_len = out_bytes.len() as i64;
    let out_ptr = out_bytes.as_ptr() as i64;

    mem::forget(out_bytes);

    (out_ptr << 32) | (out_len & 0xFFFF_FFFF)
}}
"#,
        clean_name, clean_name
    );
    let _ = fs::write(src_dir.join("lib.rs"), lib_rs);

    // 3. README.md
    let readme = format!(
        r#"# Lumina WASM Plugin: {}

Plugin WebAssembly untuk Lumina Framework.

## Cara Kompilasi:
```bash
lumina plugin:build {}
```
Binary output `.wasm` akan otomatis disalin ke `plugins/{}.wasm`.

## Penggunaan di Blade View:
```html
{{{{ $data | {} }}}}
@wasm('{}', 'transform', $data)
```
"#,
        clean_name, clean_name, clean_name, clean_name, clean_name
    );
    let _ = fs::write(plugin_dir.join("README.md"), readme);

    println!("✨ Plugin WASM '{}' berhasil dibuat di folder plugins/{}/", clean_name, clean_name);
    println!("👉 Kompilasi dengan perintah:");
    println!("   lumina plugin:build {}\n", clean_name);
}

pub async fn handle_plugin_build(name: Option<&str>) {
    println!("🔨 Memulai kompilasi plugin WebAssembly Lumina...");

    let plugins_to_build: Vec<String> = if let Some(n) = name {
        vec![n.trim().to_lowercase().replace("-", "_")]
    } else {
        // Scan folder plugins/
        let mut list = Vec::new();
        if let Ok(entries) = fs::read_dir("plugins") {
            for entry in entries.flatten() {
                let p = entry.path();
                if p.is_dir() && p.join("Cargo.toml").exists() {
                    if let Some(folder_name) = p.file_name().and_then(|s| s.to_str()) {
                        list.push(folder_name.to_string());
                    }
                }
            }
        }
        list
    };

    if plugins_to_build.is_empty() {
        println!("⚠️ Tidak ditemukan plugin crate untuk dikompilasi.");
        println!("💡 Buat plugin baru dengan: lumina plugin:new <nama>");
        return;
    }

    for plugin in plugins_to_build {
        let plugin_dir = Path::new("plugins").join(&plugin);
        if !plugin_dir.join("Cargo.toml").exists() {
            println!("❌ Folder plugin 'plugins/{}' tidak memiliki Cargo.toml!", plugin);
            continue;
        }

        println!("📦 Mengompilasi plugin '{}' ke target wasm32-unknown-unknown...", plugin);

        let status = std::process::Command::new("cargo")
            .arg("build")
            .arg("--target")
            .arg("wasm32-unknown-unknown")
            .arg("--release")
            .current_dir(&plugin_dir)
            .status();

        match status {
            Ok(s) if s.success() => {
                let wasm_src = plugin_dir
                    .join("target")
                    .join("wasm32-unknown-unknown")
                    .join("release")
                    .join(format!("{}.wasm", plugin));

                let wasm_dest = Path::new("plugins").join(format!("{}.wasm", plugin));

                if wasm_src.exists() {
                    if let Ok(_) = fs::copy(&wasm_src, &wasm_dest) {
                        let size_bytes = fs::metadata(&wasm_dest).map(|m| m.len()).unwrap_or(0);
                        let size_kb = (size_bytes as f64) / 1024.0;
                        println!("✅ Berhasil! Binary tersimpan di: {:?} ({:.2} KB)", wasm_dest, size_kb);
                    } else {
                        println!("❌ Gagal menyalin binary WASM dari {:?} ke {:?}", wasm_src, wasm_dest);
                    }
                } else {
                    println!("❌ File binary output {:?} tidak ditemukan setelah build!", wasm_src);
                }
            }
            Ok(s) => {
                println!("❌ Kompilasi gagal dengan exit code: {:?}", s.code());
                println!("💡 Catatan: Pastikan target WASM terpasang: rustup target add wasm32-unknown-unknown");
            }
            Err(e) => {
                println!("❌ Gagal menjalankan cargo: {}", e);
                println!("💡 Catatan: Pastikan target WASM terpasang: rustup target add wasm32-unknown-unknown");
            }
        }
    }
}

pub async fn handle_plugin_list() {
    println!("🔍 Memindai direktori plugin WASM dan skrip Indonesian Script...\n");

    struct Item {
        name: String,
        kind: String,
        size_str: String,
        status: String,
    }

    let mut items = Vec::new();

    // 1. Scan plugins/ folder
    if let Ok(entries) = fs::read_dir("plugins") {
        for entry in entries.flatten() {
            let p = entry.path();
            if p.is_file() {
                if let Some(ext) = p.extension().and_then(|s| s.to_str()) {
                    let file_name = p.file_name().and_then(|s| s.to_str()).unwrap_or("?").to_string();
                    let size = fs::metadata(&p).map(|m| m.len()).unwrap_or(0);
                    let size_kb = format!("{:.2} KB", (size as f64) / 1024.0);

                    if ext == "wasm" {
                        items.push(Item {
                            name: file_name,
                            kind: "WASM Binary".to_string(),
                            size_str: size_kb,
                            status: "Siap Dijalankan ✅".to_string(),
                        });
                    } else if ext == "is" {
                        let content = fs::read_to_string(&p).unwrap_or_default();
                        let func_count = content.lines().filter(|l| l.trim().starts_with("fungsi ")).count();
                        items.push(Item {
                            name: file_name,
                            kind: "Skrip (.is)".to_string(),
                            size_str: size_kb,
                            status: format!("{} fungsi", func_count),
                        });
                    }
                }
            } else if p.is_dir() && p.join("Cargo.toml").exists() {
                let folder_name = p.file_name().and_then(|s| s.to_str()).unwrap_or("?").to_string();
                let wasm_exists = Path::new("plugins").join(format!("{}.wasm", folder_name)).exists();
                let status = if wasm_exists {
                    "Terkonfigurasi & Terkompilasi".to_string()
                } else {
                    "Belum di-build (perlu lumina plugin:build)".to_string()
                };
                items.push(Item {
                    name: format!("{}/ (Crate)", folder_name),
                    kind: "WASM Source".to_string(),
                    size_str: "-".to_string(),
                    status,
                });
            }
        }
    }

    // 2. Scan app/scripts/ and resources/scripts/ folder
    for script_dir in &["app/scripts", "resources/scripts"] {
        if let Ok(entries) = fs::read_dir(script_dir) {
            for entry in entries.flatten() {
                let p = entry.path();
                if p.is_file() && p.extension().and_then(|s| s.to_str()) == Some("is") {
                    let file_name = p.file_name().and_then(|s| s.to_str()).unwrap_or("?").to_string();
                    let size = fs::metadata(&p).map(|m| m.len()).unwrap_or(0);
                    let size_kb = format!("{:.2} KB", (size as f64) / 1024.0);
                    let content = fs::read_to_string(&p).unwrap_or_default();
                    let func_count = content.lines().filter(|l| l.trim().starts_with("fungsi ")).count();
                    items.push(Item {
                        name: format!("{}/{}", script_dir, file_name),
                        kind: "Skrip (.is)".to_string(),
                        size_str: size_kb,
                        status: format!("{} fungsi", func_count),
                    });
                }
            }
        }
    }

    if items.is_empty() {
        println!("ℹ️  Tidak ditemukan plugin WASM atau skrip Indonesian Script.");
        println!("💡 Buat plugin baru dengan: lumina plugin:new <nama>");
        println!("💡 Buat skrip .is di folder app/scripts/<nama>.is\n");
        return;
    }

    println!("{:<32} {:<15} {:<12} {:<35}", "NAMA PLUGIN / SKRIP", "TIPE", "UKURAN", "STATUS / DETAIL");
    println!("{:-<32} {:-<15} {:-<12} {:-<35}", "", "", "", "");
    for item in &items {
        println!("{:<32} {:<15} {:<12} {:<35}", item.name, item.kind, item.size_str, item.status);
    }
    println!("{:-<32} {:-<15} {:-<12} {:-<35}", "", "", "", "");
    println!("Total item terdeteksi: {}\n", items.len());
}

pub async fn handle_clean(purge: bool) {
    if purge {
        println!("🧹 Menjalankan pembersihan total (cargo clean)...");
        let _ = std::process::Command::new("cargo").arg("clean").status();
        println!("✅ Seluruh target cache berhasil dibersihkan.");
        return;
    }

    println!("✨ Menjalankan Lumina Smart Clean (Optimasi Cache)...");

    let sweep_status = std::process::Command::new("cargo")
        .args(["sweep", "-m", "8GB"])
        .status();

    if let Err(_) = sweep_status {
        println!("   💡 cargo-sweep tidak terdeteksi. Silakan pasang: cargo install cargo-sweep");
    }

    let inc_path = Path::new("target/debug/incremental");
    if inc_path.exists() {
        if fs::remove_dir_all(inc_path).is_ok() {
            println!("   ✅ Sesi kompilasi inkremental usang dibersihkan.");
        }
    }

    println!("\n🎉 Cache berhasil dioptimasi! Build aktif tetap hangat dan cepat.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_plugin_new_scaffolding() {
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let test_plugin_name = format!("test_plugin_{}", nanos);

        handle_plugin_new(&test_plugin_name).await;

        let plugin_dir = Path::new("plugins").join(&test_plugin_name);
        assert!(plugin_dir.exists(), "Plugin folder should be created");
        assert!(plugin_dir.join("Cargo.toml").exists(), "Cargo.toml should exist");
        assert!(plugin_dir.join("src/lib.rs").exists(), "src/lib.rs should exist");
        assert!(plugin_dir.join("README.md").exists(), "README.md should exist");

        let cargo = fs::read_to_string(plugin_dir.join("Cargo.toml")).unwrap();
        assert!(cargo.contains(r#"crate-type = ["cdylib"]"#));
        assert!(cargo.contains(&format!(r#"name = "{}""#, test_plugin_name)));

        let lib_rs = fs::read_to_string(plugin_dir.join("src/lib.rs")).unwrap();
        assert!(lib_rs.contains("pub extern \"C\" fn alloc"));
        assert!(lib_rs.contains("pub extern \"C\" fn transform"));

        // Clean up
        let _ = fs::remove_dir_all(&plugin_dir);
    }

    #[tokio::test]
    async fn test_plugin_list_execution() {
        handle_plugin_list().await;
    }
}
