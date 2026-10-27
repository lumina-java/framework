use async_trait::async_trait;
use crate::core::error::AppError;
use crate::core::live::LiveComponent;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Komponen Reaktif Lumina Live: Counter
/// Mendukung multi-dialek: `kilat:klik`, `live:click`, `wire:click`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Counter {
    pub count: i32,
    pub step: i32,
}

impl Default for Counter {
    fn default() -> Self {
        Self { count: 0, step: 1 }
    }
}

#[async_trait]
impl LiveComponent for Counter {
    fn name(&self) -> &'static str {
        "counter"
    }

    fn view(&self) -> &'static str {
        "live.counter"
    }

    async fn call_action(&mut self, action: &str, _params: Vec<Value>) -> Result<(), AppError> {
        match action {
            "tambah" | "increment" => self.count += self.step,
            "kurang" | "decrement" => {
                if self.count > 0 {
                    self.count -= self.step;
                }
            }
            "reset" => self.count = 0,
            _ => return Err(AppError::BadRequest(format!("Aksi tidak dikenal: {}", action))),
        }
        Ok(())
    }
}
