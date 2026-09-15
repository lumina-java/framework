use axum::http::StatusCode;
use lumina::core::testing::TestApp;
use lumina::core::router::Router;
use lumina::core::request::Request as LuminaRequest;
use lumina::core::response::ApiResponse;
use axum::response::IntoResponse;
use axum::extract::Path;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
struct ProductItem {
    id: i64,
    name: String,
    price: i64,
}

#[derive(Deserialize)]
struct CreateProductInput {
    name: String,
    price: i64,
}

// Controller handlers for API CRUD testing
async fn list_products(req: LuminaRequest) -> impl IntoResponse {
    let db = req.db().get_pool();
    let rows = sqlx::query_as::<_, (i64, String, i64)>("SELECT id, name, price FROM products")
        .fetch_all(db)
        .await
        .unwrap_or_default();

    let items: Vec<ProductItem> = rows.into_iter().map(|(id, name, price)| ProductItem { id, name, price }).collect();
    ApiResponse::success(items)
}

async fn create_product(req: LuminaRequest, axum::Json(input): axum::Json<CreateProductInput>) -> impl IntoResponse {
    let db = req.db().get_pool();
    let res = sqlx::query("INSERT INTO products (name, price) VALUES (?, ?)")
        .bind(&input.name)
        .bind(input.price)
        .execute(db)
        .await
        .expect("Failed to insert product");

    let id = res.last_insert_id().unwrap_or(1) as i64;
    let product = ProductItem {
        id,
        name: input.name,
        price: input.price,
    };
    ApiResponse::with_message(product, "Product created")
}

async fn get_product(req: LuminaRequest, Path(id): Path<i64>) -> impl IntoResponse {
    let db = req.db().get_pool();
    let row = sqlx::query_as::<_, (i64, String, i64)>("SELECT id, name, price FROM products WHERE id = ?")
        .bind(id)
        .fetch_optional(db)
        .await
        .unwrap_or_default();

    if let Some((id, name, price)) = row {
        ApiResponse::success(ProductItem { id, name, price }).into_response()
    } else {
        (StatusCode::NOT_FOUND, ApiResponse::error("Product not found")).into_response()
    }
}

async fn update_product(req: LuminaRequest, Path(id): Path<i64>, axum::Json(input): axum::Json<CreateProductInput>) -> impl IntoResponse {
    let db = req.db().get_pool();
    let res = sqlx::query("UPDATE products SET name = ?, price = ? WHERE id = ?")
        .bind(&input.name)
        .bind(input.price)
        .bind(id)
        .execute(db)
        .await
        .expect("Failed to update product");

    if res.rows_affected() > 0 {
        ApiResponse::with_message(ProductItem { id, name: input.name, price: input.price }, "Product updated").into_response()
    } else {
        (StatusCode::NOT_FOUND, ApiResponse::error("Product not found")).into_response()
    }
}

async fn delete_product(req: LuminaRequest, Path(id): Path<i64>) -> impl IntoResponse {
    let db = req.db().get_pool();
    let res = sqlx::query("DELETE FROM products WHERE id = ?")
        .bind(id)
        .execute(db)
        .await
        .expect("Failed to delete product");

    if res.rows_affected() > 0 {
        ApiResponse::with_message(serde_json::json!({ "id": id }), "Product deleted").into_response()
    } else {
        (StatusCode::NOT_FOUND, ApiResponse::error("Product not found")).into_response()
    }
}

#[tokio::test]
async fn test_api_crud_workflow() {
    let mut app = TestApp::new().await;

    // Create table for products
    let db = app.state.db().get_pool();
    sqlx::query("CREATE TABLE IF NOT EXISTS products (id INTEGER PRIMARY KEY AUTOINCREMENT, name TEXT, price INTEGER)")
        .execute(db)
        .await
        .unwrap();

    // Register API CRUD routes
    let api_router = Router::new()
        .get("/products", list_products)
        .post("/products", create_product)
        .get("/products/:id", get_product)
        .put("/products/:id", update_product)
        .delete("/products/:id", delete_product);

    let application = lumina::core::application::Application::new().with_api(api_router);
    let session_store = lumina::core::session::store::LuminaSessionStore::Memory(
        tower_sessions::MemoryStore::default(),
    );
    app.router = application.build_router(app.state.clone(), session_store);

    // 1. List Products (Empty)
    let res = app.get("/api/products").await;
    res.assert_status(StatusCode::OK);

    // 2. Create Product
    let create_payload = serde_json::json!({
        "name": "Kopi Jawa",
        "price": 15000
    });
    let res = app.post("/api/products", create_payload).await;
    res.assert_status(StatusCode::OK);
    let created: serde_json::Value = res.json();
    assert_eq!(created["response"]["name"], "Kopi Jawa");
    assert_eq!(created["response"]["price"], 15000);
    let product_id = created["response"]["id"].as_i64().unwrap();

    // 3. Read Product
    let res = app.get(&format!("/api/products/{}", product_id)).await;
    res.assert_status(StatusCode::OK);
    let fetched = res.json();
    assert_eq!(fetched["response"]["name"], "Kopi Jawa");

    // 4. Update Product (using PUT request)
    let update_req = axum::http::Request::builder()
        .uri(format!("/api/products/{}", product_id))
        .method("PUT")
        .header("content-type", "application/json")
        .body(axum::body::Body::from(serde_json::to_string(&serde_json::json!({
            "name": "Kopi Jawa Super",
            "price": 20000
        })).unwrap()))
        .unwrap();

    use tower::ServiceExt;
    let update_res = app.router.clone().oneshot(update_req).await.unwrap();
    let test_update_res = lumina::core::testing::TestResponse::new(update_res).await;
    test_update_res.assert_status(StatusCode::OK);
    let updated = test_update_res.json();
    assert_eq!(updated["response"]["name"], "Kopi Jawa Super");

    // 5. Delete Product
    let delete_req = axum::http::Request::builder()
        .uri(format!("/api/products/{}", product_id))
        .method("DELETE")
        .body(axum::body::Body::empty())
        .unwrap();

    let delete_res = app.router.clone().oneshot(delete_req).await.unwrap();
    let test_delete_res = lumina::core::testing::TestResponse::new(delete_res).await;
    test_delete_res.assert_status(StatusCode::OK);

    // 6. Verify Deletion
    let res = app.get(&format!("/api/products/{}", product_id)).await;
    res.assert_status(StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_api_not_found() {
    let app = TestApp::new().await;

    app.get("/api/unknown-route")
        .await
        .assert_status(StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_indonesian_script_compiler_integration() {
    let script_source = r#"
reaktif skor = 100;
function tambahSkor() {
    skor = skor + 10;
    dudohno('Skor anyar:', skor);
}
"#;
    let compiled = lumina::core::is_engine::IsEngine::compile(script_source);
    assert!(compiled.contains("Indonesian Script (.is) Compiled Output"));
    assert!(compiled.contains("$state.skor = 100;"));
    assert!(compiled.contains("console.log"));
    assert!(compiled.contains("window.__IS_PROXY__"));
}

#[tokio::test]
async fn test_database_interaction() {
    let app = TestApp::new().await;
    let db = app.state.db().get_pool();

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
