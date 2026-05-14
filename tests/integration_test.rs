use axum::http::StatusCode;
use lumina::core::testing::TestApp;

#[tokio::test]
async fn test_homepage_works() {
    let app = TestApp::new().await;

    app.get("/")
        .await
        .assert_status(StatusCode::OK)
        .assert_see("Lumina");
}

#[tokio::test]
async fn test_api_not_found() {
    let app = TestApp::new().await;

    app.get("/api/unknown-route")
        .await
        .assert_status(StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_database_interaction() {
    let app = TestApp::new().await;
    let db = app.state.db().get_pool();

    // Create a dummy table for test if not exists (though migrations should handle real models)
    sqlx::query("CREATE TABLE IF NOT EXISTS test_items (id INTEGER PRIMARY KEY, name TEXT)")
        .execute(db)
        .await
        .unwrap();

    sqlx::query("INSERT INTO test_items (name) VALUES (?)")
        .bind("Test Item")
        .execute(db)
        .await
        .unwrap();

    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM test_items")
        .fetch_one(db)
        .await
        .unwrap();

    assert_eq!(count, 1);
}
