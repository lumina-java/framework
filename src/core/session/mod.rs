pub mod store;

use serde::{Deserialize, Serialize};
use tower_sessions::Session;

/// Key yang digunakan untuk menyimpan flash messages di session.
const FLASH_SESSION_KEY: &str = "_flash";

/// Struktur pesan Flash.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FlashMessage {
    pub kind: String,
    pub message: String,
}

/// Helper untuk mengelola Flash Messages di dalam Session.
pub struct FlashManager<'a> {
    session: &'a Session,
}

impl<'a> FlashManager<'a> {
    pub fn new(session: &'a Session) -> Self {
        Self { session }
    }

    /// Menambahkan pesan sukses (success).
    pub async fn success(&self, message: &str) {
        self.add("success", message).await;
    }

    /// Menambahkan pesan error (error).
    pub async fn error(&self, message: &str) {
        self.add("error", message).await;
    }

    /// Menambahkan pesan info (info).
    pub async fn info(&self, message: &str) {
        self.add("info", message).await;
    }

    /// Menambahkan pesan peringatan (warning).
    pub async fn warning(&self, message: &str) {
        self.add("warning", message).await;
    }

    /// Menambahkan pesan dengan kategori kustom.
    pub async fn add(&self, kind: &str, message: &str) {
        let mut messages = self.get_all().await;
        messages.push(FlashMessage {
            kind: kind.to_string(),
            message: message.to_string(),
        });
        let _ = self.session.insert(FLASH_SESSION_KEY, messages).await;
    }

    /// Mengambil semua pesan dan menghapusnya dari session (consume).
    pub async fn consume(&self) -> Vec<FlashMessage> {
        let messages = self.get_all().await;
        let _ = self
            .session
            .remove::<Vec<FlashMessage>>(FLASH_SESSION_KEY)
            .await;
        messages
    }

    /// Mengambil semua pesan tanpa menghapusnya.
    async fn get_all(&self) -> Vec<FlashMessage> {
        self.session
            .get::<Vec<FlashMessage>>(FLASH_SESSION_KEY)
            .await
            .unwrap_or_default()
            .unwrap_or_default()
    }
}
