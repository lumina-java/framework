use axum::{
    body::Body,
    extract::Request,
    middleware::Next,
    response::{IntoResponse, Response},
};
use std::time::Instant;

// ─── Request Logger ────────────────────────────────────────────────────────────

/// Middleware logger — mencatat setiap HTTP request ke terminal dengan warna.
///
/// Format: `[METHOD] /path → STATUS (ms)`
pub async fn logger(req: Request, next: Next) -> Response {
    let method = req.method().clone();
    let path   = req.uri().path().to_string();
    let start  = Instant::now();

    let response = next.run(req).await;

    let elapsed = start.elapsed();
    let status  = response.status();
    let ms      = elapsed.as_secs_f64() * 1000.0;

    let method_color = match method.as_str() {
        "GET"    => "\x1b[34m",
        "POST"   => "\x1b[32m",
        "PUT"    => "\x1b[33m",
        "DELETE" => "\x1b[31m",
        "PATCH"  => "\x1b[35m",
        _        => "\x1b[37m",
    };

    let status_color = if status.is_success() {
        "\x1b[32m"
    } else if status.is_client_error() {
        "\x1b[33m"
    } else if status.is_server_error() {
        "\x1b[31m"
    } else {
        "\x1b[37m"
    };

    println!(
        "  {}{:7}\x1b[0m {:<30} \x1b[90m→\x1b[0m {}{}\x1b[0m  \x1b[90m({:.2}ms)\x1b[0m",
        method_color,
        format!("[{}]", method),
        path,
        status_color,
        status.as_u16(),
        ms,
    );

    response
}

// ─── Auth Middleware ────────────────────────────────────────────────────────────

/// Middleware autentikasi — memvalidasi JWT Bearer token dari header `Authorization`.
///
/// Jika token valid: request diteruskan ke handler.
/// Jika tidak ada atau invalid: kembalikan `401 Unauthorized` dengan JSON error.
///
/// Penggunaan di routes:
/// ```rust
/// let protected = AxumRouter::new()
///     .route("/me", get(handler))
///     .route_layer(from_fn(auth_required));
/// ```
pub async fn auth_required(req: Request, next: Next) -> Response {
    let token = req
        .headers()
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .map(|t| t.to_string());

    match token {
        None => unauthorized_json("Token tidak ditemukan. Sertakan header: Authorization: Bearer <token>"),
        Some(t) => match crate::http::auth::validate_token(&t) {
            Ok(_claims) => next.run(req).await,
            Err(e) => unauthorized_json(&format!("Token invalid atau expired: {}", e)),
        },
    }
}

/// Helper — buat Response 401 dengan body JSON.
fn unauthorized_json(message: &str) -> Response {
    let body = serde_json::json!({
        "metaData": {
            "code": "401",
            "message": message
        },
        "response": null
    });
    axum::response::Json(body).into_response()
}

// ─── Web Auth Middleware ────────────────────────────────────────────────────────

/// Middleware autentikasi untuk Web — memvalidasi JWT dari cookie `jwt`.
///
/// Jika valid: Claims disuntikkan ke Request Extensions.
/// Jika tidak valid: Redirect ke `/auth/login`.
pub async fn web_auth_required(
    cookie_jar: axum_extra::extract::CookieJar,
    mut req: Request,
    next: Next,
) -> Response {
    let token = cookie_jar
        .get("jwt")
        .map(|c| c.value().to_string());

    match token {
        Some(t) => match crate::http::auth::validate_token(&t) {
            Ok(claims) => {
                // Simpan claims di request extension agar bisa diakses controller
                req.extensions_mut().insert(claims);
                next.run(req).await
            },
            Err(_) => Redirect::to("/auth/login?error=Sesi berakhir. Silakan login kembali.").into_response(),
        },
        None => Redirect::to("/auth/login?error=Silakan login terlebih dahulu.").into_response(),
    }
}

use axum::response::Redirect;
use axum::response::IntoResponse;
