@extends('layout')

@section('title')Home - Lumina Framework@endsection

@section('content')
<header class="hero text-center">
    <div class="container">
        <img src="/images/logo.png" alt="Lumina Logo" style="width: 120px; height: 120px; margin-bottom: 20px; filter: drop-shadow(0 10px 20px rgba(0,0,0,0.2)); border-radius: 20px;">
        <h1 class="display-3 fw-bold mb-3">Selamat Datang di Lumina</h1>
        <p class="lead mb-4">Framework web Rust yang indah, cepat, dan elegan.</p>
        <div class="d-grid gap-2 d-sm-flex justify-content-sm-center">
            <a href="https://github.com/lumina-java/lumina" class="btn btn-light btn-lg px-4 gap-3">Documentation</a>
            <a href="/api/users" class="btn btn-outline-light btn-lg px-4">Browse API</a>
        </div>
    </div>
</header>

<main class="container">
    <div class="row g-4 py-5 row-cols-1 row-cols-lg-3">
        <div class="col d-flex align-items-start">
            <div>
                <h3 class="fs-4 fw-bold">🚀 Fast & Efficient</h3>
                <p>Dibangun di atas runtime Tokio dan server Axum untuk performa maksimal tanpa mengorbankan keamanan.</p>
            </div>
        </div>
        <div class="col d-flex align-items-start">
            <div>
                <h3 class="fs-4 fw-bold">🎨 Beautiful Syntax</h3>
                <p>Syntax yang bersih dan terinspirasi dari Laravel, memudahkan developer untuk fokus pada fitur.</p>
            </div>
        </div>
        <div class="col d-flex align-items-start">
            <div>
                <h3 class="fs-4 fw-bold">🏗️ MVC Architecture</h3>
                <p>Pemisahan logic yang jelas antara Controller, Model, dan View untuk kemudahan maintain kode.</p>
            </div>
        </div>
    </div>
</main>
@endsection
