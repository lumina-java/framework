use crate::core::application::AppState;
use crate::database::connection::DatabasePool;
use axum::{
    body::Body,
    http::{header, HeaderName, HeaderValue, Request, StatusCode},
    response::Response,
    Router,
};
use serde::Serialize;
use serde_json::Value;
use std::sync::Arc;
use tower::util::ServiceExt;
use uuid::Uuid;

/// Trait helper agar assert_status menerima `StatusCode` maupun `u16`
pub trait IntoStatusCode {
    fn into_status_code(self) -> StatusCode;
}

impl IntoStatusCode for StatusCode {
    fn into_status_code(self) -> StatusCode {
        self
    }
}

impl IntoStatusCode for u16 {
    fn into_status_code(self) -> StatusCode {
        StatusCode::from_u16(self).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR)
    }
}

/// Harness pengujian in-memory Lumina Framework
pub struct TestApp {
    pub router: Router,
    pub state: AppState,
}

impl TestApp {
    /// Buat instance TestApp baru dengan database in-memory yang unik.
    pub async fn new() -> Self {
        let id = Uuid::new_v4();
        let db_url = format!("sqlite:file:{}?mode=memory&cache=shared", id);
        let pool = DatabasePool::connect(&db_url)
            .await
            .expect("Failed to connect to test database");

        crate::database::migration::run_migrations(&pool.pool, pool.kind)
            .await
            .expect("Failed to run test migrations");

        let config = Arc::new(crate::core::config::ConfigManager::new());
        let lang = Arc::new(crate::core::i18n::LangManager::new("en".to_string()));
        let view = crate::core::view::ViewEngine::new(Some(lang.clone()));
        let cache = Arc::new(crate::core::cache::CacheManager::new());
        let events = Arc::new(crate::core::event::EventDispatcher::new());
        let telescope = Arc::new(crate::core::telescope::TelescopeManager::new());
        let echo = Arc::new(crate::core::echo::EchoManager::new());
        let registry = Arc::new(crate::core::queue::JobRegistry::new());
        let pool_arc = Arc::new(pool);
        let queue = Arc::new(crate::core::queue::QueueManager::new_with_pool(pool_arc.clone()));
        let storage = Arc::new(crate::core::storage::Storage::new("local"));
        let mail = Arc::new(crate::core::mail::Mail::new(
            Arc::new(crate::core::mail::LogDriver),
            view.clone(),
        ));
        let socialite = Arc::new(crate::core::auth::socialite::SocialiteManager::new());

        let state = AppState {
            db: Some(pool_arc),
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
            sandbox: Arc::new(crate::core::sandbox::LuminaPluginManager::new()),
        };

        let session_store = crate::core::session::store::LuminaSessionStore::Memory(
            tower_sessions::MemoryStore::default(),
        );

        let app = crate::core::application::Application::new();
        let router = app.build_router(state.clone(), session_store);

        Self { router, state }
    }

    /// Buat request builder fluen untuk kustomisasi method, header, body, dan token
    pub fn req(&self, method: &str, uri: &str) -> TestRequestBuilder {
        TestRequestBuilder::new(self.router.clone(), method, uri)
    }

    pub fn get_req(&self, uri: &str) -> TestRequestBuilder {
        self.req("GET", uri)
    }

    pub fn post_req(&self, uri: &str) -> TestRequestBuilder {
        self.req("POST", uri)
    }

    pub fn put_req(&self, uri: &str) -> TestRequestBuilder {
        self.req("PUT", uri)
    }

    pub fn patch_req(&self, uri: &str) -> TestRequestBuilder {
        self.req("PATCH", uri)
    }

    pub fn delete_req(&self, uri: &str) -> TestRequestBuilder {
        self.req("DELETE", uri)
    }

    /// Shorthand request langsung (Backward Compatible)
    pub async fn get(&self, uri: &str) -> TestResponse {
        self.get_req(uri).send().await
    }

    pub async fn post(&self, uri: &str, body: Value) -> TestResponse {
        self.post_req(uri).with_json(body).send().await
    }

    pub async fn put(&self, uri: &str, body: Value) -> TestResponse {
        self.put_req(uri).with_json(body).send().await
    }

    pub async fn delete(&self, uri: &str) -> TestResponse {
        self.delete_req(uri).send().await
    }

