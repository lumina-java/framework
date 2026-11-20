use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FingerprintTemplate {
    pub sensor_model: String,
    pub quality_score: u8, // 0 - 100
    pub template_base64: String,
}

impl FingerprintTemplate {
    pub fn new(sensor_model: impl Into<String>, template_base64: impl Into<String>, quality_score: u8) -> Self {
        Self {
            sensor_model: sensor_model.into(),
            template_base64: template_base64.into(),
            quality_score,
        }
    }

    pub fn is_usable(&self) -> bool {
        self.quality_score >= 50 && !self.template_base64.is_empty()
    }
}
