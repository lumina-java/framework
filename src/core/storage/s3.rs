use async_trait::async_trait;
use chrono::Utc;
use hmac::{Hmac, Mac};
use sha2::{Digest, Sha256};
use std::time::Duration;
use super::StorageDriver;

type HmacSha256 = Hmac<Sha256>;

/// Konfigurasi koneksi ke Object Storage S3-Compatible (AWS S3, Cloudflare R2, MinIO, dll).
#[derive(Debug, Clone)]
pub struct S3Config {
    pub bucket: String,
    pub region: String,
    pub endpoint: Option<String>,
    pub access_key: String,
    pub secret_key: String,
    pub use_path_style: bool,
    pub public_url_base: Option<String>,
}

impl S3Config {
    /// Inisialisasi konfigurasi dari environment variables atau fallback.
    pub fn from_env() -> Option<Self> {
        let bucket = std::env::var("AWS_BUCKET")
            .or_else(|_| std::env::var("S3_BUCKET"))
            .ok()?;
        let access_key = std::env::var("AWS_ACCESS_KEY_ID")
            .or_else(|_| std::env::var("S3_ACCESS_KEY"))
            .unwrap_or_default();
        let secret_key = std::env::var("AWS_SECRET_ACCESS_KEY")
            .or_else(|_| std::env::var("S3_SECRET_KEY"))
            .unwrap_or_default();
        let region = std::env::var("AWS_DEFAULT_REGION")
            .or_else(|_| std::env::var("S3_REGION"))
            .unwrap_or_else(|_| "us-east-1".to_string());
        let endpoint = std::env::var("AWS_ENDPOINT")
            .or_else(|_| std::env::var("S3_ENDPOINT"))
            .ok();
        let use_path_style = std::env::var("AWS_USE_PATH_STYLE_ENDPOINT")
            .or_else(|_| std::env::var("S3_USE_PATH_STYLE"))
            .map(|v| v == "true" || v == "1")
            .unwrap_or(false);
        let public_url_base = std::env::var("AWS_URL")
            .or_else(|_| std::env::var("S3_URL"))
            .ok();

        Some(Self {
            bucket,
            region,
            endpoint,
            access_key,
            secret_key,
            use_path_style,
            public_url_base,
        })
    }
}

/// Driver S3 mandiri performa tinggi berbasis REST API dengan Signature Version 4 (SigV4).
pub struct S3Storage {
    config: S3Config,
    client: reqwest::Client,
}

impl S3Storage {
    pub fn new(config: S3Config) -> Self {
        Self {
            config,
            client: reqwest::Client::builder()
                .timeout(Duration::from_secs(30))
                .build()
                .unwrap_or_default(),
        }
    }

    /// Menghitung hostname dan URL path lengkap sesuai mode virtual-host vs path-style.
    fn resolve_endpoint(&self, path: &str) -> (String, String, String) {
        let clean_path = path.trim_start_matches('/');
        if let Some(custom_endpoint) = &self.config.endpoint {
            let ep = custom_endpoint.trim_end_matches('/');
            let is_https = ep.starts_with("https://");
            let host_part = ep
                .trim_start_matches("http://")
                .trim_start_matches("https://");

            if self.config.use_path_style {
                // MinIO Style: http://127.0.0.1:9000/mybucket/file.txt
                let scheme = if is_https { "https" } else { "http" };
                let full_url = format!("{}://{}/{}/{}", scheme, host_part, self.config.bucket, clean_path);
                let uri_path = format!("/{}/{}", self.config.bucket, clean_path);
                (full_url, host_part.to_string(), uri_path)
            } else {
                // Virtual-host style: https://mybucket.r2.cloudflarestorage.com/file.txt
                let scheme = if is_https { "https" } else { "http" };
                let host = format!("{}.{}", self.config.bucket, host_part);
                let full_url = format!("{}://{}/{}", scheme, host, clean_path);
                let uri_path = format!("/{}", clean_path);
                (full_url, host, uri_path)
            }
        } else {
            // Standard AWS S3: https://mybucket.s3.us-east-1.amazonaws.com/file.txt
            let host = if self.config.region == "us-east-1" {
                format!("{}.s3.amazonaws.com", self.config.bucket)
            } else {
                format!("{}.s3.{}.amazonaws.com", self.config.bucket, self.config.region)
            };
            let full_url = format!("https://{}/{}", host, clean_path);
            let uri_path = format!("/{}", clean_path);
            (full_url, host, uri_path)
        }
    }

