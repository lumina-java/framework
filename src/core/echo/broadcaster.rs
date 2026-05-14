use serde_json::Value;

pub trait ShouldBroadcast {
    /// Nama channel tempat event ini akan dikirim.
    fn broadcast_on(&self) -> Vec<String>;

    /// Nama event yang akan diterima oleh client.
    /// Defaultnya adalah nama struct.
    fn broadcast_as(&self) -> String {
        std::any::type_name::<Self>()
            .split("::")
            .last()
            .unwrap_or("UnknownEvent")
            .to_string()
    }

    /// Data yang akan dikirim ke client.
    fn broadcast_with(&self) -> Value {
        serde_json::json!({})
    }
}