    // =========================================================================
    // 🗄️ Fluent Database Testing Assertions
    // =========================================================================

    /// Pastikan data tertentu ada di dalam tabel database.
    ///
    /// Contoh:
    /// ```rust,ignore
    /// app.assert_database_has("contacts", json!({ "name": "Budi Santoso", "is_active": 1 })).await;
    /// ```
    pub async fn assert_database_has(&self, table: &str, criteria: Value) -> &Self {
        let pool = self.state.db().get_pool();
        let obj = criteria.as_object().expect("Criteria must be a JSON object");

        let mut query = format!("SELECT COUNT(*) as count FROM {} WHERE ", table);
        let mut clauses = Vec::new();
        let mut values = Vec::new();

        for (col, val) in obj {
            clauses.push(format!("{} = ?", col));
            values.push(val.clone());
        }
        query.push_str(&clauses.join(" AND "));

        let mut q = sqlx::query_scalar::<_, i64>(&query);
        for val in values {
            match val {
                Value::Number(n) if n.is_i64() => q = q.bind(n.as_i64().unwrap()),
                Value::Number(n) if n.is_f64() => q = q.bind(n.as_f64().unwrap()),
                Value::Bool(b) => q = q.bind(b),
                Value::String(s) => q = q.bind(s),
                _ => q = q.bind(val.to_string()),
            }
        }

        let count = q.fetch_one(pool).await.unwrap_or(0);
        assert!(
            count > 0,
            "Failed asserting that a row in [{}] matches criteria: {:?}",
            table,
            criteria
        );
        self
    }

    /// Pastikan data tertentu TIDAK ada di dalam tabel database.
    pub async fn assert_database_missing(&self, table: &str, criteria: Value) -> &Self {
        let pool = self.state.db().get_pool();
        let obj = criteria.as_object().expect("Criteria must be a JSON object");

        let mut query = format!("SELECT COUNT(*) as count FROM {} WHERE ", table);
        let mut clauses = Vec::new();
        let mut values = Vec::new();

        for (col, val) in obj {
            clauses.push(format!("{} = ?", col));
            values.push(val.clone());
        }
        query.push_str(&clauses.join(" AND "));

        let mut q = sqlx::query_scalar::<_, i64>(&query);
        for val in values {
            match val {
                Value::Number(n) if n.is_i64() => q = q.bind(n.as_i64().unwrap()),
                Value::Number(n) if n.is_f64() => q = q.bind(n.as_f64().unwrap()),
                Value::Bool(b) => q = q.bind(b),
                Value::String(s) => q = q.bind(s),
                _ => q = q.bind(val.to_string()),
            }
        }

        let count = q.fetch_one(pool).await.unwrap_or(0);
        assert_eq!(
            count,
            0,
            "Failed asserting that table [{}] does NOT contain criteria: {:?}",
            table,
            criteria
        );
        self
    }

    /// Pastikan jumlah total baris pada tabel sesuai dengan ekspektasi.
    pub async fn assert_database_count(&self, table: &str, expected_count: usize) -> &Self {
        let pool = self.state.db().get_pool();
        let query = format!("SELECT COUNT(*) as count FROM {}", table);
        let count = sqlx::query_scalar::<_, i64>(&query)
            .fetch_one(pool)
            .await
            .unwrap_or(0) as usize;

        assert_eq!(
            count,
            expected_count,
            "Failed asserting that table [{}] has count [{}]. Actual count was [{}].",
            table,
            expected_count,
            count
        );
        self
    }
}

/// Fluent HTTP Request Builder untuk pengujian
pub struct TestRequestBuilder {
    router: Router,
    method: String,
    uri: String,
    headers: Vec<(String, String)>,
    body: Vec<u8>,
    user: Option<crate::core::auth::AuthUser>,
}

impl TestRequestBuilder {
    pub fn new(router: Router, method: &str, uri: &str) -> Self {
        Self {
            router,
            method: method.to_uppercase(),
            uri: uri.to_string(),
            headers: Vec::new(),
            body: Vec::new(),
            user: None,
        }
    }

    /// Tambahkan header HTTP kustom
    pub fn with_header(mut self, key: &str, value: &str) -> Self {
        self.headers.push((key.to_string(), value.to_string()));
        self
    }

