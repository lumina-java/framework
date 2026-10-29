use hmac::{Hmac, Mac};
use sha2::Sha256;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use lumina::core::error::AppError;
use std::sync::OnceLock;

type HmacSha256 = Hmac<Sha256>;

static SECRET_KEY: OnceLock<Vec<u8>> = OnceLock::new();

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LiveSnapshot {
    pub id: String,
    pub name: String,
    pub data: Value,
}

impl LiveSnapshot {
    pub fn new(id: &str, name: &str, data: Value) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            data,
        }
    }
}

pub struct SnapshotManager;

impl SnapshotManager {
    fn get_secret_key() -> &'static [u8] {
        SECRET_KEY.get_or_init(|| {
            std::env::var("APP_KEY")
                .unwrap_or_else(|_| "lumina-live-insecure-default-key-change-in-production".to_string())
                .into_bytes()
        })
    }

    pub fn seal(snapshot: &LiveSnapshot) -> Result<String, AppError> {
        let json_str = serde_json::to_string(snapshot)
            .map_err(|e| AppError::Generic(format!("Failed to serialize snapshot: {}", e)))?;
        let encoded_payload = hex::encode(json_str.as_bytes());

        let mut mac = HmacSha256::new_from_slice(Self::get_secret_key())
            .map_err(|e| AppError::Generic(format!("HMAC init failed: {}", e)))?;
        mac.update(encoded_payload.as_bytes());
        let signature = hex::encode(mac.finalize().into_bytes());

        Ok(format!("{}.{}", encoded_payload, signature))
    }

    pub fn unseal(token: &str) -> Result<LiveSnapshot, AppError> {
        let parts: Vec<&str> = token.split('.').collect();
        if parts.len() != 2 {
            return Err(AppError::BadRequest("Token snapshot Livewire tidak valid (format salah)".to_string()));
        }

        let (payload_hex, signature_hex) = (parts[0], parts[1]);

        let mut mac = HmacSha256::new_from_slice(Self::get_secret_key())
            .map_err(|e| AppError::Generic(format!("HMAC init failed: {}", e)))?;
        mac.update(payload_hex.as_bytes());
        let expected_signature = hex::encode(mac.finalize().into_bytes());

        if signature_hex != expected_signature {
            return Err(AppError::BadRequest("Signature snapshot Livewire tidak sah! (Tampering terdeteksi)".to_string()));
        }

        let json_bytes = hex::decode(payload_hex)
            .map_err(|e| AppError::BadRequest(format!("Format payload snapshot rusak: {}", e)))?;

        let snapshot: LiveSnapshot = serde_json::from_slice(&json_bytes)
            .map_err(|e| AppError::BadRequest(format!("Deserialisasi snapshot gagal: {}", e)))?;

        Ok(snapshot)
    }
}
