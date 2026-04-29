use axum::response::{Html, IntoResponse, Response};
use std::any::Any;

/// Macro untuk mencetak variabel ke console (Terminal) secara terstruktur.
/// Berguna untuk debugging tanpa menghentikan eksekusi program.
#[macro_export]
macro_rules! dump {
    ($val:expr) => {
        let location = std::panic::Location::caller();
        println!("┌─ 🐛 DUMP at {}:{}", location.file(), location.line());
        println!("│ {:#?}", $val);
        println!("└────────────────────────────────────────────────────────");
    };
}

/// Macro untuk Dump and Die (Mencetak variabel ke browser HTML dan menghentikan request).
/// Ini bekerja dengan cara memicu Panic khusus yang akan ditangkap oleh Router Lumina
/// lalu di-render menjadi halaman HTML bergaya Dark Mode.
#[macro_export]
macro_rules! dd {
    ($val:expr) => {
        let dump_content = format!("{:#?}", $val);
        let location = std::panic::Location::caller();
        let payload = format!("__LUMINA_DD__\nLocation: {}:{}\n\n{}", location.file(), location.line(), dump_content);
        panic!("{}", payload);
    };
}

/// Handler khusus untuk mencegat Panic dari `dd!()` dan merendernya sebagai UI cantik.
pub fn handle_panic(err: Box<dyn Any + Send + 'static>) -> Response {
    let msg = if let Some(s) = err.downcast_ref::<String>() {
        s.clone()
    } else if let Some(s) = err.downcast_ref::<&str>() {
        s.to_string()
    } else {
        "Unknown panic".to_string()
    };

    if msg.starts_with("__LUMINA_DD__\n") {
        let dump_content = msg.replace("__LUMINA_DD__\n", "");
        let html = format!(r#"
            <!DOCTYPE html>
            <html>
            <head>
                <title>Lumina Dump</title>
                <style>
                    body {{ background: #18171B; color: #D4D4D4; font-family: 'Consolas', 'Menlo', monospace; padding: 30px; margin: 0; }}
                    .container {{ background: #222125; padding: 20px; border-radius: 8px; border-left: 5px solid #FF8400; box-shadow: 0 4px 6px rgba(0,0,0,0.3); }}
                    h2 {{ color: #FF8400; margin-top: 0; font-size: 16px; font-weight: normal; margin-bottom: 15px; border-bottom: 1px solid #333; padding-bottom: 10px; }}
                    pre {{ font-size: 14px; white-space: pre-wrap; word-wrap: break-word; margin: 0; }}
                </style>
            </head>
            <body>
                <div class="container">
                    <h2>🐛 Lumina Dump</h2>
                    <pre>{}</pre>
                </div>
            </body>
            </html>
        "#, dump_content);
        return Html(html).into_response();
    }

    // Jika Panic biasa (Bukan DD)
    Html("<h1>500 Internal Server Error</h1>").into_response()
}
