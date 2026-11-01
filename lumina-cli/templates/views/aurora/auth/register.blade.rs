@extends('aurora.layouts.guest')

@section('title', 'Daftar Akun — Lumina Aurora')

@section('content')
<div class="mb-6">
    <h1 class="text-2xl font-bold text-white tracking-tight">Buat Akun Baru</h1>
    <p class="text-sm text-slate-400 mt-1">Lengkapi data di bawah untuk memulai pengalaman super cepat.</p>
</div>

<form action="/register" method="POST" class="space-y-4">
    @csrf

    <div>
        <label for="name" class="block text-xs font-semibold uppercase tracking-wider text-slate-300 mb-1.5">Nama Lengkap</label>
        <div class="relative">
            <div class="absolute inset-y-0 left-0 pl-3.5 flex items-center pointer-events-none text-slate-500">
                <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z" />
                </svg>
            </div>
            <input 
                type="text" 
                id="name" 
                name="name" 
                value="{{ $old_name }}" 
                required 
                autocomplete="name" 
                placeholder="misal: Budi Pratama"
                class="w-full pl-10 pr-4 py-2.5 bg-slate-950/70 border border-slate-700/80 rounded-xl text-slate-100 placeholder-slate-500 text-sm focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:border-transparent transition-all duration-200"
            />
        </div>
        @error('name')
            <p class="mt-1.5 text-xs text-rose-400 font-medium">{{ $message }}</p>
        @enderror
    </div>

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
        <label for="password" class="block text-xs font-semibold uppercase tracking-wider text-slate-300 mb-1.5">Kata Sandi</label>
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
                autocomplete="new-password" 
                placeholder="Minimal 8 karakter"
                class="w-full pl-10 pr-4 py-2.5 bg-slate-950/70 border border-slate-700/80 rounded-xl text-slate-100 placeholder-slate-500 text-sm focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:border-transparent transition-all duration-200"
            />
        </div>
        @error('password')
            <p class="mt-1.5 text-xs text-rose-400 font-medium">{{ $message }}</p>
        @enderror
    </div>

    <div>
        <label for="password_confirmation" class="block text-xs font-semibold uppercase tracking-wider text-slate-300 mb-1.5">Konfirmasi Kata Sandi</label>
        <div class="relative">
            <div class="absolute inset-y-0 left-0 pl-3.5 flex items-center pointer-events-none text-slate-500">
                <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m5.618-4.016A11.955 11.955 0 0112 2.944a11.955 11.955 0 01-8.618 3.04A12.02 12.02 0 003 9c0 5.591 3.824 10.29 9 11.622 5.176-1.332 9-6.03 9-11.622 0-1.042-.133-2.052-.382-3.016z" />
                </svg>
            </div>
            <input 
                type="password" 
                id="password_confirmation" 
                name="password_confirmation" 
                required 
                autocomplete="new-password" 
                placeholder="Ulangi kata sandi"
                class="w-full pl-10 pr-4 py-2.5 bg-slate-950/70 border border-slate-700/80 rounded-xl text-slate-100 placeholder-slate-500 text-sm focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:border-transparent transition-all duration-200"
            />
        </div>
    </div>

    <button 
        type="submit" 
        class="w-full py-3 px-4 bg-gradient-to-r from-indigo-600 to-violet-600 hover:from-indigo-500 hover:to-violet-500 text-white font-semibold text-sm rounded-xl shadow-lg shadow-indigo-600/30 hover:shadow-indigo-600/50 active:scale-[0.99] transition-all duration-200 flex items-center justify-center gap-2 mt-2"
    >
        <span>Daftar Akun Baru</span>
        <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M14 5l7 7m0 0l-7 7m7-7H3" />
        </svg>
    </button>
</form>

<div class="mt-6 pt-5 border-t border-slate-800/80 text-center text-xs text-slate-400">
    Sudah memiliki akun?
    <a href="/login" class="text-indigo-400 hover:text-indigo-300 font-semibold ml-1 transition-colors">Masuk di sini &rarr;</a>
</div>
@endsection

