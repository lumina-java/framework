<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Database Error — Lumina Framework</title>
    <link rel="preconnect" href="https://fonts.googleapis.com">
    <link href="https://fonts.googleapis.com/css2?family=Inter:wght@300;400;500;600;700&family=JetBrains+Mono:wght@400;500&display=swap" rel="stylesheet">
    <style>
        *, *::before, *::after { box-sizing: border-box; margin: 0; padding: 0; }

        :root {
            --bg:        #0d1117;
            --surface:   #161b22;
            --surface2:  #1c2128;
            --border:    #30363d;
            --red:       #f85149;
            --red-glow:  rgba(248, 81, 73, 0.15);
            --orange:    #e3b341;
            --blue:      #58a6ff;
            --text:      #e6edf3;
            --muted:     #8b949e;
            --code-bg:   #161b22;
        }

        body {
            background: var(--bg);
            color: var(--text);
            font-family: 'Inter', sans-serif;
            min-height: 100vh;
            display: flex;
            flex-direction: column;
            align-items: center;
            justify-content: center;
            padding: 2rem;
            overflow-x: hidden;
        }

        /* Animated background grid */
        body::before {
            content: '';
            position: fixed;
            inset: 0;
            background-image:
                linear-gradient(rgba(248,81,73,0.03) 1px, transparent 1px),
                linear-gradient(90deg, rgba(248,81,73,0.03) 1px, transparent 1px);
            background-size: 60px 60px;
            animation: gridPulse 8s ease-in-out infinite alternate;
            pointer-events: none;
        }

        @keyframes gridPulse {
            from { opacity: 0.4; }
            to   { opacity: 1; }
        }

        /* Glow orb */
        body::after {
            content: '';
            position: fixed;
            top: -200px;
            left: 50%;
            transform: translateX(-50%);
            width: 600px;
            height: 600px;
            background: radial-gradient(circle, rgba(248,81,73,0.08) 0%, transparent 70%);
            pointer-events: none;
            animation: orbFloat 6s ease-in-out infinite alternate;
        }

        @keyframes orbFloat {
            from { transform: translateX(-50%) translateY(0); }
            to   { transform: translateX(-50%) translateY(30px); }
        }

        .card {
            position: relative;
            background: var(--surface);
            border: 1px solid var(--border);
            border-radius: 16px;
            padding: 3rem;
            max-width: 680px;
            width: 100%;
            box-shadow:
                0 0 0 1px rgba(248,81,73,0.1),
                0 8px 32px rgba(0,0,0,0.4),
                inset 0 1px 0 rgba(255,255,255,0.04);
            animation: slideUp 0.6s cubic-bezier(0.16, 1, 0.3, 1) forwards;
        }

        @keyframes slideUp {
            from { opacity: 0; transform: translateY(24px) scale(0.97); }
            to   { opacity: 1; transform: translateY(0) scale(1); }
        }

        /* Top red accent bar */
        .card::before {
            content: '';
            position: absolute;
            top: 0; left: 0; right: 0;
            height: 3px;
            background: linear-gradient(90deg, transparent, var(--red), transparent);
            border-radius: 16px 16px 0 0;
        }

        .icon-wrap {
            display: flex;
            align-items: center;
            justify-content: center;
            width: 72px;
            height: 72px;
            background: var(--red-glow);
            border: 1px solid rgba(248,81,73,0.3);
            border-radius: 18px;
            margin-bottom: 1.5rem;
            animation: iconPulse 2s ease-in-out infinite;
        }

        @keyframes iconPulse {
            0%, 100% { box-shadow: 0 0 0 0 rgba(248,81,73,0.3); }
            50%       { box-shadow: 0 0 0 8px rgba(248,81,73,0); }
        }

        .icon-wrap svg {
            width: 36px;
            height: 36px;
            color: var(--red);
        }

        .badge {
            display: inline-flex;
            align-items: center;
            gap: 6px;
            background: var(--red-glow);
            border: 1px solid rgba(248,81,73,0.3);
            color: var(--red);
            font-size: 0.7rem;
            font-weight: 600;
            letter-spacing: 0.08em;
            text-transform: uppercase;
            padding: 4px 10px;
            border-radius: 99px;
            margin-bottom: 1rem;
        }

        .badge .dot {
            width: 6px;
            height: 6px;
            background: var(--red);
            border-radius: 50%;
            animation: blink 1.2s step-start infinite;
        }

        @keyframes blink {
            0%, 100% { opacity: 1; }
            50%       { opacity: 0; }
        }

        h1 {
            font-size: 2rem;
            font-weight: 700;
            color: var(--text);
            margin-bottom: 0.5rem;
            line-height: 1.2;
        }

        h1 span { color: var(--red); }

        .subtitle {
            color: var(--muted);
            font-size: 0.95rem;
            line-height: 1.6;
            margin-bottom: 2rem;
        }

        /* Error detail box */
        .error-box {
            background: var(--code-bg);
            border: 1px solid var(--border);
            border-left: 3px solid var(--red);
            border-radius: 8px;
            padding: 1rem 1.25rem;
            margin-bottom: 1.75rem;
        }

        .error-box .label {
            font-size: 0.7rem;
            text-transform: uppercase;
            letter-spacing: 0.1em;
            color: var(--muted);
            margin-bottom: 0.5rem;
            font-weight: 600;
        }

        .error-box .message {
            font-family: 'JetBrains Mono', monospace;
            font-size: 0.82rem;
            color: var(--red);
            word-break: break-all;
            line-height: 1.5;
        }

        /* Checklist */
        .checklist {
            list-style: none;
            margin-bottom: 2rem;
        }

        .checklist li {
            display: flex;
            align-items: flex-start;
            gap: 0.75rem;
            padding: 0.6rem 0;
            border-bottom: 1px solid var(--border);
            font-size: 0.88rem;
            color: var(--muted);
            line-height: 1.5;
        }

        .checklist li:last-child { border-bottom: none; }

        .checklist li .num {
            flex-shrink: 0;
            width: 22px;
            height: 22px;
            background: var(--surface2);
            border: 1px solid var(--border);
            border-radius: 50%;
            display: flex;
            align-items: center;
            justify-content: center;
            font-size: 0.7rem;
            font-weight: 600;
            color: var(--orange);
            margin-top: 1px;
        }

        .checklist code {
            font-family: 'JetBrains Mono', monospace;
            font-size: 0.78rem;
            background: var(--surface2);
            border: 1px solid var(--border);
            padding: 1px 6px;
            border-radius: 4px;
            color: var(--blue);
        }

        /* Actions */
        .actions {
            display: flex;
            gap: 0.75rem;
            flex-wrap: wrap;
        }

        .btn {
            display: inline-flex;
            align-items: center;
            gap: 8px;
            padding: 0.6rem 1.25rem;
            border-radius: 8px;
            font-size: 0.88rem;
            font-weight: 500;
            font-family: 'Inter', sans-serif;
            cursor: pointer;
            text-decoration: none;
            border: none;
            transition: all 0.2s ease;
        }

        .btn-primary {
            background: var(--red);
            color: #fff;
        }

        .btn-primary:hover {
            background: #ff6b65;
            transform: translateY(-1px);
            box-shadow: 0 4px 12px rgba(248,81,73,0.35);
        }

        .btn-secondary {
            background: var(--surface2);
            color: var(--text);
            border: 1px solid var(--border);
        }

        .btn-secondary:hover {
            background: var(--border);
            transform: translateY(-1px);
        }

        .footer-note {
            margin-top: 2rem;
            padding-top: 1.25rem;
            border-top: 1px solid var(--border);
            display: flex;
            align-items: center;
            justify-content: space-between;
            gap: 1rem;
            flex-wrap: wrap;
        }

        .footer-note .brand {
            font-size: 0.8rem;
            color: var(--muted);
        }

        .footer-note .brand strong {
            color: var(--text);
        }

        .status-pill {
            display: inline-flex;
            align-items: center;
            gap: 6px;
            font-size: 0.75rem;
            color: var(--orange);
            background: rgba(227,179,65,0.1);
            border: 1px solid rgba(227,179,65,0.25);
            padding: 3px 10px;
            border-radius: 99px;
        }

        .status-pill .dot {
            width: 6px;
            height: 6px;
            background: var(--orange);
            border-radius: 50%;
        }

        @media (max-width: 480px) {
            .card { padding: 2rem 1.5rem; }
            h1 { font-size: 1.5rem; }
        }
    </style>
