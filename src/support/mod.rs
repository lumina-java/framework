pub mod debug;
pub mod str;

// Helper functions dan utilities
pub fn env(key: &str, default: &str) -> String {
    std::env::var(key).unwrap_or_else(|_| default.to_string())
}
