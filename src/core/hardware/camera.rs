use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CameraConfig {
    pub device_id: Option<String>,
    pub facing_mode: String, // "user" (front) or "environment" (rear)
    pub resolution_width: u32,
    pub resolution_height: u32,
    pub enable_torch: bool,
}

impl Default for CameraConfig {
    fn default() -> Self {
        Self {
            device_id: None,
            facing_mode: "environment".to_string(),
            resolution_width: 1280,
            resolution_height: 720,
            enable_torch: false,
        }
    }
}
