use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EntryType {
    Request,
    Query,
    Job,
    Event,
    Log,
    Exception,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelescopeEntry {
    pub id: Uuid,
    pub entry_type: EntryType,
    pub content: serde_json::Value,
    pub created_at: DateTime<Utc>,
}

impl TelescopeEntry {
    pub fn new(entry_type: EntryType, content: serde_json::Value) -> Self {
        Self {
            id: Uuid::new_v4(),
            entry_type,
            content,
            created_at: Utc::now(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestContent {
    pub method: String,
    pub uri: String,
    pub status: u16,
    pub duration_ms: u128,
    pub ip: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventContent {
    pub name: String,
    pub payload: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobContent {
    pub name: String,
    pub queue: String,
    pub status: String, // pending, running, completed, failed
    pub duration_ms: Option<u128>,
}
