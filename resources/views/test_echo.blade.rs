@extends('layout')

@section('content')
<div class="max-w-4xl mx-auto py-20 px-4">
    <div class="bg-slate-900 border border-slate-800 rounded-3xl p-10 shadow-2xl">
        <div class="flex items-center justify-between mb-10">
            <div class="flex items-center gap-4">
                <span class="text-4xl animate-pulse">📡</span>
                <h1 class="text-3xl font-bold">Lumina Echo Test</h1>
            </div>
            <div id="status" class="px-4 py-1 bg-red-500/20 text-red-400 rounded-full text-xs font-bold border border-red-500/50">
                Disconnected
            </div>
        </div>

        <div class="grid grid-cols-1 md:grid-cols-2 gap-8">
            <!-- Control Panel -->
            <div class="space-y-6">
                <div class="p-6 bg-slate-800/50 rounded-2xl border border-slate-700">
                    <h3 class="text-slate-400 text-sm font-semibold uppercase mb-4">Trigger Notification</h3>
                    <p class="text-slate-300 text-sm mb-6">Clicking this will send an event to the backend, which will then broadcast it back via WebSocket.</p>
                    <button 
                        hx-post="/test-broadcast" 
                        hx-swap="none"
                        class="w-full py-4 bg-sky-600 hover:bg-sky-500 rounded-xl font-bold transition-all transform hover:scale-[1.02] active:scale-95 shadow-lg shadow-sky-500/20"
                    >
                        Send Real-time Event
                    </button>
                </div>
            </div>

            <!-- Live Feed -->
            <div class="space-y-4">
                <h3 class="text-slate-400 text-sm font-semibold uppercase pl-2">Live Feed</h3>
                <div id="events-log" class="h-64 overflow-y-auto space-y-3 pr-2 scrollbar-hide">
                    <div class="text-slate-500 italic text-sm text-center py-10">Waiting for events...</div>
                </div>
            </div>
        </div>
    </div>
</div>

<script src="/js/echo.js"></script>
<script>
    const statusEl = document.getElementById('status');
    const logEl = document.getElementById('events-log');

    // Update connection status
    window.Echo.socket.addEventListener('open', () => {
        statusEl.innerText = 'Connected';
        statusEl.className = 'px-4 py-1 bg-emerald-500/20 text-emerald-400 rounded-full text-xs font-bold border border-emerald-500/50';
    });

    window.Echo.socket.addEventListener('close', () => {
        statusEl.innerText = 'Disconnected';
        statusEl.className = 'px-4 py-1 bg-red-500/20 text-red-400 rounded-full text-xs font-bold border border-red-500/50';
    });

    // Listen to channel
    window.Echo.channel('notifications')
        .listen('NewNotification', (data) => {
            if (logEl.querySelector('.italic')) logEl.innerHTML = '';
            
            const div = document.createElement('div');
            div.className = 'p-4 bg-slate-800 rounded-xl border-l-4 border-sky-500 animate-slide-in';
            div.innerHTML = `
                <div class="text-xs text-slate-500 mb-1">${new Date().toLocaleTimeString()}</div>
                <div class="text-slate-200 font-medium">${data.message}</div>
            `;
            logEl.prepend(div);
        });
</script>

<style>
    @keyframes slide-in {
        from { opacity: 0; transform: translateX(-20px); }
        to { opacity: 1; transform: translateX(0); }
    }
    .animate-slide-in {
        animation: slide-in 0.3s ease-out forwards;
    }
    .scrollbar-hide::-webkit-scrollbar {
        display: none;
    }
</style>
@endsection
