# ⚡ Modul 13: Lumina Live — Server-Driven Reactive UI

> *"Keajaiban Fullstack Reactive UI Tanpa Menulis Sebaris JavaScript Pun — Tulis Logika di Rust, Render di Blade, Nikmati Interaktivitas Setara SPA dengan Kecepatan Sub-Milidetik."*

---

## 🎯 1. Pengenalan Lumina Live

**Lumina Live** adalah paradigma *Server-Driven Reactive UI* pertama di dunia yang dibangun di atas performa mentah **Rust, Tokio, dan Axum**. Terinspirasi dari Laravel Livewire 3 dan Phoenix LiveView, Lumina Live memungkinkan Anda membuat komponen UI interaktif (seperti counter, live search, filter data table dinamis, modal popup, dan validasi form real-time) **murni menggunakan Rust dan Blade**.

### Keunggulan Utama:
1. **Zero JavaScript Overhead**: Tidak perlu setup bundler (Webpack, Vite, npm, node_modules).
2. **Kompak & Ringan**: Ukuran script runtime `lumina-live.js` konstan **hanya ~6.5 KB** untuk seluruh aplikasi.
3. **Tri-Dialect Engine**: Mendukung 3 dialek interaksi sekaligus:
   * **🇮🇩 Dialek Indonesia (`kilat:*`)**: `kilat:klik`, `kilat:model`, `kilat:tunggu`, `kilat:yakin`
   * **🌐 Dialek Universal (`live:*`)**: `live:click`, `live:model`, `live:loading`, `live:confirm`
   * **🅰️ Dialek Laravel Legacy (`wire:*`)**: `wire:click`, `wire:model`, `wire:loading`, `wire:confirm`
4. **DOM Morphing Cerdas (Idiomorph)**: Memperbarui hanya tag dan atribut yang berubah tanpa merusak kursor, fokus input yang sedang diketik, atau posisi scroll pengguna.
5. **Keamanan Kriptografis (HMAC-SHA256)**: State yang dikirim ke browser dienkripsi dan ditandatangani menggunakan `APP_KEY`, mencegah manipulasi parameter melalui DevTools browser.

---

## 🔨 2. Membuat Komponen Live Baru

Gunakan generator CLI Lumina:

```bash
lumina make:live Counter
```

Perintah di atas akan secara otomatis menghasilkan:
1. `src/app/live/counter.rs` (Logika state di Rust)
2. `resources/views/live/counter.blade.rs` (Tampilan antarmuka Blade)
3. Registrasi otomatis di `src/app/live/mod.rs`

---

## 💻 3. Struktur Komponen Rust (`src/app/live/counter.rs`)

```rust
use async_trait::async_trait;
use lumina::prelude::*;
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
            "kurang" | "decrement" => {
                if self.count > 0 {
                    self.count -= self.step;
                }
            }
            "reset" => self.count = 0,
            _ => return Err(AppError::BadRequest(format!("Aksi tidak dikenal: {}", action))),
        }
        Ok(())
    }
}
```

---

## 🎨 4. Tampilan Template Blade (`resources/views/live/counter.blade.rs`)

```html
<div class="p-6 bg-white rounded-2xl shadow-xl border border-slate-100 max-w-md mx-auto text-center">
    <div class="my-6">
        <span class="text-6xl font-black text-slate-800">{{ $count }}</span>
    </div>

    <!-- Tombol Aksi Reaktif -->
    <div class="flex items-center justify-center space-x-3">
        <!-- Menggunakan Dialek Indonesia: kilat:klik -->
        <button kilat:klik="kurang" class="btn btn-rose">-</button>

        <!-- Menggunakan Dialek Universal dengan konfirmasi keamanan kilat:yakin -->
        <button live:click="reset" kilat:yakin="Yakin ingin mereset data?" class="btn btn-slate">
            Reset
        </button>

        <!-- Menggunakan Dialek Indonesia: kilat:klik -->
        <button kilat:klik="tambah" class="btn btn-emerald">+</button>
    </div>

    <!-- Two-way Data Binding: kilat:model -->
    <div class="mt-6 flex items-center justify-center space-x-2">
        <label>Kelipatan:</label>
        <input type="number" min="1" max="10" kilat:model="step" value="{{ $step }}" class="input" />
    </div>

    <!-- Indikator Loading Bawaan: kilat:tunggu -->
    <div class="mt-3">
        <span kilat:tunggu class="text-xs text-slate-400 animate-pulse">Menyinkronkan...</span>
    </div>
</div>
```

---

## 📄 5. Memanggil Komponen di Halaman Utama

