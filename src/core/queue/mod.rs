use async_trait::async_trait;
use tokio::sync::mpsc;
use std::sync::Arc;
use crate::core::application::AppState;

/// Trait yang harus diimplementasikan oleh setiap Background Job.
#[async_trait]
pub trait Job: Send + Sync + std::fmt::Debug {
    /// Logika utama yang akan dijalankan di background.
    async fn handle(&self, state: Arc<AppState>) -> Result<(), String>;
}

/// QueueManager mengelola pengiriman job ke worker melalui channel.
pub struct QueueManager {
    sender: mpsc::Sender<Box<dyn Job>>,
}

impl QueueManager {
    /// Membuat instance baru QueueManager.
    pub fn new(sender: mpsc::Sender<Box<dyn Job>>) -> Self {
        Self { sender }
    }

    /// Mengirim job ke antrean untuk diproses.
    pub async fn dispatch<J: Job + 'static>(&self, job: J) -> Result<(), String> {
        self.sender
            .send(Box::new(job))
            .await
            .map_err(|e| format!("Gagal mengirim job ke antrean: {}", e))
    }
}

/// Worker yang bertugas mendengarkan antrean dan mengeksekusi job.
pub struct QueueWorker {
    receiver: mpsc::Receiver<Box<dyn Job>>,
}

impl QueueWorker {
    pub fn new(receiver: mpsc::Receiver<Box<dyn Job>>) -> Self {
        Self { receiver }
    }

    /// Menjalankan loop worker secara asinkron.
    pub async fn run(mut self, state: Arc<AppState>) {
        tracing::info!("🚀 Queue Worker started and listening for jobs...");
        
        while let Some(job) = self.receiver.recv().await {
            let state_clone = state.clone();
            tracing::info!("📦 Processing job: {:?}", job);
            
            // Eksekusi job
            tokio::spawn(async move {
                match job.handle(state_clone).await {
                    Ok(_) => tracing::info!("✅ Job completed successfully: {:?}", job),
                    Err(e) => tracing::error!("❌ Job failed: {:?}. Error: {}", job, e),
                }
            });
        }
    }
}
