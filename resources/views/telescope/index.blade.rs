<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Lumina Telescope</title>
    <script src="https://cdn.tailwindcss.com"></script>
    <script src="https://unpkg.com/htmx.org@1.9.10"></script>
    <link href="https://fonts.googleapis.com/css2?family=Inter:wght@300;400;500;600;700&family=Fira+Code:wght@400;500&display=swap" rel="stylesheet">
    <style>
        body { font-family: 'Inter', sans-serif; background-color: #0f172a; color: #f1f5f9; }
        .fira { font-family: 'Fira Code', monospace; }
        .tab-active { border-bottom: 2px solid #38bdf8; color: #38bdf8; }
        ::-webkit-scrollbar { width: 8px; }
        ::-webkit-scrollbar-track { background: #1e293b; }
        ::-webkit-scrollbar-thumb { background: #334155; border-radius: 4px; }
        ::-webkit-scrollbar-thumb:hover { background: #475569; }
    </style>
</head>
<body class="min-h-screen">
    <!-- Navbar -->
    <nav class="border-b border-slate-800 bg-slate-900/50 backdrop-blur-md sticky top-0 z-50">
        <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
            <div class="flex justify-between h-16 items-center">
                <div class="flex items-center gap-2">
                    <span class="text-2xl">✨</span>
                    <span class="text-xl font-bold tracking-tight text-white">Lumina <span class="text-sky-400">Telescope</span></span>
                </div>
                <div class="flex items-center gap-4">
                    <span class="text-xs text-slate-400 bg-slate-800 px-2 py-1 rounded">v1.0.0</span>
                    <button onclick="window.location.reload()" class="text-slate-400 hover:text-white">
                        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15"></path></svg>
                    </button>
                </div>
            </div>
        </div>
    </nav>

    <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
        <div class="flex flex-col lg:flex-row gap-8">
            <!-- Sidebar / Tabs -->
            <aside class="w-full lg:w-64 flex-shrink-0">
                <div class="space-y-1">
                    <button hx-get="/lumina/telescope/entries?tag=requests" hx-target="#entries-container" class="w-full text-left px-4 py-2 rounded-lg hover:bg-slate-800 transition-colors {% if active_tag == 'requests' %}bg-slate-800 text-sky-400 font-semibold{% else %}text-slate-400{% endif %}">
                        🚀 Requests
                    </button>
                    <button hx-get="/lumina/telescope/entries?tag=events" hx-target="#entries-container" class="w-full text-left px-4 py-2 rounded-lg hover:bg-slate-800 transition-colors {% if active_tag == 'events' %}bg-slate-800 text-sky-400 font-semibold{% else %}text-slate-400{% endif %}">
                        ⚡ Events
                    </button>
                    <button hx-get="/lumina/telescope/entries?tag=jobs" hx-target="#entries-container" class="w-full text-left px-4 py-2 rounded-lg hover:bg-slate-800 transition-colors {% if active_tag == 'jobs' %}bg-slate-800 text-sky-400 font-semibold{% else %}text-slate-400{% endif %}">
                        📦 Jobs
                    </button>
                    <button hx-get="/lumina/telescope/entries?tag=logs" hx-target="#entries-container" class="w-full text-left px-4 py-2 rounded-lg hover:bg-slate-800 transition-colors {% if active_tag == 'logs' %}bg-slate-800 text-sky-400 font-semibold{% else %}text-slate-400{% endif %}">
                        📝 Logs
                    </button>
                </div>

                <div class="mt-8 pt-8 border-t border-slate-800">
                    <h4 class="text-xs font-semibold text-slate-500 uppercase tracking-wider mb-4 px-4">System Status</h4>
                    <div class="px-4 space-y-3">
                        <div class="flex justify-between items-center text-sm">
                            <span class="text-slate-400">Environment</span>
                            <span class="text-emerald-400 font-medium">Local</span>
                        </div>
                        <div class="flex justify-between items-center text-sm">
                            <span class="text-slate-400">Database</span>
                            <span class="text-sky-400 font-medium">UP</span>
                        </div>
                    </div>
                </div>
            </aside>

            <!-- Main Content -->
            <main class="flex-1">
                <div class="bg-slate-900/50 border border-slate-800 rounded-2xl overflow-hidden shadow-xl" id="entries-container">
                    {% include "telescope/partials/entries.blade.rs" %}
                </div>
            </main>
        </div>
    </div>

    <footer class="mt-20 py-10 border-t border-slate-800 text-center text-slate-500 text-sm">
        <p>Built with ❤️ by Antigravity for Lumina Framework</p>
    </footer>
</body>
</html>
