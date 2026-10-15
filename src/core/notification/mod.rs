use async_trait::async_trait;
use serde_json::Value;

/// Trait Notification — Kontrak untuk class notifikasi.
#[async_trait]
pub trait Notification: Send + Sync {
    /// Judul notifikasi.
<<<<<<< HEAD
    fn subject(&self) -> String {
        "Notification from Lumina".to_string()
    }

    /// Representasi Mail.
    fn to_mail(&self) -> Option<(String, Value)> {
        None
    }

    /// Representasi Database (untuk disimpan di tabel notifications).
    fn to_database(&self) -> Option<Value> {
        None
    }

    /// Representasi WhatsApp.
    fn to_whatsapp(&self) -> Option<String> {
        None
    }
=======
    fn subject(&self) -> String { "Notification from Lumina".to_string() }

    /// Representasi Mail.
    fn to_mail(&self) -> Option<(String, Value)> { None }

    /// Representasi Database (untuk disimpan di tabel notifications).
    fn to_database(&self) -> Option<Value> { None }

    /// Representasi WhatsApp.
    fn to_whatsapp(&self) -> Option<String> { None }
>>>>>>> dev/master
}

/// Notify — Helper untuk mengirim notifikasi ke user.
pub struct Notify;

impl Notify {
    /// Kirim notifikasi ke email user.
<<<<<<< HEAD
    pub async fn send<N>(
        req: &crate::core::request::Request,
        to_email: &str,
        notification: N,
    ) -> Result<(), String>
=======
    pub async fn send<N>(req: &crate::core::request::Request, to_email: &str, notification: N) -> Result<(), String>
>>>>>>> dev/master
    where
        N: Notification,
    {
        if let Some((template, data)) = notification.to_mail() {
            req.mail()
                .to(to_email)
                .subject(&notification.subject())
                .send(&template, data)
                .await?;
        }
        Ok(())
    }
}
