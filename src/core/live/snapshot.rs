use hmac::{Hmac, Mac};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::Sha256;
use subtle::ConstantTimeEq;
use crate::core::error::AppError;

type HmacSha256 = Hmac<Sha256>;

/// Struktur snapshot data komponen yang dikirimkan ke dan dari browser.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiveSnapshot {
    pub id: String,
    pub name: String,
    pub data: Value,
}

impl LiveSnapshot {
    pub fn new(id: impl Into<String>, name: impl Into<String>, data: Value) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            data,
        }
    }
}

/// Pengelola tanda tangan kriptografis untuk memastikan snapshot tidak dimanipulasi di sisi client.
pub struct SnapshotManager;

impl SnapshotManager {
    /// Mengambil secret key dari environment APP_KEY atau default fallback yang aman.
    pub fn secret() -> Vec<u8> {
        std::env::var("APP_KEY")
            .unwrap_or_else(|_| "lumina-default-secret-key-change-in-production-32bytes!".to_string())
            .into_bytes()
    }

    /// Menandatangani payload snapshot dengan HMAC-SHA256 dan mengembalikan token hex aman untuk HTML.
    pub fn seal(snapshot: &LiveSnapshot) -> Result<String, AppError> {
        let json_payload = serde_json::to_string(snapshot)
            .map_err(|e| AppError::Generic(format!("Failed to serialize snapshot: {}", e)))?;

        let secret = Self::secret();
        let mut mac = HmacSha256::new_from_slice(&secret)
            .map_err(|e| AppError::Generic(format!("HMAC key error: {}", e)))?;

        mac.update(json_payload.as_bytes());
        let signature = hex::encode(mac.finalize().into_bytes());
        let payload_hex = hex::encode(json_payload.as_bytes());

        Ok(format!("{}.{}", payload_hex, signature))
    }

    /// Memverifikasi dan mendekripsi token snapshot yang dikirimkan oleh browser.
    pub fn unseal(token: &str) -> Result<LiveSnapshot, AppError> {
        let parts: Vec<&str> = token.split('.').collect();
        if parts.len() != 2 {
            return Err(AppError::BadRequest("Invalid snapshot format".to_string()));
        }

        let payload_hex = parts[0];
        let signature_hex = parts[1];

        let payload_bytes = hex::decode(payload_hex)
            .map_err(|_| AppError::BadRequest("Malformed snapshot payload encoding".to_string()))?;

        // Verifikasi tanda tangan HMAC-SHA256
        let secret = Self::secret();
        let mut mac = HmacSha256::new_from_slice(&secret)
            .map_err(|e| AppError::Generic(format!("HMAC key error: {}", e)))?;

        mac.update(&payload_bytes);
        let expected_signature = hex::encode(mac.finalize().into_bytes());

        // Constant-time comparison anti timing attack
        let is_valid: bool = expected_signature
            .as_bytes()
            .ct_eq(signature_hex.as_bytes())
            .into();

        if !is_valid {
            return Err(AppError::BadRequest(
                "Live Component snapshot tampering detected: Signature mismatch".to_string(),
            ));
        }

        let snapshot: LiveSnapshot = serde_json::from_slice(&payload_bytes)
            .map_err(|e| AppError::BadRequest(format!("Corrupt snapshot JSON: {}", e)))?;

        Ok(snapshot)
    }
}