Cukup sematkan direktif `@kilat('nama_komponen')` atau `@live('nama_komponen')` dan jangan lupa tambahkan `@kilatScripts` di layout Anda:

```html
@extends('layouts.app')

@section('content')
<div class="container mx-auto py-12">
    <h1 class="text-3xl font-bold text-center">Demo Lumina Live</h1>

    <!-- Memanggil komponen Counter -->
    @kilat('counter')
</div>

<!-- Menyuntikkan Micro-Runtime JavaScript -->
@kilatScripts
@endsection
```

---

## 📋 6. Panduan Lengkap 22 Direktif HTML

| Kategori | Dialek Indonesia | Dialek Universal | Dialek Legacy | Fungsi |
| :--- | :--- | :--- | :--- | :--- |
| **Aksi Klik** | `kilat:klik="simpan"` | `live:click="save"` | `wire:click="save"` | Memanggil fungsi server saat diklik |
| **Submit Form** | `kilat:kirim="simpan"` | `live:submit="save"` | `wire:submit="save"` | Submit formulir tanpa reload |
| **Two-way Data**| `kilat:model="cari"` | `live:model="search"` | `wire:model="search"` | Sinkronisasi input form ke properti struct |
| **Debounce** | `kilat:tunda.300ms="cari"` | `live:debounce.300ms="search"` | `wire:model.debounce.300ms` | Menunda pengiriman input (live-search instan) |
| **Loading** | `kilat:tunggu` | `live:loading` | `wire:loading` | Menampilkan indikator proses saat request aktif |
| **Target Loading**| `kilat:target="simpan"`| `live:target="save"` | `wire:target="save"` | Membatasi indikator pada tombol tertentu |
| **Konfirmasi** | `kilat:yakin="Hapus?"` | `live:confirm="Delete?"` | `wire:confirm="Delete?"` | Menampilkan konfirmasi sebelum eksekusi aksi |
| **Polling** | `kilat:pantau.5d="cek"` | `live:poll.5s="check"` | `wire:poll.5s="check"` | Polling berkala otomatis ke server |
| **SPA Navigate**| `kilat:navigasi` | `live:navigate` | `wire:navigate` | Berpindah halaman seperti SPA tanpa reload |
| **Active Route**| `kilat:aktif="active"` | `live:current="active"` | `wire:current="active"` | Memberikan class CSS jika URL saat ini cocok |
| **Anti-Flicker**| `kilat:selubung` | `live:cloak` | `wire:cloak` | Menyembunyikan elemen sebelum JS siap |
| **Form Dirty** | `kilat:ubah` | `live:dirty` | `wire:dirty` | Tanda perubahan data pada input yang belum disimpan |
| **Inisialisasi**| `kilat:awal="muat"` | `live:init="load"` | `wire:init="load"` | Menjalankan aksi saat komponen pertama kali mount |
| **Lazy Scroll** | `kilat:tampak="muat"` | `live:intersect="load"` | `wire:intersect="load"` | Menjalankan aksi saat elemen terlihat di layar (Infinite Scroll) |
| **Status Offline**| `kilat:luring` | `live:offline` | `wire:offline` | Menampilkan elemen hanya jika internet terputus |
| **Abaikan Morph**| `kilat:abaikan` | `live:ignore` | `wire:ignore` | Melindungi library pihak ketiga (ChartJS/Select2) dari re-render |
| **Rujukan DOM** | `kilat:rujukan="canvas"`| `live:ref="canvas"` | `wire:ref="canvas"` | Menandai elemen untuk diakses cepat oleh skrip lokal |
| **Ganti Utuh** | `kilat:ganti` | `live:replace` | `wire:replace` | Mengganti elemen secara utuh (bukan diff morphing) |
| **Visibilitas** | `kilat:tampil="aktif"` | `live:show="active"` | `wire:show="active"` | Toggle tampilan elemen di browser |
| **Drag & Drop** | `kilat:urut="reorder"` | `live:sort="reorder"` | `wire:sort="reorder"` | Sinkronisasi drag and drop list ke backend |
| **Streaming AI**| `kilat:alir="token"` | `live:stream="token"` | `wire:stream="token"` | Menerima streaming token teks real-time |
| **Atribut Dinamis**| `kilat:ikat="attr"` | `live:bind="attr"` | `wire:bind="attr"` | Mengikat atribut HTML secara dinamis |

---

👉 *Dengan Lumina Live, Anda tidak perlu lagi memilih antara kenyamanan SSR Monolith dan interaktivitas SPA. Anda mendapatkan keduanya dengan kecepatan maksimal Rust!*
