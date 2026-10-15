@if(is_htmx and hx_target == "app-content")
    <div id="app-content">
        @yield('content')
    </div>
@else
<!DOCTYPE html>
<html lang="en">

<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>@yield('title')</title>
    <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/bootstrap@5.3.0/dist/css/bootstrap.min.css">
    <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/toastr.js/latest/toastr.min.css">
    <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/bootstrap-icons@1.11.3/font/bootstrap-icons.min.css">
    <!-- Tailwind CSS (Play CDN for modern aesthetics) -->
    <script src="https://cdn.tailwindcss.com"></script>
    <!-- HTMX for Zero-Mouse UI -->
    <script src="https://unpkg.com/htmx.org@1.9.10"></script>
    <style>
        :root {
            --medical-blue: #448aff;
            --pharmacy-green: #2ed8b6;
            --alert-red: #ff5370;
            --retur-orange: #ffb64d;
        }

        body {
            background-color: #f4f7fa;
            font-family: 'Inter', 'Segoe UI', sans-serif;
            color: #333;
        }

        .text-primary {
            color: var(--medical-blue) !important;
        }

        .btn-primary {
            background-color: var(--medical-blue);
            border-color: var(--medical-blue);
        }

        .btn-success {
            background-color: var(--pharmacy-green);
            border-color: var(--pharmacy-green);
        }

        .navbar {
            background-color: #ffffff;
            border-bottom: 2px solid var(--medical-blue);
            box-shadow: 0 2px 4px rgba(0, 0, 0, 0.05);
        }

        #app-content {
            transition: opacity 0.2s ease-in-out;
        }

        .htmx-request #app-content {
            opacity: 0.5;
        }

        .footer {
            padding: 40px 0;
            color: #6c757d;
            font-size: 0.9rem;
        }
    </style>
</head>

<body hx-boost="true" hx-target="#app-content" hx-select="#app-content" hx-swap="innerHTML show:window:top">
    <nav class="navbar navbar-expand-lg">
        <div class="container">
            <a class="navbar-brand d-flex align-items-center fw-bold text-primary" href="/">
                <img src="/storage/assets/logo.png" alt="Lumina Logo" width="30" height="30" class="d-inline-block align-top me-2">
                LUMINA
            </a>
            <div class="collapse navbar-collapse">
                <ul class="navbar-nav ms-auto">
                    <li class="nav-item"><a class="nav-link" href="/">Home</a></li>
                    <li class="nav-item"><a class="nav-link" href="/about">About</a></li>

                    @guest
                    <li class="nav-item"><a class="nav-link" href="/auth/login">Login</a></li>
                    <li class="nav-item"><a class="nav-link btn btn-sm btn-outline-primary ms-2"
                            href="/auth/register">Register</a></li>
                    @else
                    <li class="nav-item"><a class="nav-link" href="/dashboard">Dashboard</a></li>
                    <li class="nav-item"><a class="nav-link" href="/auth/logout">Logout</a></li>
                    @endguest
                </ul>
            </div>
        </div>
    </nav>

    <div id="app-content" class="container mt-4">
        @yield('content')
    </div>

    <footer class="footer text-center">
        <div class="container">
            <hr>
            <p>&copy; 2026 Lumina Framework — **Premium Medical Edition**. Built with Rust 🦀</p>
        </div>
    </footer>

    <script src="https://code.jquery.com/jquery-3.6.0.min.js"></script>
    <script src="https://cdn.jsdelivr.net/npm/bootstrap@5.3.0/dist/js/bootstrap.bundle.min.js"></script>
    <script src="https://cdnjs.cloudflare.com/ajax/libs/toastr.js/latest/toastr.min.js"></script>
    <script>
        toastr.options = {
            "closeButton": true,
            "progressBar": true,
            "positionClass": "toast-top-right",
            "timeOut": "5000"
        };

        // 1. Flash messages dari server (Session)
        @if(flashes)
            @foreach(flashes as flash)
                toastr.{{ flash.kind }}("{{ flash.message }}");
            @endforeach
        @endif

        // Handle HTMX Errors
        document.body.addEventListener('htmx:responseError', function (evt) {
            toastr.error("Error " + evt.detail.xhr.status + ": " + evt.detail.xhr.statusText);
        });

        document.body.addEventListener('htmx:sendError', function (evt) {
            toastr.error("Network Error: Cek koneksi Anda.");
        });

        // Auto-focus first input on HTMX swap
        document.body.addEventListener('htmx:afterSwap', function(evt) {
            if (evt.detail.target.id === 'app-content') {
                const firstInput = evt.detail.target.querySelector('input:not([type="hidden"]), select, textarea');
                if (firstInput) firstInput.focus();
            }
        });
    </script>
</body>

</html>
@endif