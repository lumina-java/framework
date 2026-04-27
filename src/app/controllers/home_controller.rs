use axum::response::Html;

pub struct HomeController;

impl HomeController {
    /// GET /
    pub async fn index() -> Html<String> {
        let html = r#"<!DOCTYPE html>
<html lang="id">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Lumina Framework</title>
    <style>
        * { margin: 0; padding: 0; box-sizing: border-box; }
        body {
            font-family: 'Segoe UI', sans-serif;
            background: linear-gradient(135deg, #0f0c29, #302b63, #24243e);
            min-height: 100vh;
            display: flex;
            align-items: center;
            justify-content: center;
            color: white;
        }
        .card {
            text-align: center;
            padding: 60px 80px;
            background: rgba(255,255,255,0.05);
            border: 1px solid rgba(255,255,255,0.1);
            border-radius: 20px;
            backdrop-filter: blur(20px);
            box-shadow: 0 25px 50px rgba(0,0,0,0.5);
        }
        h1 { font-size: 3rem; margin-bottom: 10px; }
        .tagline { color: rgba(255,255,255,0.6); font-size: 1.1rem; margin-bottom: 40px; }
        .badge {
            display: inline-block;
            background: rgba(99, 102, 241, 0.3);
            border: 1px solid rgba(99, 102, 241, 0.5);
            padding: 6px 18px;
            border-radius: 999px;
            font-size: 0.85rem;
            color: #a5b4fc;
            margin: 5px;
        }
        .nav a {
            color: #a5b4fc;
            text-decoration: none;
            margin: 0 15px;
            font-size: 0.95rem;
        }
        .nav a:hover { color: white; }
        .nav { margin-top: 40px; }
        .version { font-size: 0.8rem; color: rgba(255,255,255,0.3); margin-top: 20px; }
    </style>
</head>
<body>
    <div class="card">
        <h1>✨ Lumina</h1>
        <p class="tagline">A beautiful, fast, and elegant web framework for Rust</p>
        <div>
            <span class="badge">🦀 Rust</span>
            <span class="badge">⚡ Axum</span>
            <span class="badge">🔄 Async</span>
            <span class="badge">🏗️ MVC</span>
        </div>
        <div class="nav">
            <a href="/">🏠 Home</a>
            <a href="/about">ℹ️ About</a>
        </div>
        <p class="version">v0.1.0 · Skeleton Edition</p>
    </div>
</body>
</html>"#;
        Html(html.to_string())
    }

    /// GET /about
    pub async fn about() -> Html<String> {
        let html = r#"<!DOCTYPE html>
<html lang="id">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>About - Lumina Framework</title>
    <style>
        * { margin: 0; padding: 0; box-sizing: border-box; }
        body {
            font-family: 'Segoe UI', sans-serif;
            background: linear-gradient(135deg, #0f0c29, #302b63, #24243e);
            min-height: 100vh;
            display: flex;
            align-items: center;
            justify-content: center;
            color: white;
        }
        .card {
            text-align: center;
            padding: 60px 80px;
            background: rgba(255,255,255,0.05);
            border: 1px solid rgba(255,255,255,0.1);
            border-radius: 20px;
            backdrop-filter: blur(20px);
            box-shadow: 0 25px 50px rgba(0,0,0,0.5);
            max-width: 600px;
        }
        h1 { font-size: 2.5rem; margin-bottom: 20px; }
        p { color: rgba(255,255,255,0.7); line-height: 1.8; margin-bottom: 10px; }
        .nav a {
            color: #a5b4fc;
            text-decoration: none;
            margin: 0 15px;
            font-size: 0.95rem;
        }
        .nav { margin-top: 40px; }
    </style>
</head>
<body>
    <div class="card">
        <h1>ℹ️ About Lumina</h1>
        <p>Lumina adalah web framework untuk Rust yang terinspirasi dari Laravel.</p>
        <p>Dibangun di atas <strong>Axum</strong> dan <strong>Tokio</strong> untuk performa async yang tinggi.</p>
        <p>Versi: <strong>0.1.0</strong> · Author: <strong>Slamet Sugandi</strong></p>
        <div class="nav">
            <a href="/">← Kembali ke Home</a>
        </div>
    </div>
</body>
</html>"#;
        Html(html.to_string())
    }
}
