use async_trait::async_trait;
use crate::core::queue::Job;
use crate::core::application::AppState;
use std::sync::Arc;
use tokio::time::{sleep, Duration};

#[derive(Debug)]
pub struct TestJob {
    pub message: String,
}

impl TestJob {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_string(),
        }
    }
}

#[async_trait]
impl Job for TestJob {
    async fn handle(&self, _state: Arc<AppState>) -> Result<(), String> {
        tracing::info!("🕒 TestJob started: {}", self.message);
        
        // Simulasi proses berat (5 detik)
        sleep(Duration::from_secs(5)).await;
        
        tracing::info!("🎉 Job Selesai Dikerjakan!: {}", self.message);
        Ok(())
    }
}
