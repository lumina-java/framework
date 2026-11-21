use pakem::prelude::*;

#[tokio::test]
async fn test_ai_facade_fluent_gemini() {
    let _client = Ai::gemini()
        .model("gemini-1.5-pro")
        .with_api_key("fake-gemini-key")
        .with_system("Kamu adalah konsultan Rust.")
        .temperature(0.5)
        .prompt("Jelaskan ownership di Rust.");

    // Empty prompt check
    let empty_client = Ai::gemini()
        .with_api_key("fake-key")
        .prompt("   ");
    let err = empty_client.generate().await;
    assert!(err.is_err());
    match err.unwrap_err() {
        AppError::BadRequest(msg) => assert!(msg.contains("cannot be empty")),
        other => panic!("Expected BadRequest, got: {:?}", other),
    }

    // Missing key check
    let no_key_client = Ai::gemini().prompt("Halo");
    // Ensure env is cleared
    std::env::remove_var("GEMINI_API_KEY");
    let err_no_key = no_key_client.generate().await;
    assert!(err_no_key.is_err());
    match err_no_key.unwrap_err() {
        AppError::Generic(msg) => assert!(msg.contains("Missing GEMINI_API_KEY")),
        other => panic!("Expected Generic error, got: {:?}", other),
    }
}

#[tokio::test]
async fn test_ai_facade_fluent_openai() {
    let _client = Ai::openai()
        .model("gpt-4o")
        .with_api_key("fake-openai-key")
        .with_system("You are an enterprise architect.")
        .temperature(0.7)
        .chat("Optimize this query");

    // Empty prompt check
    let empty_client = Ai::openai().with_api_key("fake-key").prompt("");
    let err = empty_client.generate().await;
    assert!(err.is_err());
    match err.unwrap_err() {
        AppError::BadRequest(msg) => assert!(msg.contains("cannot be empty")),
        other => panic!("Expected BadRequest, got: {:?}", other),
    }

    // Missing key check
    std::env::remove_var("OPENAI_API_KEY");
    let no_key_client = Ai::openai().chat("Halo");
    let err_no_key = no_key_client.generate().await;
    assert!(err_no_key.is_err());
    match err_no_key.unwrap_err() {
        AppError::Generic(msg) => assert!(msg.contains("Missing OPENAI_API_KEY")),
        other => panic!("Expected Generic error, got: {:?}", other),
    }
}

#[tokio::test]
async fn test_ai_facade_fluent_claude() {
    let _client = Ai::claude()
        .model("claude-3-7-sonnet-20250219")
        .with_api_key("fake-claude-key")
        .with_system("You are a code refactoring expert.")
        .max_tokens(2048)
        .temperature(0.3)
        .complete("Refactor this controller");

    // Empty prompt check
    let empty_client = Ai::claude().with_api_key("fake-key").prompt(" ");
    let err = empty_client.generate().await;
    assert!(err.is_err());
    match err.unwrap_err() {
        AppError::BadRequest(msg) => assert!(msg.contains("cannot be empty")),
        other => panic!("Expected BadRequest, got: {:?}", other),
    }

    // Missing key check
    std::env::remove_var("ANTHROPIC_API_KEY");
    let no_key_client = Ai::claude().complete("Halo");
    let err_no_key = no_key_client.generate().await;
    assert!(err_no_key.is_err());
    match err_no_key.unwrap_err() {
        AppError::Generic(msg) => assert!(msg.contains("Missing ANTHROPIC_API_KEY")),
        other => panic!("Expected Generic error, got: {:?}", other),
    }
}
