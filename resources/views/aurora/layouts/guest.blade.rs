<!DOCTYPE html>
<html lang="id" class="h-full bg-slate-950 text-slate-100">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>@yield('title', 'Lumina Aurora — Auth')</title>
    <script src="https://cdn.tailwindcss.com"></script>
    <link rel="preconnect" href="https://fonts.googleapis.com">
    <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
    <link href="https://fonts.googleapis.com/css2?family=Plus+Jakarta+Sans:wght@300;400;500;600;700;800&display=swap" rel="stylesheet">
    <style>
        body { font-family: 'Plus Jakarta Sans', sans-serif; }
    </style>
</head>
<body class="h-full antialiased flex flex-col justify-between selection:bg-indigo-500 selection:text-white bg-[radial-gradient(ellipse_80%_80%_at_50%_-20%,rgba(120,119,198,0.2),rgba(255,255,255,0))]">
    <!-- Navbar Minimalis -->
    <header class="py-6 px-6 max-w-7xl mx-auto w-full flex items-center justify-between">
        <a href="/" class="flex items-center gap-3 group">
            <div class="h-10 w-10 rounded-xl bg-gradient-to-tr from-indigo-600 to-violet-400 flex items-center justify-center shadow-lg shadow-indigo-500/30 group-hover:scale-105 transition-transform duration-300">
                <svg class="w-6 h-6 text-white" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2.5" d="M13 10V3L4 14h7v7l9-11h-7z" />
                </svg>
            </div>
            <div>
                <span class="text-xl font-extrabold tracking-tight bg-gradient-to-r from-white via-slate-200 to-slate-400 bg-clip-text text-transparent">Lumina</span>
                <span class="text-xs uppercase tracking-widest px-1.5 py-0.5 rounded-md font-semibold bg-indigo-500/20 text-indigo-300 ml-1 border border-indigo-500/30">Aurora</span>
            </div>
        </a>
        <div class="text-sm text-slate-400">
            <span class="hidden sm:inline">Look like Laravel, Run at Rust Speed.</span>
        </div>
    </header>

    <!-- Konten Utama Card -->
    <main class="flex-grow flex items-center justify-center px-4 py-8">
        <div class="w-full max-w-md">
            @if(flash_error)
                <div class="mb-5 p-4 rounded-xl bg-rose-500/10 border border-rose-500/30 text-rose-300 text-sm flex items-start gap-3 shadow-lg shadow-rose-950/40">
                    <svg class="w-5 h-5 text-rose-400 shrink-0 mt-0.5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z" />
                    </svg>
                    <div>{{ $flash_error }}</div>
                </div>
            @endif

            @if(flash_success)
                <div class="mb-5 p-4 rounded-xl bg-emerald-500/10 border border-emerald-500/30 text-emerald-300 text-sm flex items-start gap-3 shadow-lg shadow-emerald-950/40">
                    <svg class="w-5 h-5 text-emerald-400 shrink-0 mt-0.5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" />
                    </svg>
                    <div>{{ $flash_success }}</div>
                </div>
            @endif

            <div class="bg-slate-900/80 backdrop-blur-xl border border-slate-800 rounded-2xl p-7 shadow-2xl shadow-indigo-950/50">
                @yield('content')
            </div>
        </div>
    </main>

    <!-- Footer -->
    <footer class="py-6 text-center text-xs text-slate-500">
        &copy; 2026 Lumina Framework. Dilindungi oleh Lumina Enterprise Security & Zero-Panic Boundary.
    </footer>
</body>
</html>

