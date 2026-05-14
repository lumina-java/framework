use crate::core::application::AppState;
use crate::database::connection::DatabasePool;
use axum::{
    body::Body,
    http::{header, Request, StatusCode},
    response::Response,
    Router,
};
use serde_json::Value;
use std::sync::Arc;
use tower::util::ServiceExt;
use uuid::Uuid;

pub struct TestApp {
    pub router: Router,
    pub state: AppState,
}

impl TestApp {
    /// Buat instance TestApp baru dengan database in-memory yang unik.
    pub async fn new() -> Self {
        // 1. Setup Test Database (SQLite Memory with unique name per instance to avoid interference)
        let id = Uuid::new_v4();
        let db_url = format!("sqlite:file:{}?mode=memory&cache=shared", id);
        let pool = DatabasePool::connect(&db_url)
            .await
            .expect("Failed to connect to test database");

        // 2. Run Migrations
        crate::database::migration::run_migrations(&pool.pool, pool.kind)
            .await
            .expect("Failed to run test migrations");

        // 3. Setup State
        let config = Arc::new(crate::core::config::ConfigManager::new());
        let lang = Arc::new(crate::core::i18n::LangManager::new("en".to_string()));
        let view = crate::core::view::ViewEngine::new(Some(lang.clone()));
        let cache = Arc::new(crate::core::cache::CacheManager::new());
        let events = Arc::new(crate::core::event::EventDispatcher::new());
        let telescope = Arc::new(crate::core::telescope::TelescopeManager::new());
        let echo = Arc::new(crate::core::echo::EchoManager::new());
        let registry = Arc::new(crate::core::queue::JobRegistry::new());
        let queue = Arc::new(crate::core::queue::QueueManager::new(pool.pool.clone()));
        let storage = Arc::new(crate::core::storage::Storage::new("local"));
        let mail = Arc::new(crate::core::mail::Mail::new(
            Arc::new(crate::core::mail::LogDriver),
            view.clone(),
        ));
        let socialite = Arc::new(crate::core::auth::socialite::SocialiteManager::new());

        let state = AppState {
            db: Some(Arc::new(pool)),
            view,
            db_error: None,
            csrf_config: crate::core::security::csrf::config(),
            storage,
            config,
            queue,
            cache,
            mail,
            registry,
            events,
            telescope,
            lang,
            echo,
            socialite,
        };

        // 4. Build Router
        let session_store = crate::core::session::store::LuminaSessionStore::Memory(
            tower_sessions::MemoryStore::default(),
        );

        let app = crate::core::application::Application::new();
        let router = app.build_router(state.clone(), session_store);

        Self { router, state }
    }

    pub async fn get(&self, uri: &str) -> TestResponse {
        let req = Request::builder()
            .uri(uri)
            .method("GET")
            .body(Body::empty())
            .unwrap();

        let response = self.router.clone().oneshot(req).await.unwrap();
        TestResponse::new(response).await
    }

    pub async fn post(&self, uri: &str, body: Value) -> TestResponse {
        let req = Request::builder()
            .uri(uri)
            .method("POST")
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(serde_json::to_string(&body).unwrap()))
            .unwrap();

        let response = self.router.clone().oneshot(req).await.unwrap();
        TestResponse::new(response).await
    }
}

pub struct TestResponse {
    pub response: Response,
    pub body_bytes: Vec<u8>,
}

impl TestResponse {
    pub async fn new(response: Response) -> Self {
        use axum::body::to_bytes;
        let (parts, body) = response.into_parts();
        let bytes = to_bytes(body, usize::MAX).await.unwrap().to_vec();
        let response = Response::from_parts(parts, Body::from(bytes.clone()));

        Self {
            response,
            body_bytes: bytes,
        }
    }

    pub fn assert_status(&self, expected: StatusCode) -> &Self {
        assert_eq!(self.response.status(), expected, "HTTP Status mismatch");
        self
    }

    pub fn assert_see(&self, text: &str) -> &Self {
        let body_str = String::from_utf8_lossy(&self.body_bytes);
        assert!(
            body_str.contains(text),
            "Text '{}' not found in response body",
            text
        );
        self
    }

    pub fn json(&self) -> Value {
        serde_json::from_slice(&self.body_bytes).expect("Failed to parse response as JSON")
    }

    pub fn assert_json_has(&self, key: &str) -> &Self {
        let json = self.json();
        assert!(json.get(key).is_some(), "JSON key '{}' not found", key);
        self
    }
}
