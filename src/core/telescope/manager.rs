use crate::core::telescope::entry::{EntryType, TelescopeEntry};
use serde_json::Value;
use std::collections::VecDeque;
use std::sync::RwLock;

/// Kapasitas maksimum buffer in-memory untuk entri telescope.
const MAX_ENTRIES: usize = 100;

pub struct TelescopeManager {
    entries: RwLock<VecDeque<TelescopeEntry>>,
}

impl TelescopeManager {
    pub fn new() -> Self {
        Self {
            entries: RwLock::new(VecDeque::with_capacity(MAX_ENTRIES)),
        }
    }

    /// Mencatat entri baru ke dalam buffer.
    pub fn record(&self, entry_type: EntryType, content: Value) {
        let mut entries = self.entries.write().unwrap();

        if entries.len() >= MAX_ENTRIES {
            entries.pop_back(); // Hapus entri tertua
        }

        entries.push_front(TelescopeEntry::new(entry_type, content));
    }

    /// Mengambil semua entri.
    pub fn all(&self) -> Vec<TelescopeEntry> {
        let entries = self.entries.read().unwrap();
        entries.iter().cloned().collect()
    }

    /// Mengambil entri berdasarkan tipe.
    pub fn get_by_type(&self, entry_type: EntryType) -> Vec<TelescopeEntry> {
        let entries = self.entries.read().unwrap();
        entries
            .iter()
            .filter(|e| {
                std::mem::discriminant(&e.entry_type) == std::mem::discriminant(&entry_type)
            })
            .cloned()
            .collect()
    }

    /// Mencatat HTTP Request.
    pub fn record_request(&self, content: crate::core::telescope::entry::RequestContent) {
        self.record(
            EntryType::Request,
            serde_json::to_value(content).unwrap_or(Value::Null),
        );
    }

    /// Mencatat Event.
    pub fn record_event(&self, name: &str, payload: &str) {
        let content = crate::core::telescope::entry::EventContent {
            name: name.to_string(),
            payload: payload.to_string(),
        };
        self.record(
            EntryType::Event,
            serde_json::to_value(content).unwrap_or(Value::Null),
        );
    }
}
