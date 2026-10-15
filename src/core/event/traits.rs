<<<<<<< HEAD
use crate::core::application::AppState;
use crate::core::echo::broadcaster::ShouldBroadcast;
use async_trait::async_trait;
use std::any::Any;
use std::sync::Arc;
=======
use async_trait::async_trait;
use std::any::Any;
use std::sync::Arc;
use crate::core::application::AppState;
>>>>>>> dev/master

/// Trait untuk data Event.
/// Harus mengimplementasikan Any agar bisa di-downcast di Listener.
pub trait Event: Any + std::fmt::Debug + Send + Sync {
    fn as_any(&self) -> &dyn Any;
<<<<<<< HEAD

    /// Mengembalikan referensi ke `ShouldBroadcast` jika event ini dapat di-broadcast.
    fn broadcaster(&self) -> Option<&dyn ShouldBroadcast> {
        None
    }
=======
>>>>>>> dev/master
}

/// Trait untuk Listener yang menangani event.
#[async_trait]
pub trait Listener: Send + Sync {
    /// Menangani event yang diberikan.
    /// State aplikasi disediakan untuk akses database, dsb.
<<<<<<< HEAD
    async fn handle(
        &self,
        event: &dyn Event,
        state: Arc<AppState>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
=======
    async fn handle(&self, event: &dyn Event, state: Arc<AppState>) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
>>>>>>> dev/master
}
