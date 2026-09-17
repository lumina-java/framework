use lumina::core::echo::broadcaster::{ChannelAuthRequest, ShouldBroadcast};
use lumina::core::echo::EchoManager;
use lumina::core::event::traits::Event;
use lumina::http::middleware::security_headers;
use axum::{
    body::Body,
    http::{Request, StatusCode},
    middleware::from_fn,
    routing::get,
    Router,
};
use serde_json::json;
use std::any::Any;
use tower::ServiceExt;

#[derive(Debug)]
struct SampleUserRegisteredEvent {
    user_id: u64,
    username: String,
}

impl Event for SampleUserRegisteredEvent {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn broadcaster(&self) -> Option<&dyn ShouldBroadcast> {
        Some(self)
    }
}

impl ShouldBroadcast for SampleUserRegisteredEvent {
    fn broadcast_on(&self) -> Vec<String> {
        vec![format!("private-user-{}", self.user_id)]
    }

    fn broadcast_as(&self) -> String {
        "UserRegistered".to_string()
    }

    fn broadcast_with(&self) -> serde_json::Value {
        json!({
            "id": self.user_id,
            "username": self.username,
        })
    }
}

#[tokio::test]
async fn test_echo_channel_jwt_authorization() {
    use lumina::core::auth::AuthUser;
    use lumina::core::echo::EchoManager;
    use lumina::http::auth::{generate_token, validate_token};

    std::env::set_var("APP_KEY", "secret-test-key-12345");

    let echo = EchoManager::new();

    // Register JWT authorization handler for private user channels
    echo.authorize_channel("private-user-*", |channel_name: &str, token_or_socket: Option<&str>| {
        let token = match token_or_socket {
            Some(t) => t,
            None => return false,
        };

        let auth_user = match validate_token(token) {
            Ok(user) => user,
            Err(_) => return false,
        };

        if let Some(target_user_id) = channel_name.strip_prefix("private-user-") {
            return auth_user.sub == target_user_id;
        }

        false
    });

    // 1. Generate valid JWT token for user ID 42
    let user_42 = AuthUser::new(42, "user42@example.com".to_string(), "user".to_string(), vec![], 1);
    let jwt_token_42 = generate_token(&user_42).unwrap();

    // 2. Direct EchoManager verification: Valid channel matching sub=42
    let req_valid = ChannelAuthRequest {
        channel_name: "private-user-42".to_string(),
        socket_id: None,
        token: Some(jwt_token_42.clone()),
    };
    assert!(echo.authenticate_channel(&req_valid).authorized);

    // 3. Direct EchoManager verification: User 42 trying to access user 99's private channel
    let req_mismatched = ChannelAuthRequest {
        channel_name: "private-user-99".to_string(),
        socket_id: None,
        token: Some(jwt_token_42.clone()),
    };
    assert!(!echo.authenticate_channel(&req_mismatched).authorized);

    // 4. Direct EchoManager verification: Invalid token
    let req_invalid = ChannelAuthRequest {
        channel_name: "private-user-42".to_string(),
        socket_id: None,
        token: Some("invalid.jwt.token".to_string()),
    };
    assert!(!echo.authenticate_channel(&req_invalid).authorized);
}

#[tokio::test]
async fn test_echo_manager_subscriptions_and_auth() {
    let echo = EchoManager::new();

    // Test authorization default & registration
    let req_public = ChannelAuthRequest {
        channel_name: "public-news".to_string(),
        socket_id: None,
        token: None,
    };
    let auth_res = echo.authenticate_channel(&req_public);
    assert!(auth_res.authorized);

    // Register private authorizer
    echo.authorize_channel("private-user-*", |_channel, socket_id| {
        socket_id == Some("valid-socket")
    });

    let req_private_valid = ChannelAuthRequest {
        channel_name: "private-user-10".to_string(),
        socket_id: Some("valid-socket".to_string()),
        token: None,
    };
    assert!(echo.authenticate_channel(&req_private_valid).authorized);

    let req_private_invalid = ChannelAuthRequest {
        channel_name: "private-user-10".to_string(),
        socket_id: Some("invalid-socket".to_string()),
        token: None,
    };
    assert!(!echo.authenticate_channel(&req_private_invalid).authorized);
}

#[tokio::test]
async fn test_should_broadcast_trait() {
    let event = SampleUserRegisteredEvent {
        user_id: 42,
        username: "jules".to_string(),
    };

    assert_eq!(event.broadcast_as(), "UserRegistered");
    assert_eq!(event.broadcast_on(), vec!["private-user-42"]);
    assert_eq!(event.broadcast_with()["username"], "jules");

    // Test Event trait broadcaster() method
    let event_trait: &dyn Event = &event;
    assert!(event_trait.broadcaster().is_some());
    assert_eq!(event_trait.broadcaster().unwrap().broadcast_as(), "UserRegistered");
}

#[tokio::test]
async fn test_security_headers_middleware() {
    let app = Router::new()
        .route("/test", get(|| async { "ok" }))
        .layer(from_fn(security_headers));

    let response = app
        .oneshot(
            Request::builder()
                .uri("/test")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let headers = response.headers();
    assert_eq!(headers.get("X-Content-Type-Options").unwrap(), "nosniff");
    assert_eq!(headers.get("X-Frame-Options").unwrap(), "SAMEORIGIN");
    assert_eq!(headers.get("X-XSS-Protection").unwrap(), "1; mode=block");
    assert!(headers.contains_key("Strict-Transport-Security"));
}
