@extends('layouts.app')

@section('content')
<div class="max-w-2xl mx-auto py-12 px-4">
    <div class="text-center mb-8">
        <h1 class="text-3xl font-extrabold text-slate-800">⚡ Lumina Live Demo</h1>
        <p class="text-slate-500 mt-2">Server-Driven Reactive UI tanpa menulis sebaris JavaScript pun!</p>
    </div>

    <!-- Panggil komponen reaktif Counter -->
    @kilat('counter')
</div>

<!-- Inject Lumina Live Micro-Runtime -->
@kilatScripts
@endsection
