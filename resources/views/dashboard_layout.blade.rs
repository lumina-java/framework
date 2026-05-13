<!DOCTYPE html>
<html lang="id">

<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{{ title | default(value="Dashboard — Lumina") }}</title>
    
    <!-- Bootstrap 5 Fallback -->
    <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/bootstrap@5.3.0/dist/css/bootstrap.min.css">
    <!-- Tailwind CSS (Play CDN) -->
    <script src="https://cdn.tailwindcss.com"></script>
    
    <link href="https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&display=swap" rel="stylesheet">
    <script src="https://unpkg.com/htmx.org@1.9.10"></script>
    <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/toastr.js/latest/toastr.min.css">
    
    <style>
        body {
            font-family: 'Inter', sans-serif;
            background-color: #f3f4f6;
            margin: 0;
            padding: 0;
        }
        
        .sidebar-premium {
            width: 260px;
            background-color: #0f172a; /* slate-900 */
            color: white;
            height: 100vh;
            position: fixed;
            left: 0;
            top: 0;
            display: flex;
            flex-direction: column;
            z-index: 50;
            box-shadow: 4px 0 10px rgba(0,0,0,0.1);
        }

        .main-content {
            margin-left: 260px;
            min-height: 100vh;
            display: flex;
            flex-direction: column;
        }

        .nav-link-premium {
            display: flex;
            align-items: center;
            padding: 0.75rem 1rem;
            color: #94a3b8; /* slate-400 */
            text-decoration: none;
            border-radius: 0.5rem;
            transition: all 0.2s;
            margin-bottom: 0.5rem;
        }

        .nav-link-premium:hover {
            background-color: #1e293b; /* slate-800 */
            color: white;
        }

        .nav-link-premium.active {
            background-color: #2563eb; /* blue-600 */
            color: white;
            box-shadow: 0 4px 6px -1px rgba(37, 99, 235, 0.2);
        }

        .lumina-logo-text {
            font-weight: 800;
            font-size: 1.25rem;
            padding: 1.5rem;
            border-bottom: 1px solid #1e293b;
            display: flex;
            align-items: center;
            gap: 0.75rem;
        }

        /* SVG Sizing Fallback */
        svg {
            width: 20px;
            height: 20px;
        }
    </style>
</head>

<body hx-boost="true">

    <!-- Sidebar -->
    <aside class="sidebar-premium">
        <div class="lumina-logo-text">
            <span style="color: #60a5fa;">⚡</span> Lumina
        </div>
        
        <nav class="flex-grow-1 px-3 py-4">
            <a href="/dashboard" class="nav-link-premium active">
                <svg fill="none" stroke="currentColor" viewBox="0 0 24 24" class="me-3">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6"></path>
                </svg>
                Dashboard
            </a>
            
            <a href="/lumina/telescope" class="nav-link-premium">
                <svg fill="none" stroke="currentColor" viewBox="0 0 24 24" class="me-3">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268 2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z" />
                </svg>
                Telescope
            </a>
            
            <a href="/test-echo" class="nav-link-premium">
                <svg fill="none" stroke="currentColor" viewBox="0 0 24 24" class="me-3">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8.111 16.404a5.5 5.5 0 017.778 0M12 20h.01m-7.08-7.071c3.904-3.905 10.236-3.905 14.141 0M1.394 9.393c5.857-5.857 15.355-5.857 21.213 0" />
                </svg>
                Echo Test
            </a>
        </nav>

        <div class="px-3 py-3">
            <div style="background: rgba(37, 99, 235, 0.1); border: 1px solid rgba(37, 99, 235, 0.2); border-radius: 0.75rem; padding: 0.75rem;">
                <p style="font-size: 10px; text-transform: uppercase; font-weight: 800; color: #60a5fa; margin-bottom: 0.25rem;">System Health</p>
                <div style="display: flex; align-items: center; font-size: 11px; color: #bfdbfe;">
                    <span style="width: 8px; height: 8px; background: #22c55e; border-radius: 50%; margin-right: 0.5rem; display: inline-block;"></span>
                    Database Connected
                </div>
            </div>
        </div>

        <div class="p-3 border-top border-secondary border-opacity-10">
            <a href="/auth/logout" class="nav-link-premium text-danger">
                <svg fill="none" stroke="currentColor" viewBox="0 0 24 24" class="me-3">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 16l4-4m0 0l-4-4m4 4H7m6 4v1a3 3 0 01-3 3H6a3 3 0 01-3-3V7a3 3 0 013-3h4a3 3 0 013 3v1"></path>
                </svg>
                Logout
            </a>
        </div>
    </aside>

    <!-- Main Content -->
    <main class="main-content">
        <!-- Top Header -->
        <header class="navbar navbar-light bg-white border-bottom px-4 shadow-sm" style="height: 64px;">
            <h1 class="h5 mb-0 fw-bold text-dark">{{ title | default(value="Dashboard — Lumina") }}</h1>
            <div class="d-flex align-items-center">
                <div class="rounded-circle bg-primary bg-opacity-10 text-primary d-flex align-items-center justify-content-center fw-bold me-2" style="width: 32px; height: 32px; font-size: 14px;">
                    {{ email | default(value="U") | truncate(length=1, end="") | upper }}
                </div>
                <span class="small fw-medium text-muted">{{ email }}</span>
            </div>
        </header>

        <!-- Dashboard Content -->
        <div id="dashboard-content" class="p-4 overflow-auto">
            @yield('content')
        </div>
    </main>

    <script src="https://code.jquery.com/jquery-3.6.0.min.js"></script>
    <script src="https://cdnjs.cloudflare.com/ajax/libs/toastr.js/latest/toastr.min.js"></script>
    <script>
        toastr.options = { "closeButton": true, "progressBar": true, "positionClass": "toast-top-right", "timeOut": "5000" };
        @if(flashes)
            @foreach(flashes as flash)
                toastr.{{ flash.kind }}("{{ flash.message }}");
            @endforeach
        @endif
    </script>
</body>

</html>