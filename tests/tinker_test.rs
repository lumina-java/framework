use pakem::database::connection::DatabasePool;
use pakem::prelude::*;
use std::sync::Arc;
use uuid::Uuid;

async fn setup_test_pool() -> Arc<DatabasePool> {
    let id = Uuid::new_v4();
    let db_url = format!("sqlite:file:{}?mode=memory&cache=shared", id);
    let pool = DatabasePool::connect(&db_url)
        .await
        .expect("Failed to connect to test db");

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name VARCHAR(255) NOT NULL,
            email VARCHAR(255) NOT NULL,
            role VARCHAR(50) DEFAULT 'user'
        )",
    )
    .execute(&pool.pool)
    .await
    .unwrap();

    // Insert dummy records
    sqlx::query("INSERT INTO users (name, email, role) VALUES (?, ?, ?)")
        .bind("Budi Santoso")
        .bind("budi@example.com")
        .bind("admin")
        .execute(&pool.pool)
        .await
        .unwrap();

    sqlx::query("INSERT INTO users (name, email, role) VALUES (?, ?, ?)")
        .bind("Siti Rahma")
        .bind("siti@example.com")
        .bind("user")
        .execute(&pool.pool)
        .await
        .unwrap();

    Arc::new(pool)
}

#[tokio::test]
async fn test_tinker_helpers_and_facades() {
    // 1. Uji Help
    let res = Tinker::eval("help", None).await;
    match res {
        TinkerResult::Info { message } => {
            assert!(message.contains("Lumina Tinker"));
            assert!(message.contains("Hash::make"));
        }
        _ => panic!("Expected Info result for help"),
    }

    // 2. Uji AutoSlug
    let res = Tinker::eval("AutoSlug::slugify(\"Belajar Rust & Lumina 2026!\")", None).await;
    match res {
        TinkerResult::Scalar { value, .. } => {
            assert_eq!(value, "\"belajar-rust-lumina-2026\"");
        }
        _ => panic!("Expected Scalar result for AutoSlug"),
    }

    // 3. Uji Uuid
    let res = Tinker::eval("Uuid::new()", None).await;
    match res {
        TinkerResult::Scalar { value, .. } => {
            assert!(value.starts_with('"') && value.ends_with('"'));
            let clean = value.trim_matches('"');
            assert!(Uuid::parse_str(clean).is_ok());
        }
        _ => panic!("Expected Scalar result for Uuid"),
    }

    // 4. Uji Hash Make & Check
    let res = Tinker::eval("Hash::make(\"rahasia123\")", None).await;
    let hash_val = match res {
        TinkerResult::Scalar { value, .. } => value.trim_matches('"').to_string(),
        _ => panic!("Expected Scalar result for Hash::make"),
    };
    assert!(hash_val.starts_with("$2b$") || hash_val.starts_with("$2a$") || hash_val.starts_with("$2y$"));

    let check_cmd = format!("Hash::check(\"rahasia123\", \"{}\")", hash_val);
    let res = Tinker::eval(&check_cmd, None).await;
    match res {
        TinkerResult::Scalar { value, .. } => {
            assert_eq!(value, "true");
        }
        _ => panic!("Expected Scalar result for Hash::check"),
    }
}

#[tokio::test]
async fn test_tinker_database_and_eloquent_queries() {
    let pool = setup_test_pool().await;

    // 1. Uji Raw SELECT
    let res = Tinker::eval("SELECT id, name, email FROM users ORDER BY id ASC", Some(pool.clone())).await;
    match res {
        TinkerResult::Table { columns, rows, .. } => {
            assert_eq!(columns, vec!["id", "name", "email"]);
            assert_eq!(rows.len(), 2);
            assert_eq!(rows[0][1], "Budi Santoso");
            assert_eq!(rows[1][1], "Siti Rahma");
        }
        _ => panic!("Expected Table result for SELECT"),
    }

    // 2. Uji Eloquent Model::all()
    let res = Tinker::eval("User::all()", Some(pool.clone())).await;
    match res {
        TinkerResult::Table { columns, rows, .. } => {
            assert!(columns.contains(&"name".to_string()));
            assert_eq!(rows.len(), 2);
        }
        _ => panic!("Expected Table result for User::all()"),
    }

    // 3. Uji Eloquent Model::count()
    let res = Tinker::eval("User::count()", Some(pool.clone())).await;
    match res {
        TinkerResult::Table { rows, .. } => {
            assert_eq!(rows.len(), 1);
            assert_eq!(rows[0][0], "2");
        }
        _ => panic!("Expected Table result for User::count()"),
    }

    // 4. Uji Eloquent Model::find(1)
    let res = Tinker::eval("User::find(1)", Some(pool.clone())).await;
    match res {
        TinkerResult::Table { rows, .. } => {
            assert_eq!(rows.len(), 1);
            assert_eq!(rows[0][1], "Budi Santoso");
        }
        _ => panic!("Expected Table result for User::find(1)"),
    }

    // 5. Uji DB::table("users").get()
    let res = Tinker::eval("DB::table(\"users\").get()", Some(pool.clone())).await;
    match res {
        TinkerResult::Table { rows, .. } => {
            assert_eq!(rows.len(), 2);
        }
        _ => panic!("Expected Table result for DB::table().get()"),
    }

    // 6. Uji Perintah 'tables'
    let res = Tinker::eval("tables", Some(pool.clone())).await;
    match &res {
        TinkerResult::Table { rows, .. } => {
            let table_names: Vec<String> = rows.iter().map(|r| r[1].clone()).collect();
            assert!(table_names.contains(&"users".to_string()));
        }
        _ => panic!("Expected Table result for tables"),
    }

    // 7. Uji Render Formatter Box Drawing
    let table_output = res.render();
    assert!(table_output.contains('┌'));
    assert!(table_output.contains('┐'));
    assert!(table_output.contains('└'));
    assert!(table_output.contains('┘'));
    assert!(table_output.contains("users"));
}
