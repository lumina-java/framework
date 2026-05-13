@extends('layout')

@section('title')
    About Lumina Framework
@endsection

@section('content')
<style>
    .hero-section {
        background: radial-gradient(circle at top right, rgba(124, 58, 237, 0.1), transparent),
                    radial-gradient(circle at bottom left, rgba(59, 130, 246, 0.1), transparent);
        padding: 80px 0;
        border-radius: 30px;
        position: relative;
        overflow: hidden;
    }

    .logo-container {
        position: relative;
        width: 220px;
        height: 220px;
        margin: 0 auto;
        animation: float 6s ease-in-out infinite;
    }

    .logo-img {
        width: 100%;
        height: 100%;
        object-fit: contain;
        filter: drop-shadow(0 20px 40px rgba(124, 58, 237, 0.4));
        border-radius: 20%;
    }

    .glass-card {
        background: rgba(255, 255, 255, 0.8);
        backdrop-filter: blur(10px);
        -webkit-backdrop-filter: blur(10px);
        border: 1px solid rgba(255, 255, 255, 0.3);
        border-radius: 24px;
        box-shadow: 0 10px 30px rgba(0, 0, 0, 0.05);
        transition: transform 0.3s ease;
    }

    .glass-card:hover {
        transform: translateY(-5px);
    }

    .badge-premium {
        background: linear-gradient(45deg, #7c3aed, #3b82f6);
        color: white;
        padding: 5px 15px;
        border-radius: 50px;
        font-weight: 600;
        font-size: 0.8rem;
        text-transform: uppercase;
        letter-spacing: 1px;
    }

    @keyframes float {
        0%, 100% { transform: translateY(0); }
        50% { transform: translateY(-20px); }
    }

    .feature-icon {
        width: 50px;
        height: 50px;
        display: flex;
        align-items: center;
        justify-content: center;
        border-radius: 12px;
        margin-bottom: 15px;
        font-size: 1.5rem;
    }

    .icon-bg-purple { background: rgba(124, 58, 237, 0.1); color: #7c3aed; }
    .icon-bg-blue { background: rgba(59, 130, 246, 0.1); color: #3b82f6; }
    .icon-bg-orange { background: rgba(249, 115, 22, 0.1); color: #f97316; }

    .display-title {
        font-size: 3.5rem;
        font-weight: 800;
        background: linear-gradient(to right, #1e293b, #7c3aed);
        -webkit-background-clip: text;
        -webkit-text-fill-color: transparent;
        margin-top: 20px;
    }

    /* Fallback styles for missing Tailwind classes */
    .bg-slate-900 { background-color: #0f172a !important; }
    .text-slate-600 { color: #475569 !important; }
    .text-slate-500 { color: #64748b !important; }
    .text-white { color: #ffffff !important; }
    .text-purple-400 { color: #c084fc !important; }
    .text-blue-400 { color: #60a5fa !important; }
    .text-green-400 { color: #4ade80 !important; }
    .max-w-2xl { max-width: 42rem; margin-left: auto; margin-right: auto; }
    .rounded-4 { border-radius: 1rem !important; }
    .shadow-lg { box-shadow: 0 10px 25px rgba(0, 0, 0, 0.2) !important; }
</style>

<div class="container py-5">
    <div class="hero-section text-center mb-5">
        <div class="logo-container">
            <img src="/images/logo.png" alt="Lumina Logo" class="logo-img">
        </div>
        <h1 class="display-title">LUMINA</h1>
        <p class="lead text-slate-600 max-w-2xl mx-auto px-4">
            The high-performance, elegant, and developer-friendly Rust web framework. 
            Crafted for speed, safety, and ultimate developer happiness.
        </p>
        <div class="mt-4">
            <span class="badge-premium">v0.1.0-ALPHA • ENTERPRISE READY</span>
        </div>
    </div>

    <div class="row g-4 mb-5">
        <div class="col-md-4">
            <div class="glass-card p-4 h-100">
                <div class="feature-icon icon-bg-purple">
                    <span>⚡</span>
                </div>
                <h5 class="fw-bold">Blazing Fast</h5>
                <p class="text-muted mb-0">Powered by Axum and Tokio, Lumina delivers sub-millisecond response times under heavy load.</p>
            </div>
        </div>
        <div class="col-md-4">
            <div class="glass-card p-4 h-100">
                <div class="feature-icon icon-bg-blue">
                    <span>🛡️</span>
                </div>
                <h5 class="fw-bold">Memory Safe</h5>
                <p class="text-muted mb-0">Lumina leverages Rust's ownership model to eliminate null pointers and data races entirely.</p>
            </div>
        </div>
        <div class="col-md-4">
            <div class="glass-card p-4 h-100">
                <div class="feature-icon icon-bg-orange">
                    <span>🎨</span>
                </div>
                <h5 class="fw-bold">Elegant Syntax</h5>
                <p class="text-muted mb-0">A Laravel-inspired developer experience in Rust. Simple, expressive, and highly productive.</p>
            </div>
        </div>
    </div>

    <div class="glass-card p-5 mb-5">
        <div class="row align-items-center">
            <div class="col-lg-6">
                <h3 class="fw-bold mb-4">The Stack Behind the Magic</h3>
                <ul class="list-unstyled">
                    <li class="mb-3 d-flex align-items-center">
                        <span class="me-3 fs-5">✅</span>
                        <span><strong>Engine:</strong> Axum 0.7 & Tokio Runtime</span>
                    </li>
                    <li class="mb-3 d-flex align-items-center">
                        <span class="me-3 fs-5">✅</span>
                        <span><strong>Database:</strong> SQLx (Compile-time checked queries)</span>
                    </li>
                    <li class="mb-3 d-flex align-items-center">
                        <span class="me-3 fs-5">✅</span>
                        <span><strong>Templates:</strong> Tera (Jinja2 inspired engine)</span>
                    </li>
                    <li class="mb-3 d-flex align-items-center">
                        <span class="me-3 fs-5">✅</span>
                        <span><strong>Assets:</strong> Vite & Tailwind Ready</span>
                    </li>
                </ul>
            </div>
            <div class="col-lg-6 text-center">
                <div class="p-4 bg-slate-900 rounded-4 text-start font-monospace small text-white shadow-lg">
                    <div class="text-slate-500 mb-2">// Initializing Lumina App</div>
                    <div><span class="text-purple-400">let</span> app = <span class="text-blue-400">Application::new</span>();</div>
                    <div>app.<span class="text-blue-400">serve</span>(<span class="text-green-400">"127.0.0.1:8000"</span>).<span class="text-purple-400">await</span>;</div>
                </div>
            </div>
        </div>
    </div>

    <div class="text-center pb-5">
        <a href="/" class="btn btn-primary btn-lg px-5 rounded-pill shadow-lg me-3">
            Get Started
        </a>
        <a href="https://github.com/lumina-rust/lumina" class="btn btn-outline-dark btn-lg px-5 rounded-pill shadow-sm">
            <i class="bi bi-github me-2"></i>GitHub
        </a>
    </div>
</div>
@endsection
