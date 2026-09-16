use axum::extract::{Path, Query};
use axum::http::{HeaderValue, StatusCode};
use axum::middleware;
use axum::response::IntoResponse;
use lumina::core::auth::AuthUser;
use lumina::core::request::Request as LuminaRequest;
use lumina::core::response::ApiResponse;
use lumina::core::router::Router;
use lumina::core::testing::TestApp;
use serde::{Deserialize, Serialize};
use tower::ServiceExt;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
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

#[derive(Deserialize)]
struct PaginationQuery {
    page: Option<i64>,
    limit: Option<i64>,
}

// Controller handlers for API CRUD testing
async fn list_products(
    req: LuminaRequest,
    Query(query): Query<PaginationQuery>,
) -> impl IntoResponse {
    let db = req.db().get_pool();
    let page = query.page.unwrap_or(1).max(1);
    let limit = query.limit.unwrap_or(10).max(1);
    let offset = (page - 1) * limit;

    let rows = sqlx::query_as::<_, (i64, String, i64)>(
        "SELECT id, name, price FROM products ORDER BY id ASC LIMIT ? OFFSET ?",
    )
    .bind(limit)
    .bind(offset)
    .fetch_all(db)
    .await
    .unwrap_or_default();

    let items: Vec<ProductItem> = rows
        .into_iter()
        .map(|(id, name, price)| ProductItem { id, name, price })
        .collect();
    ApiResponse::success(items)
}

async fn create_product(
    req: LuminaRequest,
    axum::Json(input): axum::Json<CreateProductInput>,
) -> impl IntoResponse {
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
    let row =
        sqlx::query_as::<_, (i64, String, i64)>("SELECT id, name, price FROM products WHERE id = ?")
            .bind(id)
            .fetch_optional(db)
            .await
            .unwrap_or_default();

    if let Some((id, name, price)) = row {
        ApiResponse::success(ProductItem { id, name, price }).into_response()
    } else {
        (
            StatusCode::NOT_FOUND,
            ApiResponse::error("Product not found"),
        )
            .into_response()
    }
}

