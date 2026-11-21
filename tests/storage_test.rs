use pakem::core::storage::{LocalStorage, S3Config, S3Storage, Storage, StorageDriver};
use std::sync::Arc;
use std::time::Duration;

#[tokio::test]
async fn test_local_storage_crud_and_traversal_guard() {
    let temp_dir = std::env::temp_dir().join("lumina_storage_test");
    let storage = LocalStorage::new(temp_dir.to_str().unwrap(), "http://localhost:8000/storage");

    let file_path = "avatars/user_42.txt";
    let content = b"Hello, Lumina Universal Storage!";

    // 1. Put
    let put_res = storage.put(file_path, content).await;
    assert!(put_res.is_ok(), "Put should succeed: {:?}", put_res.err());

    // 2. Exists
    assert!(storage.exists(file_path).await, "File should exist");

    // 3. Get
    let read_res = storage.get(file_path).await;
    assert!(read_res.is_ok());
    assert_eq!(read_res.unwrap(), content);

    // 4. URL
    let url = storage.url(file_path);
    assert_eq!(url, "http://localhost:8000/storage/avatars/user_42.txt");

    // 5. Anti-Directory Traversal
    let attack_path = "../../etc/passwd";
    let attack_res = storage.put(attack_path, b"malicious").await;
    assert!(attack_res.is_err(), "Directory traversal should be rejected");
    assert!(attack_res.unwrap_err().contains("directory traversal"));

    // 6. Delete
    let del_res = storage.delete(file_path).await;
    assert!(del_res.is_ok());
    assert!(!storage.exists(file_path).await, "File should no longer exist");

    // Clean up temp dir
    let _ = tokio::fs::remove_dir_all(temp_dir).await;
}

#[test]
fn test_s3_presigned_url_generation() {
    let config = S3Config {
        bucket: "hospital-records".to_string(),
        region: "ap-southeast-1".to_string(),
        endpoint: None,
        access_key: "AKIAIOSFODNN7EXAMPLE".to_string(),
        secret_key: "wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY".to_string(),
        use_path_style: false,
        public_url_base: None,
    };

    let s3 = S3Storage::new(config);
    let presigned_res = s3.temporary_url("invoices/2026/inv_001.pdf", Duration::from_secs(3600));

    assert!(presigned_res.is_ok(), "Presigned URL generation should succeed");
    let url = presigned_res.unwrap();

    assert!(url.contains("https://hospital-records.s3.ap-southeast-1.amazonaws.com/invoices/2026/inv_001.pdf"));
    assert!(url.contains("X-Amz-Algorithm=AWS4-HMAC-SHA256"));
    assert!(url.contains("X-Amz-Credential="));
    assert!(url.contains("X-Amz-Date="));
    assert!(url.contains("X-Amz-Expires=3600"));
    assert!(url.contains("X-Amz-SignedHeaders=host"));
    assert!(url.contains("X-Amz-Signature="));
}

#[test]
fn test_s3_minio_path_style_url() {
    let config = S3Config {
        bucket: "local-bucket".to_string(),
        region: "us-east-1".to_string(),
        endpoint: Some("http://127.0.0.1:9000".to_string()),
        access_key: "minioadmin".to_string(),
        secret_key: "minioadmin".to_string(),
        use_path_style: true,
        public_url_base: None,
    };

    let s3 = S3Storage::new(config);
    let presigned_res = s3.temporary_url("uploads/photo.jpg", Duration::from_secs(1800));

    assert!(presigned_res.is_ok());
    let url = presigned_res.unwrap();

    assert!(url.contains("http://127.0.0.1:9000/local-bucket/uploads/photo.jpg"));
    assert!(url.contains("X-Amz-Algorithm=AWS4-HMAC-SHA256"));
    assert!(url.contains("X-Amz-Expires=1800"));
}

#[tokio::test]
async fn test_storage_facade_multi_disk() {
    let mut storage = Storage::new("local");

    let temp_dir = std::env::temp_dir().join("lumina_facade_test");
    let local = Arc::new(LocalStorage::new(temp_dir.to_str().unwrap(), "/files"));
    storage.add_disk("local", local);

    let s3_config = S3Config {
        bucket: "cloud-bucket".to_string(),
        region: "us-east-1".to_string(),
        endpoint: None,
        access_key: "key".to_string(),
        secret_key: "secret".to_string(),
        use_path_style: false,
        public_url_base: Some("https://cdn.lumina.dev".to_string()),
    };
    storage.add_disk("s3", Arc::new(S3Storage::new(s3_config)));

    // Test default disk
    let put_res = storage.put("hello.txt", b"World").await;
    assert!(put_res.is_ok());
    assert!(storage.exists("hello.txt").await);

    // Test S3 disk public CDN URL
    let s3_disk = storage.disk("s3");
    let cdn_url = s3_disk.url("assets/logo.png");
    assert_eq!(cdn_url, "https://cdn.lumina.dev/assets/logo.png");

    // Clean up
    let _ = storage.delete("hello.txt").await;
    let _ = tokio::fs::remove_dir_all(temp_dir).await;
}

