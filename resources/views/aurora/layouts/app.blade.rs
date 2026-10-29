<!DOCTYPE html>
<html lang="id" class="h-full bg-slate-950 text-slate-100">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>@yield('title', 'Dashboard — Lumina Aurora')</title>
    <script src="https://cdn.tailwindcss.com"></script>
    <link rel="preconnect" href="https://fonts.googleapis.com">
    <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
    <link href="https://fonts.googleapis.com/css2?family=Plus+Jakarta+Sans:wght@300;400;500;600;700;800&display=swap" rel="stylesheet">
    <style>
        body { font-family: 'Plus Jakarta Sans', sans-serif; }
    </style>
</head>
<body class="h-full antialiased flex bg-slate-950 text-slate-100 selection:bg-indigo-500 selection:text-white">
    <!-- Sidebar Desktop -->
    <aside class="hidden md:flex flex-col w-64 bg-slate-900/70 border-r border-slate-800 backdrop-blur-xl shrink-0 justify-between">
        <div>
            <!-- Brand Logo -->
            <div class="h-16 flex items-center px-6 border-b border-slate-800 gap-3">
                <div class="h-9 w-9 rounded-xl bg-gradient-to-tr from-indigo-600 to-violet-400 flex items-center justify-center shadow-md shadow-indigo-500/20">
                    <svg class="w-5 h-5 text-white" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2.5" d="M13 10V3L4 14h7v7l9-11h-7z" />
                    </svg>
                </div>
                <div>
                    <span class="font-extrabold text-lg tracking-tight bg-gradient-to-r from-white to-slate-300 bg-clip-text text-transparent">Lumina</span>
                    <span class="text-[10px] uppercase tracking-widest font-bold px-1.5 py-0.5 rounded bg-indigo-500/20 text-indigo-300 border border-indigo-500/30 ml-1">Aurora</span>
                </div>
            </div>

            <!-- Navigation Links -->
            <nav class="p-4 space-y-1.5 text-sm font-medium">
                <a href="/dashboard" class="flex items-center gap-3 px-3.5 py-2.5 rounded-xl bg-indigo-600/10 text-indigo-400 border border-indigo-500/20 shadow-sm transition-all">
                    <svg class="w-5 h-5 shrink-0" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6" />
                    </svg>
                    <span>Dashboard</span>
                </a>

                <a href="/profile" class="flex items-center gap-3 px-3.5 py-2.5 rounded-xl text-slate-400 hover:text-slate-200 hover:bg-slate-800/60 transition-all">
                    <svg class="w-5 h-5 shrink-0" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z" />
                    </svg>
                    <span>Profil Pengguna</span>
                </a>

                @can('users.manage')
                <div class="pt-4 pb-1 px-3.5 text-[10px] font-bold uppercase tracking-wider text-slate-500">Administrasi</div>
                <a href="/admin/users" class="flex items-center gap-3 px-3.5 py-2.5 rounded-xl text-slate-400 hover:text-slate-200 hover:bg-slate-800/60 transition-all">
                    <svg class="w-5 h-5 shrink-0" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4.354a4 4 0 110 5.292M15 21H3v-1a6 6 0 0112 0v1zm0 0h6v-1a6 6 0 00-9-5.197M13 7a4 4 0 11-8 0 4 4 0 018 0z" />
                    </svg>
                    <span>Kelola Pengguna</span>
                    <span class="ml-auto text-[10px] font-semibold bg-violet-500/20 text-violet-300 border border-violet-500/30 px-1.5 py-0.5 rounded">RBAC</span>
                </a>
                @endcan
            </nav>
        </div>

        <!-- User Card & Logout Bottom -->
        <div class="p-4 border-t border-slate-800/80 bg-slate-900/30">
            <div class="flex items-center gap-3 mb-3">
                <div class="h-9 w-9 rounded-full bg-indigo-500/20 border border-indigo-500/40 text-indigo-300 font-bold flex items-center justify-center shrink-0">
                    {{ $user.name | substring(0, 1) | uppercase }}
                </div>
                <div class="min-w-0 flex-grow">
                    <div class="text-sm font-semibold text-slate-200 truncate">{{ $user.name }}</div>
                    <div class="text-xs text-slate-500 truncate">{{ $user.email }}</div>
                </div>
            </div>

            <form action="/logout" method="POST">
                @csrf
                <button type="submit" class="w-full py-2 px-3 rounded-lg text-xs font-semibold text-slate-400 hover:text-rose-400 hover:bg-rose-500/10 border border-transparent hover:border-rose-500/20 transition-all flex items-center justify-center gap-2">
                    <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 16l4-4m0 0l-4-4m4 4H7m6 4v1a3 3 0 01-3 3H6a3 3 0 01-3-3V7a3 3 0 013-3h4a3 3 0 013 3v1" />
                    </svg>
                    <span>Keluar Akun (Logout)</span>
                </button>
            </form>
        </div>
    </aside>

    <!-- Main Content Area -->
    <div class="flex-grow flex flex-col min-w-0 overflow-y-auto">
        <!-- Topbar -->
        <header class="h-16 border-b border-slate-800 bg-slate-900/40 backdrop-blur-md px-6 flex items-center justify-between sticky top-0 z-20">
            <div class="flex items-center gap-3">
                <h2 class="text-base font-semibold text-slate-200">@yield('page_title', 'Dashboard')</h2>
                <div class="hidden sm:flex items-center gap-1.5 px-2.5 py-1 rounded-full bg-emerald-500/10 text-emerald-400 border border-emerald-500/20 text-xs font-medium">
                    <span class="w-1.5 h-1.5 rounded-full bg-emerald-400 animate-pulse"></span>
                    <span>Zero-Panic Active</span>
                </div>
            </div>

            <div class="flex items-center gap-3">
                <span class="text-xs text-slate-400 hidden sm:inline">Role:</span>
                <span class="text-xs font-semibold px-2 py-0.5 rounded-md bg-indigo-500/20 text-indigo-300 border border-indigo-500/30">
                    {{ $user.role | uppercase }}
                </span>
            </div>
        </header>

        <!-- Flash Message Banners -->
        <div class="p-6 pb-0">
            @if(flash_error)
                <div class="mb-4 p-4 rounded-xl bg-rose-500/10 border border-rose-500/30 text-rose-300 text-sm flex items-start gap-3 shadow-lg shadow-rose-950/40">
                    <svg class="w-5 h-5 text-rose-400 shrink-0 mt-0.5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z" />
                    </svg>
                    <div>{{ $flash_error }}</div>
                </div>
            @endif

            @if(flash_success)
                <div class="mb-4 p-4 rounded-xl bg-emerald-500/10 border border-emerald-500/30 text-emerald-300 text-sm flex items-start gap-3 shadow-lg shadow-emerald-950/40">
                    <svg class="w-5 h-5 text-emerald-400 shrink-0 mt-0.5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" />
                    </svg>
                    <div>{{ $flash_success }}</div>
                </div>
            @endif
        </div>

        <!-- Page Body Content -->
        <main class="p-6 flex-grow">
            @yield('content')
        </main>

        <!-- Footer -->
        <footer class="py-4 px-6 border-t border-slate-800/80 text-xs text-slate-500 flex items-center justify-between">
            <span>&copy; 2026 Lumina Aurora Starter Kit</span>
            <span class="text-slate-400 font-mono">Fast &bull; Beauty &bull; Safety</span>
        </footer>
    </div>
</body>
</html>

