<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>500 — Server Error · Lumina</title>
    <link rel="preconnect" href="https://fonts.googleapis.com">
    <link href="https://fonts.googleapis.com/css2?family=Inter:wght@300;400;500;600;700&family=JetBrains+Mono:wght@400;500&display=swap" rel="stylesheet">
    <style>
        *, *::before, *::after { box-sizing: border-box; margin: 0; padding: 0; }

        :root {
            --bg:          #0d1117;
            --surface:     #161b22;
            --surface2:    #1c2128;
            --border:      #30363d;
            --orange:      #e3b341;
            --orange-glow: rgba(227,179,65,0.12);
            --red:         #f85149;
            --text:        #e6edf3;
            --muted:       #8b949e;
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
            overflow: hidden;
        }

        body::before {
            content: '';
            position: fixed;
            inset: 0;
            background-image:
                linear-gradient(rgba(227,179,65,0.03) 1px, transparent 1px),
                linear-gradient(90deg, rgba(227,179,65,0.03) 1px, transparent 1px);
            background-size: 48px 48px;
            pointer-events: none;
        }

        /* Animated warning stripes top bar */
        .stripe-bar {
            position: fixed;
            top: 0; left: 0; right: 0;
            height: 4px;
            background: repeating-linear-gradient(
                -45deg,
                var(--orange) 0px,
                var(--orange) 10px,
                #1c1a10 10px,
                #1c1a10 20px
            );
            background-size: 200% 100%;
            animation: stripeScroll 1.5s linear infinite;
        }

        @keyframes stripeScroll {
            from { background-position: 0 0; }
            to   { background-position: 28px 0; }
        }

        .card {
            position: relative;
            background: var(--surface);
            border: 1px solid var(--border);
            border-radius: 16px;
            padding: 3rem;
            max-width: 660px;
            width: 100%;
            box-shadow:
                0 0 0 1px rgba(227,179,65,0.08),
                0 8px 40px rgba(0,0,0,0.4);
            animation: slideUp 0.6s cubic-bezier(0.16, 1, 0.3, 1) forwards;
        }

        @keyframes slideUp {
            from { opacity: 0; transform: translateY(24px); }
            to   { opacity: 1; transform: translateY(0); }
        }

        .header {
            display: flex;
            align-items: flex-start;
            gap: 1.25rem;
            margin-bottom: 1.75rem;
        }

        .icon-wrap {
            flex-shrink: 0;
            width: 64px;
            height: 64px;
            background: var(--orange-glow);
            border: 1px solid rgba(227,179,65,0.3);
            border-radius: 14px;
            display: flex;
            align-items: center;
            justify-content: center;
            animation: iconShake 3s ease-in-out infinite;
        }

        @keyframes iconShake {
            0%, 80%, 100% { transform: rotate(0deg); }
            85%            { transform: rotate(-4deg); }
            90%            { transform: rotate(4deg); }
            95%            { transform: rotate(-2deg); }
        }

        .icon-wrap svg {
            width: 32px;
            height: 32px;
            color: var(--orange);
        }

        .header-text .code-badge {
            display: inline-block;
            font-family: 'JetBrains Mono', monospace;
            font-size: 0.7rem;
            font-weight: 600;
            letter-spacing: 0.1em;
            background: var(--orange-glow);
            border: 1px solid rgba(227,179,65,0.3);
            color: var(--orange);
            padding: 2px 8px;
            border-radius: 99px;
            margin-bottom: 0.5rem;
        }

        h1 {
            font-size: 1.6rem;
            font-weight: 700;
            margin-bottom: 0.25rem;
        }

        .sub {
            color: var(--muted);
            font-size: 0.9rem;
        }

        /* Error trace box */
        .trace-box {
            background: #0d1117;
            border: 1px solid var(--border);
            border-left: 3px solid var(--orange);
            border-radius: 8px;
            padding: 1rem 1.25rem;
            margin-bottom: 1.75rem;
            overflow-x: auto;
        }

        .trace-box .trace-label {
            display: flex;
            align-items: center;
            justify-content: space-between;
            margin-bottom: 0.75rem;
        }

        .trace-box .trace-label span {
            font-size: 0.7rem;
            text-transform: uppercase;
            letter-spacing: 0.1em;
            color: var(--muted);
            font-weight: 600;
        }

        .trace-box .trace-label .status {
            font-family: 'JetBrains Mono', monospace;
            font-size: 0.72rem;
            color: var(--orange);
            background: var(--orange-glow);
            padding: 2px 8px;
            border-radius: 4px;
        }

        .trace-box pre {
            font-family: 'JetBrains Mono', monospace;
            font-size: 0.8rem;
            color: #cdd9e5;
            line-height: 1.6;
            white-space: pre-wrap;
            word-break: break-word;
        }

        /* Info grid */
        .info-grid {
            display: grid;
            grid-template-columns: repeat(2, 1fr);
            gap: 0.75rem;
            margin-bottom: 1.75rem;
        }

        .info-item {
            background: var(--surface2);
            border: 1px solid var(--border);
            border-radius: 8px;
            padding: 0.75rem 1rem;
        }

        .info-item .info-label {
            font-size: 0.7rem;
            text-transform: uppercase;
            letter-spacing: 0.08em;
            color: var(--muted);
            margin-bottom: 0.25rem;
        }

        .info-item .info-value {
            font-family: 'JetBrains Mono', monospace;
            font-size: 0.82rem;
            color: var(--text);
        }

        /* Actions */
        .actions {
            display: flex;
            gap: 0.75rem;
            flex-wrap: wrap;
            margin-bottom: 1.5rem;
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

        .btn-warning {
            background: var(--orange);
            color: #0d1117;
        }

        .btn-warning:hover {
            transform: translateY(-2px);
            box-shadow: 0 6px 16px rgba(227,179,65,0.3);
        }

        .btn-ghost {
            background: var(--surface2);
            color: var(--text);
            border: 1px solid var(--border);
        }

        .btn-ghost:hover {
            border-color: var(--orange);
            transform: translateY(-2px);
        }

        .footer-row {
            padding-top: 1.25rem;
            border-top: 1px solid var(--border);
            display: flex;
            align-items: center;
            justify-content: space-between;
            flex-wrap: wrap;
            gap: 0.75rem;
        }

        .brand {
            font-size: 0.8rem;
            color: var(--muted);
        }

        .brand strong { color: var(--text); }

        .status-pill {
            display: inline-flex;
            align-items: center;
            gap: 6px;
            font-size: 0.75rem;
            color: var(--orange);
            background: var(--orange-glow);
            border: 1px solid rgba(227,179,65,0.25);
            padding: 3px 10px;
            border-radius: 99px;
        }

        .status-pill .dot {
            width: 6px;
            height: 6px;
            background: var(--orange);
            border-radius: 50%;
            animation: blink 1s step-start infinite;
        }

        @keyframes blink {
            0%, 100% { opacity: 1; }
            50%       { opacity: 0; }
        }

        @media (max-width: 520px) {
            .card { padding: 1.75rem; }
            .info-grid { grid-template-columns: 1fr; }
            .header { flex-direction: column; }
        }
    </style>
</head>
<body>
    <div class="stripe-bar"></div>

    <div class="card">
        <!-- Header -->
        <div class="header">
            <div class="icon-wrap">
                <svg fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="1.5">
                    <path stroke-linecap="round" stroke-linejoin="round"
                        d="M12 9v3.75m-9.303 3.376c-.866 1.5.217 3.374 1.948 3.374h14.71c1.73 0 2.813-1.874 1.948-3.374L13.949 3.378c-.866-1.5-3.032-1.5-3.898 0L2.697 16.126ZM12 15.75h.007v.008H12v-.008Z"/>
                </svg>
            </div>
            <div class="header-text">
                <div class="code-badge">HTTP 500 · Internal Server Error</div>
                <h1>Terjadi Kesalahan Server</h1>
                <p class="sub">Server mengalami error yang tidak terduga. Tim kami telah diberitahu.</p>
            </div>
        </div>

        <!-- Error trace -->
        <div class="trace-box">
            <div class="trace-label">
                <span>Error Detail</span>
                <span class="status">500 Internal Server Error</span>
            </div>
            <pre>{{ error_message }}</pre>
        </div>

        <!-- Info grid -->
        <div class="info-grid">
            <div class="info-item">
                <div class="info-label">Request Path</div>
                <div class="info-value">{{ path }}</div>
            </div>
            <div class="info-item">
                <div class="info-label">Error Code</div>
                <div class="info-value">{{ error_code | default(value="INTERNAL_ERROR") }}</div>
            </div>
            <div class="info-item">
                <div class="info-label">Timestamp</div>
                <div class="info-value">{{ timestamp }}</div>
            </div>
            <div class="info-item">
                <div class="info-label">Request ID</div>
                <div class="info-value">{{ request_id | default(value="—") }}</div>
            </div>
        </div>

        <!-- Actions -->
        <div class="actions">
            <button class="btn btn-warning" onclick="window.location.reload()">
                <svg width="16" height="16" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                    <path stroke-linecap="round" stroke-linejoin="round"
                        d="M16.023 9.348h4.992v-.001M2.985 19.644v-4.992m0 0h4.992m-4.993 0 3.181 3.183a8.25 8.25 0 0 0 13.803-3.7M4.031 9.865a8.25 8.25 0 0 1 13.803-3.7l3.181 3.182m0-4.991v4.99"/>
                </svg>
                Coba Lagi
            </button>
            <a class="btn btn-ghost" href="/">
                <svg width="16" height="16" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                    <path stroke-linecap="round" stroke-linejoin="round"
                        d="m3 9 9-7 9 7v11a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z"/>
                    <polyline stroke-linecap="round" stroke-linejoin="round" points="9 22 9 12 15 12 15 22"/>
                </svg>
                Kembali ke Beranda
            </a>
        </div>

        <!-- Footer -->
        <div class="footer-row">
            <span class="brand">✨ <strong>Lumina Framework</strong> — Built with Rust 🦀</span>
            <span class="status-pill">
                <span class="dot"></span>
                HTTP Active · Error Occurred
            </span>
        </div>
    </div>
</body>
</html>
