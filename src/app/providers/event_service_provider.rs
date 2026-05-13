use std::sync::Arc;
use crate::core::event::EventDispatcher;
use crate::app::events::user_registered::UserRegistered;
use crate::app::listeners::log_user_registered::LogUserRegistered;

/// Daftarkan semua event dan listener aplikasi di sini.
pub fn register_events(events: &EventDispatcher) {
    events.listen::<UserRegistered>(Arc::new(LogUserRegistered));
}
