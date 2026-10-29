use crate::core::{request::Request as LuminaRequest, response::Response as LuminaResponse};
use async_trait::async_trait;
use axum::{
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};
use futures::FutureExt;
use std::panic::AssertUnwindSafe;

#[async_trait]
pub trait Middleware: Send + Sync {
    async fn handle(&self, request: LuminaRequest) -> Result<LuminaResponse, String>;
}

/// Global Zero-Panic Error Boundary Middleware.
/// Menjamin bahwa `panic!` di dalam controller atau library pihak ketiga
/// TIDAK AKAN PERNAH meruntuhkan worker thread / server Lumina.
/// Menangkap panic secara aman, mencatat pesan panic ke log,
/// serta mengembalikan HTTP 500 berformat rapi.
pub async fn zero_panic_boundary(req: Request, next: Next) -> Response {
    let method = req.method().clone();
    let uri = req.uri().clone();
    let wants_json = req
        .headers()
        .get("accept")
        .and_then(|h| h.to_str().ok())
        .map(|a| a.contains("application/json") || a.contains("*/*"))
        .unwrap_or(false)
        || uri.path().starts_with("/api");

    let result = AssertUnwindSafe(next.run(req)).catch_unwind().await;

    match result {
        Ok(response) => response,
        Err(err) => {
            let panic_msg = if let Some(s) = err.downcast_ref::<&str>() {
                s.to_string()
            } else if let Some(s) = err.downcast_ref::<String>() {
                s.clone()
            } else {
                "Unknown panic payload".to_string()
            };

            tracing::error!(
                target: "lumina::panic_boundary",
                method = %method,
                uri = %uri,
                panic = %panic_msg,
                "🚨 CRITICAL PANIC CAUGHT: Server dilindungi oleh Zero-Panic Boundary!"
            );

            if wants_json {
                let body = serde_json::json!({
                    "metaData": {
                        "code": "500",
                        "message": "Kesalahan internal server (Protected by Zero-Panic Safety Net)"
                    },
                    "response": null
                });
                (StatusCode::INTERNAL_SERVER_ERROR, axum::Json(body)).into_response()
            } else {
                axum::response::Html(format!(
                    r#"<!DOCTYPE html>
<html lang="id">
<head>
    <meta charset="UTF-8">
    <title>500 — Kesalahan Internal Server</title>
    <style>
        body {{ font-family: ui-sans-serif, system-ui, sans-serif; background: #0f172a; color: #f8fafc; display: flex; align-items: center; justify-content: center; height: 100vh; margin: 0; }}
        .card {{ background: #1e293b; padding: 2.5rem; border-radius: 1rem; border: 1px solid #334155; max-width: 32rem; text-align: center; box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.5); }}
        h1 {{ font-size: 3rem; margin: 0 0 1rem; color: #f43f5e; }}
        p {{ color: #94a3b8; line-height: 1.6; margin-bottom: 1.5rem; }}
        .badge {{ background: #0284c7; color: white; padding: 0.25rem 0.75rem; border-radius: 9999px; font-size: 0.75rem; font-weight: bold; text-transform: uppercase; letter-spacing: 0.05em; }}
    </style>
</head>
<body>
    <div class="card">
        <span class="badge">Zero-Panic Boundary</span>
        <h1>500</h1>
        <p>Terjadi kesalahan tak terduga saat memproses permintaan ini. Sistem telah mengamankan proses sehingga server tetap berjalan normal.</p>
    </div>
</body>
</html>"#
                )).into_response()
            }
        }
    }
}
