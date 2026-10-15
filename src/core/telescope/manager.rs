<<<<<<< HEAD
use crate::core::telescope::entry::{EntryType, TelescopeEntry};
use serde_json::Value;
use std::collections::VecDeque;
use std::sync::RwLock;
=======
use std::sync::RwLock;
use std::collections::VecDeque;
use crate::core::telescope::entry::{TelescopeEntry, EntryType};
use serde_json::Value;
>>>>>>> dev/master

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
<<<<<<< HEAD

        if entries.len() >= MAX_ENTRIES {
            entries.pop_back(); // Hapus entri tertua
        }

=======

        if entries.len() >= MAX_ENTRIES {
            entries.pop_back(); // Hapus entri tertua
        }

>>>>>>> dev/master
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
<<<<<<< HEAD
        entries
            .iter()
            .filter(|e| {
                std::mem::discriminant(&e.entry_type) == std::mem::discriminant(&entry_type)
            })
=======
        entries.iter()
            .filter(|e| std::mem::discriminant(&e.entry_type) == std::mem::discriminant(&entry_type))
>>>>>>> dev/master
            .cloned()
            .collect()
    }

    /// Mencatat HTTP Request.
    pub fn record_request(&self, content: crate::core::telescope::entry::RequestContent) {
<<<<<<< HEAD
        self.record(
            EntryType::Request,
            serde_json::to_value(content).unwrap_or(Value::Null),
        );
=======
        self.record(EntryType::Request, serde_json::to_value(content).unwrap_or(Value::Null));
>>>>>>> dev/master
    }

    /// Mencatat Event.
    pub fn record_event(&self, name: &str, payload: &str) {
        let content = crate::core::telescope::entry::EventContent {
            name: name.to_string(),
            payload: payload.to_string(),
        };
<<<<<<< HEAD
        self.record(
            EntryType::Event,
            serde_json::to_value(content).unwrap_or(Value::Null),
        );
=======
        self.record(EntryType::Event, serde_json::to_value(content).unwrap_or(Value::Null));
>>>>>>> dev/master
    }
}
