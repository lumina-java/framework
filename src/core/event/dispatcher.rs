use crate::core::application::AppState;
use crate::core::echo::broadcaster::ShouldBroadcast;
use crate::core::event::traits::{Event, Listener};
use std::any::TypeId;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use tracing::{error, info};

/// Pengatur event yang mengelola pendaftaran listener dan pengiriman event.
pub struct EventDispatcher {
    listeners: RwLock<HashMap<TypeId, Vec<Arc<dyn Listener>>>>,
}

impl EventDispatcher {
    /// Membuat dispatcher baru.
    pub fn new() -> Self {
        Self {
            listeners: RwLock::new(HashMap::new()),
        }
    }

    /// Mendaftarkan listener untuk tipe event tertentu.
    pub fn listen<E: Event + 'static>(&self, listener: Arc<dyn Listener>) {
        let mut listeners = self.listeners.write().unwrap();
        listeners
            .entry(TypeId::of::<E>())
            .or_insert_with(Vec::new)
            .push(listener);
    }

    /// Mengirimkan event ke semua listener yang terdaftar secara asinkron.
    /// Jika event mengimplementasikan `ShouldBroadcast` (via `event.broadcaster()`),
    /// otomatis mem-broadcast via WebSocket/Echo.
    pub async fn dispatch<E: Event + std::fmt::Debug + 'static>(
        &self,
        event: E,
        state: Arc<AppState>,
    ) {
        let event_name = std::any::type_name::<E>();
        info!("Dispatching event: {}", event_name);

        let event_arc = Arc::new(event);
        let type_id = TypeId::of::<E>();

        // Catat ke Telescope
        state
            .telescope
            .record_event(event_name, &format!("{:?}", event_arc));

        // Cek apakah event ini dipromosikan sebagai ShouldBroadcast
        if let Some(broadcaster) = event_arc.broadcaster() {
            let event_name = broadcaster.broadcast_as();
            let data = broadcaster.broadcast_with();
            for channel in broadcaster.broadcast_on() {
                state.echo.broadcast(&channel, &event_name, data.clone());
            }
        }

        let listeners_to_run = {
            let listeners = self.listeners.read().unwrap();
            listeners.get(&type_id).cloned()
        };

        if let Some(handlers) = listeners_to_run {
            for listener in handlers {
                let state_clone = state.clone();
                let event_clone = event_arc.clone();

                // Jalankan di background agar tidak memblokir request utama
                tokio::spawn(async move {
                    if let Err(e) = listener.handle(&*event_clone, state_clone).await {
                        error!("Error in listener for {}: {}", event_name, e);
                    }
                });
            }
        }
    }

    /// Helper untuk langsung mem-broadcast event secara manual jika mengimplementasikan ShouldBroadcast.
    pub fn dispatch_broadcast<B: ShouldBroadcast>(&self, broadcaster: &B, state: &AppState) {
        let event_name = broadcaster.broadcast_as();
        let data = broadcaster.broadcast_with();
        for channel in broadcaster.broadcast_on() {
            state.echo.broadcast(&channel, &event_name, data.clone());
        }
    }
}
