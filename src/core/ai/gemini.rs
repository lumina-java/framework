use crate::core::error::AppError;

/// Google Gemini API Fluent Client
#[derive(Debug, Clone)]
pub struct GeminiClient {
    model: String,
    api_key: Option<String>,
    system_instruction: Option<String>,
    temperature: Option<f32>,
    prompt_text: String,
}

impl Default for GeminiClient {
    fn default() -> Self {
        Self::new()
    }
}

impl GeminiClient {
    pub fn new() -> Self {
        let key = std::env::var("GEMINI_API_KEY").ok();
        Self {
            model: "gemini-1.5-flash".to_string(),
            api_key: key,
            system_instruction: None,
            temperature: None,
            prompt_text: String::new(),
        }
    }

    pub fn model(mut self, model: impl Into<String>) -> Self {
        self.model = model.into();
        self
    }

    pub fn with_api_key(mut self, key: impl Into<String>) -> Self {
        self.api_key = Some(key.into());
        self
    }

    pub fn with_system(mut self, instruction: impl Into<String>) -> Self {
        self.system_instruction = Some(instruction.into());
        self
    }

    pub fn temperature(mut self, temp: f32) -> Self {
        self.temperature = Some(temp);
        self
    }

    pub fn prompt(mut self, text: impl Into<String>) -> Self {
        self.prompt_text = text.into();
        self
    }

    /// Execute prompt generation against Gemini API
    pub async fn generate(&self) -> Result<String, AppError> {
        let key = self.api_key.as_deref().ok_or_else(|| {
            AppError::Generic("Missing GEMINI_API_KEY environment variable".to_string())
        })?;

        if self.prompt_text.trim().is_empty() {
            return Err(AppError::BadRequest("Prompt text cannot be empty".to_string()));
        }

        let url = format!(
            "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent?key={}",
            self.model, key
        );

        let mut body = serde_json::json!({
            "contents": [
                {
                    "parts": [
                        { "text": self.prompt_text }
                    ]
                }
            ]
        });

        if let Some(sys) = &self.system_instruction {
            body["systemInstruction"] = serde_json::json!({
                "parts": [
                    { "text": sys }
                ]
            });
        }

        if let Some(t) = self.temperature {
            body["generationConfig"] = serde_json::json!({
                "temperature": t
            });
        }

        let client = reqwest::Client::new();
        let res = client
            .post(&url)
            .json(&body)
            .send()
            .await
            .map_err(|e| AppError::Generic(format!("Gemini HTTP request failed: {}", e)))?;

        let status = res.status();
        let json_val: serde_json::Value = res
            .json()
            .await
            .map_err(|e| AppError::Generic(format!("Failed to parse Gemini response: {}", e)))?;

        if !status.is_success() {
            let err_msg = json_val["error"]["message"].as_str().unwrap_or("Unknown Gemini error");
            return Err(AppError::Generic(format!("Gemini API Error ({}): {}", status, err_msg)));
        }

        let candidate_text = json_val["candidates"][0]["content"]["parts"][0]["text"]
            .as_str()
            .ok_or_else(|| AppError::Generic("Gemini response did not contain candidates text".to_string()))?;

        Ok(candidate_text.to_string())
    }
}
