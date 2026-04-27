use axum::{extract::Path, response::Html};

pub struct UserController;

impl UserController {
    /// GET /users — Tampilkan daftar semua user (HTML)
    pub async fn index() -> Html<String> {
        let html = r#"<!DOCTYPE html>
<html lang="id">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Users — Lumina</title>
    <style>
        * { margin: 0; padding: 0; box-sizing: border-box; }
        body {
            font-family: 'Segoe UI', sans-serif;
            background: linear-gradient(135deg, #0f0c29, #302b63, #24243e);
            min-height: 100vh;
            padding: 40px 20px;
            color: white;
        }
        .container { max-width: 800px; margin: 0 auto; }
        h1 { font-size: 2rem; margin-bottom: 8px; }
        .sub { color: rgba(255,255,255,0.5); margin-bottom: 30px; font-size: 0.9rem; }
        .badge {
            display: inline-block;
            background: rgba(99,102,241,0.3);
            border: 1px solid rgba(99,102,241,0.5);
            padding: 3px 12px; border-radius: 999px;
            font-size: 0.75rem; color: #a5b4fc; margin-bottom: 20px;
        }
        table {
            width: 100%; border-collapse: collapse;
            background: rgba(255,255,255,0.04);
            border: 1px solid rgba(255,255,255,0.08);
            border-radius: 12px; overflow: hidden;
        }
        th {
            background: rgba(99,102,241,0.2);
            padding: 14px 20px; text-align: left;
            font-size: 0.8rem; text-transform: uppercase;
            letter-spacing: 0.1em; color: #a5b4fc;
        }
        td { padding: 14px 20px; border-top: 1px solid rgba(255,255,255,0.06); }
        tr:hover td { background: rgba(255,255,255,0.04); }
        a { color: #818cf8; text-decoration: none; }
        a:hover { color: white; }
        .nav { margin-bottom: 30px; }
        .nav a { color: rgba(255,255,255,0.5); margin-right: 20px; font-size: 0.9rem; }
        .nav a:hover { color: white; }
    </style>
</head>
<body>
    <div class="container">
        <div class="nav"><a href="/">← Home</a><a href="/about">About</a></div>
        <div class="badge">GET /users</div>
        <h1>👥 All Users</h1>
        <p class="sub">Contoh dynamic routing — klik ID untuk detail</p>
        <table>
            <thead>
                <tr><th>#</th><th>Name</th><th>Email</th><th>Role</th></tr>
            </thead>
            <tbody>
                <tr>
                    <td><a href="/users/1">1</a></td>
                    <td>Slamet Sugandi</td>
                    <td>slamet@lumina.rs</td>
                    <td>Admin</td>
                </tr>
                <tr>
                    <td><a href="/users/2">2</a></td>
                    <td>Alice Rust</td>
                    <td>alice@lumina.rs</td>
                    <td>Developer</td>
                </tr>
                <tr>
                    <td><a href="/users/3">3</a></td>
                    <td>Bob Axum</td>
                    <td>bob@lumina.rs</td>
                    <td>Designer</td>
                </tr>
            </tbody>
        </table>
    </div>
</body>
</html>"#;
        Html(html.to_string())
    }

    /// GET /users/:id — Tampilkan detail user berdasarkan ID (dynamic routing)
    pub async fn show(Path(id): Path<u32>) -> Html<String> {
        let (name, email, role) = match id {
            1 => ("Slamet Sugandi", "slamet@lumina.rs", "Admin"),
            2 => ("Alice Rust",    "alice@lumina.rs",  "Developer"),
            3 => ("Bob Axum",      "bob@lumina.rs",    "Designer"),
            _ => ("Unknown User",  "unknown@lumina.rs", "Guest"),
        };

        let html = format!(r#"<!DOCTYPE html>
<html lang="id">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>User #{id} — Lumina</title>
    <style>
        * {{ margin: 0; padding: 0; box-sizing: border-box; }}
        body {{
            font-family: 'Segoe UI', sans-serif;
            background: linear-gradient(135deg, #0f0c29, #302b63, #24243e);
            min-height: 100vh; display: flex;
            align-items: center; justify-content: center; color: white;
        }}
        .card {{
            padding: 50px 60px; text-align: center;
            background: rgba(255,255,255,0.05);
            border: 1px solid rgba(255,255,255,0.1);
            border-radius: 20px; backdrop-filter: blur(20px);
            box-shadow: 0 25px 50px rgba(0,0,0,0.5);
        }}
        .badge {{
            display: inline-block;
            background: rgba(99,102,241,0.3);
            border: 1px solid rgba(99,102,241,0.5);
            padding: 3px 14px; border-radius: 999px;
            font-size: 0.78rem; color: #a5b4fc; margin-bottom: 20px;
        }}
        .avatar {{
            width: 80px; height: 80px; border-radius: 50%;
            background: linear-gradient(135deg, #6366f1, #a78bfa);
            display: flex; align-items: center; justify-content: center;
            font-size: 2rem; margin: 0 auto 20px;
        }}
        h1 {{ font-size: 1.8rem; margin-bottom: 8px; }}
        .info {{ color: rgba(255,255,255,0.5); margin-bottom: 6px; font-size: 0.9rem; }}
        .role {{
            display: inline-block; margin-top: 16px;
            background: rgba(16,185,129,0.2); border: 1px solid rgba(16,185,129,0.4);
            padding: 4px 16px; border-radius: 999px; color: #6ee7b7; font-size: 0.82rem;
        }}
        .nav {{ margin-top: 30px; }}
        .nav a {{ color: #818cf8; text-decoration: none; margin: 0 12px; font-size: 0.9rem; }}
        .nav a:hover {{ color: white; }}
        .route-info {{
            margin-top: 24px; padding: 12px 20px;
            background: rgba(0,0,0,0.3); border-radius: 8px;
            font-family: monospace; font-size: 0.82rem; color: #94a3b8;
        }}
    </style>
</head>
<body>
    <div class="card">
        <div class="badge">GET /users/{id}</div>
        <div class="avatar">👤</div>
        <h1>{name}</h1>
        <p class="info">{email}</p>
        <span class="role">{role}</span>
        <div class="route-info">
            🛣️ Path param: <strong style="color:#a5b4fc">id = {id}</strong>
        </div>
        <div class="nav">
            <a href="/users">← All Users</a>
            <a href="/api/users/{id}">JSON →</a>
        </div>
    </div>
</body>
</html>"#);
        Html(html)
    }
}
