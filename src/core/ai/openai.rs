use crate::core::error::AppError;

/// OpenAI API Fluent Client (GPT-4o, GPT-4o-mini, o1)
#[derive(Debug, Clone)]
pub struct OpenAiClient {
    model: String,
    api_key: Option<String>,
    system_instruction: Option<String>,
    temperature: Option<f32>,
    prompt_text: String,
}

impl Default for OpenAiClient {
    fn default() -> Self {
        Self::new()
    }
}

impl OpenAiClient {
    pub fn new() -> Self {
        let key = std::env::var("OPENAI_API_KEY").ok();
        Self {
            model: "gpt-4o-mini".to_string(),
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

    pub fn chat(mut self, text: impl Into<String>) -> Self {
        self.prompt_text = text.into();
        self
    }

    pub fn prompt(mut self, text: impl Into<String>) -> Self {
        self.prompt_text = text.into();
        self
    }

    /// Execute chat completion against OpenAI API
    pub async fn generate(&self) -> Result<String, AppError> {
        let key = self.api_key.as_deref().ok_or_else(|| {
            AppError::Generic("Missing OPENAI_API_KEY environment variable".to_string())
        })?;

        if self.prompt_text.trim().is_empty() {
            return Err(AppError::BadRequest("Prompt text cannot be empty".to_string()));
        }

        let mut messages = Vec::new();
        if let Some(sys) = &self.system_instruction {
            messages.push(serde_json::json!({
                "role": "system",
                "content": sys
            }));
        }
        messages.push(serde_json::json!({
            "role": "user",
            "content": self.prompt_text
        }));

        let mut body = serde_json::json!({
            "model": self.model,
            "messages": messages
        });

        if let Some(t) = self.temperature {
            body["temperature"] = serde_json::json!(t);
        }

        let client = reqwest::Client::new();
        let res = client
            .post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", key))
            .json(&body)
            .send()
            .await
            .map_err(|e| AppError::Generic(format!("OpenAI HTTP request failed: {}", e)))?;

        let status = res.status();
        let json_val: serde_json::Value = res
            .json()
            .await
            .map_err(|e| AppError::Generic(format!("Failed to parse OpenAI response: {}", e)))?;

        if !status.is_success() {
            let err_msg = json_val["error"]["message"].as_str().unwrap_or("Unknown OpenAI error");
            return Err(AppError::Generic(format!("OpenAI API Error ({}): {}", status, err_msg)));
        }

        let reply_text = json_val["choices"][0]["message"]["content"]
            .as_str()
            .ok_or_else(|| AppError::Generic("OpenAI response did not contain message content".to_string()))?;

        Ok(reply_text.to_string())
    }
}
