use tera::{Tera, Context};
use std::sync::Arc;
use std::cell::RefCell;
use regex::Regex;

thread_local! {
    static CURRENT_ERRORS: RefCell<serde_json::Value> = RefCell::new(serde_json::json!({}));
    static CURRENT_FLASHES: RefCell<serde_json::Value> = RefCell::new(serde_json::json!([]));
    static CURRENT_OLD: RefCell<serde_json::Value> = RefCell::new(serde_json::json!({}));
}

#[derive(Clone)]
pub struct ViewEngine {
    inner: Arc<Tera>,
}

impl ViewEngine {
    /// Inisialisasi Tera engine dan load semua template dari resources/views
    pub fn new() -> Self {
        let mut tera = Tera::default();
        
        // Kumpulkan SEMUA template dahulu ke dalam Vec,
        // baru daftarkan ke Tera sekaligus agar inheritance (@extends)
        // tidak gagal akibat urutan loading yang tidak pasti.
        let views_path = "resources/views";
        if std::fs::metadata(views_path).is_ok() {
            let mut templates: Vec<(String, String)> = Vec::new();
            Self::collect_templates(views_path, "", &mut templates);
            
            // KRITIS: Sort agar parent templates (layout.html, dll)
            // selalu terdaftar SEBELUM child yang meng-extend-nya.
            // Root-level templates (tanpa '/') = parent → depth 0 → duluan.
            templates.sort_by_key(|(name, _)| {
                let depth = name.chars().filter(|&c| c == '/').count();
                (depth, name.clone())
            });
            
            let raw: Vec<(&str, &str)> = templates
                .iter()
                .map(|(name, content)| (name.as_str(), content.as_str()))
                .collect();
            
            if let Err(e) = tera.add_raw_templates(raw) {
                eprintln!("❌ Gagal mendaftarkan templates ke Tera: {}", e);
            } else {
                println!("✅ {} template(s) berhasil dimuat.", templates.len());
            }
        } else {
            eprintln!("⚠️  Direktori 'resources/views' tidak ditemukan. Pastikan server dijalankan dari root project.");
        }

        tera.autoescape_on(vec![".html", ".htm", ".xml"]);
        
        // Register custom functions
        tera.register_function("dump", dump_fn);
        tera.register_function("form_error", form_error_fn);
        tera.register_function("alert_flash", alert_flash_fn);
        tera.register_function("old", old_fn);
        tera.register_function("error_class", error_class_fn);
        tera.register_function("has_error", has_error_fn);

        Self {
            inner: Arc::new(tera),
        }
    }

    /// Kumpulkan semua template secara rekursif ke dalam Vec<(name, content)>.
    /// Tidak langsung daftarkan ke Tera agar urutan tidak masalah.
    fn collect_templates(base_path: &str, prefix: &str, out: &mut Vec<(String, String)>) {
        let path = if prefix.is_empty() {
            base_path.to_string()
        } else {
            format!("{}/{}", base_path, prefix)
        };

        if let Ok(entries) = std::fs::read_dir(&path) {
            for entry in entries.flatten() {
                let file_path = entry.path();
                let file_name = entry.file_name().into_string().unwrap_or_default();
                
                if file_path.is_dir() {
                    let new_prefix = if prefix.is_empty() {
                        file_name
                    } else {
                        format!("{}/{}", prefix, file_name)
                    };
                    Self::collect_templates(base_path, &new_prefix, out);
                } else if file_name.ends_with(".html") {
                    if let Ok(content) = std::fs::read_to_string(&file_path) {
                        let processed = Self::preprocess_blade(&content);
                        let template_name = if prefix.is_empty() {
                            file_name
                        } else {
                            format!("{}/{}", prefix, file_name)
                        };
                        out.push((template_name, processed));
                    }
                }
            }
        }
    }