    /// Tambahkan Bearer token ke header Authorization
    pub fn with_token(mut self, token: &str) -> Self {
        self.headers.push((
            "Authorization".to_string(),
            format!("Bearer {}", token.trim_start_matches("Bearer ").trim()),
        ));
        self
    }

    /// Simulasikan user yang sedang login via AuthUser extension
    pub fn acting_as(mut self, user: crate::core::auth::AuthUser) -> Self {
        self.user = Some(user);
        self
    }

    /// Kirim payload JSON
    pub fn with_json<T: Serialize>(mut self, body: T) -> Self {
        self.headers.push((
            header::CONTENT_TYPE.to_string(),
            "application/json".to_string(),
        ));
        self.headers.push((
            header::ACCEPT.to_string(),
            "application/json".to_string(),
        ));
        self.body = serde_json::to_vec(&body).unwrap_or_default();
        self
    }

    /// Kirim payload URL-encoded form data
    pub fn with_form(mut self, pairs: &[(&str, &str)]) -> Self {
        self.headers.push((
            header::CONTENT_TYPE.to_string(),
            "application/x-www-form-urlencoded".to_string(),
        ));
        let form_str = serde_urlencoded::to_string(pairs).unwrap_or_default();
        self.body = form_str.into_bytes();
        self
    }

    /// Eksekusi request dan kembalikan TestResponse fluen
    pub async fn send(self) -> TestResponse {
        let mut builder = Request::builder()
            .uri(&self.uri)
            .method(self.method.as_str());

        for (k, v) in self.headers {
            if let (Ok(name), Ok(val)) = (HeaderName::from_bytes(k.as_bytes()), HeaderValue::from_str(&v)) {
                builder = builder.header(name, val);
            }
        }

        let mut req = builder.body(Body::from(self.body)).unwrap();

        if let Some(user) = self.user {
            req.extensions_mut().insert(user);
        }

        let response = self.router.clone().oneshot(req).await.unwrap();
        TestResponse::new(response).await
    }
}

