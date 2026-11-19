use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};

/// Statistik eksekusi individu per plugin WASM
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginStats {
    pub name: String,
    pub calls_count: u64,
    pub errors_count: u64,
    pub total_duration_us: u64,
    pub avg_duration_us: f64,
    pub last_executed_at: Option<String>,
    pub last_error: Option<String>,
}

impl PluginStats {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            calls_count: 0,
            errors_count: 0,
            total_duration_us: 0,
            avg_duration_us: 0.0,
            last_executed_at: None,
            last_error: None,
        }
    }

    /// Catat eksekusi sukses dengan durasi
    pub fn record_success(&mut self, duration: Duration) {
        self.calls_count += 1;
        let us = duration.as_micros() as u64;
        self.total_duration_us += us;
        self.avg_duration_us = (self.total_duration_us as f64) / (self.calls_count as f64);
        self.last_executed_at = Some(chrono::Utc::now().to_rfc3339());
    }

    /// Catat eksekusi gagal / error / trap
    pub fn record_error(&mut self, err: &str) {
        self.calls_count += 1;
        self.errors_count += 1;
        self.last_error = Some(err.to_string());
        self.last_executed_at = Some(chrono::Utc::now().to_rfc3339());
    }
}

/// Tracker performa & isolasi kesehatan plugin WASM (Thread-Safe)
#[derive(Debug, Clone, Default)]
pub struct PluginMetricsTracker {
    stats: Arc<RwLock<HashMap<String, PluginStats>>>,
}

impl PluginMetricsTracker {
    pub fn new() -> Self {
        Self {
            stats: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Ukur eksekusi sebuah fungsi closure pada plugin
    pub fn measure<F, R, E>(&self, plugin_name: &str, f: F) -> Result<R, E>
    where
        F: FnOnce() -> Result<R, E>,
        E: std::fmt::Display,
    {
        let start = Instant::now();
        let res = f();
        let elapsed = start.elapsed();

        let mut map = self.stats.write().unwrap();
        let entry = map.entry(plugin_name.to_string()).or_insert_with(|| PluginStats::new(plugin_name));

        match &res {
            Ok(_) => entry.record_success(elapsed),
            Err(e) => entry.record_error(&e.to_string()),
        }

        res
    }

    /// Ambil seluruh statistik plugin yang tersimpan
    pub fn all_stats(&self) -> Vec<PluginStats> {
        let map = self.stats.read().unwrap();
        map.values().cloned().collect()
    }

    /// Ambil statistik satu plugin tertentu
    pub fn get_stats(&self, plugin_name: &str) -> Option<PluginStats> {
        let map = self.stats.read().unwrap();
        map.get(plugin_name).cloned()
    }
}
