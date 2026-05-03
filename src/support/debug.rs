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

use std::cell::RefCell;

thread_local! {
    pub static PANIC_INFO: RefCell<Option<(String, String)>> = RefCell::new(None);
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

    let thread_panic_info = PANIC_INFO.with(|p| p.borrow_mut().take());

    if msg.starts_with("__LUMINA_DD__\n") {
        let parts: Vec<&str> = msg.splitn(3, "\n\n").collect();
        let header = parts.get(0).unwrap_or(&"").replace("__LUMINA_DD__\n", "");
        let body = parts.get(1).unwrap_or(&"");
        
        let html = format!(r#"
            <!DOCTYPE html>
            <html lang="en">
            <head>
                <meta charset="UTF-8">
                <meta name="viewport" content="width=device-width, initial-scale=1.0">
                <title>Lumina Dump & Die</title>
                <link href="https://fonts.googleapis.com/css2?family=Inter:wght@400;600&family=Fira+Code:wght@400;500&display=swap" rel="stylesheet">
                <style>
                    :root {{
                        --bg: #0f172a;
                        --card-bg: #1e293b;
                        --accent: #38bdf8;
                        --text: #f1f5f9;
                        --muted: #94a3b8;
                        --border: #334155;
                        --orange: #f59e0b;
                    }}
                    body {{ 
                        background: var(--bg); 
                        color: var(--text); 
                        font-family: 'Inter', sans-serif; 
                        padding: 40px 20px; 
                        margin: 0; 
                        line-height: 1.5;
                    }}
                    .container {{ 
                        max-width: 1000px; 
                        margin: 0 auto; 
                    }}
                    header {{
                        display: flex;
                        justify-content: space-between;
                        align-items: center;
                        margin-bottom: 24px;
                        padding-bottom: 16px;
                        border-bottom: 1px solid var(--border);
                    }}
                    .logo {{
                        font-weight: 600;
                        font-size: 20px;
                        color: var(--accent);
                        display: flex;
                        align-items: center;
                        gap: 10px;
                    }}
                    .badge {{
                        background: rgba(56, 189, 248, 0.1);
                        color: var(--accent);
                        padding: 4px 12px;
                        border-radius: 99px;
                        font-size: 12px;
                        font-weight: 600;
                        text-transform: uppercase;
                        letter-spacing: 0.05em;
                    }}
                    .location {{
                        color: var(--muted);
                        font-size: 13px;
                        font-family: 'Fira Code', monospace;
                    }}
                    .card {{ 
                        background: var(--card-bg); 
                        padding: 24px; 
                        border-radius: 12px; 
                        border: 1px solid var(--border);
                        box-shadow: 0 10px 15px -3px rgba(0, 0, 0, 0.1), 0 4px 6px -2px rgba(0, 0, 0, 0.05);
                    }}
                    pre {{ 
                        font-family: 'Fira Code', monospace;
                        font-size: 14px; 
                        white-space: pre-wrap; 
                        word-wrap: break-word; 
                        margin: 0; 
                        color: #7dd3fc;
                    }}
                    .footer {{
                        margin-top: 24px;
                        text-align: center;
                        color: var(--muted);
                        font-size: 12px;
                    }}
                </style>
            </head>
            <body>
                <div class="container">
                    <header>
                        <div class="logo">
                            <span>✨ Lumina</span>
                            <span class="badge">Dump & Die</span>
                        </div>
                        <div class="location">{}</div>
                    </header>
                    <div class="card">
                        <pre>{}</pre>
                    </div>
                    <div class="footer">
                        Rendered by Lumina Framework Debugger • {}
                    </div>
                </div>
            </body>
            </html>
        "#, header, body, chrono::Local::now().format("%Y-%m-%d %H:%M:%S"));
        return Html(html).into_response();
    }

    // Standard Panic (Ignition style error page)
    let (err_msg, loc) = thread_panic_info.unwrap_or((msg.clone(), "Unknown location".to_string()));
    
    let solution = if err_msg.contains("no such table") || err_msg.contains("Table") && err_msg.contains("doesn't exist") {
        "Saran: Jalankan migrasi database Anda dengan perintah <code>./lumina migrate</code> atau <code>cargo run -- make:migration</code>."
    } else if err_msg.contains("Connection refused") || err_msg.contains("database connection") {
        "Saran: Pastikan database Anda sudah menyala dan URL koneksi di <code>.env</code> sudah benar."
    } else if err_msg.contains("Tera") {
        "Saran: Terjadi kesalahan saat merender template Tera. Periksa nama variabel atau filter di file view Anda."
    } else {
        "Saran: Periksa kembali baris kode yang menyebabkan panic ini terjadi."
    };

    let html = format!(r#"
        <!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="UTF-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <title>Lumina Exception Handler</title>
            <link href="https://fonts.googleapis.com/css2?family=Inter:wght@400;600&family=Fira+Code:wght@400;500&display=swap" rel="stylesheet">
            <style>
                :root {{
                    --bg: #0f172a;
                    --card-bg: #1e293b;
                    --accent: #ef4444;
                    --text: #f1f5f9;
                    --muted: #94a3b8;
                    --border: #334155;
                    --green: #10b981;
                }}
                body {{ 
                    background: var(--bg); 
                    color: var(--text); 
                    font-family: 'Inter', sans-serif; 
                    padding: 40px 20px; 
                    margin: 0; 
                    line-height: 1.5;
                }}
                .container {{ 
                    max-width: 1000px; 
                    margin: 0 auto; 
                }}
                header {{
                    display: flex;
                    justify-content: space-between;
                    align-items: center;
                    margin-bottom: 24px;
                    padding-bottom: 16px;
                    border-bottom: 1px solid var(--border);
                }}
                .logo {{
                    font-weight: 600;
                    font-size: 20px;
                    color: var(--accent);
                    display: flex;
                    align-items: center;
                    gap: 10px;
                }}
                .badge {{
                    background: rgba(239, 68, 68, 0.1);
                    color: var(--accent);
                    padding: 4px 12px;
                    border-radius: 99px;
                    font-size: 12px;
                    font-weight: 600;
                    text-transform: uppercase;
                    letter-spacing: 0.05em;
                }}
                .location {{
                    color: var(--muted);
                    font-size: 13px;
                    font-family: 'Fira Code', monospace;
                }}
                .card {{ 
                    background: var(--card-bg); 
                    padding: 24px; 
                    border-radius: 12px; 
                    border: 1px solid var(--border);
                    box-shadow: 0 10px 15px -3px rgba(0, 0, 0, 0.1), 0 4px 6px -2px rgba(0, 0, 0, 0.05);
                    margin-bottom: 24px;
                }}
                .solution-card {{
                    background: #1e1b4b;
                    border-left: 4px solid var(--green);
                }}
                pre {{ 
                    font-family: 'Fira Code', monospace;
                    font-size: 14px; 
                    white-space: pre-wrap; 
                    word-wrap: break-word; 
                    margin: 0; 
                    color: #fca5a5;
                }}
                .footer {{
                    margin-top: 24px;
                    text-align: center;
                    color: var(--muted);
                    font-size: 12px;
                }}
            </style>
        </head>
        <body>
            <div class="container">
                <header>
                    <div class="logo">
                        <span>💥 Lumina Exception</span>
                        <span class="badge">Unhandled Panic</span>
                    </div>
                    <div class="location">{}</div>
                </header>
                
                <div class="card">
                    <h3 style="margin-top:0; color: #f87171;">Exception Details</h3>
                    <pre>{}</pre>
                </div>

                <div class="card solution-card">
                    <h3 style="margin-top:0; color: #34d399;">💡 Saran Solusi</h3>
                    <p style="margin: 0; color: #a7f3d0; font-size: 15px;">{}</p>
                </div>

                <div class="footer">
                    Rendered by Lumina Framework Debugger • {}
                </div>
            </div>
        </body>
        </html>
    "#, loc, err_msg, solution, chrono::Local::now().format("%Y-%m-%d %H:%M:%S"));
    
    Html(html).into_response()
}
