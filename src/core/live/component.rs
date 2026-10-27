use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};
use serde_json::Value;
use crate::core::error::AppError;
use crate::core::request::Request;

/// Trait utama untuk setiap komponen reaktif Lumina Live.
/// Komponen harus dapat diserialisasi (JSON), thread-safe (Send + Sync), dan memiliki state yang terisolasi.
#[async_trait]
pub trait LiveComponent: Serialize + DeserializeOwned + Default + Send + Sync + 'static {
    /// Nama unik komponen (misal: "counter", "search_user", "cart_badge").
    fn name(&self) -> &'static str;

    /// Path template Blade yang digunakan untuk merender komponen (misal: "live.counter").
    fn view(&self) -> &'static str;

    /// Lifecycle hook saat komponen pertama kali dimuat di server.
    async fn mount(&mut self, _req: &Request) -> Result<(), AppError> {
        Ok(())
    }

    /// Pemanggil aksi dinamis saat tombol atau event dipicu dari browser (e.g. `kilat:klik="tambah"`).
    async fn call_action(&mut self, action: &str, params: Vec<Value>) -> Result<(), AppError>;

    /// Render state komponen saat ini ke dalam format HTML menggunakan Blade Engine.
    fn render_html(&self, engine: &crate::core::blade::BladeEngine) -> Result<String, AppError> {
        let val = serde_json::to_value(self)
            .map_err(|e| AppError::Generic(format!("Failed to serialize live component: {}", e)))?;

        let mut context = crate::core::blade::BladeContext::new();
        if let Some(map) = val.as_object() {
            for (k, v) in map {
                context.insert_val(k, v.clone());
            }
        }

        Ok(engine.render(self.view(), &context))
    }
}