    /// Menghitung AWS Signature Version 4 Signing Key.
    fn get_signing_key(&self, date_stamp: &str) -> Vec<u8> {
        let k_date = hmac_sha256(format!("AWS4{}", self.config.secret_key).as_bytes(), date_stamp.as_bytes());
        let k_region = hmac_sha256(&k_date, self.config.region.as_bytes());
        let k_service = hmac_sha256(&k_region, b"s3");
        hmac_sha256(&k_service, b"aws4_request")
    }
}

/// Helper fungsi HMAC-SHA256
fn hmac_sha256(key: &[u8], data: &[u8]) -> Vec<u8> {
    let mut mac = HmacSha256::new_from_slice(key).expect("HMAC can take key of any size");
    mac.update(data);
    mac.finalize().into_bytes().to_vec()
}

/// Helper fungsi SHA256 Hex
fn sha256_hex(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hex::encode(hasher.finalize())
}

/// Helper URL encode untuk query parameter SigV4
fn url_encode(input: &str) -> String {
    let mut encoded = String::new();
    for b in input.bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                encoded.push(b as char);
            }
            _ => {
                encoded.push_str(&format!("%{:02X}", b));
            }
        }
    }
    encoded
}

#[async_trait]
impl StorageDriver for S3Storage {
    async fn put(&self, path: &str, contents: &[u8]) -> Result<(), String> {
        let (full_url, host, uri_path) = self.resolve_endpoint(path);
        let now = Utc::now();
        let amz_date = now.format("%Y%m%dT%H%M%SZ").to_string();
        let date_stamp = now.format("%Y%m%d").to_string();
        let payload_hash = sha256_hex(contents);

        let canonical_headers = format!("host:{}\nx-amz-content-sha256:{}\nx-amz-date:{}\n", host, payload_hash, amz_date);
        let signed_headers = "host;x-amz-content-sha256;x-amz-date";

        let canonical_request = format!(
            "PUT\n{}\n\n{}\n{}\n{}",
            uri_path, canonical_headers, signed_headers, payload_hash
        );

        let credential_scope = format!("{}/{}/s3/aws4_request", date_stamp, self.config.region);
        let string_to_sign = format!(
            "AWS4-HMAC-SHA256\n{}\n{}\n{}",
            amz_date, credential_scope, sha256_hex(canonical_request.as_bytes())
        );

        let signing_key = self.get_signing_key(&date_stamp);
        let signature = hex::encode(hmac_sha256(&signing_key, string_to_sign.as_bytes()));

        let authorization = format!(
            "AWS4-HMAC-SHA256 Credential={}/{}, SignedHeaders={}, Signature={}",
            self.config.access_key, credential_scope, signed_headers, signature
        );

        let res = self
            .client
            .put(&full_url)
            .header("Host", &host)
            .header("x-amz-date", &amz_date)
            .header("x-amz-content-sha256", &payload_hash)
            .header("Authorization", &authorization)
            .body(contents.to_vec())
            .send()
            .await
            .map_err(|e| format!("HTTP request error: {}", e))?;

        if res.status().is_success() {
            Ok(())
        } else {
            let status = res.status();
            let body = res.text().await.unwrap_or_default();
            Err(format!("S3 PUT failed with HTTP {}: {}", status, body))
        }
    }

