use axum::response::IntoResponse;
use pakem::core::router::Router;
use pakem::testing::TestApp;
use serde_json::json;

async fn test_json_endpoint() -> impl IntoResponse {
    axum::Json(json!({
        "success": true,
        "message": "Operasi Berhasil",
        "data": {
            "id": 42,
            "user": {
                "name": "Budi Santoso",
                "email": "budi@lumina.dev",
                "roles": ["admin", "editor"]
            },
            "stats": {
                "total_items": 10
            }
        }
    }))
}

async fn test_error_endpoint() -> impl IntoResponse {
    (
        axum::http::StatusCode::UNPROCESSABLE_ENTITY,
        axum::Json(json!({
            "success": false,
            "message": "Validasi gagal",
            "errors": {
                "name": ["Field nama wajib diisi"],
                "phone": ["Format nomor telepon tidak valid"]
            }
        })),
    )
}

async fn test_redirect_endpoint() -> impl IntoResponse {
    axum::response::Redirect::to("/dashboard")
}

#[tokio::test]
async fn test_fluent_assertions_status_and_json() {
    let mut app = TestApp::new().await;

    let router = Router::new()
        .get("/api/data", test_json_endpoint)
        .post("/api/submit", test_error_endpoint)
        .get("/legacy-redirect", test_redirect_endpoint);

    app.router = router.into_axum();

    // 1. Uji Fluent GET Request & Assertions
    let res = app.get_req("/api/data")
        .with_header("X-App-Client", "LuminaTest")
        .send().await;

    res.assert_ok()
        .assert_status(200u16)
        .assert_json_path("success", true)
        .assert_json_path("message", "Operasi Berhasil")
        .assert_json_path("data.id", 42)
        .assert_json_path("data.user.name", "Budi Santoso")
        .assert_json_path("data.user.roles[0]", "admin")
        .assert_json_path("data.user.roles[1]", "editor")
        .assert_json_count("data.user.roles", 2)
        .assert_json_has("data.user.email")
        .assert_json_missing("data.user.password")
        .assert_see("Budi Santoso")
        .assert_dont_see("PasswordRahasia");

    // 2. Uji 422 Unprocessable Entity
    let res_err = app.post_req("/api/submit")
        .with_json(json!({ "name": "" }))
        .send().await;

    res_err
        .assert_unprocessable()
        .assert_status(422u16)
        .assert_json_path("success", false)
        .assert_json_has("errors.name")
        .assert_json_has("errors.phone");

    // 3. Uji Redirection
    let res_redirect = app.get("/legacy-redirect").await;
    res_redirect.assert_redirect("/dashboard");
}

#[tokio::test]
async fn test_database_testing_assertions() {
    let app = TestApp::new().await;
    let pool = app.state.db().get_pool();

    // Buat tabel pengujian sementara
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS test_users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name VARCHAR(255) NOT NULL,
            email VARCHAR(255) NOT NULL UNIQUE,
            is_active INTEGER NOT NULL DEFAULT 1
        )",
    )
    .execute(pool)
    .await
    .expect("Failed to create test_users table");

    // Insert data awal
    sqlx::query("INSERT INTO test_users (name, email, is_active) VALUES (?, ?, ?)")
        .bind("Siti Rahma")
        .bind("siti@lumina.dev")
        .bind(1)
        .execute(pool)
        .await
        .unwrap();

    sqlx::query("INSERT INTO test_users (name, email, is_active) VALUES (?, ?, ?)")
        .bind("Ahmad Dahlan")
        .bind("ahmad@lumina.dev")
        .bind(0)
        .execute(pool)
        .await
        .unwrap();

    // 1. assert_database_has
    app.assert_database_has("test_users", json!({ "email": "siti@lumina.dev" })).await;
    app.assert_database_has("test_users", json!({ "name": "Ahmad Dahlan", "is_active": 0 })).await;

    // 2. assert_database_missing
    app.assert_database_missing("test_users", json!({ "email": "tidak_ada@lumina.dev" })).await;
    app.assert_database_missing("test_users", json!({ "name": "Siti Rahma", "is_active": 0 })).await;

    // 3. assert_database_count
    app.assert_database_count("test_users", 2).await;
}

