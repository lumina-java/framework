use crate::core::error::AppError;

/// Anthropic Claude API Fluent Client (Claude 3.5 / 3.7 Sonnet)
#[derive(Debug, Clone)]
pub struct ClaudeClient {
    model: String,
    api_key: Option<String>,
    system_instruction: Option<String>,
    temperature: Option<f32>,
    max_tokens: u32,
    prompt_text: String,
}

impl Default for ClaudeClient {
    fn default() -> Self {
        Self::new()
    }
}

impl ClaudeClient {
    pub fn new() -> Self {
        let key = std::env::var("ANTHROPIC_API_KEY").ok();
        Self {
            model: "claude-3-5-sonnet-20241022".to_string(),
            api_key: key,
            system_instruction: None,
            temperature: None,
            max_tokens: 4096,
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

    pub fn max_tokens(mut self, tokens: u32) -> Self {
        self.max_tokens = tokens;
        self
    }

    pub fn complete(mut self, text: impl Into<String>) -> Self {
        self.prompt_text = text.into();
        self
    }

    pub fn prompt(mut self, text: impl Into<String>) -> Self {
        self.prompt_text = text.into();
        self
    }

    /// Execute completion against Anthropic Claude Messages API
    pub async fn generate(&self) -> Result<String, AppError> {
        let key = self.api_key.as_deref().ok_or_else(|| {
            AppError::Generic("Missing ANTHROPIC_API_KEY environment variable".to_string())
        })?;

        if self.prompt_text.trim().is_empty() {
            return Err(AppError::BadRequest("Prompt text cannot be empty".to_string()));
        }

        let mut body = serde_json::json!({
            "model": self.model,
            "max_tokens": self.max_tokens,
            "messages": [
                {
                    "role": "user",
                    "content": self.prompt_text
                }
            ]
        });

        if let Some(sys) = &self.system_instruction {
            body["system"] = serde_json::json!(sys);
        }

        if let Some(t) = self.temperature {
            body["temperature"] = serde_json::json!(t);
        }

        let client = reqwest::Client::new();
        let res = client
            .post("https://api.anthropic.com/v1/messages")
            .header("x-api-key", key)
            .header("anthropic-version", "2023-06-01")
            .header("content-type", "application/json")
            .json(&body)
            .send()
            .await
            .map_err(|e| AppError::Generic(format!("Claude HTTP request failed: {}", e)))?;

        let status = res.status();
        let json_val: serde_json::Value = res
            .json()
            .await
            .map_err(|e| AppError::Generic(format!("Failed to parse Claude response: {}", e)))?;

        if !status.is_success() {
            let err_msg = json_val["error"]["message"].as_str().unwrap_or("Unknown Claude error");
            return Err(AppError::Generic(format!("Claude API Error ({}): {}", status, err_msg)));
        }

        let reply_text = json_val["content"][0]["text"]
            .as_str()
            .ok_or_else(|| AppError::Generic("Claude response did not contain content text".to_string()))?;

        Ok(reply_text.to_string())
    }
}
