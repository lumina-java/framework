use lumina::prelude::*;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow, Clone, Default, LuminaModel, PartialEq)]
#[lumina(table = "contacts")]
pub struct Contact {
    pub id: i64,
    pub name: String,
    pub email: String,
    pub phone: String,
    pub address: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub deleted_at: Option<String>,
}

#[tokio::test]
async fn test_eloquent_crud_and_query_builder() {
    let pool = DatabasePool::connect("sqlite:file:memdb_eloquent?mode=memory&cache=shared").await.unwrap();

    // 1. Setup table
    sqlx::query(
        r#"CREATE TABLE contacts (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            email TEXT NOT NULL,
            phone TEXT NOT NULL,
            address TEXT,
            created_at TEXT DEFAULT CURRENT_TIMESTAMP,
            updated_at TEXT DEFAULT CURRENT_TIMESTAMP,
            deleted_at TEXT
        )"#,
    )
    .execute(&pool.pool)
    .await
    .unwrap();

    // 2. Active Record: INSERT using $contact.save(&pool)
    let mut c1 = Contact {
        name: "Budi Santoso".to_string(),
        email: "budi@example.com".to_string(),
        phone: "08123456789".to_string(),
        address: Some("Jakarta".to_string()),
        ..Default::default()
    };
    let id1 = c1.save(&pool).await.expect("Failed to insert c1");
    assert_eq!(id1, 1);
    c1.id = id1;

    let mut c2 = Contact {
        name: "Siti Rahma".to_string(),
        email: "siti@example.com".to_string(),
        phone: "08987654321".to_string(),
        address: Some("Bandung".to_string()),
        ..Default::default()
    };
    let id2 = c2.save(&pool).await.expect("Failed to insert c2");
    assert_eq!(id2, 2);
    c2.id = id2;

    // 3. Eloquent: Contact::count(&pool)
    let total = Contact::count(&pool).await.unwrap();
    assert_eq!(total, 2);

    // 4. Eloquent: Contact::all(&pool)
    let all = Contact::all(&pool).await.unwrap();
    assert_eq!(all.len(), 2);

    // 5. Eloquent: Contact::find(&pool, id)
    let found = Contact::find(&pool, id1).await.unwrap();
    assert!(found.is_some());
    let found = found.unwrap();
    assert_eq!(found.name, "Budi Santoso");
    assert_eq!(found.email, "budi@example.com");

    // 6. Active Record: UPDATE using $contact.save(&pool)
    let mut to_update = found;
    to_update.name = "Budi Santoso Updated".to_string();
    to_update.save(&pool).await.expect("Failed to update");

    let re_read = Contact::find(&pool, id1).await.unwrap().unwrap();
    assert_eq!(re_read.name, "Budi Santoso Updated");

    // 7. QueryBuilder / Where: Contact::where_eq(&pool, "email", "siti@example.com")
    let siti = Contact::where_eq(&pool, "email", "siti@example.com")
        .first()
        .await
        .unwrap()
        .expect("Contact Siti should exist");
    assert_eq!(siti.name, "Siti Rahma");

    // 8. Eloquent: Contact::paginate(&pool, page, per_page)
    let paginator = Contact::paginate(&pool, 1, 1).await.unwrap();
    assert_eq!(paginator.total, 2);
    assert_eq!(paginator.items.len(), 1);
    assert_eq!(paginator.per_page, 1);
    assert_eq!(paginator.current_page, 1);
    assert_eq!(paginator.last_page, 2);

    // 9. Eloquent: Contact::destroy(&pool, id) (Soft Delete)
    let deleted = Contact::destroy(&pool, id2).await.unwrap();
    assert!(deleted);

    // After soft delete, find should return None and all should have 1
    let find_deleted = Contact::find(&pool, id2).await.unwrap();
    assert!(find_deleted.is_none());

    let all_after_delete = Contact::all(&pool).await.unwrap();
    assert_eq!(all_after_delete.len(), 1);
    assert_eq!(all_after_delete[0].id, id1);
}

