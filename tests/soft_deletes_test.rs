use lumina::database::connection::DatabasePool;

#[tokio::test]
async fn test_smart_soft_deletes_and_zero_ampersand_transactions() {
    let _ = std::fs::remove_file("target/test_soft_deletes.db");
    let pool = DatabasePool::connect("sqlite:target/test_soft_deletes.db?mode=rwc")
        .await
        .expect("Koneksi DB SQLite gagal");

    // 1. Tabel Users (dengan soft deletes: memiliki deleted_at)
    pool.execute_raw(
        "CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT,
            email TEXT,
            deleted_at TEXT
        );",
    )
    .await
    .unwrap();

    // 2. Tabel Roles (TIDAK MEMILIKI deleted_at — uji anti-crash)
    pool.execute_raw(
        "CREATE TABLE IF NOT EXISTS roles (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT
        );",
    )
    .await
    .unwrap();

    // ==========================================
    // UJI GAYA 1: Direct Transaction (tx.table, tx.find)
    // ==========================================
    let tx1 = pool.begin().await.expect("Gagal mulai transaksi 1");

    // Insert tanpa generic <> dan tanpa &tx
    tx1.table("users")
        .insert([("name", "Budi"), ("email", "budi@lumina.dev")])
        .await
        .unwrap();

    tx1.table("roles")
        .insert([("title", "Superadministrator")])
        .await
        .unwrap();

    tx1.commit().await.expect("Commit gagal");

    // Verifikasi data masuk
    let tx2 = pool.begin().await.unwrap();
    let user_json = tx2.find("users", 1).await.unwrap().expect("User harus ada");
    assert_eq!(user_json["name"], "Budi");

    let role_json = tx2.find("roles", 1).await.unwrap().expect("Role harus ada");
    assert_eq!(role_json["title"], "Superadministrator");

    // ==========================================
    // UJI SMART SOFT DELETES PADA TABEL USERS
    // ==========================================
    // Hapus sementara (Soft Delete)
    tx2.table("users")
        .soft_deletes()
        .where_eq("id", 1)
        .delete()
        .await
        .unwrap();

    // Pastikan user tersembunyi dari query default
    let active_users = tx2.table("users").soft_deletes().get().await.unwrap();
    assert_eq!(active_users.len(), 0, "User harusnya tersembunyi karena soft delete");

    // Pastikan user muncul pada with_trashed()
    let all_users = tx2.table("users").with_trashed().get().await.unwrap();
    assert_eq!(all_users.len(), 1, "User harus muncul di with_trashed");

    // Pastikan user muncul pada only_trashed()
    let trashed_users = tx2.table("users").only_trashed().get().await.unwrap();
    assert_eq!(trashed_users.len(), 1, "User harus muncul di only_trashed");

    // Pulihkan kembali (Restore)
    tx2.table("users").where_eq("id", 1).restore().await.unwrap();
    let restored_users = tx2.table("users").soft_deletes().get().await.unwrap();
    assert_eq!(restored_users.len(), 1, "User harus kembali aktif setelah restore");

    // Hapus fisik permanen (Force Delete)
    tx2.table("users").where_eq("id", 1).force_delete().await.unwrap();
    let remaining_users = tx2.table("users").with_trashed().get().await.unwrap();
    assert_eq!(remaining_users.len(), 0, "User harus musnah total setelah force_delete");

    // ==========================================
    // UJI ANTI-CRASH PADA TABEL ROLES (TANPA deleted_at)
    // ==========================================
    // Tabel roles sama sekali tidak punya kolom deleted_at!
    // Query get() dan delete() tidak boleh menambahkan `deleted_at IS NULL` yang menyebabkan crash.
    let roles = tx2.table("roles").get().await.expect("Query roles tidak boleh crash!");
    assert_eq!(roles.len(), 1);

    tx2.table("roles").where_eq("id", 1).delete().await.expect("Delete roles tidak boleh crash!");
    let roles_after = tx2.table("roles").get().await.unwrap();
    assert_eq!(roles_after.len(), 0);

    tx2.commit().await.unwrap();

    println!("✅ Seluruh tes Smart Soft Deletes & Zero-Ampersand Transaction sukses 100%!");
}

