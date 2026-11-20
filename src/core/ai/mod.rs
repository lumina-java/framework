pub mod gemini;
pub mod openai;
pub mod claude;

pub use gemini::GeminiClient;
pub use openai::OpenAiClient;
pub use claude::ClaudeClient;

/// Fluent AI Facade for Multi-Model LLM Interactions in Lumina Framework
pub struct Ai;

impl Ai {
    /// Initialize Google Gemini Client (Gemini 1.5 Pro / Flash)
    pub fn gemini() -> GeminiClient {
        GeminiClient::new()
    }

    /// Initialize OpenAI Client (GPT-4o, GPT-4o-mini, o1)
    pub fn openai() -> OpenAiClient {
        OpenAiClient::new()
    }

    /// Initialize Anthropic Claude Client (Claude 3.5 / 3.7 Sonnet)
    pub fn claude() -> ClaudeClient {
        ClaudeClient::new()
    }
}
