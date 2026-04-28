use tera::{Tera, Context};
use std::sync::Arc;

#[derive(Clone)]
pub struct ViewEngine {
    inner: Arc<Tera>,
}

impl ViewEngine {
    /// Inisialisasi Tera engine dan load semua template dari resources/views
    pub fn new() -> Self {
        // Mendapatkan path absolut ke folder resources/views agar aman saat dijalankan dari mana saja
        // Namun untuk skeleton, kita asumsikan dijalankan dari root project
        let mut tera = match Tera::new("resources/views/**/*.html") {
            Ok(t) => t,
            Err(e) => {
                println!("❌ Parsing error(s): {}", e);
                std::process::exit(1);
            }
        };
        
        // Nonaktifkan autoescape jika diinginkan, tapi default Tera cukup aman
        tera.autoescape_on(vec![".html", ".htm", ".xml"]);

        Self {
            inner: Arc::new(tera),
        }
    }

    /// Render template dengan context data
    pub fn render(&self, template_name: &str, context: &Context) -> String {
        match self.inner.render(template_name, context) {
            Ok(s) => s,
            Err(e) => {
                println!("❌ Render error: {}", e);
                format!("Template error: {}", e)
            }
        }
    }

    /// Render template dengan dukungan Session & Flash Messages
    pub async fn render_with_session(
        &self,
        template_name: &str,
        mut context: Context,
        session: &tower_sessions::Session
    ) -> String {
        let flash_manager = crate::core::session::FlashManager::new(session);
        let flashes = flash_manager.consume().await;
        context.insert("flashes", &flashes);
        
        self.render(template_name, &context)
    }
}
