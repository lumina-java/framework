use async_trait::async_trait;
use std::sync::Arc;
use crate::core::application::AppState;
use crate::core::queue::Job;
use serde_json::Value;

#[derive(Debug)]
pub struct TestJob;

#[async_trait]
impl Job for TestJob {
    fn name(&self) -> &'static str {
        "test_job"
    }

    async fn handle(&self, _state: Arc<AppState>, payload: Value) -> Result<(), String> {
        tracing::info!("🏃 Memproses TestJob dengan data: {:?}", payload);
        
        // Simulasi kerja berat
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
        
        println!("✅ TestJob Selesai!");
        Ok(())
    }
}
