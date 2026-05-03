use tera::{Tera, Context};
use std::sync::Arc;
use std::cell::RefCell;

thread_local! {
    static CURRENT_ERRORS: RefCell<serde_json::Value> = RefCell::new(serde_json::json!({}));
    static CURRENT_FLASHES: RefCell<serde_json::Value> = RefCell::new(serde_json::json!([]));
}

#[derive(Clone)]
pub struct ViewEngine {
    inner: Arc<Tera>,
}

impl ViewEngine {
    /// Inisialisasi Tera engine dan load semua template dari resources/views
    pub fn new() -> Self {
        let mut tera = match Tera::new("resources/views/**/*.html") {
            Ok(t) => t,
            Err(e) => {
                println!("❌ Parsing error(s): {}", e);
                std::process::exit(1);
            }
        };
        
        tera.autoescape_on(vec![".html", ".htm", ".xml"]);
        
        // Register custom functions
        tera.register_function("dump", dump_fn);
        tera.register_function("form_error", form_error_fn);
        tera.register_function("alert_flash", alert_flash_fn);

        Self {
            inner: Arc::new(tera),
        }
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
    /// Sekarang mendukung deteksi HTMX otomatis jika Request dilewatkan.
    pub async fn render(mut self, req: &crate::core::request::Request) -> ViewResponse {
        self.context.insert("is_htmx", &req.is_htmx());
        
        let html = req.state.view.render_with_session(&self.template, self.context, &req.session).await;
        ViewResponse { html }
    }
}

pub struct ViewResponse {
    pub html: String,
}

impl ViewResponse {
    /// Mengonversi hasil render ke Axum Response dengan sinkronisasi CsrfToken
    pub fn into_response(self, token: axum_csrf::CsrfToken) -> axum::response::Response {
        use axum::response::IntoResponse;
        (token, axum::response::Html(self.html)).into_response()
    }
}

impl axum::response::IntoResponse for ViewResponse {
    fn into_response(self) -> axum::response::Response {
        axum::response::Html(self.html).into_response()
    }
}
