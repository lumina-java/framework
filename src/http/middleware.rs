use crate::core::auth::AuthUser;
use axum::{
    extract::Request,
    middleware::Next,
    response::{IntoResponse, Redirect, Response},
};
use std::time::Instant;

// ─── Request Logger ────────────────────────────────────────────────────────────

/// Middleware logger — mencatat setiap HTTP request ke terminal dengan warna.
///
/// Format: `[METHOD] /path → STATUS (ms)`
pub async fn logger(req: Request, next: Next) -> Response {
    let method = req.method().clone();
    let path = req.uri().path().to_string();
    let start = Instant::now();

    let response = next.run(req).await;

    let elapsed = start.elapsed();
    let status = response.status();
    let ms = elapsed.as_secs_f64() * 1000.0;

    let method_color = match method.as_str() {
        "GET" => "\x1b[34m",
        "POST" => "\x1b[32m",
        "PUT" => "\x1b[33m",
        "DELETE" => "\x1b[31m",
        "PATCH" => "\x1b[35m",
        _ => "\x1b[37m",
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
pub async fn auth_required(mut req: Request, next: Next) -> Response {
    let token = req
        .headers()
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .map(|t| t.to_string());

    match token {
        None => unauthorized_json(
            "Token tidak ditemukan. Sertakan header: Authorization: Bearer <token>",
        ),
        Some(t) => match crate::http::auth::validate_token(&t) {
            Ok(user) => {
                req.extensions_mut().insert(user);
                next.run(req).await
            }
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

/// Middleware autentikasi untuk Web — memvalidasi JWT dari Session.
pub async fn web_auth_required(
    session: tower_sessions::Session,
    mut req: Request,
    next: Next,
) -> Response {
    let token = session.get::<String>("jwt").await.unwrap_or_default();
    let flash = crate::core::session::FlashManager::new(&session);

    match token {
        Some(t) => match crate::http::auth::validate_token(&t) {
            Ok(user) => {
                req.extensions_mut().insert(user);
                next.run(req).await
            }
            Err(_) => {
                flash.error("Sesi berakhir. Silakan login kembali.").await;
                Redirect::to("/auth/login").into_response()
            }
        },
        None => {
            flash.error("Silakan login terlebih dahulu.").await;
            Redirect::to("/auth/login").into_response()
        }
    }
}

// ─── RBAC Middleware ───────────────────────────────────────────────────────────

/// Middleware untuk mengecek Role.
pub async fn role_required(req: Request, next: Next, role: &str) -> Response {
    let auth_user = req.extensions().get::<AuthUser>();

    match auth_user {
        Some(user) if user.has_role(role) => next.run(req).await,
        _ => {
            if req.uri().path().starts_with("/api") {
                unauthorized_json(&format!("Role '{}' diperlukan.", role))
            } else {
                // Sederhananya kita beri 403
                axum::response::Html(format!(
                    "<h1>403 Forbidden</h1><p>Role '{}' diperlukan.</p>",
                    role
                ))
                .into_response()
            }
        }
    }
}

/// Middleware untuk mengecek Permission.
pub async fn permission_required(req: Request, next: Next, permission: &str) -> Response {
    let auth_user = req.extensions().get::<AuthUser>();

    match auth_user {
        Some(user) if user.can(permission) => next.run(req).await,
        _ => {
            if req.uri().path().starts_with("/api") {
                unauthorized_json(&format!("Izin '{}' diperlukan.", permission))
            } else {
                axum::response::Html(format!(
                    "<h1>403 Forbidden</h1><p>Izin '{}' diperlukan.</p>",
                    permission
                ))
                .into_response()
            }
        }
    }
}

// ─── Production Hardening ──────────────────────────────────────────────────────

/// Middleware untuk menambahkan Security Headers standar industri.
pub async fn security_headers(req: Request, next: Next) -> Response {
    let mut response = next.run(req).await;
    let headers = response.headers_mut();

    headers.insert("X-Content-Type-Options", "nosniff".parse().unwrap());
    headers.insert("X-Frame-Options", "SAMEORIGIN".parse().unwrap());
    headers.insert("X-XSS-Protection", "1; mode=block".parse().unwrap());
    headers.insert(
        "Strict-Transport-Security",
        "max-age=31536000; includeSubDomains".parse().unwrap(),
    );
    headers.insert(
        "Referrer-Policy",
        "strict-origin-when-cross-origin".parse().unwrap(),
    );

    // Content-Security-Policy (Sangat dasar, silakan disesuaikan)
    headers.insert("Content-Security-Policy", "default-src 'self'; script-src 'self' unpkg.com cdn.jsdelivr.net code.jquery.com cdnjs.cloudflare.com; style-src 'self' 'unsafe-inline' cdn.jsdelivr.net cdnjs.cloudflare.com; img-src 'self' data:;".parse().unwrap());

    response
}
