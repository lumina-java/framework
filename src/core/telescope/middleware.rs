use axum::{
    extract::{Request, State},
    middleware::Next,
    response::Response,
};
use std::time::Instant;
use crate::core::application::AppState;
use crate::core::telescope::entry::RequestContent;

pub async fn telescope_middleware(
    State(state): State<AppState>,
    req: Request,
    next: Next,
) -> Response {
    // Abaikan request ke dashboard telescope itu sendiri agar tidak rekursif berlebihan
    let path = req.uri().path();
    if path.starts_with("/lumina/telescope") || path.starts_with("/storage") {
        return next.run(req).await;
    }

    let start = Instant::now();
    let method = req.method().to_string();
    let uri = req.uri().to_string();
    
    // IP detection (simple version)
    let ip = req.headers()
        .get("x-forwarded-for")
        .and_then(|h| h.to_str().ok())
        .unwrap_or("127.0.0.1")
        .to_string();

    let response = next.run(req).await;
    
    let duration = start.elapsed().as_millis();
    let status = response.status().as_u16();

    let content = RequestContent {
        method,
        uri,
        status,
        duration_ms: duration,
        ip,
    };

    state.telescope.record_request(content);

    response
}
