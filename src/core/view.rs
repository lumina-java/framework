use tera::{Tera, Context};
use std::sync::Arc;

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
        
        // Register custom function 'dump'
        tera.register_function("dump", dump_fn);

        Self {
            inner: Arc::new(tera),
        }
    }

    /// Render template dengan context data
    pub fn render(&self, template_name: &str, context: &Context) -> String {
        match self.inner.render(template_name, context) {
            Ok(s) => s,
            Err(e) => {
                println!("❌ Render error: {:?}", e);
                format!("Template error: {:?}", e)
            }
        }
    }

    /// Render template dengan dukungan Session, Flash Messages, dan Validation Errors
    pub async fn render_with_session(
        &self,
        template_name: &str,
        mut context: Context,
        session: &tower_sessions::Session
    ) -> String {
        // 1. Flash Messages
        let flash_manager = crate::core::session::FlashManager::new(session);
        let flashes = flash_manager.consume().await;
        context.insert("flashes", &flashes);

        // 2. Validation Errors (Single use)
        let errors = session.get::<serde_json::Value>("_errors").await.unwrap_or_default()
            .unwrap_or_else(|| serde_json::json!({}));
        context.insert("errors", &errors);
        if !errors.as_object().unwrap().is_empty() {
             session.remove::<serde_json::Value>("_errors").await.unwrap();
        }

        // 3. Old Input (Single use)
        let old = session.get::<serde_json::Value>("_old").await.unwrap_or_default()
            .unwrap_or_else(|| serde_json::json!({}));
        context.insert("old", &old);
        if !old.as_object().unwrap().is_empty() {
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
            "<div style='background: #222125; color: #D4D4D4; font-family: Consolas, monospace; padding: 15px; border-radius: 8px; border-left: 5px solid #FF8400; box-shadow: 0 4px 6px rgba(0,0,0,0.3); margin-bottom: 20px;'><h4 style='color: #FF8400; margin-top: 0; font-size: 14px; font-weight: normal; margin-bottom: 10px; border-bottom: 1px solid #333; padding-bottom: 5px;'>🐛 Tera Dump</h4><pre style='margin: 0; font-size: 13px; white-space: pre-wrap; word-wrap: break-word;'>{}</pre></div>", 
            pretty
        );
        Ok(tera::Value::String(html))
    } else {
        Ok(tera::Value::String("".to_string()))
    }
}
