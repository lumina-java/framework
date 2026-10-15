pub mod debug;
<<<<<<< HEAD
pub mod json;
pub mod str;

pub use json::{json_decode, json_encode};

=======
pub mod str;

>>>>>>> dev/master
// Helper functions dan utilities
pub fn env(key: &str, default: &str) -> String {
    std::env::var(key).unwrap_or_else(|_| default.to_string())
}
