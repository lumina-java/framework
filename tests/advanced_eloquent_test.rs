use pakem::database::connection::DatabasePool;
use pakem::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::FromRow;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow, Clone, Default, LuminaModel)]
#[table("test_contacts")]
pub struct TestContact {
    pub id: i64,
    pub name: String,
    pub email: String,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub deleted_at: Option<String>,
}

async fn setup_test_pool() -> DatabasePool {
    let id = Uuid::new_v4();
    let db_url = format!("sqlite:file:{}?mode=memory&cache=shared", id);
    let pool = DatabasePool::connect(&db_url)
        .await
        .expect("Failed to connect to test db");

    // Siapkan tabel
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS test_contacts (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name VARCHAR(255) NOT NULL,
            email VARCHAR(255) NOT NULL,
            created_at TEXT DEFAULT CURRENT_TIMESTAMP,
            updated_at TEXT DEFAULT CURRENT_TIMESTAMP,
            deleted_at TEXT NULL
        )",
    )
    .execute(&pool.pool)
    .await
    .unwrap();

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS test_articles (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title VARCHAR(255) NOT NULL,
            slug VARCHAR(255) NOT NULL UNIQUE
        )",
    )
    .execute(&pool.pool)
    .await
    .unwrap();

    pool
}

#[tokio::test]
async fn test_auto_slug_generation_and_collision_resolution() {
    let pool = setup_test_pool().await;

    // 1. Uji slugify
    let raw = "Tutorial Rust: Menulis Code 'Ciamik' & Cepat! (Tahun 2026)";
    let slug = AutoSlug::slugify(raw);
    assert_eq!(slug, "tutorial-rust-menulis-code-ciamik-cepat-tahun-2026");

    // 2. Simpan entitas pertama
    let s1 = AutoSlug::generate(&pool, "test_articles", "Belajar Lumina Framework", None)
        .await
        .unwrap();
    assert_eq!(s1, "belajar-lumina-framework");

    sqlx::query("INSERT INTO test_articles (title, slug) VALUES (?, ?)")
        .bind("Belajar Lumina Framework")
        .bind(&s1)
        .execute(&pool.pool)
        .await
        .unwrap();

    // 3. Simpan entitas kedua dengan judul yang sama (Collision Resolution)
    let s2 = AutoSlug::generate(&pool, "test_articles", "Belajar Lumina Framework", None)
        .await
        .unwrap();
    assert_eq!(s2, "belajar-lumina-framework-1");

    sqlx::query("INSERT INTO test_articles (title, slug) VALUES (?, ?)")
        .bind("Belajar Lumina Framework 2")
        .bind(&s2)
        .execute(&pool.pool)
        .await
        .unwrap();

    // 4. Simpan entitas ketiga (benturan berikutnya)
    let s3 = AutoSlug::generate(&pool, "test_articles", "Belajar Lumina Framework", None)
        .await
        .unwrap();
    assert_eq!(s3, "belajar-lumina-framework-2");

    // 5. Update entitas 1 mengecualikan ID dirinya sendiri
    let s_self = AutoSlug::generate(&pool, "test_articles", "Belajar Lumina Framework", Some(1))
        .await
        .unwrap();
    assert_eq!(s_self, "belajar-lumina-framework");
}

#[tokio::test]
async fn test_auto_hash_bcrypt_password() {
    let plain = "rahasia123";

    // 1. Hash awal
    let hashed = AutoHash::hash(plain).expect("Bcrypt hashing failed");
    assert!(AutoHash::is_hashed(&hashed));

    // 2. Verifikasi kata sandi
    let is_valid = bcrypt::verify(plain, &hashed).unwrap();
    assert!(is_valid);

    // 3. Idempotensi — tidak meng-hash string yang sudah terhash
    let re_hashed = AutoHash::hash(&hashed).expect("Re-hash failed");
    assert_eq!(hashed, re_hashed);
}

