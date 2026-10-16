use std::cell::RefCell;
use std::sync::Arc;
pub use crate::core::blade::{BladeEngine, BladeContext};

thread_local! {
    static CURRENT_ERRORS: RefCell<serde_json::Value> = RefCell::new(serde_json::json!({}));
    static CURRENT_FLASHES: RefCell<serde_json::Value> = RefCell::new(serde_json::json!([]));
    static CURRENT_OLD: RefCell<serde_json::Value> = RefCell::new(serde_json::json!({}));
    static CURRENT_LOCALE: RefCell<String> = RefCell::new("en".to_string());
}

#[derive(Clone)]
pub struct ViewEngine {
    inner: Arc<BladeEngine>,
    pub lang: Option<Arc<crate::core::i18n::LangManager>>,
}

impl ViewEngine {
    /// Inisialisasi Native Blade engine dan load semua template dari resources/views
    pub fn new(lang: Option<Arc<crate::core::i18n::LangManager>>) -> Self {
        let engine = BladeEngine::new();
        Self {
            inner: Arc::new(engine),
            lang,
        }
    }

    /// Render template dengan context data
    pub fn render(&self, template_name: &str, context: &BladeContext) -> String {
        self.inner.render(template_name, context)
    }

    /// Render template dengan dukungan Session, Flash Messages, dan Validation Errors
    pub async fn render_with_session(
        &self,
        template_name: &str,
        mut context: BladeContext,
        session: &tower_sessions::Session,
        locale: Option<String>,
    ) -> String {
        // 0. Inject Locale
        let current_locale = locale.unwrap_or_else(|| "en".to_string());
        context.insert("locale", &current_locale);

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

        // 2. Validation Errors
        let errors = session
            .get::<serde_json::Value>("_errors")
            .await
            .unwrap_or_default()
            .unwrap_or_else(|| serde_json::json!({}));
        context.insert("errors", &errors);

        if !errors
            .as_object()
            .unwrap_or(&serde_json::Map::new())
            .is_empty()
        {
            let _ = session.remove::<serde_json::Value>("_errors").await;
        }

        // 3. Old Input
        let old = session
            .get::<serde_json::Value>("_old")
            .await
            .unwrap_or_default()
            .unwrap_or_else(|| serde_json::json!({}));
        context.insert("old", &old);

        if !old
            .as_object()
            .unwrap_or(&serde_json::Map::new())
            .is_empty()
        {
            let _ = session.remove::<serde_json::Value>("_old").await;
        }

        self.render(template_name, &context)
    }
}

/// Helper untuk merender view secara elegan (Laravel-style)
pub struct View;

pub struct ViewBuilder {
    template: String,
    context: BladeContext,
}

impl View {
    /// Membuat instance ViewBuilder baru. Gunakan titik sebagai separator subdirektori (misal: "auth.login")
    pub fn make(template: &str) -> ViewBuilder {
        ViewBuilder::new(template)
    }
}

impl ViewBuilder {
    pub fn new(template: &str) -> Self {
        Self {
            template: template.to_string(),
            context: BladeContext::new(),
        }
    }

    /// Menambahkan data ke dalam context template
    pub fn with<T: serde::Serialize>(mut self, key: &str, value: T) -> Self {
        self.context.insert(key, value);
        self
    }

    /// Mengeksekusi render dan mengembalikan ViewResponse.
    /// Sekarang mendukung Injeksi CSRF otomatis.
    pub async fn render(mut self, req: &crate::core::request::Request) -> ViewResponse {
        // Auto-inject CSRF Token jika tersedia
        if let Ok(token) = req.token.authenticity_token() {
            self.context.insert("csrf_token", &token);
        }

        // Ambil locale dari request extension (set oleh middleware)
        let locale = req
            .extensions
            .get::<crate::core::i18n::Locale>()
            .map(|l| l.0.clone());

        let html = req
            .state
            .view
            .render_with_session(&self.template, self.context, &req.session, locale)
            .await;
        ViewResponse {
            html,
            token: Some(req.token.clone()),
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
