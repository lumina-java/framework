# ⚡ Lumina Live (`lumina-live`)

> **"Look like Laravel, Run at Rust Speed."**  
> *Server-Driven Reactive Fullstack UI for Lumina Framework — Tulis Logika di Rust, Render di Blade, Nikmati Interaktivitas Setara SPA Tanpa Menulis JavaScript.*

[![License: MIT](https://img.shields.io/badge/License-MIT-emerald.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-2021%20edition-orange.svg)](https://www.rust-lang.org)
[![Lumina Framework](https://img.shields.io/badge/ecosystem-Lumina%20Framework-blue.svg)](https://github.com/lumina-java/framework)

---

## 🌟 Mengapa Lumina Live?

Terinspirasi dari keindahan **Laravel Livewire 3** dan kecepatan **Phoenix LiveView**, `lumina-live` menghadirkan paradigma *Server-Driven Reactive UI* di atas performa mentah **Rust + Tokio + Axum**:

- 🚀 **Sub-Milidetik Latency**: Komunikasi real-time melalui WebSocket Streaming murni (< 1 ms latency) dengan fallback otomatis ke HTTP Micro-RPC.
- 🇮🇩 **Tri-Dialect Directives**: Bebas menggunakan dialek Indonesia (`kilat:*`), Universal (`live:*`), atau sintaks legasi (`wire:*`).
- 🔐 **Tamper-Proof State Security**: State client-side dilindungi secara kriptografis menggunakan tanda tangan HMAC-SHA256 (`APP_KEY`). Manipulasi nilai kritis di DevTools browser otomatis ditolak.
- 🪶 **Zero-Dependency Micro-Runtime (~7 KB)**: Mengintegrasikan algoritma DOM Morphing cerdas (*Idiomorph*) yang menjaga fokus input form, teks yang sedang diketik, dan posisi scroll browser.

---

## 📦 Instalasi

Tambahkan `lumina-live` ke dalam berkas `Cargo.toml` proyek Lumina Anda:

```toml
[dependencies]
lumina = { package = "lumina-framework", version = "0.4.0", git = "https://github.com/lumina-java/framework" }
lumina-live = { git = "https://github.com/lumina-java/lumina-live" }
```

---

## 🚀 Panduan Cepat (Quickstart)

### 1. Aktifkan di `src/main.rs`
Gunakan trait ekstensi `LiveApplicationExt` bawaan `lumina_live::prelude::*`:

```rust
use lumina::prelude::*;
use lumina_live::prelude::*;

#[tokio::main]
async fn main() {
    let app = Application::new()
        .with_web(routes::web::routes())
        .with_live() // ⚡ Mengaktifkan WebSocket streaming & rute liveview
        .with_live_component::<Counter>("counter"); // Daftarkan komponen

    app.serve("127.0.0.1:8000").await;
}
```

### 2. Definisikan Komponen di Rust (`src/app/live/counter.rs`)

```rust
use async_trait::async_trait;
use lumina::core::error::AppError;
use lumina_live::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Counter {
    pub count: i32,
    pub step: i32,
}

impl Default for Counter {
    fn default() -> Self {
        Self { count: 0, step: 1 }
    }
}

#[async_trait]
impl LiveComponent for Counter {
    fn name(&self) -> &'static str {
        "counter"
    }

    fn view(&self) -> &'static str {
        "live.counter"
    }

    async fn call_action(&mut self, action: &str, _params: Vec<Value>) -> Result<(), AppError> {
        match action {
            "tambah" | "increment" => self.count += self.step,
            "kurang" | "decrement" => if self.count > 0 { self.count -= self.step; },
            "reset" => self.count = 0,
            _ => return Err(AppError::BadRequest(format!("Aksi tidak dikenal: {}", action))),
        }
        Ok(())
    }
}
```

### 3. Buat Template Blade Komponen (`resources/views/live/counter.blade.rs`)

```html
<div class="p-6 bg-white rounded-2xl shadow-xl text-center max-w-sm mx-auto">
    <span class="text-6xl font-black text-slate-800">{{ $count }}</span>

    <div class="mt-4 flex justify-center space-x-2">
        <!-- Mendukung dialek Indonesia: kilat:klik -->
        <button kilat:klik="kurang" class="btn btn-danger">-</button>
        <button live:click="reset" kilat:yakin="Yakin ingin reset?" class="btn">Reset</button>
        <button kilat:klik="tambah" class="btn btn-success">+</button>
    </div>

    <!-- Two-way Data Binding: kilat:model -->
    <div class="mt-4">
        <label>Kelipatan:</label>
        <input type="number" kilat:model="step" value="{{ $step }}" class="border rounded px-2" />
    </div>

    <!-- Loading state indicator -->
    <span kilat:tunggu class="text-xs text-slate-400">Sinkronisasi ke Rust...</span>
</div>
```

### 4. Panggil di Halaman Blade Anda

```html
@extends('layouts.app')

@section('content')
    <!-- Format PHP Array khas Laravel Blade maupun format JSON didukung -->
    @kilat('counter', ['count' => 10, 'step' => 2])
@endsection

<!-- Menyematkan runtime lumina-live.js -->
@kilatScripts
```

---

## 📑 Kamus Direktif (Tri-Dialect)

| Fitur | 🇮🇩 Indonesia | 🌐 Universal | 🅰️ Laravel Legacy | Deskripsi |
| :--- | :--- | :--- | :--- | :--- |
| **Click Event** | `kilat:klik="action"` | `live:click="action"` | `wire:click="action"` | Menjalankan aksi di server saat klik |
| **Form Submit** | `kilat:kirim="save"` | `live:submit="save"` | `wire:submit="save"` | Submit form via WebSocket tanpa reload |
| **Two-Way Binding** | `kilat:model="field"` | `live:model="field"` | `wire:model="field"` | Sinkronisasi input langsung ke struct Rust |
| **Debounce** | `kilat:tunda.300ms` | `live:debounce.300ms`| `wire:model.debounce`| Tunda pengiriman aksi ketik input |
| **Loading State** | `kilat:tunggu` | `live:loading` | `wire:loading` | Muncul saat request sedang diproses server |
| **Confirm Dialog** | `kilat:yakin="Pesan?"`| `live:confirm="Msg?"`| `wire:confirm="Msg?"`| Dialog konfirmasi browser sebelum aksi |
| **Polling** | `kilat:pantau.5d` | `live:poll.5s` | `wire:poll.5s` | Polling periodik berkala dari server |
| **SPA Navigation** | `kilat:navigasi` | `live:navigate` | `wire:navigate` | Navigasi halaman cepat tanpa reload |
| **Offline State** | `kilat:luring` | `live:offline` | `wire:offline` | Muncul ketika koneksi internet terputus |
| **Cloak** | `kilat:selubung` | `live:cloak` | `wire:cloak` | Sembunyikan elemen sebelum JS siap |

---

## 🤝 Berkontribusi (Contributing)

Kami sangat menyambut kontribusi dari komunitas Rust dan Web Developer!
1. Fork repositori ini.
2. Buat branch fitur baru (`git checkout -b feature/nama-fitur`).
3. Jalankan unit test (`cargo test`).
4. Commit perubahan Anda (`git commit -m 'feat: tambah fitur keren'`).
5. Push ke branch Anda (`git push origin feature/nama-fitur`).
6. Buka **Pull Request**.

---

## 📜 Lisensi

Lumina Live dirilis di bawah lisensi [MIT License](LICENSE).  
Dikembangkan dengan ❤️ oleh tim [Lumina Framework](https://github.com/lumina-java/framework).

