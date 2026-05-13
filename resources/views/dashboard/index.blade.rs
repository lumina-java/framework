@extends('dashboard_layout')

@section('title')
    Dashboard — Lumina Overview
@endsection

@section('content')
    <!-- Dashboard Stats Grid -->
    <div class="row g-4 mb-4">
        <!-- Card 1: Role -->
        <div class="col-12 col-md-3">
            <div class="card bg-white border-0 shadow-sm h-100" style="border-bottom: 4px solid #cbd5e1 !important; border-radius: 12px;">
                <div class="card-body p-4">
                    <div class="d-flex align-items-center justify-content-between mb-3">
                        <h6 class="text-muted text-uppercase fw-bold mb-0" style="font-size: 11px; letter-spacing: 1px;">Status Akun</h6>
                        <span class="badge bg-success bg-opacity-25 text-success rounded-pill" style="font-size: 10px;">ACTIVE</span>
                    </div>
                    <div class="fs-3 fw-bold text-dark">{{ role | capitalize }}</div>
                    <div class="text-muted mt-2" style="font-size: 11px;">Level akses sistem saat ini</div>
                </div>
            </div>
        </div>

        <!-- Card 2: User ID -->
        <div class="col-12 col-md-3">
            <div class="card bg-white border-0 shadow-sm h-100" style="border-bottom: 4px solid #3b82f6 !important; border-radius: 12px;">
                <div class="card-body p-4">
                    <div class="d-flex align-items-center justify-content-between mb-3">
                        <h6 class="text-muted text-uppercase fw-bold mb-0" style="font-size: 11px; letter-spacing: 1px;">User ID</h6>
                        <div class="bg-primary bg-opacity-10 text-primary p-2 rounded">
                            <svg width="16" height="16" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 6H5a2 2 0 00-2 2v9a2 2 0 002 2h14a2 2 0 002-2V8a2 2 0 00-2-2h-5m-4 0V5a2 2 0 114 0v1m-4 0a2 2 0 104 0m-5 8a2 2 0 100-4 2 2 0 000 4zm0 0c1.306 0 2.417.835 2.83 2M9 14a3.001 3.001 0 00-2.83 2M15 11h3m-3 4h2"></path></svg>
                        </div>
                    </div>
                    <div class="fs-3 fw-bold text-dark">#{{ user_id }}</div>
                    <div class="text-muted mt-2" style="font-size: 11px;">Identifier unik database</div>
                </div>
            </div>
        </div>

        <!-- Card 3: Framework -->
        <div class="col-12 col-md-3">
            <div class="card bg-white border-0 shadow-sm h-100" style="border-bottom: 4px solid #a855f7 !important; border-radius: 12px;">
                <div class="card-body p-4">
                    <div class="d-flex align-items-center justify-content-between mb-3">
                        <h6 class="text-muted text-uppercase fw-bold mb-0" style="font-size: 11px; letter-spacing: 1px;">Engine</h6>
                        <div class="p-2 rounded" style="background-color: rgba(168, 85, 247, 0.1); color: #a855f7;">
                            <svg width="16" height="16" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z"></path></svg>
                        </div>
                    </div>
                    <div class="fs-3 fw-bold text-dark">Lumina 0.1</div>
                    <div class="text-muted mt-2" style="font-size: 11px;">Rust Powered Framework</div>
                </div>
            </div>
        </div>

        <!-- Card 4: Database -->
        <div class="col-12 col-md-3">
            <div class="card bg-white border-0 shadow-sm h-100" style="border-bottom: 4px solid #10b981 !important; border-radius: 12px;">
                <div class="card-body p-4">
                    <div class="d-flex align-items-center justify-content-between mb-3">
                        <h6 class="text-muted text-uppercase fw-bold mb-0" style="font-size: 11px; letter-spacing: 1px;">Database</h6>
                        <div class="bg-success bg-opacity-10 text-success p-2 rounded">
                            <svg width="16" height="16" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 7v10c0 2.21 3.582 4 8 4s8-1.79 8-4V7M4 7c0 2.21 3.582 4 8 4s8-1.79 8-4M4 7c0-2.21 3.582-4 8-4s8 1.79 8 4m0 5c0 2.21-3.582 4-8 4s-8-1.79-8-4"></path></svg>
                        </div>
                    </div>
                    <div class="fs-3 fw-bold text-dark">Connected</div>
                    <div class="text-muted mt-2" style="font-size: 11px;">SQLx Active Session</div>
                </div>
            </div>
        </div>
    </div>

    <div class="row g-4">
        <!-- Eager Loading Table -->
        <div class="col-12 col-lg-8">
            <div class="card border-0 shadow-sm" style="border-radius: 12px; overflow: hidden;">
                <div class="card-header bg-light border-bottom-0 d-flex justify-content-between align-items-center py-3 px-4">
                    <div>
                        <h5 class="mb-0 fw-bold text-dark">Eager Loading Demo</h5>
                        <small class="text-muted">Solusi N+1 Query dalam ORM Lumina</small>
                    </div>
                    <span class="badge bg-primary px-3 py-2 shadow-sm" style="font-size: 10px; letter-spacing: 0.5px;">2 QUERIES TOTAL</span>
                </div>
                <div class="table-responsive">
                    <table class="table table-hover align-middle mb-0 text-secondary">
                        <thead class="table-light text-uppercase text-muted" style="font-size: 11px; font-weight: bold;">
                            <tr>
                                <th class="px-4 py-3 border-0">Nama User</th>
                                <th class="px-4 py-3 border-0">Email</th>
                                <th class="px-4 py-3 border-0 text-center">Jumlah Post</th>
                            </tr>
                        </thead>
                        <tbody style="border-top: none;">
                            @foreach(users as u)
                            <tr style="cursor: pointer;">
                                <td class="px-4 py-3 border-light">
                                    <div class="d-flex align-items-center">
                                        <div class="rounded-circle bg-light d-flex justify-content-center align-items-center text-muted fw-bold me-3" style="width: 35px; height: 35px; font-size: 12px;">
                                            {{ u.name | truncate(length=1, end="") | upper }}
                                        </div>
                                        <span class="fw-bold text-dark">{{ u.name }}</span>
                                    </div>
                                </td>
                                <td class="px-4 py-3 border-light">{{ u.email }}</td>
                                <td class="px-4 py-3 border-light text-center">
                                    <span class="badge bg-light text-dark px-3 py-2 rounded-pill border">
                                        {{ u.posts | length }} posts
                                    </span>
                                </td>
                            </tr>
                            @endforeach
                        </tbody>
                    </table>
                </div>
                <div class="card-footer bg-white border-top py-3 px-4">
                    <small class="text-muted fst-italic" style="font-size: 11px;">Data di atas diambil menggunakan <code>User::query().with("posts").get()</code></small>
                </div>
            </div>
        </div>

        <!-- Right Side Panel -->
        <div class="col-12 col-lg-4">
            
            <!-- JWT Debugger -->
            <div class="card border-0 shadow-lg text-white mb-4" style="background-color: #0f172a; border-radius: 12px;">
                <div class="card-header border-bottom border-secondary border-opacity-25 d-flex justify-content-between align-items-center py-3 px-4 bg-transparent">
                    <h6 class="mb-0 text-uppercase tracking-widest fw-bold" style="font-size: 12px; letter-spacing: 1px;">Active Token</h6>
                    <span class="rounded-circle bg-success" style="width: 8px; height: 8px; box-shadow: 0 0 8px rgba(34,197,94,0.8);"></span>
                </div>
                <div class="card-body p-4 font-monospace" style="font-size: 12px;">
                    <div class="d-flex justify-content-between border-bottom border-secondary border-opacity-25 pb-2 mb-2">
                        <span class="text-secondary">subject</span>
                        <span style="color: #60a5fa;">{{ user_id }}</span>
                    </div>
                    <div class="d-flex justify-content-between border-bottom border-secondary border-opacity-25 pb-2 mb-2">
                        <span class="text-secondary">email</span>
                        <span style="color: #34d399;">{{ email }}</span>
                    </div>
                    <div class="d-flex justify-content-between border-bottom border-secondary border-opacity-25 pb-2 mb-2">
                        <span class="text-secondary">role</span>
                        <span style="color: #c084fc;">{{ role }}</span>
                    </div>
                    <div class="d-flex justify-content-between">
                        <span class="text-secondary">issued_at</span>
                        <span style="color: #fb923c;">{{ now() | date(format="%s") }}</span>
                    </div>
                    
                    <div class="mt-4 p-3 rounded" style="background-color: rgba(255,255,255,0.05); border: 1px solid rgba(255,255,255,0.1);">
                        <small class="text-secondary" style="font-size: 10px;">Lumina menggunakan <strong>Secure JWT Cookies</strong> untuk otentikasi stateless yang aman dan cepat.</small>
                    </div>
                </div>
            </div>

            <!-- System Info -->
            <div class="card bg-white border-0 shadow-sm" style="border-radius: 12px;">
                <div class="card-body p-4">
                    <h6 class="fw-bold text-dark d-flex align-items-center mb-4">
                        <span class="me-2" style="font-size: 1.2rem;">🚀</span> Info Sistem
                    </h6>
                    
                    <div class="d-flex justify-content-between align-items-center mb-3" style="font-size: 13px;">
                        <span class="text-muted">OS</span>
                        <span class="fw-bold text-dark">Windows (Dev)</span>
                    </div>
                    <div class="d-flex justify-content-between align-items-center mb-3" style="font-size: 13px;">
                        <span class="text-muted">Rust Version</span>
                        <span class="fw-bold text-dark">1.75+</span>
                    </div>
                    <div class="d-flex justify-content-between align-items-center" style="font-size: 13px;">
                        <span class="text-muted">Environment</span>
                        <span class="badge bg-primary bg-opacity-10 text-primary">DEVELOPMENT</span>
                    </div>
                </div>
            </div>
            
        </div>
    </div>
@endsection