    fn preprocess_blade(content: &str) -> String {
        let mut processed = content.to_string();
        
        // 1. @extends('layout') -> {% extends "layout.html" %}
        let re_extends = Regex::new(r#"@extends\s*\(\s*['"](.*?)['"]\s*\)"#).unwrap();
        processed = re_extends.replace_all(&processed, "{% extends \"$1.html\" %}").to_string();
        
        // 2. @section('content') -> {% block content %}
        let re_section = Regex::new(r#"@section\s*\(\s*['"](.*?)['"]\s*\)"#).unwrap();
        processed = re_section.replace_all(&processed, "{% block $1 %}").to_string();
        
        // 3. @endsection -> {% endblock %}
        processed = processed.replace("@endsection", "{% endblock %}");
        
        // 4. @yield('content') -> {% block content %}{% endblock %}
        let re_yield = Regex::new(r#"@yield\s*\(\s*['"](.*?)['"]\s*\)"#).unwrap();
        processed = re_yield.replace_all(&processed, "{% block $1 %}{% endblock %}").to_string();
        
        // 5. @if(cond) -> {% if cond %}
        let re_if = Regex::new(r"@if\s*\((.*?)\)").unwrap();
        processed = re_if.replace_all(&processed, "{% if $1 %}").to_string();
        
        // 6. @elseif(cond) -> {% elif cond %}
        let re_elif = Regex::new(r"@elseif\s*\((.*?)\)").unwrap();
        processed = re_elif.replace_all(&processed, "{% elif $1 %}").to_string();
        
        // 7. @else -> {% else %}
        processed = processed.replace("@else", "{% else %}");
        
        // 8. @endif -> {% endif %}
        processed = processed.replace("@endif", "{% endif %}");
        
        // 9. @foreach(items as item) -> {% for item in items %}
        // Mendukung @foreach(items as item) atau @foreach($items as $item)
        let re_foreach = Regex::new(r"@foreach\s*\(\s*(\$)?(.*?)\s+as\s+(\$)?(.*?)\s*\)").unwrap();
        processed = re_foreach.replace_all(&processed, "{% for $4 in $2 %}").to_string();
        
        // 10. @endforeach -> {% endfor %}
        processed = processed.replace("@endforeach", "{% endfor %}");
        
        // 11. @csrf -> <input type="hidden" name="csrf_token" value="{{ csrf_token }}">
        processed = processed.replace("@csrf", "<input type=\"hidden\" name=\"csrf_token\" value=\"{{ csrf_token }}\">");
        
        // 12. {{ $var }} -> {{ var }} (Opsional, karena Tera sudah mendukung {{ var }})
        let re_var = Regex::new(r"\{\{\s*\$(.*?)\s*\}\}").unwrap();
        processed = re_var.replace_all(&processed, "{{ $1 }}").to_string();

        // 13. @include('path') -> {% include "path.html" %}
        let re_include = Regex::new(r#"@include\s*\(\s*['"](.*?)['"]\s*\)"#).unwrap();
        processed = re_include.replace_all(&processed, "{% include \"$1.html\" %}").to_string();

        // 14. @auth -> {% if email != "" %}
        processed = processed.replace("@auth", "{% if email != \"\" %}");
        // 15. @endauth -> {% endif %}
        processed = processed.replace("@endauth", "{% endif %}");
        
        // 16. @guest -> {% if email == "" %}
        processed = processed.replace("@guest", "{% if email == \"\" %}");
        // 17. @endguest -> {% endif %}
        processed = processed.replace("@endguest", "{% endif %}");

        processed
    }

    /// Render template dengan context data
    pub fn render(&self, template_name: &str, context: &Context) -> String {
        let res = match self.inner.render(template_name, context) {
            Ok(s) => s,
            Err(e) => {
                println!("❌ Render error: {:?}", e);
                format!("Template error: {:?}", e)
            }
        };
        // Reset thread locals
        CURRENT_ERRORS.with(|e| *e.borrow_mut() = serde_json::json!({}));
        CURRENT_FLASHES.with(|f| *f.borrow_mut() = serde_json::json!([]));
        CURRENT_OLD.with(|o| *o.borrow_mut() = serde_json::json!({}));
        res
    }

    /// Render template dengan dukungan Session, Flash Messages, dan Validation Errors
    pub async fn render_with_session(
        &self,
        template_name: &str,
        mut context: Context,
        session: &tower_sessions::Session
    ) -> String {
        // 0. Inject User Info if logged in
        if let Ok(Some(user)) = session.get::<crate::core::auth::AuthUser>("user").await {
            context.insert("email", &user.email);
            context.insert("user_id", &user.sub);
            context.insert("role", &user.role);
        } else {
            context.insert("email", "");
            context.insert("user_id", "");
            context.insert("role", "");
        }

        // 1. Flash Messages
        let flash_manager = crate::core::session::FlashManager::new(session);
        let flashes = flash_manager.consume().await;
        context.insert("flashes", &flashes);
        
        let flashes_val = serde_json::to_value(&flashes).unwrap_or_default();
        CURRENT_FLASHES.with(|f| *f.borrow_mut() = flashes_val);

        // 2. Validation Errors (Single use)
        let errors = session.get::<serde_json::Value>("_errors").await.unwrap_or_default()
            .unwrap_or_else(|| serde_json::json!({}));
        let mut errors_with_defaults = serde_json::json!({
            "name": ""
        });
        if let Some(obj) = errors.as_object() {
            if let Some(merge) = errors_with_defaults.as_object_mut() {
                for (k, v) in obj {
                    merge.insert(k.clone(), v.clone());
                }
            }
        }
        context.insert("errors", &errors_with_defaults);
        
        CURRENT_ERRORS.with(|e| *e.borrow_mut() = errors.clone());
        if !errors.as_object().unwrap_or(&serde_json::Map::new()).is_empty() {
             session.remove::<serde_json::Value>("_errors").await.unwrap();
        }

        // 3. Old Input (Single use)
        let old = session.get::<serde_json::Value>("_old").await.unwrap_or_default()
            .unwrap_or_else(|| serde_json::json!({}));
        let mut old_with_defaults = serde_json::json!({
            "name": ""
        });
        if let Some(obj) = old.as_object() {
            if let Some(merge) = old_with_defaults.as_object_mut() {
                for (k, v) in obj {
                    merge.insert(k.clone(), v.clone());
                }
            }
        }
        context.insert("old", &old_with_defaults);
        
        CURRENT_OLD.with(|o| *o.borrow_mut() = old.clone());
        if !old.as_object().unwrap_or(&serde_json::Map::new()).is_empty() {
             session.remove::<serde_json::Value>("_old").await.unwrap();
        }
        
        self.render(template_name, &context)
    }
}

/// Custom function untuk mencetak variabel JSON di Tera.
fn dump_fn(args: &std::collections::HashMap<String, tera::Value>) -> tera::Result<tera::Value> {
    if let Some(val) = args.get("var") {
        let pretty = serde_json::to_string_pretty(val).unwrap_or_else(|_| "Error serializing".to_string());
        let html = format!(
            r#"<div style="background: #1e293b; color: #7dd3fc; font-family: 'Fira Code', monospace; padding: 20px; border-radius: 12px; border: 1px solid #334155; box-shadow: 0 4px 6px rgba(0,0,0,0.3); margin: 20px 0; position: relative;">
                <div style="position: absolute; top: 0; right: 0; background: rgba(56, 189, 248, 0.1); color: #38bdf8; padding: 2px 10px; border-radius: 0 12px 0 12px; font-size: 10px; font-weight: 600; text-transform: uppercase;">Tera Dump</div>
                <pre style="margin: 0; font-size: 13px; white-space: pre-wrap; word-wrap: break-word;">{}</pre>
            </div>"#, 
            pretty
        );
        Ok(tera::Value::String(html))
    } else {
        Ok(tera::Value::String("".to_string()))
    }
}

fn form_error_fn(args: &std::collections::HashMap<String, tera::Value>) -> tera::Result<tera::Value> {
    if let Some(tera::Value::String(field)) = args.get("field") {
        let errors = CURRENT_ERRORS.with(|e| e.borrow().clone());
        if let Some(err_msg) = errors.get(field).and_then(|v| v.as_str()) {
            return Ok(tera::Value::String(format!(
                r#"<div class="text-red-500 text-sm mt-1">{}</div>"#,
                err_msg
            )));
        }
    }
    Ok(tera::Value::String("".to_string()))
}

fn alert_flash_fn(_args: &std::collections::HashMap<String, tera::Value>) -> tera::Result<tera::Value> {
    let flashes = CURRENT_FLASHES.with(|f| f.borrow().clone());
    let mut html = String::new();
    
    if let Some(arr) = flashes.as_array() {
        for item in arr {
            if let (Some(level), Some(message)) = (
                item.get("level").and_then(|v| v.as_str()),
                item.get("message").and_then(|v| v.as_str()),
            ) {
                let bg_color = match level {
                    "error" | "danger" => "bg-red-100 border-red-400 text-red-700",
                    "warning" => "bg-yellow-100 border-yellow-400 text-yellow-700",
                    _ => "bg-green-100 border-green-400 text-green-700",
                };
                html.push_str(&format!(
                    r#"<div class="border-l-4 p-4 mb-4 {}" role="alert">
                        <p class="font-bold capitalize">{}</p>
                        <p>{}</p>
                    </div>"#,
                    bg_color, level, message
                ));
            }
        }
    }
    
    Ok(tera::Value::String(html))
}

fn old_fn(args: &std::collections::HashMap<String, tera::Value>) -> tera::Result<tera::Value> {
    let field = args.get("field").and_then(|v| v.as_str()).unwrap_or("");
    let default = args.get("default").unwrap_or(&tera::Value::String("".to_string())).clone();
    
    let old_data = CURRENT_OLD.with(|o| o.borrow().clone());
    if let Some(val) = old_data.get(field) {
        // Convert serde_json::Value to tera::Value
        return Ok(tera::to_value(val).unwrap_or(default));
    }
    
    Ok(default)
}

fn error_class_fn(args: &std::collections::HashMap<String, tera::Value>) -> tera::Result<tera::Value> {
    let field = args.get("field").and_then(|v| v.as_str()).unwrap_or("");
    let class = args.get("class").and_then(|v| v.as_str()).unwrap_or("is-invalid");
    
    let errors = CURRENT_ERRORS.with(|e| e.borrow().clone());
    if errors.get(field).is_some() {
        return Ok(tera::Value::String(class.to_string()));
    }
    
    Ok(tera::Value::String("".to_string()))
}

fn has_error_fn(args: &std::collections::HashMap<String, tera::Value>) -> tera::Result<tera::Value> {
    let field = args.get("field").and_then(|v| v.as_str()).unwrap_or("");
    let errors = CURRENT_ERRORS.with(|e| e.borrow().clone());
    
    Ok(tera::Value::Bool(errors.get(field).is_some()))
}

/// Helper untuk merender view secara elegan (Laravel-style)
pub struct View;

pub struct ViewBuilder {
    template: String,
    context: Context,
}

impl View {
    /// Membuat instance ViewBuilder baru. Gunakan titik sebagai separator subdirektori (misal: "auth.login")
    pub fn make(template: &str) -> ViewBuilder {
        ViewBuilder::new(template)
    }
}

impl ViewBuilder {
    pub fn new(template: &str) -> Self {
        let mut template_name = template.replace(".", "/");
        if !template_name.ends_with(".html") {
            template_name.push_str(".html");
        }
        
        Self {
            template: template_name,
            context: Context::new(),
        }
    }

    /// Menambahkan data ke dalam context template
    pub fn with<T: serde::Serialize>(mut self, key: &str, value: T) -> Self {
        self.context.insert(key, &value);
        self
    }

    /// Mengeksekusi render dan mengembalikan ViewResponse.
    /// Sekarang mendukung deteksi HTMX otomatis dan Injeksi CSRF otomatis.
    pub async fn render(mut self, req: &crate::core::request::Request) -> ViewResponse {
        self.context.insert("is_htmx", &req.is_htmx());
        self.context.insert("hx_target", &req.hx_target());
        
        // Auto-inject CSRF Token jika tersedia
        if let Ok(token) = req.token.authenticity_token() {
            self.context.insert("csrf_token", &token);
        }
        
        let html = req.state.view.render_with_session(&self.template, self.context, &req.session).await;
        ViewResponse { 
            html,
            token: Some(req.token.clone())
        }
    }
}

pub struct ViewResponse {
    pub html: String,
    pub token: Option<axum_csrf::CsrfToken>,
}

impl axum::response::IntoResponse for ViewResponse {
    fn into_response(self) -> axum::response::Response {
        if let Some(token) = self.token {
            (token, axum::response::Html(self.html)).into_response()
        } else {
            axum::response::Html(self.html).into_response()
        }
    }
}
