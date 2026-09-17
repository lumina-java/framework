use serde::{de::DeserializeOwned, Serialize};

/// Converts any serializable data into a JSON string, returning a default empty JSON string on error.
/// Human-friendly utility similar to `json_encode` in PHP / JavaScript `JSON.stringify`.
pub fn json_encode<T: Serialize>(data: &T) -> String {
    serde_json::to_string(data).unwrap_or_else(|_| "{}".to_string())
}

/// Converts a JSON string into a deserialized Rust struct/type `T`.
/// Human-friendly utility similar to `json_decode` in PHP / JavaScript `JSON.parse`.
pub fn json_decode<T: DeserializeOwned>(json_str: &str) -> Result<T, serde_json::Error> {
    serde_json::from_str(json_str)
}
