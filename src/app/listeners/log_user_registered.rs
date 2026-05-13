use async_trait::async_trait;
use crate::core::event::traits::{Event, Listener};
use crate::core::application::AppState;
use crate::app::events::user_registered::UserRegistered;
use std::sync::Arc;
use tracing::info;

pub struct LogUserRegistered;

#[async_trait]
impl Listener for LogUserRegistered {
    async fn handle(&self, event: &dyn Event, _state: Arc<AppState>) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // Downcast event ke tipe yang spesifik
        if let Some(user_event) = event.as_any().downcast_ref::<UserRegistered>() {
            info!("Listener: User baru terdaftar! Nama: {}, Email: {}", user_event.name, user_event.email);
        }
        
        Ok(())
    }
}