#[tokio::test]
async fn test_audit_trail_logging_and_history() {
    let pool = setup_test_pool().await;

    // 1. Catat event CREATE
    let log1_id = Audit::record(&pool)
        .on("test_contacts", 42)
        .action("CREATE")
        .by_user(Some(1001))
        .diff(json!({}), json!({ "name": "Budi Santoso", "email": "budi@lumina.dev" }))
        .ip(Some("192.168.1.10"))
        .user_agent(Some("Mozilla/5.0 LuminaClient"))
        .save()
        .await
        .unwrap();

    assert!(log1_id > 0);

    // 2. Catat event UPDATE
    let log2_id = Audit::record(&pool)
        .on("test_contacts", 42)
        .action("UPDATE")
        .by_user(Some(1001))
        .diff(
            json!({ "name": "Budi Santoso" }),
            json!({ "name": "Budi S. Pratama" }),
        )
        .save()
        .await
        .unwrap();

    assert!(log2_id > log1_id);

    // 3. Baca riwayat audit untuk entitas test_contacts ID 42
    let history = Audit::history(&pool, "test_contacts", 42).await.unwrap();
    assert_eq!(history.len(), 2);
    assert_eq!(history[0].action, "UPDATE");
    assert_eq!(history[1].action, "CREATE");
    assert_eq!(history[0].user_id, Some(1001));

    // 4. Baca daftar log terbaru global
    let latest = Audit::latest(&pool, 5).await.unwrap();
    assert_eq!(latest.len(), 2);
}

#[derive(Default)]
struct DummyContactSeeder;

#[async_trait::async_trait]
impl Seeder for DummyContactSeeder {
    async fn run(&self, db: &DatabasePool) -> Result<(), sqlx::Error> {
        for i in 1..=5 {
            sqlx::query("INSERT INTO test_contacts (name, email) VALUES (?, ?)")
                .bind(format!("User {}", i))
                .bind(format!("user{}@lumina.dev", i))
                .execute(&db.pool)
                .await?;
        }
        Ok(())
    }
}

#[tokio::test]
async fn test_seeder_class_runner_and_registry() {
    let pool = setup_test_pool().await;

    // 1. Eksekusi langsung via call_seeder
    call_seeder::<DummyContactSeeder>(&pool).await.unwrap();

    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM test_contacts")
        .fetch_one(&pool.pool)
        .await
        .unwrap();
    assert_eq!(count, 5);

    // 2. Eksekusi via SeederRegistry
    let mut registry = SeederRegistry::new();
    registry.register::<DummyContactSeeder>("DummyContactSeeder");

    let ran = registry.run_class("DummyContactSeeder", &pool).await.unwrap();
    assert!(ran);

    let count_after: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM test_contacts")
        .fetch_one(&pool.pool)
        .await
        .unwrap();
    assert_eq!(count_after, 10);
}

#[tokio::test]
async fn test_query_builder_chunk_and_update_or_create() {
    let pool = setup_test_pool().await;

    // Isi 7 data awal
    for i in 1..=7 {
        sqlx::query("INSERT INTO test_contacts (name, email) VALUES (?, ?)")
            .bind(format!("Kontak {}", i))
            .bind(format!("kontak{}@lumina.dev", i))
            .execute(&pool.pool)
            .await
            .unwrap();
    }

    // 1. Uji .chunk(3, callback)
    let total_batches = Arc::new(AtomicUsize::new(0));
    let total_items = Arc::new(AtomicUsize::new(0));

    let b_clone = total_batches.clone();
    let i_clone = total_items.clone();

    TestContact::query(&pool)
        .chunk(3, move |batch| {
            let b = b_clone.clone();
            let i = i_clone.clone();
            async move {
                b.fetch_add(1, Ordering::SeqCst);
                i.fetch_add(batch.len(), Ordering::SeqCst);
                Ok(())
            }
        })
        .await
        .unwrap();

    // 7 items dengan chunk_size 3 => 3 batches (3, 3, 1)
    assert_eq!(total_batches.load(Ordering::SeqCst), 3);
    assert_eq!(total_items.load(Ordering::SeqCst), 7);

    // 2. Uji .update_or_create (Kondisi belum ada -> INSERT)
    let created = TestContact::query(&pool)
        .where_eq("email", "baru@lumina.dev")
        .update_or_create([("name", "Kontak Baru"), ("email", "baru@lumina.dev")])
        .await
        .unwrap();

    assert_eq!(created.name, "Kontak Baru");
    assert_eq!(created.email, "baru@lumina.dev");

    // 3. Uji .update_or_create (Kondisi sudah ada -> UPDATE)
    let updated = TestContact::query(&pool)
        .where_eq("email", "baru@lumina.dev")
        .update_or_create([("name", "Kontak Baru (Terupdate)")])
        .await
        .unwrap();

    assert_eq!(updated.id, created.id);
    assert_eq!(updated.name, "Kontak Baru (Terupdate)");
}
