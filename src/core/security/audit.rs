use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

/// Entri rekaman log audit keamanan enterprise (Standar SOC 2 / ISO 27001).
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AuditLogEntry {
    pub timestamp: DateTime<Utc>,
    pub user_id: Option<String>,
    pub ip_address: String,
    pub user_agent: Option<String>,
    pub action: String,
    pub resource: String,
    pub resource_id: Option<String>,
    pub details: Option<Value>,
    pub status: String,
}

/// Enterprise Audit Trail Logger.
/// Berjalan secara non-blocking untuk menjamin performa server tidak terbebani saat pencatatan transaksi audit.
pub struct AuditTrail;

impl AuditTrail {
    /// Catat aktivitas sensitif ke berkas log audit (`storage/logs/audit.log`).
    pub fn record(
        user_id: Option<String>,
        ip_address: &str,
        user_agent: Option<String>,
        action: &str,
        resource: &str,
        resource_id: Option<String>,
        details: Option<Value>,
        status: &str,
    ) {
        let entry = AuditLogEntry {
            timestamp: Utc::now(),
            user_id,
            ip_address: ip_address.to_string(),
            user_agent,
            action: action.to_string(),
            resource: resource.to_string(),
            resource_id,
            details,
            status: status.to_string(),
        };

        // Kirim pencatatan ke background thread agar tidak menghambat response HTTP
        tokio::spawn(async move {
            Self::write_to_file(&entry);
        });
    }

    fn write_to_file(entry: &AuditLogEntry) {
        let dir = Path::new("storage/logs");
        if !dir.exists() {
            let _ = std::fs::create_dir_all(dir);
        }

        let file_path = dir.join("audit.log");
        if let Ok(mut file) = OpenOptions::new().create(true).append(true).open(file_path) {
            if let Ok(json_line) = serde_json::to_string(entry) {
                let _ = writeln!(file, "{}", json_line);
            }
        }
    }
}