async fn update_product(
    req: LuminaRequest,
    Path(id): Path<i64>,
    axum::Json(input): axum::Json<CreateProductInput>,
) -> impl IntoResponse {
    let db = req.db().get_pool();
    let res = sqlx::query("UPDATE products SET name = ?, price = ? WHERE id = ?")
        .bind(&input.name)
        .bind(input.price)
        .bind(id)
        .execute(db)
        .await
        .expect("Failed to update product");

    if res.rows_affected() > 0 {
        ApiResponse::with_message(
            ProductItem {
                id,
                name: input.name,
                price: input.price,
            },
            "Product updated",
        )
        .into_response()
    } else {
        (
            StatusCode::NOT_FOUND,
            ApiResponse::error("Product not found"),
        )
            .into_response()
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
        (
            StatusCode::NOT_FOUND,
            ApiResponse::error("Product not found"),
        )
            .into_response()
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
        .body(axum::body::Body::from(
            serde_json::to_string(&serde_json::json!({
                "name": "Kopi Jawa Super",
                "price": 20000
            }))
            .unwrap(),
        ))
        .unwrap();

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
async fn test_api_crud_input_validation() {
    let mut app = TestApp::new().await;

    let db = app.state.db().get_pool();
    sqlx::query("CREATE TABLE IF NOT EXISTS products (id INTEGER PRIMARY KEY AUTOINCREMENT, name TEXT, price INTEGER)")
        .execute(db)
        .await
        .unwrap();

    let api_router = Router::new().post("/products", create_product);
    let application = lumina::core::application::Application::new().with_api(api_router);
    let session_store = lumina::core::session::store::LuminaSessionStore::Memory(
        tower_sessions::MemoryStore::default(),
    );
    app.router = application.build_router(app.state.clone(), session_store);

    // 1. Missing mandatory field "price"
    let invalid_payload_1 = serde_json::json!({
        "name": "Teh Hijau"
    });
    let res = app.post("/api/products", invalid_payload_1).await;
    res.assert_status(StatusCode::UNPROCESSABLE_ENTITY);

    // 2. Incorrect data type for "price" (string instead of integer)
    let invalid_payload_2 = serde_json::json!({
        "name": "Teh Hijau",
        "price": "seribu"
    });
    let res = app.post("/api/products", invalid_payload_2).await;
    res.assert_status(StatusCode::UNPROCESSABLE_ENTITY);

    // 3. Malformed JSON payload
    let req = axum::http::Request::builder()
        .uri("/api/products")
        .method("POST")
        .header("content-type", "application/json")
        .body(axum::body::Body::from("{ invalid_json }"))
        .unwrap();
    let res = app.router.clone().oneshot(req).await.unwrap();
    let test_res = lumina::core::testing::TestResponse::new(res).await;
    test_res.assert_status(StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_api_crud_not_found_edge_cases() {
    let mut app = TestApp::new().await;

    let db = app.state.db().get_pool();
    sqlx::query("CREATE TABLE IF NOT EXISTS products (id INTEGER PRIMARY KEY AUTOINCREMENT, name TEXT, price INTEGER)")
        .execute(db)
        .await
        .unwrap();

    let api_router = Router::new()
        .get("/products/:id", get_product)
        .put("/products/:id", update_product)
        .delete("/products/:id", delete_product);

    let application = lumina::core::application::Application::new().with_api(api_router);
    let session_store = lumina::core::session::store::LuminaSessionStore::Memory(
        tower_sessions::MemoryStore::default(),
    );
    app.router = application.build_router(app.state.clone(), session_store);

    // 1. GET non-existent product
    let res = app.get("/api/products/999999").await;
    res.assert_status(StatusCode::NOT_FOUND);

    // 2. PUT non-existent product
    let req = axum::http::Request::builder()
        .uri("/api/products/999999")
        .method("PUT")
        .header("content-type", "application/json")
        .body(axum::body::Body::from(
            serde_json::to_string(&serde_json::json!({
                "name": "Non Existent Item",
                "price": 50000
            }))
            .unwrap(),
        ))
        .unwrap();
    let res = app.router.clone().oneshot(req).await.unwrap();
    let test_res = lumina::core::testing::TestResponse::new(res).await;
    test_res.assert_status(StatusCode::NOT_FOUND);

    // 3. DELETE non-existent product
    let req = axum::http::Request::builder()
        .uri("/api/products/999999")
        .method("DELETE")
        .body(axum::body::Body::empty())
        .unwrap();
    let res = app.router.clone().oneshot(req).await.unwrap();
    let test_res = lumina::core::testing::TestResponse::new(res).await;
    test_res.assert_status(StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_api_crud_pagination() {
    let mut app = TestApp::new().await;

    let db = app.state.db().get_pool();
    sqlx::query("CREATE TABLE IF NOT EXISTS products (id INTEGER PRIMARY KEY AUTOINCREMENT, name TEXT, price INTEGER)")
        .execute(db)
        .await
        .unwrap();

    // Insert 15 items
    for i in 1..=15 {
        sqlx::query("INSERT INTO products (name, price) VALUES (?, ?)")
            .bind(format!("Product {}", i))
            .bind(i * 1000)
            .execute(db)
            .await
            .unwrap();
    }

    let api_router = Router::new().get("/products", list_products);
    let application = lumina::core::application::Application::new().with_api(api_router);
    let session_store = lumina::core::session::store::LuminaSessionStore::Memory(
        tower_sessions::MemoryStore::default(),
    );
    app.router = application.build_router(app.state.clone(), session_store);

    // Page 1, limit 5
    let res = app.get("/api/products?page=1&limit=5").await;
    res.assert_status(StatusCode::OK);
    let data: serde_json::Value = res.json();
    let items = data["response"].as_array().unwrap();
    assert_eq!(items.len(), 5);
    assert_eq!(items[0]["name"], "Product 1");
    assert_eq!(items[4]["name"], "Product 5");

    // Page 2, limit 5
    let res = app.get("/api/products?page=2&limit=5").await;
    res.assert_status(StatusCode::OK);
    let data: serde_json::Value = res.json();
    let items = data["response"].as_array().unwrap();
    assert_eq!(items.len(), 5);
    assert_eq!(items[0]["name"], "Product 6");
    assert_eq!(items[4]["name"], "Product 10");

    // Page 3, limit 5
    let res = app.get("/api/products?page=3&limit=5").await;
    res.assert_status(StatusCode::OK);
    let data: serde_json::Value = res.json();
    let items = data["response"].as_array().unwrap();
    assert_eq!(items.len(), 5);
    assert_eq!(items[0]["name"], "Product 11");
    assert_eq!(items[4]["name"], "Product 15");
}

#[tokio::test]
async fn test_api_crud_jwt_auth_middleware() {
    let mut app = TestApp::new().await;

    let db = app.state.db().get_pool();
    sqlx::query("CREATE TABLE IF NOT EXISTS products (id INTEGER PRIMARY KEY AUTOINCREMENT, name TEXT, price INTEGER)")
        .execute(db)
        .await
        .unwrap();

    let protected_router = Router::new()
        .get("/products", list_products)
        .layer(middleware::from_fn(lumina::http::middleware::auth_required));

    let application = lumina::core::application::Application::new().with_api(protected_router);
    let session_store = lumina::core::session::store::LuminaSessionStore::Memory(
        tower_sessions::MemoryStore::default(),
    );
    app.router = application.build_router(app.state.clone(), session_store);

    // 1. Unauthenticated request (no Authorization header)
    let res = app.get("/api/products").await;
    res.assert_status(StatusCode::UNAUTHORIZED);

    // 2. Invalid Bearer Token
    let req = axum::http::Request::builder()
        .uri("/api/products")
        .method("GET")
        .header("authorization", "Bearer invalid.token.here")
        .body(axum::body::Body::empty())
        .unwrap();
    let res = app.router.clone().oneshot(req).await.unwrap();
    let test_res = lumina::core::testing::TestResponse::new(res).await;
    test_res.assert_status(StatusCode::UNAUTHORIZED);

    // 3. Valid Bearer Token
    let user = AuthUser::new(
        1,
        "admin@lumina.dev".to_string(),
        "admin".to_string(),
        vec!["read".to_string(), "write".to_string()],
        24,
    );
    let valid_token = lumina::http::auth::generate_token(&user).unwrap();

    let req = axum::http::Request::builder()
        .uri("/api/products")
        .method("GET")
        .header(
            "authorization",
            HeaderValue::from_str(&format!("Bearer {}", valid_token)).unwrap(),
        )
        .body(axum::body::Body::empty())
        .unwrap();
    let res = app.router.clone().oneshot(req).await.unwrap();
    let test_res = lumina::core::testing::TestResponse::new(res).await;
    test_res.assert_status(StatusCode::OK);
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
