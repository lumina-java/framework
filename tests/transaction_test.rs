use lumina::database::connection::DatabasePool;

#[tokio::test]
async fn test_transaction_commit_and_rollback() {
    let _ = std::fs::remove_file("target/test_tx.db");
    let pool = DatabasePool::connect("sqlite:target/test_tx.db?mode=rwc")
        .await
        .expect("Koneksi DB SQLite gagal");

    // Setup tabel
    pool.execute_raw(
        "CREATE TABLE IF NOT EXISTS accounts (id INTEGER PRIMARY KEY, name TEXT, balance INTEGER);",
    )
    .await
    .unwrap();

    pool.execute_raw("INSERT INTO accounts (id, name, balance) VALUES (1, 'Alice', 100000);")
        .await
        .unwrap();

    pool.execute_raw("INSERT INTO accounts (id, name, balance) VALUES (2, 'Bob', 50000);")
        .await
        .unwrap();

    // 1. Uji Transaksi Commit
    let tx = pool.begin().await.expect("Gagal mulai transaksi");
    tx.table("accounts")
        .where_eq("id", 1)
        .decrement("balance", 20000)
        .await
        .unwrap();

    tx.table("accounts")
        .where_eq("id", 2)
        .increment("balance", 20000)
        .await
        .unwrap();

    tx.commit().await.expect("Commit gagal");

    // Verifikasi saldo setelah commit
    let alice: (i64,) = sqlx::query_as("SELECT balance FROM accounts WHERE id = 1;")
        .fetch_one(&pool.pool)
        .await
        .unwrap();
    assert_eq!(alice.0, 80000);

    let bob: (i64,) = sqlx::query_as("SELECT balance FROM accounts WHERE id = 2;")
        .fetch_one(&pool.pool)
        .await
        .unwrap();
    assert_eq!(bob.0, 70000);

    // 2. Uji Auto-Rollback saat Drop (tanpa commit)
    {
        let tx_aborted = pool.begin().await.expect("Gagal mulai transaksi 2");
        tx_aborted
            .table("accounts")
            .where_eq("id", 1)
            .decrement("balance", 50000)
            .await
            .unwrap();
        // tx_aborted keluar dari scope di sini TANPA commit!
    }

    // Beri sedikit jeda untuk background rollback task
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    // Verifikasi saldo Alice tetap 80000 (tidak berkurang 50000 karena auto-rollback!)
    let alice_aborted: (i64,) = sqlx::query_as("SELECT balance FROM accounts WHERE id = 1;")
        .fetch_one(&pool.pool)
        .await
        .unwrap();
    assert_eq!(alice_aborted.0, 80000);

    // 3. Uji Savepoint & Rollback To
    let tx_save = pool.begin().await.expect("Gagal mulai transaksi 3");
    tx_save
        .table("accounts")
        .where_eq("id", 1)
        .decrement("balance", 10000)
        .await
        .unwrap();

    tx_save.savepoint("point_1").await.unwrap();

    // Perubahan yang nantinya dibatalkan
    tx_save
        .table("accounts")
        .where_eq("id", 1)
        .decrement("balance", 99999)
        .await
        .unwrap();

    tx_save.rollback_to("point_1").await.unwrap();
    tx_save.commit().await.unwrap();

    let alice_final: (i64,) = sqlx::query_as("SELECT balance FROM accounts WHERE id = 1;")
        .fetch_one(&pool.pool)
        .await
        .unwrap();
    assert_eq!(alice_final.0, 70000);

    println!("✅ Seluruh tes transaksi ACID tanpa tanda pipa '|' dan tanpa generic '<>' berhasil 100%!");
}