struct TestContactController;
impl TestContactController {
    /// GET /contacts — Hanya `req: Request` -> `LuminaResult`
    async fn index(req: Request) -> LuminaResult {
        let contacts = Contact::all(req.db()).await?;
        req.ok(contacts)
    }

    /// GET /contacts/paged — Paginasi otomatis dari `req.pagination()`
    async fn paginate(req: Request) -> LuminaResult {
        let (page, per_page) = req.pagination();
        let paginator = Contact::paginate(req.db(), page, per_page).await?;
        req.paginate(paginator)
    }

    /// GET /contacts/:id — ID diambil langsung lewat `req.id_or_fail()?`
    async fn show(req: Request) -> LuminaResult {
        let id = req.id_or_fail()?;
        match Contact::find(req.db(), id).await? {
            Some(contact) => req.ok(contact),
            None => req.not_found("Kontak tidak ditemukan"),
        }
    }

    /// DELETE /contacts/:id — Hapus data
    async fn destroy(req: Request) -> LuminaResult {
        let id = req.id_or_fail()?;
        if Contact::destroy(req.db(), id).await? {
            req.success(serde_json::json!({ "id": id }), "Kontak berhasil dihapus")
        } else {
            req.not_found("Kontak tidak ditemukan")
        }
    }
}

#[tokio::test]
async fn test_beautiful_controller_routes() {
    let mut app = lumina::core::testing::TestApp::new().await;

    // Setup table contacts in app's test DB
    sqlx::query(
        r#"CREATE TABLE IF NOT EXISTS contacts (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            email TEXT NOT NULL,
            phone TEXT NOT NULL,
            address TEXT,
            created_at TEXT DEFAULT CURRENT_TIMESTAMP,
            updated_at TEXT DEFAULT CURRENT_TIMESTAMP,
            deleted_at TEXT
        )"#,
    )
    .execute(app.state.db().get_pool())
    .await
    .unwrap();

    // Insert 2 records
    let c1 = Contact {
        name: "Budi Santoso".to_string(),
        email: "budi@example.com".to_string(),
        phone: "08123456789".to_string(),
        ..Default::default()
    };
    c1.save(app.state.db()).await.unwrap();

    let c2 = Contact {
        name: "Siti Rahma".to_string(),
        email: "siti@example.com".to_string(),
        phone: "08987654321".to_string(),
        ..Default::default()
    };
    c2.save(app.state.db()).await.unwrap();

    let api_router = Router::new()
        .get("/contacts", TestContactController::index)
        .get("/contacts/paged", TestContactController::paginate)
        .get("/contacts/:id", TestContactController::show)
        .delete("/contacts/:id", TestContactController::destroy);

    let application = lumina::core::application::Application::new().with_api(api_router);
    let session_store = lumina::core::session::store::LuminaSessionStore::Memory(
        tower_sessions::MemoryStore::default(),
    );
    app.router = application.build_router(app.state.clone(), session_store);

    // 1. Test GET /api/contacts -> 200 OK
    let res = app.get("/api/contacts").await;
    res.assert_status(axum::http::StatusCode::OK);

    // 2. Test GET /api/contacts/paged?page=1&per_page=1 -> 200 OK dengan pagination
    let res = app.get("/api/contacts/paged?page=1&per_page=1").await;
    res.assert_status(axum::http::StatusCode::OK);

    // 3. Test GET /api/contacts/1 -> 200 OK
    let res = app.get("/api/contacts/1").await;
    res.assert_status(axum::http::StatusCode::OK);

    // 4. Test GET /api/contacts/9999 -> 404 Not Found
    let res = app.get("/api/contacts/9999").await;
    res.assert_status(axum::http::StatusCode::NOT_FOUND);

    // 5. Test DELETE /api/contacts/1 -> 200 OK
    let res = app.delete("/api/contacts/1").await;
    res.assert_status(axum::http::StatusCode::OK);
}
