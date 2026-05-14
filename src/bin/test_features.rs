use lumina::core::resource::{JsonResource, ResourceCollection};
use lumina::database::connection::DatabaseKind;
use lumina::database::schema::Schema;
use serde_json::json;

#[tokio::main]
async fn main() {
    println!("--- Testing Schema Builder ---");

    let sqlite_sql = Schema::create_sql("users", DatabaseKind::Sqlite, |table| {
        table.id();
        table.string("name", Some(100));
        table.string("email", None).unique();
        table.timestamps();
        table.soft_deletes();
    });
    println!("SQLite SQL:\n{}", sqlite_sql);

    let mysql_sql = Schema::create_sql("users", DatabaseKind::MySql, |table| {
        table.id();
        table.string("name", Some(100));
        table.string("email", None).unique();
        table.timestamps();
        table.soft_deletes();
    });
    println!("\nMySQL SQL:\n{}", mysql_sql);

    println!("\n--- Testing API Resources ---");

    struct User {
        id: i64,
        name: String,
        email: String,
    }

    impl JsonResource for User {
        fn to_json(&self) -> serde_json::Value {
            json!({
                "id": self.id,
                "name": self.name,
                "email": self.email,
                "type": "User"
            })
        }
    }

    let user = User {
        id: 1,
        name: "Antigravity".to_string(),
        email: "ai@lumina.rs".to_string(),
    };

    println!(
        "Single Resource:\n{}",
        serde_json::to_string_pretty(&user.to_json()).unwrap()
    );

    let users = vec![
        User {
            id: 1,
            name: "Alice".to_string(),
            email: "alice@test.com".to_string(),
        },
        User {
            id: 2,
            name: "Bob".to_string(),
            email: "bob@test.com".to_string(),
        },
    ];

    let collection = ResourceCollection::new(users);
    // Note: In a real app, you'd return collection as an IntoResponse
    // For testing, we just manually transform
    let collection_json: Vec<serde_json::Value> =
        collection.data.iter().map(|u| u.to_json()).collect();
    println!(
        "\nCollection Resource:\n{}",
        serde_json::to_string_pretty(&json!({ "data": collection_json })).unwrap()
    );
}
