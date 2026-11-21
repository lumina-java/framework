@extends('aurora.layouts.guest')

@section('title', 'Masuk — Lumina Aurora')

@section('content')
<div class="mb-6">
    <h1 class="text-2xl font-bold text-white tracking-tight">Selamat Datang</h1>
    <p class="text-sm text-slate-400 mt-1">Masuk ke akun Anda untuk melanjutkan ke dashboard.</p>
</div>

<form action="/login" method="POST" class="space-y-4">
    @csrf

    <div>
        <label for="email" class="block text-xs font-semibold uppercase tracking-wider text-slate-300 mb-1.5">Alamat Email</label>
        <div class="relative">
            <div class="absolute inset-y-0 left-0 pl-3.5 flex items-center pointer-events-none text-slate-500">
                <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 12a4 4 0 10-8 0 4 4 0 008 0zm0 0v1.5a2.5 2.5 0 005 0V12a9 9 0 10-9 9m4.5-1.206a8.959 8.959 0 01-4.5 1.207" />
                </svg>
            </div>
            <input 
                type="email" 
                id="email" 
                name="email" 
                value="{{ $old_email }}" 
                required 
                autocomplete="email" 
                placeholder="nama@perusahaan.com"
                class="w-full pl-10 pr-4 py-2.5 bg-slate-950/70 border border-slate-700/80 rounded-xl text-slate-100 placeholder-slate-500 text-sm focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:border-transparent transition-all duration-200"
            />
        </div>
        @error('email')
            <p class="mt-1.5 text-xs text-rose-400 font-medium">{{ $message }}</p>
        @enderror
    </div>

    <div>
        <div class="flex items-center justify-between mb-1.5">
            <label for="password" class="block text-xs font-semibold uppercase tracking-wider text-slate-300">Kata Sandi</label>
            <a href="/forgot-password" class="text-xs text-indigo-400 hover:text-indigo-300 transition-colors">Lupa sandi?</a>
        </div>
        <div class="relative">
            <div class="absolute inset-y-0 left-0 pl-3.5 flex items-center pointer-events-none text-slate-500">
                <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z" />
                </svg>
            </div>
            <input 
                type="password" 
                id="password" 
                name="password" 
                required 
                autocomplete="current-password" 
                placeholder="••••••••••••"
                class="w-full pl-10 pr-4 py-2.5 bg-slate-950/70 border border-slate-700/80 rounded-xl text-slate-100 placeholder-slate-500 text-sm focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:border-transparent transition-all duration-200"
            />
        </div>
        @error('password')
            <p class="mt-1.5 text-xs text-rose-400 font-medium">{{ $message }}</p>
        @enderror
    </div>

    <div class="flex items-center justify-between pt-1">
        <label class="flex items-center gap-2 cursor-pointer select-none">
            <input 
                type="checkbox" 
                name="remember" 
                value="1" 
                class="rounded border-slate-700 bg-slate-950 text-indigo-600 focus:ring-indigo-500/50 w-4 h-4"
            />
            <span class="text-xs text-slate-400">Ingat saya di perangkat ini</span>
        </label>
    </div>

    <button 
        type="submit" 
        class="w-full py-3 px-4 bg-gradient-to-r from-indigo-600 to-violet-600 hover:from-indigo-500 hover:to-violet-500 text-white font-semibold text-sm rounded-xl shadow-lg shadow-indigo-600/30 hover:shadow-indigo-600/50 active:scale-[0.99] transition-all duration-200 flex items-center justify-center gap-2"
    >
        <span>Masuk ke Dashboard</span>
        <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M14 5l7 7m0 0l-7 7m7-7H3" />
        </svg>
    </button>
</form>

<div class="mt-6 pt-5 border-t border-slate-800/80 text-center text-xs text-slate-400">
    Belum memiliki akun?
    <a href="/register" class="text-indigo-400 hover:text-indigo-300 font-semibold ml-1 transition-colors">Daftar sekarang &rarr;</a>
</div>
@endsection

