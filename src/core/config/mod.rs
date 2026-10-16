use std::collections::HashMap;
use std::sync::Arc;

/// ConfigManager menyimpan semua konfigurasi aplikasi yang dimuat dari .env atau file lain.
pub struct ConfigManager {
    settings: HashMap<String, String>,
}

impl ConfigManager {
    /// Inisialisasi ConfigManager dengan memuat data dari environment variables.
    pub fn new() -> Self {
        let mut settings = HashMap::new();

        // Muat semua environment variables ke dalam HashMap
        for (key, value) in std::env::vars() {
            settings.insert(key, value);
        }

        Self { settings }
    }

    /// Ambil nilai konfigurasi berdasarkan key.
    /// Contoh: config.get("APP_NAME")
    pub fn get(&self, key: &str) -> Option<String> {
        self.settings.get(key).cloned()
    }

    /// Ambil nilai konfigurasi dengan fallback default.
    pub fn get_or(&self, key: &str, default: &str) -> String {
        self.settings
            .get(key)
            .cloned()
            .unwrap_or_else(|| default.to_string())
    }

    /// Ambil nilai konfigurasi sebagai integer.
    pub fn get_int(&self, key: &str, default: i32) -> i32 {
        self.get(key)
            .and_then(|v| v.parse::<i32>().ok())
            .unwrap_or(default)
    }

    /// Ambil nilai konfigurasi sebagai boolean.
    pub fn get_bool(&self, key: &str, default: bool) -> bool {
        match self.get(key).as_deref() {
            Some("true") | Some("1") | Some("yes") => true,
            Some("false") | Some("0") | Some("no") => false,
            _ => default,
        }
    }
    /// Membangun DATABASE_URL dari variabel DB_* ala Laravel atau mengambil langsung dari DATABASE_URL.
    pub fn get_db_url(&self) -> String {
        // Jika DATABASE_URL sudah ada di .env secara eksplisit, gunakan itu (prioritas).
        if let Some(url) = self.get("DATABASE_URL") {
            return url;
        }

        let connection = self.get_or("DB_CONNECTION", "sqlite");

        match connection.as_str() {
            "sqlite" => {
                let db = self.get_or("DB_DATABASE", "./lumina.db");
                format!("sqlite:{}", db)
            }
            "mysql" | "postgres" | "postgresql" => {
                let host = self.get_or("DB_HOST", "127.0.0.1");
                let port = self.get_or(
                    "DB_PORT",
                    if connection == "mysql" {
                        "3306"
                    } else {
                        "5432"
                    },
                );
                let user = self.get_or("DB_USERNAME", "root");
                let pass = self.get_or("DB_PASSWORD", "");
                let db = self.get_or("DB_DATABASE", "lumina");

                if connection == "mysql" {
                    format!("mysql://{}:{}@{}:{}/{}", user, pass, host, port, db)
                } else {
                    format!("postgres://{}:{}@{}:{}/{}", user, pass, host, port, db)
                }
            }
            _ => self.get_or("DATABASE_URL", "sqlite:./lumina.db"),
        }
    }
}

/// Helper untuk memudahkan akses config jika menggunakan Arc.
pub type Config = Arc<ConfigManager>;
