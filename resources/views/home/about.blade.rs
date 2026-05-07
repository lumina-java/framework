@extends('layout')

@section('title')
    About Lumina Framework
@endsection

@section('content')
<div class="row justify-content-center">
    <div class="col-md-8 text-center">
        <!-- Logo Lumina (Medical Style) -->
        <div class="mb-4">
            <div class="d-inline-block p-4 bg-white rounded-circle shadow-sm" style="width: 150px; height: 150px; line-height: 100px;">
                <span style="font-size: 80px;">✨</span>
            </div>
            <h1 class="display-4 fw-bold mt-3 text-primary">LUMINA</h1>
            <p class="text-muted lead">The Modern Rust Framework for Human Beings</p>
        </div>

        <div class="card shadow-sm border-0 text-start">
            <div class="card-body p-4">
                <h4 class="fw-bold mb-4 border-bottom pb-2">
                    <i class="bi bi-info-circle me-2"></i>System Information
                </h4>
                
                <div class="row mb-3">
                    <div class="col-sm-4 text-muted">Version</div>
                    <div class="col-sm-8"><span class="badge bg-primary">v0.1.0-alpha</span> (Premium Medical Edition)</div>
                </div>

                <div class="row mb-3">
                    <div class="col-sm-4 text-muted">Engine</div>
                    <div class="col-sm-8">Rust 1.75+ (Axum + SQLx + Tera)</div>
                </div>

                <div class="row mb-3">
                    <div class="col-sm-4 text-muted">Architecture</div>
                    <div class="col-sm-8">MVC (Model View Controller) with Laravel-style DX</div>
                </div>

                <h4 class="fw-bold mt-5 mb-4 border-bottom pb-2">
                    <i class="bi bi-github me-2"></i>Open Source
                </h4>
                <p>Lumina Framework is built with focus on speed, safety, and developer happiness.</p>
                <a href="https://github.com/lumina-rust/lumina" target="_blank" class="btn btn-dark">
                    <i class="bi bi-github me-2"></i>View on GitHub
                </a>
                
                <hr class="my-5">
                
                <div class="text-center text-muted small">
                    <p>Designed with ❤️ for SIMRS Pharmacy & Medical Applications.</p>
                </div>
            </div>
        </div>
        
        <div class="mt-4">
            <a href="/" class="btn btn-outline-primary">
                <i class="bi bi-arrow-left me-1"></i>Back to Home
            </a>
        </div>
    </div>
</div>
@endsection
