use axum::{extract::Request, middleware::Next, response::Response};
use std::time::Instant;

/// Middleware logger — mencatat setiap HTTP request masuk ke terminal.
///
/// Output format:
/// ```
///   [GET]    /users      → 200  (1.23ms)
///   [POST]   /api/users  → 201  (0.87ms)
///   [GET]    /not-found  → 404  (0.12ms)
/// ```
pub async fn logger(req: Request, next: Next) -> Response {
    let method = req.method().clone();
    let path   = req.uri().path().to_string();
    let start  = Instant::now();

    let response = next.run(req).await;

    let elapsed = start.elapsed();
    let status  = response.status();
    let ms      = elapsed.as_secs_f64() * 1000.0;

    // Pilih warna berdasarkan HTTP method
    let method_color = match method.as_str() {
        "GET"    => "\x1b[34m", // biru
        "POST"   => "\x1b[32m", // hijau
        "PUT"    => "\x1b[33m", // kuning
        "DELETE" => "\x1b[31m", // merah
        "PATCH"  => "\x1b[35m", // magenta
        _        => "\x1b[37m", // putih
    };

    // Pilih warna berdasarkan status code
    let status_color = if status.is_success() {
        "\x1b[32m"      // hijau
    } else if status.is_client_error() {
        "\x1b[33m"      // kuning
    } else if status.is_server_error() {
        "\x1b[31m"      // merah
    } else {
        "\x1b[37m"      // putih
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
