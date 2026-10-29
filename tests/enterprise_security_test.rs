use lumina::core::auth::{AuthUser, HasRolesAndPermissions};
use lumina::core::blade::{BladeContext, BladeEngine};
use lumina::core::error::AppError;
use lumina::core::router::Router;
use lumina::core::security::{AuditTrail, BruteForceConfig, BruteForceGuard};
use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use serde_json::json;
use tower::ServiceExt;

#[tokio::test]
async fn test_rbac_traits_and_wildcard_permissions() {
    let member = AuthUser::new(
        1,
        "member@example.com".to_string(),
        "member".to_string(),
        vec!["articles.view".to_string(), "articles.create".to_string()],
        1,
    );

    assert!(member.can("articles.view"));
    assert!(member.can("articles.create"));
    assert!(!member.can("articles.delete"));
    assert!(member.cannot("articles.delete"));
    assert!(member.has_role("member"));
    assert!(!member.has_role("admin"));
    assert!(member.has_any_role(&["admin", "member"]));
    assert!(!member.has_all_roles(&["admin", "member"]));

    // Admin otomatis bypass semua izin
    let admin = AuthUser::new(
        2,
        "admin@example.com".to_string(),
        "admin".to_string(),
        vec![],
        1,
    );
    assert!(admin.can("any.permission.whatsoever"));
    assert!(admin.has_role("admin"));

    // Superadmin otomatis bypass semua izin & role
    let superadmin = AuthUser::new(
        3,
        "super@example.com".to_string(),
        "superadmin".to_string(),
        vec![],
        1,
    );
    assert!(superadmin.can("nuclear.launch"));
    assert!(superadmin.has_role("editor")); // Superadmin has all roles

    // Wildcard permission
    let editor = AuthUser::new(
        4,
        "editor@example.com".to_string(),
        "editor".to_string(),
        vec!["*".to_string()],
        1,
    );
    assert!(editor.can("anything"));
}

#[tokio::test]
async fn test_router_rbac_declarative_guards() {
    let app = Router::new()
        .get("/protected/secret", || async { "super secret data" })
        .can("data.view")
        .has_role("vip")
        .into_axum();

    // 1. Request tanpa login -> 401 Unauthorized
    let res_no_auth = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/protected/secret")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(res_no_auth.status(), StatusCode::UNAUTHORIZED);

    // 2. Request user login tapi tidak punya izin -> 403 Forbidden
    let regular_user = AuthUser::new(
        10,
        "regular@example.com".to_string(),
        "member".to_string(),
        vec!["other.perm".to_string()],
        1,
    );
    let mut req_forbidden = Request::builder()
        .uri("/protected/secret")
        .body(Body::empty())
        .unwrap();
    req_forbidden.extensions_mut().insert(regular_user);

    let res_forbidden = app.clone().oneshot(req_forbidden).await.unwrap();
    assert_eq!(res_forbidden.status(), StatusCode::FORBIDDEN);

    // 3. Request user login dengan izin & role yang tepat -> 200 OK
    let vip_user = AuthUser::new(
        11,
        "vip@example.com".to_string(),
        "vip".to_string(),
        vec!["data.view".to_string()],
        1,
    );
    let mut req_ok = Request::builder()
        .uri("/protected/secret")
        .body(Body::empty())
        .unwrap();
    req_ok.extensions_mut().insert(vip_user);

    let res_ok = app.oneshot(req_ok).await.unwrap();
    assert_eq!(res_ok.status(), StatusCode::OK);
}

async fn panicking_handler() -> String {
    panic!("SIMULATED CRITICAL CRASH IN CONTROLLER!");
}

