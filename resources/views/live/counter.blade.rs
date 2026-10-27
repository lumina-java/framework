<div class="p-6 bg-white rounded-2xl shadow-xl border border-slate-100 max-w-md mx-auto my-6 text-center">
    <div class="mb-4">
        <span class="inline-flex items-center px-3 py-1 rounded-full text-xs font-semibold bg-emerald-100 text-emerald-800">
            ⚡ Lumina Live (Tri-Dialect Reactive UI)
        </span>
    </div>

    <!-- State Tampilan Reaktif -->
    <div class="my-6">
        <span class="text-6xl font-black text-slate-800 tracking-tight">{{ $count }}</span>
    </div>

    <!-- Tombol Interaksi -->
    <div class="flex items-center justify-center space-x-3">
        <!-- Menggunakan Dialek Indonesia: kilat:klik -->
        <button kilat:klik="kurang" class="w-12 h-12 flex items-center justify-center bg-rose-500 hover:bg-rose-600 active:scale-95 text-white text-2xl font-bold rounded-xl shadow-md transition">
            -
        </button>

        <!-- Menggunakan Dialek Universal: live:click dengan konfirmasi kilat:yakin -->
        <button live:click="reset" kilat:yakin="Apakah Anda yakin ingin mereset hitungan?" class="px-4 h-12 bg-slate-100 hover:bg-slate-200 text-slate-700 text-sm font-semibold rounded-xl transition">
            Reset
        </button>

        <!-- Menggunakan Dialek Indonesia: kilat:klik -->
        <button kilat:klik="tambah" class="w-12 h-12 flex items-center justify-center bg-emerald-500 hover:bg-emerald-600 active:scale-95 text-white text-2xl font-bold rounded-xl shadow-md transition">
            +
        </button>
    </div>

    <!-- Two-way Data Binding: kilat:model -->
    <div class="mt-6 pt-4 border-t border-slate-100 flex items-center justify-center space-x-2 text-sm text-slate-600">
        <label for="step">Kelipatan (Step):</label>
        <input id="step" type="number" min="1" max="10" kilat:model="step" value="{{ $step }}" class="w-16 px-2 py-1 text-center border border-slate-200 rounded-lg focus:outline-none focus:ring-2 focus:ring-emerald-500 font-bold" />
    </div>

    <!-- Indikator Loading: kilat:tunggu -->
    <div class="mt-3 min-h-[20px]">
        <span kilat:tunggu class="text-xs font-medium text-slate-400 animate-pulse">
            ⚡ Sinkronisasi state ke server Rust...
        </span>
    </div>
</div>
