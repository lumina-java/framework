use async_trait::async_trait;
use std::any::Any;
use std::sync::Arc;
use crate::core::application::AppState;

/// Trait untuk data Event.
/// Harus mengimplementasikan Any agar bisa di-downcast di Listener.
pub trait Event: Any + std::fmt::Debug + Send + Sync {
    fn as_any(&self) -> &dyn Any;
}

/// Trait untuk Listener yang menangani event.
#[async_trait]
pub trait Listener: Send + Sync {
    /// Menangani event yang diberikan.
    /// State aplikasi disediakan untuk akses database, dsb.
    async fn handle(&self, event: &dyn Event, state: Arc<AppState>) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
}
