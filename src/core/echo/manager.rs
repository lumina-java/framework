use std::collections::{HashMap, HashSet};
use std::sync::RwLock;
use tokio::sync::mpsc;
use uuid::Uuid;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EchoMessage {
    pub channel: String,
    pub event: String,
    pub data: serde_json::Value,
}

pub struct EchoManager {
    /// Map dari channel_name -> Set of connection_ids
    channels: RwLock<HashMap<String, HashSet<Uuid>>>,
    /// Map dari connection_id -> mpsc sender
    connections: RwLock<HashMap<Uuid, mpsc::UnboundedSender<EchoMessage>>>,
}

impl EchoManager {
    pub fn new() -> Self {
        Self {
            channels: RwLock::new(HashMap::new()),
            connections: RwLock::new(HashMap::new()),
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
}
