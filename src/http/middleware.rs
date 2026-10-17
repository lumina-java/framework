use crate::core::auth::AuthUser;
pub use axum::{
    extract::Request,
    middleware::{from_fn, from_fn_with_state, Next},
    response::{IntoResponse, Redirect, Response},
};
use std::time::Instant;

// ─── Re-exports & Helper Respon Standar Lumina ─────────────────────────────────

/// Helper — buat Response error JSON standar Lumina.
pub fn error_json(status: axum::http::StatusCode, message: &str) -> Response {
    let body = serde_json::json!({
        "metaData": {
            "code": status.as_u16().to_string(),
            "message": message
        },
        "response": null
    });
    (status, axum::response::Json(body)).into_response()
}

/// Helper — buat Response 401 Unauthorized dengan body JSON standar Lumina.
pub fn unauthorized_json(message: &str) -> Response {
    error_json(axum::http::StatusCode::UNAUTHORIZED, message)
}

/// Helper — buat Response 403 Forbidden dengan body JSON standar Lumina.
pub fn forbidden_json(message: &str) -> Response {
    error_json(axum::http::StatusCode::FORBIDDEN, message)
}

/// Helper — buat Response 400 Bad Request dengan body JSON standar Lumina.
pub fn bad_request_json(message: &str) -> Response {
    error_json(axum::http::StatusCode::BAD_REQUEST, message)
}

/// Helper — ambil string nilai header dari HTTP Request.
pub fn get_header<'a>(req: &'a Request, header_name: &str) -> Option<&'a str> {
    req.headers()
        .get(header_name)
        .and_then(|v| v.to_str().ok())
}

/// Helper — ambil Bearer token dari header `Authorization`.
pub fn get_bearer_token(req: &Request) -> Option<String> {
    get_header(req, "Authorization")
        .and_then(|v| v.strip_prefix("Bearer "))
        .map(|t| t.trim().to_string())
}

/// Helper validator — pastikan request memiliki semua header wajib yang ditentukan.
pub fn validate_required_headers(req: &Request, required_headers: &[&str]) -> Result<(), String> {
    for &h in required_headers {
        match get_header(req, h) {
            Some(v) if !v.trim().is_empty() => continue,
            _ => return Err(format!("Header '{}' diperlukan.", h)),
        }
    }
    Ok(())
}

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

/// Middleware autentikasi standar — memvalidasi JWT Bearer token dari header `Authorization`.
pub async fn auth_required(mut req: Request, next: Next) -> Response {
    let token = get_bearer_token(&req);

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

// ─── Custom API Auth & Security Guards ─────────────────────────────────────────

/// Middleware autentikasi API: Memvalidasi JWT Bearer token DAN header pairing `X-Token-Pairing`.
///
/// Sangat ideal untuk API perbankan, mobile client device pairing, atau multi-device session check.
pub async fn api_auth_with_pairing(mut req: Request, next: Next) -> Response {
    let token = match get_bearer_token(&req) {
        Some(t) => t,
        None => {
            return unauthorized_json(
                "Token autentikasi tidak ditemukan. Sertakan header: Authorization: Bearer <token>",
            )
        }
    };

    let user = match crate::http::auth::validate_token(&token) {
        Ok(u) => u,
        Err(e) => return unauthorized_json(&format!("Token JWT tidak valid atau expired: {}", e)),
    };

    let pairing = match get_header(&req, "X-Token-Pairing") {
        Some(p) if !p.trim().is_empty() => p.trim(),
        _ => return unauthorized_json("Header keamanan 'X-Token-Pairing' diperlukan."),
    };

    if pairing.len() < 8 {
        return unauthorized_json("Header 'X-Token-Pairing' tidak valid (minimal 8 karakter).");
    }

    req.extensions_mut().insert(user);
    next.run(req).await
}

/// Middleware verifikasi API Signature (misal untuk integrasi webhook, perbankan / SNAP BI, HMAC).
/// Memvalidasi header `X-Signature` dan `X-Timestamp`.
pub async fn verify_api_signature(req: Request, next: Next) -> Response {
    let signature = match get_header(&req, "X-Signature") {
        Some(sig) if !sig.trim().is_empty() => sig.trim(),
        _ => return unauthorized_json("Header keamanan 'X-Signature' diperlukan."),
    };

    let timestamp = match get_header(&req, "X-Timestamp") {
        Some(ts) if !ts.trim().is_empty() => ts.trim(),
        _ => return unauthorized_json("Header 'X-Timestamp' diperlukan untuk verifikasi signature."),
    };

    if signature.len() < 8 || timestamp.is_empty() {
        return unauthorized_json("Nilai signature atau timestamp tidak valid.");
    }

    next.run(req).await
}

/// Middleware komprehensif: Autentikasi JWT Bearer + verifikasi `X-Signature` + verifikasi `X-Token-Pairing`.
pub async fn api_secure_guard(mut req: Request, next: Next) -> Response {
    let token = match get_bearer_token(&req) {
        Some(t) => t,
        None => {
            return unauthorized_json(
                "Token autentikasi tidak ditemukan. Sertakan header: Authorization: Bearer <token>",
            )
        }
    };

    let user = match crate::http::auth::validate_token(&token) {
        Ok(u) => u,
        Err(e) => return unauthorized_json(&format!("Token JWT tidak valid atau expired: {}", e)),
    };

    if let Err(e) = validate_required_headers(&req, &["X-Signature", "X-Token-Pairing"]) {
        return unauthorized_json(&e);
    }

    req.extensions_mut().insert(user);
    next.run(req).await
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
                forbidden_json(&format!("Role '{}' diperlukan.", role))
            } else {
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
                forbidden_json(&format!("Izin '{}' diperlukan.", permission))
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

    response
}
