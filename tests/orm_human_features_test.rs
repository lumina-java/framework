use lumina::database::connection::DatabasePool;
use lumina::database::model::Model;
use lumina::support::{json_decode, json_encode};
use lumina::LuminaModel;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, sqlx::FromRow, LuminaModel)]
#[table("users")]
struct User {
    #[primary_key]
    id: i64,
    name: String,
    email: String,
    score: i64,
}

async fn setup_test_db() -> DatabasePool {
    let pool = DatabasePool::connect("sqlite:file:memdb1?mode=memory&cache=shared")
        .await
        .expect("Failed to create memory database pool");

    sqlx::query(
        r#"
        CREATE TABLE users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            email TEXT NOT NULL,
            score INTEGER NOT NULL,
            deleted_at TEXT NULL
        );
        "#,
    )
    .execute(&pool.pool)
    .await
    .unwrap();

    pool
}

#[tokio::test]
async fn test_orm_aggregations_and_clauses() {
    let pool = setup_test_db().await;

    // Seed test data using User struct save method
    let u1 = User { id: 0, name: "Alice".to_string(), email: "alice@test.com".to_string(), score: 100 };
    let u2 = User { id: 0, name: "Bob".to_string(), email: "bob@test.com".to_string(), score: 200 };
    let u3 = User { id: 0, name: "Charlie".to_string(), email: "charlie@test.com".to_string(), score: 300 };

    u1.save(&pool).await.unwrap();
    u2.save(&pool).await.unwrap();
    u3.save(&pool).await.unwrap();

    // 1. Aggregations: count, sum, avg, min, max, exists, doesnt_exist
    let count = User::query(&pool).count().await.unwrap();
    assert_eq!(count, 3);

    let total_score = User::query(&pool).sum("score").await.unwrap();
    assert_eq!(total_score, 600.0);

    let avg_score = User::query(&pool).avg("score").await.unwrap();
    assert_eq!(avg_score, 200.0);

    let min_score = User::query(&pool).min("score").await.unwrap();
    assert_eq!(min_score, Some(100.0));

    let max_score = User::query(&pool).max("score").await.unwrap();
    assert_eq!(max_score, Some(300.0));

    let exists = User::query(&pool).where_eq("email", "alice@test.com").exists().await.unwrap();
    assert!(exists);

    let doesnt_exist = User::query(&pool).where_eq("email", "unknown@test.com").doesnt_exist().await.unwrap();
    assert!(doesnt_exist);

    // 2. Query Clauses: where_between, where_not_in, or_where, offset
    let between = User::query(&pool).where_between("score", 150, 350).get().await.unwrap();
    assert_eq!(between.len(), 2);

    let not_in = User::query(&pool).where_not_in("id", vec![1, 2]).order_by("id", "ASC").get().await.unwrap();
    assert_eq!(not_in.len(), 1);
    assert_eq!(not_in[0].name, "Charlie");

    let or_clause = User::query(&pool)
        .where_eq("name", "Alice")
        .or_where("name", "=", "Bob")
        .get()
        .await
        .unwrap();
    assert_eq!(or_clause.len(), 2);

    // 3. Search: find_or_fail, first_or_fail, first_or_create
    let found = User::find_or_fail(&pool, 1).await.unwrap();
    assert_eq!(found.name, "Alice");

    let first = User::query(&pool).where_eq("score", 200).first_or_fail().await.unwrap();
    assert_eq!(first.name, "Bob");

    let found_existing = User::first_or_create(&pool, [("email", "bob@test.com")], [("name", "Bob"), ("score", "200")]).await.unwrap();
    assert_eq!(found_existing.name, "Bob");

    let created_new = User::first_or_create(&pool, [("email", "david@test.com")], [("name", "David"), ("score", "400")]).await.unwrap();
    assert_eq!(created_new.name, "David");
    assert_eq!(created_new.score, 400);

    let count_after_create = User::query(&pool).count().await.unwrap();
    assert_eq!(count_after_create, 4);

    // 4. Pagination: paginate(page, per_page)
    let paginator = User::query(&pool).order_by("id", "DESC").paginate(1, 2).await.unwrap();
    assert_eq!(paginator.total, 4);
    assert_eq!(paginator.per_page, 2);
    assert_eq!(paginator.current_page, 1);
    assert_eq!(paginator.last_page, 2);
    assert_eq!(paginator.items.len(), 2);
    assert_eq!(paginator.items[0].name, "David");

    // 5. Mass Update
    let updated_rows = sqlx::query("UPDATE users SET score = 999 WHERE score BETWEEN 300 AND 500 AND deleted_at IS NULL")
        .execute(&pool.pool)
        .await
        .unwrap()
        .rows_affected();
    assert_eq!(updated_rows, 2);

    // 6. JSON Utils
    let json_str = json_encode(&found);
    assert!(json_str.contains("Alice"));
    let decoded: User = json_decode(&json_str).unwrap();
    assert_eq!(decoded, found);
}
