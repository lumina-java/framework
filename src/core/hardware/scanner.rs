use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BarcodeScanEvent {
    pub barcode: String,
    pub format: Option<String>,
    pub timestamp_ms: u64,
}

impl BarcodeScanEvent {
    pub fn new(barcode: impl Into<String>) -> Self {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64;

        Self {
            barcode: barcode.into(),
            format: None,
            timestamp_ms: now,
        }
    }

    pub fn with_format(mut self, format: impl Into<String>) -> Self {
        self.format = Some(format.into());
        self
    }

    /// Validate EAN-13 check digit
    pub fn is_valid_ean13(&self) -> bool {
        let digits: Vec<u32> = self.barcode.chars().filter_map(|c| c.to_digit(10)).collect();
        if digits.len() != 13 {
            return false;
        }
        let sum: u32 = digits[..12]
            .iter()
            .enumerate()
            .map(|(i, &d)| if i % 2 == 0 { d } else { d * 3 })
            .sum();
        let check = (10 - (sum % 10)) % 10;
        check == digits[12]
    }
}