    async fn get(&self, path: &str) -> Result<Vec<u8>, String> {
        let (full_url, host, uri_path) = self.resolve_endpoint(path);
        let now = Utc::now();
        let amz_date = now.format("%Y%m%dT%H%M%SZ").to_string();
        let date_stamp = now.format("%Y%m%d").to_string();
        let payload_hash = sha256_hex(b"");

        let canonical_headers = format!("host:{}\nx-amz-content-sha256:{}\nx-amz-date:{}\n", host, payload_hash, amz_date);
        let signed_headers = "host;x-amz-content-sha256;x-amz-date";

        let canonical_request = format!(
            "GET\n{}\n\n{}\n{}\n{}",
            uri_path, canonical_headers, signed_headers, payload_hash
        );

        let credential_scope = format!("{}/{}/s3/aws4_request", date_stamp, self.config.region);
        let string_to_sign = format!(
            "AWS4-HMAC-SHA256\n{}\n{}\n{}",
            amz_date, credential_scope, sha256_hex(canonical_request.as_bytes())
        );

        let signing_key = self.get_signing_key(&date_stamp);
        let signature = hex::encode(hmac_sha256(&signing_key, string_to_sign.as_bytes()));

        let authorization = format!(
            "AWS4-HMAC-SHA256 Credential={}/{}, SignedHeaders={}, Signature={}",
            self.config.access_key, credential_scope, signed_headers, signature
        );

        let res = self
            .client
            .get(&full_url)
            .header("Host", &host)
            .header("x-amz-date", &amz_date)
            .header("x-amz-content-sha256", &payload_hash)
            .header("Authorization", &authorization)
            .send()
            .await
            .map_err(|e| format!("HTTP request error: {}", e))?;

        if res.status().is_success() {
            res.bytes()
                .await
                .map(|b| b.to_vec())
                .map_err(|e| format!("Failed to read S3 body: {}", e))
        } else {
            let status = res.status();
            let body = res.text().await.unwrap_or_default();
            Err(format!("S3 GET failed with HTTP {}: {}", status, body))
        }
    }

    async fn exists(&self, path: &str) -> bool {
        let (full_url, host, uri_path) = self.resolve_endpoint(path);
        let now = Utc::now();
        let amz_date = now.format("%Y%m%dT%H%M%SZ").to_string();
        let date_stamp = now.format("%Y%m%d").to_string();
        let payload_hash = sha256_hex(b"");

        let canonical_headers = format!("host:{}\nx-amz-content-sha256:{}\nx-amz-date:{}\n", host, payload_hash, amz_date);
        let signed_headers = "host;x-amz-content-sha256;x-amz-date";

        let canonical_request = format!(
            "HEAD\n{}\n\n{}\n{}\n{}",
            uri_path, canonical_headers, signed_headers, payload_hash
        );

        let credential_scope = format!("{}/{}/s3/aws4_request", date_stamp, self.config.region);
        let string_to_sign = format!(
            "AWS4-HMAC-SHA256\n{}\n{}\n{}",
            amz_date, credential_scope, sha256_hex(canonical_request.as_bytes())
        );

        let signing_key = self.get_signing_key(&date_stamp);
        let signature = hex::encode(hmac_sha256(&signing_key, string_to_sign.as_bytes()));

        let authorization = format!(
            "AWS4-HMAC-SHA256 Credential={}/{}, SignedHeaders={}, Signature={}",
            self.config.access_key, credential_scope, signed_headers, signature
        );

        match self
            .client
            .head(&full_url)
            .header("Host", &host)
            .header("x-amz-date", &amz_date)
            .header("x-amz-content-sha256", &payload_hash)
            .header("Authorization", &authorization)
            .send()
            .await
        {
            Ok(res) => res.status().is_success(),
            Err(_) => false,
        }
    }

