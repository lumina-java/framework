use axum::body::Body;
use axum::http::{header, Request, StatusCode};
use lumina::core::application::Application;
use lumina::core::auth::AuthUser;
use lumina::core::router::Router;
use lumina::core::session::store::LuminaSessionStore;
use lumina::core::testing::TestApp;
use lumina::http::auth::generate_token;
use lumina::http::middleware::{
    api_auth_with_pairing, api_secure_guard, from_fn, get_header, verify_api_signature, Next,
    Response,
};
use lumina::prelude::*;
use tower::ServiceExt;

// ─── Custom Middleware Definitions for Tests ───────────────────────────────────

/// Custom header injector / tracker (Global)
async fn inject_tracking_header(req: axum::extract::Request, next: Next) -> Response {
    let mut resp = next.run(req).await;
    resp.headers_mut().insert(
        "X-App-Tracker",
        "lumina-active-tracking".parse().unwrap(),
    );
    resp
}

/// Custom Web-only middleware
async fn web_only_guard(req: axum::extract::Request, next: Next) -> Response {
    let mut resp = next.run(req).await;
    resp.headers_mut()
        .insert("X-Pipeline", "web-pipeline".parse().unwrap());
    resp
}

/// Custom API-only middleware (e.g. inject API version or check api key)
async fn api_only_guard(req: axum::extract::Request, next: Next) -> Response {
    let mut resp = next.run(req).await;
    resp.headers_mut()
        .insert("X-Pipeline", "api-pipeline".parse().unwrap());
    resp
}

/// Custom Route-level middleware (e.g. check X-Tenant-Id)
async fn tenant_guard(req: axum::extract::Request, next: Next) -> Response {
    match get_header(&req, "X-Tenant-Id") {
        Some("tenant-vip") => next.run(req).await,
        Some(_) => middleware::forbidden_json("Tenant ID tidak memiliki akses VIP."),
        None => middleware::bad_request_json("Header 'X-Tenant-Id' wajib disertakan."),
    }
}

// ─── Controller Handlers ───────────────────────────────────────────────────────

async fn web_home() -> impl IntoResponse {
    Html("<h1>Selamat Datang di Lumina Web</h1>")
}

async fn api_ping() -> impl IntoResponse {
    Json(serde_json::json!({ "status": "pong" }))
}

async fn api_secure_data(req: lumina::core::request::Request) -> impl IntoResponse {
    let user_email = req.user.map(|u| u.email).unwrap_or_else(|| "Guest".to_string());
    Json(serde_json::json!({
        "success": true,
        "message": format!("Halo, {}! Akses data rahasia berhasil.", user_email)
    }))
}

async fn vip_data() -> impl IntoResponse {
    Json(serde_json::json!({ "tier": "VIP Exclusive" }))
}

// ─── TESTS ─────────────────────────────────────────────────────────────────────

#[tokio::test]
async fn test_pipeline_and_global_middleware() {
    let test_app = TestApp::new().await;
    let state = test_app.state.clone();
    let session_store = LuminaSessionStore::Memory(tower_sessions::MemoryStore::default());

    let web = Router::new().get("/home", web_home);
    let api = Router::new().get("/ping", api_ping);

    let app = Application::new()
        .with_web(web)
        .with_api(api)
        .with_global_middleware(from_fn(inject_tracking_header))
        .with_web_middleware(from_fn(web_only_guard))
        .with_api_middleware(from_fn(api_only_guard));

    let router = app.build_router(state, session_store);

    // 1. Test Web Route: Should have X-App-Tracker (Global) AND X-Pipeline: web-pipeline
    let req = Request::builder()
        .uri("/home")
        .method("GET")
        .body(Body::empty())
        .unwrap();
    let res = router.clone().oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);
    assert_eq!(
        res.headers().get("X-App-Tracker").and_then(|v| v.to_str().ok()),
        Some("lumina-active-tracking")
    );
    assert_eq!(
        res.headers().get("X-Pipeline").and_then(|v| v.to_str().ok()),
        Some("web-pipeline")
    );

    // 2. Test API Route: Should have X-App-Tracker (Global) AND X-Pipeline: api-pipeline
    let req = Request::builder()
        .uri("/api/ping")
        .method("GET")
        .body(Body::empty())
        .unwrap();
    let res = router.clone().oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);
    assert_eq!(
        res.headers().get("X-App-Tracker").and_then(|v| v.to_str().ok()),
        Some("lumina-active-tracking")
    );
    assert_eq!(
        res.headers().get("X-Pipeline").and_then(|v| v.to_str().ok()),
        Some("api-pipeline")
    );
}