</head>
<body>
    <div class="card">
        <!-- Icon -->
        <div class="icon-wrap">
            <svg fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="1.5">
                <path stroke-linecap="round" stroke-linejoin="round"
                    d="M12 9v3.75m-9.303 3.376c-.866 1.5.217 3.374 1.948 3.374h14.71c1.73 0 2.813-1.874 1.948-3.374L13.949 3.378c-.866-1.5-3.032-1.5-3.898 0L2.697 16.126ZM12 15.75h.007v.008H12v-.008Z"/>
            </svg>
        </div>

        <!-- Badge -->
        <div class="badge">
            <span class="dot"></span>
            Database Unreachable
        </div>

        <!-- Heading -->
        <h1>Whoops! <span>Something went wrong.</span></h1>
        <p class="subtitle">
            Server Lumina tetap berjalan, namun koneksi ke database gagal.
            Semua fitur yang membutuhkan database sementara tidak tersedia.
        </p>

        <!-- Error detail -->
        <div class="error-box">
            <div class="label">Error Detail</div>
            <div class="message">{{ db_error }}</div>
        </div>

        <!-- Checklist -->
        <ul class="checklist">
            <li>
                <span class="num">1</span>
                <span>Pastikan MySQL/PostgreSQL/SQLite sudah berjalan dan dapat diakses.</span>
            </li>
            <li>
                <span class="num">2</span>
                <span>Periksa konfigurasi <code>DATABASE_URL</code> di file <code>.env</code> Anda.</span>
            </li>
            <li>
                <span class="num">3</span>
                <span>Jika menggunakan MySQL, buat database terlebih dahulu: <code>CREATE DATABASE db_lumina;</code></span>
            </li>
            <li>
                <span class="num">4</span>
                <span>Verifikasi username, password, host, dan port sudah benar.</span>
            </li>
            <li>
                <span class="num">5</span>
                <span>Setelah memperbaiki konfigurasi, restart server dengan <code>./lumina serve</code>.</span>
            </li>
        </ul>

        <!-- Actions -->
        <div class="actions">
            <button class="btn btn-primary" onclick="window.location.reload()">
                <svg width="16" height="16" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                    <path stroke-linecap="round" stroke-linejoin="round" d="M16.023 9.348h4.992v-.001M2.985 19.644v-4.992m0 0h4.992m-4.993 0 3.181 3.183a8.25 8.25 0 0 0 13.803-3.7M4.031 9.865a8.25 8.25 0 0 1 13.803-3.7l3.181 3.182m0-4.991v4.99"/>
                </svg>
                Coba Lagi
            </button>
            <a class="btn btn-secondary" href="https://lumina-framework.dev/docs/database" target="_blank" rel="noopener">
                <svg width="16" height="16" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                    <path stroke-linecap="round" stroke-linejoin="round" d="M12 6.042A8.967 8.967 0 0 0 6 3.75c-1.052 0-2.062.18-3 .512v14.25A8.987 8.987 0 0 1 6 18c2.305 0 4.408.867 6 2.292m0-14.25a8.966 8.966 0 0 1 6-2.292c1.052 0 2.062.18 3 .512v14.25A8.987 8.987 0 0 0 18 18a8.966 8.966 0 0 0-6 2.292m0-14.25v14.25"/>
                </svg>
                Dokumentasi
            </a>
        </div>

        <!-- Footer -->
        <div class="footer-note">
            <span class="brand">✨ <strong>Lumina Framework</strong> — Built with Rust 🦀</span>
            <span class="status-pill">
                <span class="dot"></span>
                HTTP Server Active · DB Offline
            </span>
        </div>
    </div>

    <script>
        // Auto-retry setiap 15 detik
        let countdown = 15;
        const btn = document.querySelector('.btn-primary');
        const originalText = btn.innerHTML;

        const timer = setInterval(() => {
            countdown--;
            if (countdown <= 0) {
                clearInterval(timer);
                window.location.reload();
            } else {
                btn.innerHTML = `
                    <svg width="16" height="16" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M16.023 9.348h4.992v-.001M2.985 19.644v-4.992m0 0h4.992m-4.993 0 3.181 3.183a8.25 8.25 0 0 0 13.803-3.7M4.031 9.865a8.25 8.25 0 0 1 13.803-3.7l3.181 3.182m0-4.991v4.99"/>
                    </svg>
                    Retry otomatis dalam ${countdown}s
                `;
            }
        }, 1000);
    </script>
</body>
</html>
