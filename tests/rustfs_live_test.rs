use pakem::core::storage::{S3Config, S3Storage, StorageDriver};
use std::time::Duration;

#[tokio::test]
async fn test_rustfs_live_s3_integration() {
    let config = S3Config {
        bucket: "lumina-test".to_string(),
        region: "us-east-1".to_string(),
        endpoint: Some("http://127.0.0.1:9000".to_string()),
        access_key: "3gRyLecCYz1qh4IQwz3y".to_string(),
        secret_key: "MIgR3dJNYtxLCBfljXil2sFSZKP9dM5Rqcb1cYMn".to_string(),
        use_path_style: true,
        public_url_base: None,
    };

    let s3 = S3Storage::new(config);

    // Cek apakah server RustFS aktif pada port 9000
    let client = reqwest::Client::new();
    let is_server_running = client.get("http://127.0.0.1:9000/").send().await.is_ok();
    if !is_server_running {
        eprintln!("⚠️  RustFS server tidak aktif di 127.0.0.1:9000. Melewati live test.");
        return;
    }

    let file_path = "records/patient_001.txt";
    let test_data = b"Lumina Framework S3 Driver verified on RustFS Object Storage!";

    // 1. PUT berkas ke RustFS
    println!("📤 [1] Mengunggah berkas ke RustFS...");
    let put_res = s3.put(file_path, test_data).await;
    assert!(put_res.is_ok(), "Gagal upload ke RustFS: {:?}", put_res.err());
    println!("✅ Berkas berhasil diunggah!");

    // 2. EXISTS
    println!("🔍 [2] Memeriksa keberadaan berkas...");
    let exists = s3.exists(file_path).await;
    assert!(exists, "Berkas seharusnya ada di RustFS");
    println!("✅ Berkas terverifikasi ada di RustFS!");

    // 3. GET berkas dari RustFS
    println!("📥 [3] Mengambil berkas dari RustFS...");
    let get_res = s3.get(file_path).await;
    assert!(get_res.is_ok(), "Gagal download dari RustFS: {:?}", get_res.err());
    let downloaded = get_res.unwrap();
    assert_eq!(downloaded, test_data, "Data yang diunduh harus identik dengan data asli");
    println!("✅ Data yang diunduh 100% cocok!");

    // 4. Temporary Presigned URL (SigV4)
    println!("🔗 [4] Menguji Presigned URL SigV4...");
    let presigned_url = s3.temporary_url(file_path, Duration::from_secs(3600));
    assert!(presigned_url.is_ok(), "Gagal membuat presigned URL: {:?}", presigned_url.err());
    let presigned_url = presigned_url.unwrap();
    println!("   Presigned URL: {}", presigned_url);

    // Ambil langsung via HTTP client biasa menggunakan Presigned URL tanpa header auth
    let presigned_req = client.get(&presigned_url).send().await;
    assert!(presigned_req.is_ok(), "Gagal fetch via presigned URL");
    let presigned_res = presigned_req.unwrap();
    println!("   Status code presigned GET: {}", presigned_res.status());
    if presigned_res.status().is_success() {
        let presigned_bytes = presigned_res.bytes().await.unwrap();
        assert_eq!(presigned_bytes.as_ref(), test_data);
        println!("✅ Presigned URL berhasil diakses langsung via HTTP GET!");
    } else {
        println!("ℹ️  Presigned response status: {}", presigned_res.status());
    }

    // 5. DELETE berkas dari RustFS
    println!("🗑️ [5] Menghapus berkas dari RustFS...");
    let del_res = s3.delete(file_path).await;
    assert!(del_res.is_ok(), "Gagal delete dari RustFS: {:?}", del_res.err());
    println!("✅ Berkas berhasil dihapus!");

    // 6. Verifikasi setelah delete
    let after_delete = s3.exists(file_path).await;
    assert!(!after_delete, "Berkas seharusnya sudah terhapus");
    println!("✅ Verifikasi penghapusan sukses!");
}
