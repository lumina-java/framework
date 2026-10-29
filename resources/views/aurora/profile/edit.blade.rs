@extends('aurora.layouts.app')

@section('page_title', 'Pengaturan Profil & Keamanan')

@section('content')
<div class="max-w-4xl space-y-6">
    <!-- Header -->
    <div>
        <h1 class="text-xl font-bold text-white tracking-tight">Akun & Preferensi</h1>
        <p class="text-xs text-slate-400 mt-1">Kelola data identitas akun dan perbarui kata sandi Anda.</p>
    </div>

    <!-- Form 1: Ubah Profil -->
    <div class="p-6 rounded-2xl bg-slate-900/70 border border-slate-800">
        <h2 class="text-sm font-bold text-white uppercase tracking-wider mb-1">Informasi Akun</h2>
        <p class="text-xs text-slate-400 mb-5">Perbarui nama tampilan dan alamat email akun Anda.</p>

        <form action="/profile" method="POST" class="space-y-4 max-w-lg">
            @csrf

            <div>
                <label for="name" class="block text-xs font-semibold uppercase tracking-wider text-slate-300 mb-1.5">Nama Lengkap</label>
                <input 
                    type="text" 
                    id="name" 
                    name="name" 
                    value="{{ $user.name }}" 
                    required 
                    class="w-full px-3.5 py-2.5 bg-slate-950/70 border border-slate-700/80 rounded-xl text-slate-100 text-sm focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:border-transparent transition-all"
                />
                @error('name')
                    <p class="mt-1 text-xs text-rose-400">{{ $message }}</p>
                @enderror
            </div>

            <div>
                <label for="email" class="block text-xs font-semibold uppercase tracking-wider text-slate-300 mb-1.5">Alamat Email</label>
                <input 
                    type="email" 
                    id="email" 
                    name="email" 
                    value="{{ $user.email }}" 
                    required 
                    class="w-full px-3.5 py-2.5 bg-slate-950/70 border border-slate-700/80 rounded-xl text-slate-100 text-sm focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:border-transparent transition-all"
                />
                @error('email')
                    <p class="mt-1 text-xs text-rose-400">{{ $message }}</p>
                @enderror
            </div>

            <button 
                type="submit" 
                class="py-2.5 px-4 bg-indigo-600 hover:bg-indigo-500 text-white font-semibold text-xs rounded-xl shadow-md shadow-indigo-600/20 transition-all"
            >
                Simpan Perubahan
            </button>
        </form>
    </div>

    <!-- Form 2: Perbarui Password -->
    <div class="p-6 rounded-2xl bg-slate-900/70 border border-slate-800">
        <h2 class="text-sm font-bold text-white uppercase tracking-wider mb-1">Perbarui Kata Sandi</h2>
        <p class="text-xs text-slate-400 mb-5">Pastikan akun Anda menggunakan kata sandi yang kuat dan unik.</p>

        <form action="/profile/password" method="POST" class="space-y-4 max-w-lg">
            @csrf

            <div>
                <label for="current_password" class="block text-xs font-semibold uppercase tracking-wider text-slate-300 mb-1.5">Kata Sandi Saat Ini</label>
                <input 
                    type="password" 
                    id="current_password" 
                    name="current_password" 
                    required 
                    placeholder="••••••••••••"
                    class="w-full px-3.5 py-2.5 bg-slate-950/70 border border-slate-700/80 rounded-xl text-slate-100 text-sm focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:border-transparent transition-all"
                />
                @error('current_password')
                    <p class="mt-1 text-xs text-rose-400">{{ $message }}</p>
                @enderror
            </div>

            <div>
                <label for="new_password" class="block text-xs font-semibold uppercase tracking-wider text-slate-300 mb-1.5">Kata Sandi Baru</label>
                <input 
                    type="password" 
                    id="new_password" 
                    name="new_password" 
                    required 
                    placeholder="Minimal 8 karakter"
                    class="w-full px-3.5 py-2.5 bg-slate-950/70 border border-slate-700/80 rounded-xl text-slate-100 text-sm focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:border-transparent transition-all"
                />
                @error('new_password')
                    <p class="mt-1 text-xs text-rose-400">{{ $message }}</p>
                @enderror
            </div>

            <div>
                <label for="new_password_confirmation" class="block text-xs font-semibold uppercase tracking-wider text-slate-300 mb-1.5">Konfirmasi Kata Sandi Baru</label>
                <input 
                    type="password" 
                    id="new_password_confirmation" 
                    name="new_password_confirmation" 
                    required 
                    placeholder="Ulangi kata sandi baru"
                    class="w-full px-3.5 py-2.5 bg-slate-950/70 border border-slate-700/80 rounded-xl text-slate-100 text-sm focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:border-transparent transition-all"
                />
            </div>

            <button 
                type="submit" 
                class="py-2.5 px-4 bg-slate-800 hover:bg-slate-700 text-slate-200 border border-slate-700 font-semibold text-xs rounded-xl transition-all"
            >
                Ubah Kata Sandi
            </button>
        </form>
    </div>
</div>
@endsection

