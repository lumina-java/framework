use lumina::prelude::*;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow, Clone, Default, LuminaModel, PartialEq)]
#[lumina(table = "customers")]
pub struct Customer {
    pub id: i64,
    pub name: String,
    pub email: String,
    pub phone: String,
    pub city: String,
    pub company: String,
    pub balance: i64,
    pub is_vip: i64,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub deleted_at: Option<String>,
}

pub struct CustomerFactory;

impl CustomerFactory {
    pub fn new() -> Self {
        Self
    }
}

impl Factory<Customer> for CustomerFactory {
    fn definition(&self) -> Customer {
        Customer {
            id: 0,
            name: Faker::name(),
            email: Faker::unique_email(),
            phone: Faker::phone(),
            city: Faker::city().to_string(),
            company: Faker::company(),
            balance: Faker::rupiah_amount(50_000, 5_000_000) as i64,
            is_vip: 0,
            ..Default::default()
        }
    }
}

impl HasFactory<CustomerFactory> for Customer {
    fn factory() -> CustomerFactory {
        CustomerFactory::new()
    }
}

#[tokio::test]
async fn test_faker_and_factory_in_memory() {
    let factory = CustomerFactory::new();

    // 1. make_one test
    let customer = factory.make_one();
    assert!(!customer.name.is_empty());
    assert!(customer.email.contains("@example.com"));
    assert!(customer.phone.starts_with("08"));
    assert!(!customer.city.is_empty());
    assert!(!customer.company.is_empty());
    assert_eq!(customer.is_vip, 0);

    // 2. count test
    let customers = factory.count(15).make();
    assert_eq!(customers.len(), 15);

    // 3. state modifier test
    let vip_customer = Customer::factory()
        .state(|c| {
            c.is_vip = 1;
            c.balance = 100_000_000;
        })
        .make_one();

    assert_eq!(vip_customer.is_vip, 1);
    assert_eq!(vip_customer.balance, 100_000_000);
}

#[cfg(feature = "sqlite")]
#[tokio::test]
async fn test_factory_persistence_sqlite() {
    let pool = DatabasePool::connect("sqlite:file:memdb_factory?mode=memory&cache=shared").await.unwrap();
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS customers (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            email TEXT NOT NULL,
            phone TEXT NOT NULL,
            city TEXT NOT NULL,
            company TEXT NOT NULL,
            balance INTEGER NOT NULL,
            is_vip INTEGER NOT NULL DEFAULT 0,
            created_at TEXT DEFAULT CURRENT_TIMESTAMP,
            updated_at TEXT DEFAULT CURRENT_TIMESTAMP,
            deleted_at TEXT
        )"
    )
    .execute(&pool.pool)
    .await
    .unwrap();

    // 1. create_one test
    let saved_one = Customer::factory().create_one(&pool).await.unwrap();
    assert!(saved_one.id > 0);
    assert!(!saved_one.name.is_empty());

    // 2. count(5).create(&pool) test with state modifier
    let saved_batch = Customer::factory()
        .count(5)
        .state(|c| {
            c.is_vip = 1;
            c.company = "PT VIP Indonesia".to_string();
        })
        .create(&pool)
        .await
        .unwrap();

    assert_eq!(saved_batch.len(), 5);
    for item in &saved_batch {
        assert!(item.id > 0);
        assert_eq!(item.is_vip, 1);
        assert_eq!(item.company, "PT VIP Indonesia");
    }

    // Verify in database
    let count: i64 = Customer::count(&pool).await.unwrap();
    assert_eq!(count, 6); // 1 + 5
}