#[tokio::test]
async fn test_route_and_group_middleware() {
    let test_app = TestApp::new().await;
    let state = test_app.state.clone();
    let session_store = LuminaSessionStore::Memory(tower_sessions::MemoryStore::default());

    let api = Router::new()
        .get("/open", api_ping)
        // Group route dengan custom tenant middleware
        .group("/tenants", |r| {
            r.middleware(from_fn(tenant_guard))
                .get("/vip", vip_data)
        });

    let app = Application::new().with_api(api);
    let router = app.build_router(state, session_store);

    // 1. /api/open -> No tenant header required
    let req = Request::builder()
        .uri("/api/open")
        .method("GET")
        .body(Body::empty())
        .unwrap();
    let res = router.clone().oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);

    // 2. /api/tenants/vip without X-Tenant-Id -> 400 Bad Request
    let req = Request::builder()
        .uri("/api/tenants/vip")
        .method("GET")
        .body(Body::empty())
        .unwrap();
    let res = router.clone().oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::BAD_REQUEST);

    // 3. /api/tenants/vip with regular tenant -> 403 Forbidden
    let req = Request::builder()
        .uri("/api/tenants/vip")
        .method("GET")
        .header("X-Tenant-Id", "tenant-regular")
        .body(Body::empty())
        .unwrap();
    let res = router.clone().oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::FORBIDDEN);

    // 4. /api/tenants/vip with tenant-vip -> 200 OK
    let req = Request::builder()
        .uri("/api/tenants/vip")
        .method("GET")
        .header("X-Tenant-Id", "tenant-vip")
        .body(Body::empty())
        .unwrap();
    let res = router.clone().oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_jwt_with_signature_and_pairing_guards() {
    let test_app = TestApp::new().await;
    let state = test_app.state.clone();
    let session_store = LuminaSessionStore::Memory(tower_sessions::MemoryStore::default());

    // Generate valid test JWT
    let user = AuthUser::new(
        42,
        "packercyber@gmail.com".to_string(),
        "admin".to_string(),
        vec!["manage-api".to_string()],
        24,
    );
    let token = generate_token(&user).unwrap();

    let api = Router::new()
        // 1. Endpoint dengan api_auth_with_pairing (JWT + X-Token-Pairing)
        .group("/pairing", |r| {
            r.middleware(from_fn(api_auth_with_pairing))
                .get("/info", api_secure_data)
        })
        // 2. Endpoint dengan verify_api_signature (X-Signature + X-Timestamp)
        .group("/signature", |r| {
            r.middleware(from_fn(verify_api_signature))
                .get("/check", api_ping)
        })
        // 3. Endpoint dengan api_secure_guard (JWT + X-Signature + X-Token-Pairing)
        .group("/secure-vault", |r| {
            r.middleware(from_fn(api_secure_guard))
                .get("/data", api_secure_data)
        });

    let app = Application::new().with_api(api);
    let router = app.build_router(state, session_store);

    // ── Test 1: api_auth_with_pairing ──────────────────────────
    // a. Missing token -> 401
    let req = Request::builder()
        .uri("/api/pairing/info")
        .method("GET")
        .body(Body::empty())
        .unwrap();
    let res = router.clone().oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::UNAUTHORIZED);

    // b. Valid token but missing X-Token-Pairing -> 401
    let req = Request::builder()
        .uri("/api/pairing/info")
        .method("GET")
        .header(header::AUTHORIZATION, format!("Bearer {}", token))
        .body(Body::empty())
        .unwrap();
    let res = router.clone().oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::UNAUTHORIZED);

    // c. Valid token + valid X-Token-Pairing -> 200 OK
    let req = Request::builder()
        .uri("/api/pairing/info")
        .method("GET")
        .header(header::AUTHORIZATION, format!("Bearer {}", token))
        .header("X-Token-Pairing", "device-uuid-987654321")
        .body(Body::empty())
        .unwrap();
    let res = router.clone().oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);

    // ── Test 2: verify_api_signature ───────────────────────────
    // a. Missing signature -> 401
    let req = Request::builder()
        .uri("/api/signature/check")
        .method("GET")
        .body(Body::empty())
        .unwrap();
    let res = router.clone().oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::UNAUTHORIZED);

    // b. Valid signature + timestamp -> 200 OK
    let req = Request::builder()
        .uri("/api/signature/check")
        .method("GET")
        .header("X-Signature", "hmac-sha256-abcdef1234567890")
        .header("X-Timestamp", "2026-10-16T19:30:00Z")
        .body(Body::empty())
        .unwrap();
    let res = router.clone().oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);

    // ── Test 3: api_secure_guard (Composite) ───────────────────
    // All 3 present: JWT + X-Signature + X-Token-Pairing -> 200 OK
    let req = Request::builder()
        .uri("/api/secure-vault/data")
        .method("GET")
        .header(header::AUTHORIZATION, format!("Bearer {}", token))
        .header("X-Signature", "sig-production-secret-999")
        .header("X-Token-Pairing", "pairing-token-abc-123")
        .body(Body::empty())
        .unwrap();
    let res = router.clone().oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);
}