    async fn delete(&self, path: &str) -> Result<(), String> {
        let (full_url, host, uri_path) = self.resolve_endpoint(path);
        let now = Utc::now();
        let amz_date = now.format("%Y%m%dT%H%M%SZ").to_string();
        let date_stamp = now.format("%Y%m%d").to_string();
        let payload_hash = sha256_hex(b"");

        let canonical_headers = format!("host:{}\nx-amz-content-sha256:{}\nx-amz-date:{}\n", host, payload_hash, amz_date);
        let signed_headers = "host;x-amz-content-sha256;x-amz-date";

        let canonical_request = format!(
            "DELETE\n{}\n\n{}\n{}\n{}",
            uri_path, canonical_headers, signed_headers, payload_hash
        );

        let credential_scope = format!("{}/{}/s3/aws4_request", date_stamp, self.config.region);
        let string_to_sign = format!(
            "AWS4-HMAC-SHA256\n{}\n{}\n{}",
            amz_date, credential_scope, sha256_hex(canonical_request.as_bytes())
        );

        let signing_key = self.get_signing_key(&date_stamp);
        let signature = hex::encode(hmac_sha256(&signing_key, string_to_sign.as_bytes()));

        let authorization = format!(
            "AWS4-HMAC-SHA256 Credential={}/{}, SignedHeaders={}, Signature={}",
            self.config.access_key, credential_scope, signed_headers, signature
        );

        let res = self
            .client
            .delete(&full_url)
            .header("Host", &host)
            .header("x-amz-date", &amz_date)
            .header("x-amz-content-sha256", &payload_hash)
            .header("Authorization", &authorization)
            .send()
            .await
            .map_err(|e| format!("HTTP request error: {}", e))?;

        if res.status().is_success() || res.status() == reqwest::StatusCode::NO_CONTENT {
            Ok(())
        } else {
            let status = res.status();
            let body = res.text().await.unwrap_or_default();
            Err(format!("S3 DELETE failed with HTTP {}: {}", status, body))
        }
    }

    fn url(&self, path: &str) -> String {
        let clean_path = path.trim_start_matches('/');
        if let Some(base) = &self.config.public_url_base {
            format!("{}/{}", base.trim_end_matches('/'), clean_path)
        } else {
            let (full_url, _, _) = self.resolve_endpoint(clean_path);
            full_url
        }
    }

    fn temporary_url(&self, path: &str, ttl: Duration) -> Result<String, String> {
        let clean_path = path.trim_start_matches('/');
        let (full_url, host, uri_path) = self.resolve_endpoint(clean_path);
        let now = Utc::now();
        let amz_date = now.format("%Y%m%dT%H%M%SZ").to_string();
        let date_stamp = now.format("%Y%m%d").to_string();
        let expires = ttl.as_secs().to_string();

        let credential_scope = format!("{}/{}/s3/aws4_request", date_stamp, self.config.region);
        let credential = format!("{}/{}", self.config.access_key, credential_scope);

        let query_params = vec![
            ("X-Amz-Algorithm", "AWS4-HMAC-SHA256".to_string()),
            ("X-Amz-Credential", credential),
            ("X-Amz-Date", amz_date.clone()),
            ("X-Amz-Expires", expires),
            ("X-Amz-SignedHeaders", "host".to_string()),
        ];

        let mut sorted_params = query_params.clone();
        sorted_params.sort_by(|a, b| a.0.cmp(b.0));

        let canonical_query = sorted_params
            .iter()
            .map(|(k, v)| format!("{}={}", url_encode(k), url_encode(v)))
            .collect::<Vec<_>>()
            .join("&");

        let canonical_headers = format!("host:{}\n", host);
        let signed_headers = "host";
        let payload_hash = "UNSIGNED-PAYLOAD";

        let canonical_request = format!(
            "GET\n{}\n{}\n{}\n{}\n{}",
            uri_path, canonical_query, canonical_headers, signed_headers, payload_hash
        );

        let string_to_sign = format!(
            "AWS4-HMAC-SHA256\n{}\n{}\n{}",
            amz_date, credential_scope, sha256_hex(canonical_request.as_bytes())
        );

        let signing_key = self.get_signing_key(&date_stamp);
        let signature = hex::encode(hmac_sha256(&signing_key, string_to_sign.as_bytes()));

        let separator = if full_url.contains('?') { "&" } else { "?" };
        Ok(format!("{}{}X-Amz-Algorithm=AWS4-HMAC-SHA256&X-Amz-Credential={}&X-Amz-Date={}&X-Amz-Expires={}&X-Amz-SignedHeaders=host&X-Amz-Signature={}",
            full_url, separator, url_encode(&sorted_params[1].1), amz_date, sorted_params[3].1, signature
        ))
    }
}
