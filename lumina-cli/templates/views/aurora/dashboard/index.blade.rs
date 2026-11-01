@extends('aurora.layouts.app')

@section('page_title', 'Dashboard Utama')

@section('content')
<div class="space-y-6">
    <!-- Hero Banner -->
    <div class="relative overflow-hidden rounded-2xl bg-gradient-to-r from-indigo-900/60 via-slate-900 to-violet-900/40 border border-indigo-500/20 p-6 md:p-8 shadow-xl">
        <div class="relative z-10 max-w-2xl">
            <span class="inline-flex items-center gap-1.5 px-3 py-1 rounded-full bg-indigo-500/20 text-indigo-300 text-xs font-semibold mb-3 border border-indigo-500/30">
                <span>⚡ Lumina Aurora</span>
                <span>&bull;</span>
                <span>Official Starter Kit</span>
            </span>
            <h1 class="text-2xl md:text-3xl font-extrabold text-white tracking-tight">
                Halo, {{ $user.name }}! 👋
            </h1>
            <p class="text-slate-300 text-sm mt-2 leading-relaxed">
                Aplikasi Anda berjalan pada arsitektur Rust murni dengan kecepatan SSR sub-milidetik, keamanan enterprise bawaan, dan bebas dari kerumitan JavaScript tooling.
            </p>
        </div>
        <div class="absolute -right-10 -bottom-10 w-60 h-60 bg-indigo-500/10 rounded-full blur-3xl pointer-events-none"></div>
    </div>

    <!-- 4 Metrik Kartu (Stats Cards) -->
    <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4">
        <!-- Card 1: SSR Latency -->
        <div class="p-5 rounded-2xl bg-slate-900/70 border border-slate-800 backdrop-blur-sm">
            <div class="flex items-center justify-between text-slate-400 mb-3">
                <span class="text-xs font-semibold uppercase tracking-wider">Performa SSR</span>
                <span class="p-2 rounded-xl bg-indigo-500/10 text-indigo-400">
                    <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z" />
                    </svg>
                </span>
            </div>
            <div class="text-2xl font-bold text-white">&lt; 0.5 ms</div>
            <div class="text-xs text-emerald-400 font-medium mt-1">Super Cepat (Rust Native)</div>
        </div>

        <!-- Card 2: Zero-Panic Safety -->
        <div class="p-5 rounded-2xl bg-slate-900/70 border border-slate-800 backdrop-blur-sm">
            <div class="flex items-center justify-between text-slate-400 mb-3">
                <span class="text-xs font-semibold uppercase tracking-wider">Zero-Panic Boundary</span>
                <span class="p-2 rounded-xl bg-emerald-500/10 text-emerald-400">
                    <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m5.618-4.016A11.955 11.955 0 0112 2.944a11.955 11.955 0 01-8.618 3.04A12.02 12.02 0 003 9c0 5.591 3.824 10.29 9 11.622 5.176-1.332 9-6.03 9-11.622 0-1.042-.133-2.052-.382-3.016z" />
                    </svg>
                </span>
            </div>
            <div class="text-2xl font-bold text-white">100% Aktif</div>
            <div class="text-xs text-slate-400 font-medium mt-1">Server Anti-Crash</div>
        </div>

        <!-- Card 3: Total Pengguna -->
        <div class="p-5 rounded-2xl bg-slate-900/70 border border-slate-800 backdrop-blur-sm">
            <div class="flex items-center justify-between text-slate-400 mb-3">
                <span class="text-xs font-semibold uppercase tracking-wider">Total Pengguna</span>
                <span class="p-2 rounded-xl bg-sky-500/10 text-sky-400">
                    <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4.354a4 4 0 110 5.292M15 21H3v-1a6 6 0 0112 0v1zm0 0h6v-1a6 6 0 00-9-5.197M13 7a4 4 0 11-8 0 4 4 0 018 0z" />
                    </svg>
                </span>
            </div>
            <div class="text-2xl font-bold text-white">{{ total_users }}</div>
            <div class="text-xs text-sky-400 font-medium mt-1">Terdaftar di Database</div>
        </div>

        <!-- Card 4: Audit & Security -->
        <div class="p-5 rounded-2xl bg-slate-900/70 border border-slate-800 backdrop-blur-sm">
            <div class="flex items-center justify-between text-slate-400 mb-3">
                <span class="text-xs font-semibold uppercase tracking-wider">Audit & Brute-Force</span>
                <span class="p-2 rounded-xl bg-violet-500/10 text-violet-400">
                    <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z" />
                    </svg>
                </span>
            </div>
            <div class="text-2xl font-bold text-white">Terkawal</div>
            <div class="text-xs text-violet-400 font-medium mt-1">Moka Lock-Free Cache</div>
        </div>
    </div>

    <!-- Quick Actions & Profile Overview -->
    <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
        <!-- Akun Saya -->
        <div class="lg:col-span-2 p-6 rounded-2xl bg-slate-900/70 border border-slate-800 shadow-sm">
            <h2 class="text-base font-bold text-white mb-4 flex items-center gap-2">
                <span>Informasi Sesi Anda</span>
            </h2>
            <div class="space-y-3 text-sm">
                <div class="flex items-center justify-between py-2.5 border-b border-slate-800/80">
                    <span class="text-slate-400">Nama Pengguna</span>
                    <span class="font-semibold text-slate-200">{{ $user.name }}</span>
                </div>
                <div class="flex items-center justify-between py-2.5 border-b border-slate-800/80">
                    <span class="text-slate-400">Email Akun</span>
                    <span class="font-semibold text-slate-200">{{ $user.email }}</span>
                </div>
                <div class="flex items-center justify-between py-2.5 border-b border-slate-800/80">
                    <span class="text-slate-400">Tingkat Hak Akses (Role)</span>
                    <span class="px-2.5 py-0.5 rounded-full text-xs font-bold uppercase tracking-wider bg-indigo-500/20 text-indigo-300 border border-indigo-500/30">
                        {{ $user.role }}
                    </span>
                </div>
                <div class="flex items-center justify-between py-2.5">
                    <span class="text-slate-400">CSRF & Session Token</span>
                    <span class="text-xs text-emerald-400 font-mono">Tervalidasi & Terproteksi</span>
                </div>
            </div>

            <div class="mt-6 pt-4 border-t border-slate-800 flex gap-3">
                <a href="/profile" class="px-4 py-2 bg-indigo-600 hover:bg-indigo-500 text-white text-xs font-semibold rounded-xl transition-all shadow-md shadow-indigo-600/20">
                    Perbarui Profil Akun &rarr;
                </a>
            </div>
        </div>

        <!-- Panel Aksi Cepat -->
        <div class="p-6 rounded-2xl bg-slate-900/70 border border-slate-800 flex flex-col justify-between shadow-sm">
            <div>
                <h2 class="text-base font-bold text-white mb-2">Aksi Cepat</h2>
                <p class="text-xs text-slate-400 mb-4">Pintasan navigasi penting untuk mengelola akun dan konfigurasi sistem.</p>
                
                <div class="space-y-2">
                    <a href="/profile" class="block w-full p-3 rounded-xl bg-slate-800/40 hover:bg-slate-800 border border-slate-700/50 text-xs font-medium text-slate-200 transition-all">
                        ⚙️ Pengaturan Profil & Kata Sandi
                    </a>
                    <a href="#tabel-pengguna" class="block w-full p-3 rounded-xl bg-indigo-600/10 hover:bg-indigo-600/20 border border-indigo-500/30 text-xs font-medium text-indigo-300 transition-all">
                        👥 Kelola Pengguna & Hak Akses
                    </a>
                    <a href="/" class="block w-full p-3 rounded-xl bg-slate-800/40 hover:bg-slate-800 border border-slate-700/50 text-xs font-medium text-slate-200 transition-all">
                        🏠 Halaman Utama (Welcome Page)
                    </a>
                </div>
            </div>

            <div class="mt-6 pt-4 border-t border-slate-800/80 text-[11px] text-slate-500">
                Server Engine: Lumina v0.5.0 + Tokio 1.x
            </div>
        </div>
    </div>

    <!-- Keterangan & Panduan Hak Akses (Role & Permission Guide) -->
    <div class="p-6 rounded-2xl bg-slate-900/70 border border-slate-800 shadow-sm">
        <div class="flex flex-col md:flex-row md:items-center justify-between gap-2 border-b border-slate-800 pb-4 mb-4">
            <div>
                <h2 class="text-base font-bold text-white flex items-center gap-2">
                    <span>🛡️ Sistem Peran & Hak Akses (Role & Permission Guide)</span>
                </h2>
                <p class="text-xs text-slate-400 mt-1">Penjelasan hak akses dan wewenang akun pengguna di dalam ekosistem Lumina Framework.</p>
            </div>
            <div class="inline-flex items-center gap-2 px-3 py-1 rounded-lg bg-slate-800 text-xs text-slate-300">
                <span>Peran Anda saat ini:</span>
                <span class="font-bold text-indigo-400 uppercase">{{ $user.role }}</span>
            </div>
        </div>

        <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
            <!-- Role 1: Super Admin / Admin -->
            <div class="p-4 rounded-xl bg-slate-950/60 border border-amber-500/20">
                <div class="flex items-center justify-between mb-2">
                    <span class="text-xs font-bold uppercase tracking-wider text-amber-400">👑 Administrator</span>
                    <span class="text-[10px] bg-amber-500/20 text-amber-300 px-2 py-0.5 rounded border border-amber-500/30">Akses Penuh</span>
                </div>
                <p class="text-xs text-slate-300 leading-relaxed mb-3">
                    Memiliki wewenang mutlak atas seluruh sistem. Bypass otomatis seluruh izin (<code class="text-amber-400">*</code>), mengelola user, role, dan konfigurasi database.
                </p>
                <div class="text-[11px] text-slate-500">Izin bawaan: <span class="text-slate-400 font-mono">* (All Permissions)</span></div>
            </div>

            <!-- Role 2: Editor / Operator -->
            <div class="p-4 rounded-xl bg-slate-950/60 border border-sky-500/20">
                <div class="flex items-center justify-between mb-2">
                    <span class="text-xs font-bold uppercase tracking-wider text-sky-400">✍️ Editor / Operator</span>
                    <span class="text-[10px] bg-sky-500/20 text-sky-300 px-2 py-0.5 rounded border border-sky-500/30">Kelola Konten</span>
                </div>
                <p class="text-xs text-slate-300 leading-relaxed mb-3">
                    Dapat membuat, mengedit, dan mempublikasikan data bisnis/artikel. Tidak memiliki akses ke konfigurasi sistem server.
                </p>
                <div class="text-[11px] text-slate-500">Izin bawaan: <span class="text-slate-400 font-mono">content.create, content.edit</span></div>
            </div>

            <!-- Role 3: Member / User -->
            <div class="p-4 rounded-xl bg-slate-950/60 border border-indigo-500/20">
                <div class="flex items-center justify-between mb-2">
                    <span class="text-xs font-bold uppercase tracking-wider text-indigo-400">👤 Member (Pengguna)</span>
                    <span class="text-[10px] bg-indigo-500/20 text-indigo-300 px-2 py-0.5 rounded border border-indigo-500/30">Pengguna Standar</span>
                </div>
                <p class="text-xs text-slate-300 leading-relaxed mb-3">
                    Pengguna terdaftar publik. Dapat mengakses dashboard pribadi, memperbarui profil, dan mengganti kata sandi mandiri.
                </p>
                <div class="text-[11px] text-slate-500">Izin bawaan: <span class="text-slate-400 font-mono">dashboard.view, profile.edit</span></div>
            </div>
        </div>
    </div>

    <!-- Tabel Daftar Pengguna & Manajemen Role -->
    <div id="tabel-pengguna" class="p-6 rounded-2xl bg-slate-900/70 border border-slate-800 shadow-sm overflow-hidden">
        <div class="flex flex-col sm:flex-row sm:items-center justify-between gap-2 border-b border-slate-800 pb-4 mb-4">
            <div>
                <h2 class="text-base font-bold text-white flex items-center gap-2">
                    <span>👥 Kelola Pengguna & Manajemen Peran (RBAC)</span>
                </h2>
                <p class="text-xs text-slate-400 mt-1">Daftar akun yang terdaftar dalam database beserta peran hak akses masing-masing.</p>
            </div>
            <div class="text-xs text-slate-400">
                Total: <span class="font-bold text-white">{{ total_users }}</span> pengguna
            </div>
        </div>

        <div class="overflow-x-auto">
            <table class="w-full text-left text-sm text-slate-300">
                <thead class="text-xs uppercase bg-slate-950/50 text-slate-400 border-b border-slate-800">
                    <tr>
                        <th class="py-3 px-4">Pengguna</th>
                        <th class="py-3 px-4">Email</th>
                        <th class="py-3 px-4">Hak Akses (Role)</th>
                        <th class="py-3 px-4">Status Akun</th>
                    </tr>
                </thead>
                <tbody class="divide-y divide-slate-800/60">
                    @foreach($users_list as $item)
                    <tr class="hover:bg-slate-800/30 transition-colors">
                        <td class="py-3.5 px-4 font-medium text-white flex items-center gap-3">
                            <div class="w-8 h-8 rounded-full bg-indigo-500/20 text-indigo-300 font-bold flex items-center justify-center text-xs border border-indigo-500/30">
                                {{ $item.name | initial }}
                            </div>
                            <span>{{ $item.name }}</span>
                        </td>
                        <td class="py-3.5 px-4 font-mono text-xs text-slate-300">{{ $item.email }}</td>
                        <td class="py-3.5 px-4">
                            <span class="px-2.5 py-1 rounded-full text-xs font-semibold uppercase tracking-wider bg-indigo-500/15 text-indigo-300 border border-indigo-500/25">
                                {{ $item.role }}
                            </span>
                        </td>
                        <td class="py-3.5 px-4">
                            <span class="inline-flex items-center gap-1.5 text-xs text-emerald-400">
                                <span class="w-1.5 h-1.5 rounded-full bg-emerald-400"></span>
                                Aktif
                            </span>
                        </td>
                    </tr>
                    @endforeach
                </tbody>
            </table>
        </div>
    </div>
</div>
@endsection
