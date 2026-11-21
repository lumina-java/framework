use axum::response::IntoResponse;
use pakem::core::router::Router;
use pakem::testing::TestApp;

async fn dummy_get() -> impl IntoResponse {
    axum::Json(serde_json::json!({ "success": true, "message": "Get OK" }))
}

async fn dummy_post() -> impl IntoResponse {
    axum::Json(serde_json::json!({ "success": true, "message": "Post Created" }))
}

#[tokio::test]
async fn test_openapi_spec_generation_and_metadata() {
    let router: Router = Router::new()
        .get("/api/v1/users", dummy_get)
        .summary("Daftar Pengguna")
        .description("Mengambil seluruh data pengguna aktif")
        .tag("Users")
        .operation_id("getUsers")
        .post("/api/v1/users", dummy_post)
        .summary("Buat Pengguna Baru")
        .tag("Users")
        .get("/api/v1/users/:id", dummy_get)
        .summary("Detail Pengguna")
        .tag("Users")
        .group("/admin", |r| {
            r.get("/metrics", dummy_get)
                .summary("Metrik Sistem")
                .tag("Admin")
                .deprecated()
        });

    let spec = router.openapi_spec("Lumina Core API", "1.0.0");

    assert_eq!(spec.openapi, "3.0.3");
    assert_eq!(spec.info.title, "Lumina Core API");
    assert_eq!(spec.info.version, "1.0.0");

    // Path /api/v1/users
    let users_path = spec.paths.get("/api/v1/users").expect("Path /api/v1/users not found");
    let get_users = users_path.get("get").expect("GET /api/v1/users not found");
    assert_eq!(get_users.summary, Some("Daftar Pengguna".to_string()));
    assert_eq!(get_users.description, Some("Mengambil seluruh data pengguna aktif".to_string()));
    assert_eq!(get_users.tags, vec!["Users".to_string()]);
    assert_eq!(get_users.operation_id, Some("getUsers".to_string()));

    let post_users = users_path.get("post").expect("POST /api/v1/users not found");
    assert_eq!(post_users.summary, Some("Buat Pengguna Baru".to_string()));

    // Path :id dinormalisasi ke {id}
    let user_detail_path = spec.paths.get("/api/v1/users/{id}").expect("Normalized path /api/v1/users/{id} not found");
    let get_detail = user_detail_path.get("get").expect("GET /api/v1/users/{id} not found");
    assert_eq!(get_detail.parameters.len(), 1);
    assert_eq!(get_detail.parameters[0].name, "id");
    assert_eq!(get_detail.parameters[0].r#in, "path");

    // Route Group /admin/metrics
    let admin_path = spec.paths.get("/admin/metrics").expect("Path /admin/metrics not found");
    let get_admin = admin_path.get("get").expect("GET /admin/metrics not found");
    assert_eq!(get_admin.summary, Some("Metrik Sistem".to_string()));
    assert_eq!(get_admin.deprecated, Some(true));

    // Test JSON Serialization
    let json = router.openapi_json("Lumina Core API", "1.0.0");
    assert!(json.contains("Lumina Core API"));
    assert!(json.contains("/api/v1/users"));
    assert!(json.contains("3.0.3"));

    // Test YAML Serialization
    let yaml = router.openapi_yaml("Lumina Core API", "1.0.0");
    assert!(yaml.contains("openapi: 3.0.3"));
    assert!(yaml.contains("Lumina Core API"));

    // Test Postman Collection Export
    let postman = spec.to_postman_collection("Lumina Postman");
    assert_eq!(postman["info"]["name"], "Lumina Postman");
    let items = postman["item"].as_array().expect("Expected postman item array");
    assert!(items.len() >= 4);
}

#[tokio::test]
async fn test_swagger_ui_and_openapi_endpoints() {
    let mut app = TestApp::new().await;

    // Pasang Swagger UI pada router
    let custom_router = Router::new()
        .get("/api/health", dummy_get)
        .summary("Health Check")
        .tag("System")
        .swagger_ui("/docs", "Lumina Test API", "1.0.0");

    app.router = custom_router.into_axum();

    // 1. Uji endpoint UI /docs/swagger
    let res_ui = app.get("/docs/swagger").await;
    res_ui
        .assert_ok()
        .assert_see("Lumina Test API — Swagger UI")
        .assert_see("swagger-ui-dist")
        .assert_see("/docs/openapi.json");

    // 2. Uji endpoint UI root /docs
    let res_root = app.get("/docs").await;
    res_root.assert_ok().assert_see("Lumina Test API");

    // 3. Uji endpoint JSON spec /docs/openapi.json
    let res_json = app.get("/docs/openapi.json").await;
    res_json
        .assert_ok()
        .assert_header("content-type", "application/json")
        .assert_json_path("openapi", "3.0.3")
        .assert_json_path("info.title", "Lumina Test API")
        .assert_json_has("paths./api/health");
}
