use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};
use serde_json::Value;
use lumina::core::error::AppError;
use lumina::core::request::Request;
use lumina::core::blade::{BladeEngine, BladeContext};

/// Trait utama untuk setiap komponen reaktif Lumina Live.
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
    fn render_html(&self, engine: &BladeEngine) -> Result<String, AppError> {
        let val = serde_json::to_value(self)
            .map_err(|e| AppError::Generic(format!("Failed to serialize live component: {}", e)))?;

        let mut context = BladeContext::new();
        if let Some(map) = val.as_object() {
            for (k, v) in map {
                context.insert_val(k, v.clone());
            }
        }

        Ok(engine.render(self.view(), &context))
    }
}

/// Helper untuk mem-parsing argumen komponen Live: mendukung format JSON (`{"step": 5}`)
/// maupun format array PHP / Laravel Blade (`['step' => 5, 'count' => 10]` atau `[step: 5]`).
pub fn parse_blade_args_to_json(raw: &str) -> Value {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return serde_json::json!({});
    }

    if let Ok(v) = serde_json::from_str(trimmed) {
        return v;
    }

    if (trimmed.starts_with('[') && trimmed.ends_with(']'))
        || (trimmed.starts_with('{') && trimmed.ends_with('}'))
    {
        let inner = trimmed[1..trimmed.len() - 1].trim();
        if inner.is_empty() {
            return serde_json::json!({});
        }

        let mut map = serde_json::Map::new();
        let mut pairs = Vec::new();
        let mut current = String::new();
        let mut in_single_quote = false;
        let mut in_double_quote = false;

        for ch in inner.chars() {
            match ch {
                '\'' if !in_double_quote => {
                    in_single_quote = !in_single_quote;
                    current.push(ch);
                }
                '"' if !in_single_quote => {
                    in_double_quote = !in_double_quote;
                    current.push(ch);
                }
                ',' if !in_single_quote && !in_double_quote => {
                    if !current.trim().is_empty() {
                        pairs.push(current.trim().to_string());
                    }
                    current.clear();
                }
                _ => current.push(ch),
            }
        }
        if !current.trim().is_empty() {
            pairs.push(current.trim().to_string());
        }

        for pair in pairs {
            let (key_str, val_str) = if let Some((k, v)) = pair.split_once("=>") {
                (k.trim(), v.trim())
            } else if let Some((k, v)) = pair.split_once(':') {
                (k.trim(), v.trim())
            } else {
                continue;
            };

            let clean_key = key_str.trim_matches(|c| c == '\'' || c == '"' || c == ' ').to_string();

            let clean_val = if val_str == "true" {
                Value::Bool(true)
            } else if val_str == "false" {
                Value::Bool(false)
            } else if val_str == "null" {
                Value::Null
            } else if let Ok(i) = val_str.parse::<i64>() {
                Value::Number(i.into())
            } else if let Ok(f) = val_str.parse::<f64>() {
                if let Some(num) = serde_json::Number::from_f64(f) {
                    Value::Number(num)
                } else {
                    Value::String(val_str.to_string())
                }
            } else {
                let s = val_str.trim_matches(|c| c == '\'' || c == '"').to_string();
                Value::String(s)
            };

            map.insert(clean_key, clean_val);
        }

        return Value::Object(map);
    }

    serde_json::json!({})
}
