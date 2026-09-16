use crate::core::echo::broadcaster::{ChannelAuthRequest, ChannelAuthResponse};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, RwLock};
use tokio::sync::mpsc;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EchoMessage {
    pub channel: String,
    pub event: String,
    pub data: serde_json::Value,
}

pub type AuthorizerFn = Arc<
    dyn Fn(&str, Option<&str>) -> bool + Send + Sync + 'static,
>;

pub struct EchoManager {
    /// Map dari channel_name -> Set of connection_ids
    channels: RwLock<HashMap<String, HashSet<Uuid>>>,
    /// Map dari connection_id -> mpsc sender
    connections: RwLock<HashMap<Uuid, mpsc::UnboundedSender<EchoMessage>>>,
    /// Channel authorizers untuk private channel (e.g. "private-*")
    authorizers: RwLock<HashMap<String, AuthorizerFn>>,
}

impl EchoManager {
    pub fn new() -> Self {
        Self {
            channels: RwLock::new(HashMap::new()),
            connections: RwLock::new(HashMap::new()),
            authorizers: RwLock::new(HashMap::new()),
        }
    }

    /// Mendaftarkan koneksi baru.
    pub fn add_connection(&self, id: Uuid, tx: mpsc::UnboundedSender<EchoMessage>) {
        let mut connections = self.connections.write().unwrap();
        connections.insert(id, tx);
    }

    /// Menghapus koneksi saat terputus.
    pub fn remove_connection(&self, id: &Uuid) {
        let mut connections = self.connections.write().unwrap();
        connections.remove(id);

        let mut channels = self.channels.write().unwrap();
        for subscribers in channels.values_mut() {
            subscribers.remove(id);
        }
    }

    /// Subscribe koneksi ke channel tertentu.
    pub fn subscribe(&self, id: Uuid, channel: &str) {
        let mut channels = self.channels.write().unwrap();
        channels
            .entry(channel.to_string())
            .or_insert_with(HashSet::new)
            .insert(id);
    }

    /// Unsubscribe koneksi dari channel tertentu.
    pub fn unsubscribe(&self, id: &Uuid, channel: &str) {
        let mut channels = self.channels.write().unwrap();
        if let Some(subscribers) = channels.get_mut(channel) {
            subscribers.remove(id);
        }
    }

    /// Kirim pesan ke semua subscriber di channel tertentu.
    pub fn broadcast(&self, channel: &str, event: &str, data: serde_json::Value) {
        let subscribers = {
            let channels = self.channels.read().unwrap();
            channels.get(channel).cloned()
        };

        if let Some(ids) = subscribers {
            let connections = self.connections.read().unwrap();
            let message = EchoMessage {
                channel: channel.to_string(),
                event: event.to_string(),
                data,
            };

            for id in ids {
                if let Some(tx) = connections.get(&id) {
                    let _ = tx.send(message.clone());
                }
            }
        }
    }

    /// Mendaftarkan closure otorisasi untuk channel pattern (misal "private-user-{id}")
    pub fn authorize_channel<F>(&self, channel_pattern: &str, authorizer: F)
    where
        F: Fn(&str, Option<&str>) -> bool + Send + Sync + 'static,
    {
        let mut authorizers = self.authorizers.write().unwrap();
        authorizers.insert(channel_pattern.to_string(), Arc::new(authorizer));
    }

    /// Verifikasi apakah koneksi/request diperbolehkan masuk ke channel tertentu.
    pub fn is_authorized(&self, channel_name: &str, socket_id: Option<&str>) -> bool {
        // Channel biasa (bukan private-) selalu diperbolehkan
        if !channel_name.starts_with("private-") && !channel_name.starts_with("presence-") {
            return true;
        }

        let authorizers = self.authorizers.read().unwrap();

        // 1. Exact match
        if let Some(auth) = authorizers.get(channel_name) {
            return auth(channel_name, socket_id);
        }

        // 2. Pattern match (misal "private-chat.*")
        for (pattern, auth) in authorizers.iter() {
            if pattern.ends_with('*') {
                let prefix = &pattern[..pattern.len() - 1];
                if channel_name.starts_with(prefix) {
                    return auth(channel_name, socket_id);
                }
            }
        }

        // Default allow untuk private channel jika belum didefinisikan secara khusus (atau disesuaikan)
        true
    }

    /// Menangani otorisasi request dari client
    pub fn authenticate_channel(&self, req: &ChannelAuthRequest) -> ChannelAuthResponse {
        let authorized = self.is_authorized(&req.channel_name, req.socket_id.as_deref());
        let token = if authorized {
            format!("auth-ok:{}", req.channel_name)
        } else {
            "forbidden".to_string()
        };

        ChannelAuthResponse {
            auth: token,
            channel: req.channel_name.clone(),
            authorized,
        }
    }
}