#[tokio::test]
async fn test_zero_panic_safety_boundary() {
    // Route yang sengaja memicu panic
    let app = Router::new()
        .get("/panicking-route", panicking_handler)
        .panic_boundary()
        .into_axum();

    let res = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/panicking-route")
                .header("Accept", "application/json")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Server TIDAK crash! Mengembalikan HTTP 500 rapi berformat JSON
    assert_eq!(res.status(), StatusCode::INTERNAL_SERVER_ERROR);

    // Pastikan server tetap sehat dan siap melayani request berikutnya
    let app_safe = Router::new()
        .get("/healthy", || async { "alive and kicking" })
        .panic_boundary()
        .into_axum();

    let res_healthy = app_safe
        .oneshot(
            Request::builder()
                .uri("/healthy")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(res_healthy.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_anti_brute_force_lockout_and_constant_time_comparison() {
    let key = "192.168.1.100:test_user";
    let config = BruteForceConfig {
        max_attempts: 3,
        lockout_duration: std::time::Duration::from_secs(60),
        window_duration: std::time::Duration::from_secs(60),
    };

    // Bersihkan status awal
    BruteForceGuard::reset(key).await;
    assert!(!BruteForceGuard::is_locked(key).await);
    assert!(BruteForceGuard::check_with_config(key, &config).await.is_ok());

    // Percobaan gagal 1 & 2 -> Masih diizinkan
    BruteForceGuard::hit_with_config(key, &config).await;
    BruteForceGuard::hit_with_config(key, &config).await;
    assert!(!BruteForceGuard::is_locked(key).await);

    // Percobaan gagal ke-3 (mencapai batas) -> Terkunci!
    BruteForceGuard::hit_with_config(key, &config).await;
    assert!(BruteForceGuard::is_locked(key).await);

    let check_result = BruteForceGuard::check_with_config(key, &config).await;
    assert!(check_result.is_err());
    match check_result {
        Err(AppError::Forbidden(msg)) => {
            assert!(msg.contains("Akun/IP dikunci sementara"));
        }
        _ => panic!("Expected AppError::Forbidden"),
    }

    // Reset saat sukses
    BruteForceGuard::reset(key).await;
    assert!(!BruteForceGuard::is_locked(key).await);
    assert!(BruteForceGuard::check_with_config(key, &config).await.is_ok());

    // Constant-time comparison
    assert!(BruteForceGuard::constant_time_eq("secret123", "secret123"));
    assert!(!BruteForceGuard::constant_time_eq("secret123", "secret124"));
}

#[tokio::test]
async fn test_blade_directives_can_cannot_and_role() {
    let engine = BladeEngine::empty();

    let template = r#"
    @can('edit_post')
        <button id="btn-edit">Edit Post</button>
    @endcan
    @cannot('delete_post')
        <span id="cant-del">No Delete</span>
    @endcannot
    @role('manager')
        <div id="manager-panel">Manager Access</div>
    @endrole
    "#;

    let mut context = BladeContext::new();
    context.insert("user", &json!({
        "name": "Budi",
        "role": "manager",
        "permissions": ["edit_post", "view_post"]
    }));

    let html = engine.render_string(template, &context);

    // Izin 'edit_post' ada -> tombol edit muncul
    assert!(html.contains("id=\"btn-edit\""));
    // Izin 'delete_post' tidak ada -> @cannot muncul
    assert!(html.contains("id=\"cant-del\""));
    // Role 'manager' cocok -> panel manager muncul
    assert!(html.contains("id=\"manager-panel\""));
}

#[tokio::test]
async fn test_enterprise_audit_trail_logging() {
    AuditTrail::record(
        Some("42".to_string()),
        "127.0.0.1",
        Some("Mozilla/5.0 (Windows NT 10.0; Win64; x64)".to_string()),
        "UPDATE_ROLE",
        "roles",
        Some("3".to_string()),
        Some(json!({ "old_role": "member", "new_role": "admin" })),
        "SUCCESS",
    );

    // Menunggu sedikit agar background spawn selesai menulis
    tokio::time::sleep(std::time::Duration::from_millis(50)).await;
}


#[test]
fn test_lumina_aurora_views_rendering() {
    let mut engine = BladeEngine::empty();
    engine.reload_templates("resources/views");

    let mut ctx = BladeContext::new();
    ctx.insert("user", &json!({
        "name": "Budi Pratama",
        "email": "budi@lumina-framework.org",
        "role": "admin",
        "permissions": ["users.manage"]
    }));
    ctx.insert("total_users", &42);
    ctx.insert("old_email", &"budi@lumina-framework.org");
    ctx.insert("old_name", &"Budi Pratama");

    let login_html = engine.render("aurora/auth/login.blade.rs", &ctx);
    assert!(login_html.contains("Selamat Datang"));
    assert!(login_html.contains("Lumina"));
    assert!(login_html.contains("name=\"csrf_token\""));

    let register_html = engine.render("aurora/auth/register.blade.rs", &ctx);
    assert!(register_html.contains("Buat Akun Baru"));

    let dashboard_html = engine.render("aurora/dashboard/index.blade.rs", &ctx);
    assert!(dashboard_html.contains("Halo, Budi Pratama!"));
    assert!(dashboard_html.contains("Kelola Pengguna"));

    let profile_html = engine.render("aurora/profile/edit.blade.rs", &ctx);
    assert!(profile_html.contains("Akun & Preferensi"));
}


#[tokio::test]
async fn test_lumina_aurora_routes_integration() {
    let mut app = lumina::core::testing::TestApp::new().await;
    let aurora_router = lumina::http::aurora::routes();

    let application = lumina::core::application::Application::new().with_web(aurora_router);
    let session_store = lumina::core::session::store::LuminaSessionStore::Memory(
        tower_sessions::MemoryStore::default(),
    );
    app.router = application.build_router(app.state.clone(), session_store);

    // 1. GET /login -> 200 OK
    let res = app.get("/login").await;
    res.assert_status(StatusCode::OK);

    // 2. GET /register -> 200 OK
    let res = app.get("/register").await;
    res.assert_status(StatusCode::OK);

    // 3. GET /dashboard without login -> 303 Redirect to /login
    let res = app.get("/dashboard").await;
    assert!(res.response.status().is_redirection());
}