/// Representasi Response pengujian dengan 20+ fluent assertions
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

    // =========================================================================
    // 🚦 HTTP Status Assertions
    // =========================================================================

    pub fn status(&self) -> StatusCode {
        self.response.status()
    }

    pub fn assert_status<T: IntoStatusCode>(&self, expected: T) -> &Self {
        let expected_code = expected.into_status_code();
        assert_eq!(
            self.response.status(),
            expected_code,
            "HTTP Status mismatch. Expected {}, got {}. Body: {}",
            expected_code,
            self.response.status(),
            self.text()
        );
        self
    }

    pub fn assert_ok(&self) -> &Self {
        self.assert_status(StatusCode::OK)
    }

    pub fn assert_created(&self) -> &Self {
        self.assert_status(StatusCode::CREATED)
    }

    pub fn assert_accepted(&self) -> &Self {
        self.assert_status(StatusCode::ACCEPTED)
    }

    pub fn assert_no_content(&self) -> &Self {
        self.assert_status(StatusCode::NO_CONTENT)
    }

    pub fn assert_bad_request(&self) -> &Self {
        self.assert_status(StatusCode::BAD_REQUEST)
    }

    pub fn assert_unauthorized(&self) -> &Self {
        self.assert_status(StatusCode::UNAUTHORIZED)
    }

    pub fn assert_forbidden(&self) -> &Self {
        self.assert_status(StatusCode::FORBIDDEN)
    }

    pub fn assert_not_found(&self) -> &Self {
        self.assert_status(StatusCode::NOT_FOUND)
    }

    pub fn assert_unprocessable(&self) -> &Self {
        self.assert_status(StatusCode::UNPROCESSABLE_ENTITY)
    }

    pub fn assert_server_error(&self) -> &Self {
        self.assert_status(StatusCode::INTERNAL_SERVER_ERROR)
    }

    // =========================================================================
    // 📦 JSON Structure & Content Assertions
    // =========================================================================

    pub fn json(&self) -> Value {
        serde_json::from_slice(&self.body_bytes).unwrap_or_else(|e| {
            panic!(
                "Failed to parse response body as JSON: {}. Body was: '{}'",
                e,
                self.text()
            )
        })
    }

    /// Evaluasi nilai JSON pada path tertentu menggunakan dot notation (misal: "data.user.name" atau "items[0].id")
    pub fn json_path(&self, path: &str) -> Option<Value> {
        let root = self.json();
        get_json_path(&root, path)
    }

    pub fn assert_json_path(&self, path: &str, expected: impl Into<Value>) -> &Self {
        let expected_val = expected.into();
        let actual_val = self.json_path(path);
        assert_eq!(
            actual_val.as_ref(),
            Some(&expected_val),
            "JSON path '{}' mismatch. Expected: {:?}, Got: {:?}",
            path,
            expected_val,
            actual_val
        );
        self
    }

    pub fn assert_json_has(&self, path: &str) -> &Self {
        assert!(
            self.json_path(path).is_some(),
            "Expected JSON path '{}' to exist, but not found in: {:?}",
            path,
            self.json()
        );
        self
    }

    pub fn assert_json_missing(&self, path: &str) -> &Self {
        assert!(
            self.json_path(path).is_none(),
            "Expected JSON path '{}' to NOT exist, but found: {:?}",
            path,
            self.json_path(path)
        );
        self
    }

    pub fn assert_json_count(&self, path: &str, expected_count: usize) -> &Self {
        let val = self.json_path(path).unwrap_or_else(|| {
            panic!("JSON path '{}' not found", path);
        });

        match val {
            Value::Array(arr) => assert_eq!(
                arr.len(),
                expected_count,
                "JSON array at path '{}' has length {}, expected {}",
                path,
                arr.len(),
                expected_count
            ),
            Value::Object(map) => assert_eq!(
                map.len(),
                expected_count,
                "JSON object at path '{}' has {} keys, expected {}",
                path,
                map.len(),
                expected_count
            ),
            _ => panic!("JSON path '{}' is neither an Array nor an Object", path),
        }
        self
    }

    // =========================================================================
    // 📝 Text, Headers, & Redirection Assertions
    // =========================================================================

    pub fn text(&self) -> String {
        String::from_utf8_lossy(&self.body_bytes).to_string()
    }

    pub fn assert_see(&self, text: &str) -> &Self {
        let body = self.text();
        assert!(
            body.contains(text),
            "Expected text '{}' was not found in response body: '{}'",
            text,
            body
        );
        self
    }

    pub fn assert_dont_see(&self, text: &str) -> &Self {
        let body = self.text();
        assert!(
            !body.contains(text),
            "Unexpected text '{}' was found in response body",
            text
        );
        self
    }

    pub fn assert_header(&self, key: &str, expected: &str) -> &Self {
        let actual = self
            .response
            .headers()
            .get(key)
            .and_then(|v| v.to_str().ok());
        assert_eq!(
            actual,
            Some(expected),
            "Header '{}' mismatch. Expected '{:?}', got '{:?}'",
            key,
            expected,
            actual
        );
        self
    }

    pub fn assert_header_missing(&self, key: &str) -> &Self {
        assert!(
            !self.response.headers().contains_key(key),
            "Header '{}' unexpectedly present in response",
            key
        );
        self
    }

    pub fn assert_redirect(&self, expected_location: &str) -> &Self {
        let status = self.response.status();
        assert!(
            status.is_redirection(),
            "Expected redirect status (3xx), got: {}",
            status
        );

        let location = self
            .response
            .headers()
            .get(header::LOCATION)
            .and_then(|v| v.to_str().ok())
            .unwrap_or_default();

        assert!(
            location.ends_with(expected_location) || location == expected_location,
            "Expected redirect to '{}', but redirected to '{}'",
            expected_location,
            location
        );
        self
    }
}

/// Helper parsing dot notation untuk traversal JSON Value
fn get_json_path(root: &Value, path: &str) -> Option<Value> {
    let mut current = root;
    for part in path.split('.') {
        if let Some(bracket_idx) = part.find('[') {
            let key = &part[..bracket_idx];
            let rest = &part[bracket_idx..];
            if !key.is_empty() {
                current = current.get(key)?;
            }
            for num_str in rest.split(']').filter_map(|s| s.strip_prefix('[')) {
                let idx: usize = num_str.parse().ok()?;
                current = current.get(idx)?;
            }
        } else {
            current = current.get(part)?;
        }
    }
    Some(current.clone())
}
