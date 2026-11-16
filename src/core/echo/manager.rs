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
    #[cfg(feature = "redis")]
    redis_client: Option<redis::Client>,
}

impl EchoManager {
    pub fn new() -> Self {
        Self {
            channels: RwLock::new(HashMap::new()),
            connections: RwLock::new(HashMap::new()),
            authorizers: RwLock::new(HashMap::new()),
            #[cfg(feature = "redis")]
            redis_client: None,
        }
    }

    #[cfg(feature = "redis")]
    pub fn with_redis(redis_url: &str) -> Result<Self, String> {
        let client = redis::Client::open(redis_url)
            .map_err(|e| format!("Gagal inisialisasi Redis client untuk Echo: {}", e))?;
        Ok(Self {
            channels: RwLock::new(HashMap::new()),
            connections: RwLock::new(HashMap::new()),
            authorizers: RwLock::new(HashMap::new()),
            redis_client: Some(client),
        })
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

    /// Kirim pesan hanya ke subscriber lokal pada instance server ini.
    pub fn broadcast_local(&self, channel: &str, event: &str, data: serde_json::Value) {
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

    /// Kirim pesan ke semua subscriber di channel tertentu (lokal dan klaster Redis multi-pod).
    pub fn broadcast(&self, channel: &str, event: &str, data: serde_json::Value) {
        self.broadcast_local(channel, event, data.clone());

        #[cfg(feature = "redis")]
        if let Some(ref client) = self.redis_client {
            let client = client.clone();
            let msg = EchoMessage {
                channel: channel.to_string(),
                event: event.to_string(),
                data,
            };
            tokio::spawn(async move {
                if let Ok(mut conn) = client.get_multiplexed_async_connection().await {
                    if let Ok(serialized) = serde_json::to_string(&msg) {
                        let _: Result<(), redis::RedisError> = redis::cmd("PUBLISH")
                            .arg("lumina_echo_events")
                            .arg(serialized)
                            .query_async(&mut conn)
                            .await;
                    }
                }
            });
        }
    }

    /// Menjalankan listener Redis Pub/Sub untuk menerima event dari pod/server lain dalam cluster.
    #[cfg(feature = "redis")]
    pub fn start_redis_subscriber(self: &Arc<Self>, redis_url: &str) {
        use futures::StreamExt;
        let echo = self.clone();
        let url = redis_url.to_string();

        tokio::spawn(async move {
            match redis::Client::open(url) {
                Ok(client) => match client.get_async_pubsub().await {
                    Ok(mut pubsub) => {
                        if let Err(e) = pubsub.subscribe("lumina_echo_events").await {
                            eprintln!("⚠️  [Lumina Echo Redis] Gagal subscribe: {}", e);
                            return;
                        }
                        println!("📡 [Lumina Echo] Redis Pub/Sub cluster listener aktif.");

                        let mut msg_stream = pubsub.on_message();
                        while let Some(msg) = msg_stream.next().await {
                            if let Ok(payload_str) = msg.get_payload::<String>() {
                                if let Ok(echo_msg) = serde_json::from_str::<EchoMessage>(&payload_str) {
                                    echo.broadcast_local(&echo_msg.channel, &echo_msg.event, echo_msg.data);
                                }
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("⚠️  [Lumina Echo Redis] Gagal inisialisasi pubsub: {}", e);
                    }
                },
                Err(e) => {
                    eprintln!("⚠️  [Lumina Echo Redis] URL Redis tidak valid: {}", e);
                }
            }
        });
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
        let auth_param = req.token.as_deref().or(req.socket_id.as_deref());
        let authorized = self.is_authorized(&req.channel_name, auth_param);
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
