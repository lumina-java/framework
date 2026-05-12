use async_trait::async_trait;
use std::sync::Arc;
use crate::core::view::ViewEngine;
use serde_json::Value;

#[async_trait]
pub trait MailDriver: Send + Sync {
    async fn send(&self, to: &str, subject: &str, html: String) -> Result<(), String>;
}

/// Driver untuk logging (Development)
pub struct LogDriver;

#[async_trait]
impl MailDriver for LogDriver {
    async fn send(&self, to: &str, subject: &str, html: String) -> Result<(), String> {
        println!();
        println!("📧 [MAIL LOG]");
        println!("To: {}", to);
        println!("Subject: {}", subject);
        println!("Content (HTML Preview): {}...", &html[..std::cmp::min(100, html.len())]);
        println!("--------------------------------------------------");
        Ok(())
    }
}

/// Driver untuk SMTP (Placeholder)
pub struct SmtpDriver;

#[async_trait]
impl MailDriver for SmtpDriver {
    async fn send(&self, _to: &str, _subject: &str, _html: String) -> Result<(), String> {
        Err("SMTP Driver belum diimplementasikan sepenuhnya. Silakan instal 'lettre'.".to_string())
    }
}

/// Mailer Manager
pub struct Mail {
    driver: Arc<dyn MailDriver>,
    view: ViewEngine,
}

impl Mail {
    pub fn new(driver: Arc<dyn MailDriver>, view: ViewEngine) -> Self {
        Self { driver, view }
    }

    /// Mulai membangun pengiriman email.
    pub fn to(&self, address: &str) -> MailBuilder<'_> {
        MailBuilder {
            mailer: self,
            to: address.to_string(),
            subject: String::new(),
        }
    }
}

/// Fluent Builder untuk pengiriman email.
pub struct MailBuilder<'a> {
    mailer: &'a Mail,
    to: String,
    subject: String,
}

impl<'a> MailBuilder<'a> {
    pub fn subject(mut self, subject: &str) -> Self {
        self.subject = subject.to_string();
        self
    }

    /// Kirim email menggunakan template View.
    pub async fn send(self, template: &str, data: Value) -> Result<(), String> {
        let mut ctx = tera::Context::new();
        if let Some(obj) = data.as_object() {
            for (k, v) in obj {
                ctx.insert(k, v);
            }
        }

        let html = self.mailer.view.render(template, &ctx);
        self.mailer.driver.send(&self.to, &self.subject, html).await
    }

    /// Kirim email dengan body raw (biasanya untuk testing).
    pub async fn send_raw(self, body: &str) -> Result<(), String> {
        self.mailer.driver.send(&self.to, &self.subject, body.to_string()).await
    }
}
