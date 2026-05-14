use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::sync::RwLock;
use tracing::{error, warn};

pub struct LangManager {
    /// Cache untuk menyimpan terjemahan yang sudah dimuat.
    /// Struktur: locale -> group -> key -> value
    translations: RwLock<HashMap<String, HashMap<String, Value>>>,
    default_locale: String,
}

impl LangManager {
    pub fn new(default_locale: String) -> Self {
        Self {
            translations: RwLock::new(HashMap::new()),
            default_locale,
        }
    }

    /// Mendapatkan terjemahan berdasarkan key (format: "group.key").
    pub fn get(&self, locale: &str, key: &str, params: HashMap<String, String>) -> String {
        let parts: Vec<&str> = key.splitn(2, '.').collect();
        if parts.len() < 2 {
            return key.to_string();
        }

        let group = parts[0];
        let item_key = parts[1];

        // Pastikan locale dimuat
        self.ensure_loaded(locale, group);

        let translations = self.translations.read().unwrap();
        if let Some(group_data) = translations.get(locale).and_then(|g| g.get(group)) {
            if let Some(value) = self.resolve_key(group_data, item_key) {
                return self.replace_params(value, params);
            }
        }

        // Fallback ke default locale jika berbeda
        if locale != self.default_locale {
            return self.get(&self.default_locale, key, params);
        }

        key.to_string()
    }

    /// Memuat file JSON terjemahan jika belum ada di cache.
    fn ensure_loaded(&self, locale: &str, group: &str) {
        {
            let translations = self.translations.read().unwrap();
            if translations.contains_key(locale)
                && translations.get(locale).unwrap().contains_key(group)
            {
                return;
            }
        }

        let path = format!("lang/{}/{}.json", locale, group);
        if !Path::new(&path).exists() {
            return;
        }

        match fs::read_to_string(&path) {
            Ok(content) => match serde_json::from_str::<Value>(&content) {
                Ok(value) => {
                    let mut translations = self.translations.write().unwrap();
                    translations
                        .entry(locale.to_string())
                        .or_insert_with(HashMap::new)
                        .insert(group.to_string(), value);
                }
                Err(e) => error!("Gagal parse JSON terjemahan {}: {}", path, e),
            },
            Err(e) => warn!("Gagal membaca file terjemahan {}: {}", path, e),
        }
    }

    /// Mencari value dalam JSON object menggunakan dot notation (misal: "auth.login.failed").
    fn resolve_key(&self, data: &Value, key: &str) -> Option<String> {
        let mut current = data;
        for part in key.split('.') {
            current = current.get(part)?;
        }
        current.as_str().map(|s| s.to_string())
    }

    /// Mengganti placeholder :name dengan value dari params.
    fn replace_params(&self, text: String, params: HashMap<String, String>) -> String {
        let mut result = text;
        for (key, value) in params {
            result = result.replace(&format!(":{}", key), &value);
        }
        result
    }
}
