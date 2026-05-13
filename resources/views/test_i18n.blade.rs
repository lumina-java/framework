@extends('layout')

@section('content')
<div class="max-w-4xl mx-auto py-20 px-4">
    <div class="bg-slate-900 border border-slate-800 rounded-3xl p-10 shadow-2xl">
        <div class="flex items-center gap-4 mb-8">
            <span class="text-4xl">🌐</span>
            <h1 class="text-3xl font-bold">Lumina Localization Test</h1>
        </div>

        <div class="space-y-8">
            <div class="p-6 bg-slate-800/50 rounded-2xl border border-slate-700">
                <h3 class="text-slate-400 text-sm font-semibold uppercase tracking-wider mb-2">Current Locale</h3>
                <p class="text-2xl font-mono text-sky-400">{{ locale }}</p>
            </div>

            <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                <div class="p-6 bg-slate-800/50 rounded-2xl border border-slate-700">
                    <h3 class="text-slate-400 text-sm font-semibold uppercase tracking-wider mb-2">Simple Translation</h3>
                    <p class="text-xl">{{ __("messages.welcome") }}</p>
                </div>

                <div class="p-6 bg-slate-800/50 rounded-2xl border border-slate-700">
                    <h3 class="text-slate-400 text-sm font-semibold uppercase tracking-wider mb-2">Translation with Params</h3>
                    <p class="text-xl">{{ __("messages.hello", name=name) }}</p>
                </div>
            </div>

            <div class="p-6 bg-slate-800/50 rounded-2xl border border-slate-700">
                <h3 class="text-slate-400 text-sm font-semibold uppercase tracking-wider mb-2">Nested Keys (Auth)</h3>
                <div class="space-y-2">
                    <p class="text-slate-300"><b>Login:</b> {{ __("messages.auth.login") }}</p>
                    <p class="text-slate-300"><b>Error Message:</b> {{ __("messages.auth.failed") }}</p>
                </div>
            </div>

            <div class="pt-8 border-t border-slate-800 flex flex-wrap gap-4">
                <a href="/test-i18n?lang=en" class="px-6 py-3 bg-sky-600 hover:bg-sky-500 rounded-xl font-semibold transition-all">English (EN)</a>
                <a href="/test-i18n?lang=id" class="px-6 py-3 bg-emerald-600 hover:bg-emerald-500 rounded-xl font-semibold transition-all">Indonesia (ID)</a>
            </div>
        </div>
    </div>
</div>
@endsection
