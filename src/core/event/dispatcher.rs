use std::any::TypeId;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use crate::core::event::traits::{Event, Listener};
use crate::core::application::AppState;
use tracing::{info, error};

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
    pub async fn dispatch<E: Event + std::fmt::Debug + 'static>(&self, event: E, state: Arc<AppState>) {
        let event_name = std::any::type_name::<E>();
        info!("Dispatching event: {}", event_name);

        let event_arc = Arc::new(event);
        let type_id = TypeId::of::<E>();

        // Catat ke Telescope
        state.telescope.record_event(event_name, &format!("{:?}", event_arc));

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
}
