use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", content = "payload")]
pub enum DeviceMessage {
    /// Send raw bytes to client printer
    PrintRaw {
        device: String,
        data_base64: String,
    },
    /// Request client to initiate barcode scanning
    ScanBarcode {
        timeout_ms: u64,
    },
    /// Request client to capture fingerprint
    CaptureFingerprint {
        min_quality: u8,
    },
    /// Request camera frame / QR decode
    CaptureCameraFrame {
        config: super::camera::CameraConfig,
    },
    /// Ping / Heartbeat
    Ping,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "status", content = "result")]
pub enum DeviceResponse {
    Success {
        device: String,
        message: String,
        data: Option<serde_json::Value>,
    },
    Error {
        device: String,
        code: String,
        message: String,
    },
    Pong,
}
