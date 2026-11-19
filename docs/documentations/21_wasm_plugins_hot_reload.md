# 🧩 Modul 21 — Dynamic WASM Plugin Registry & Zero-Downtime Hot-Reloading

> *"Look like Laravel, Run at Rust Speed. Menghadirkan ekosistem ekstensi WebAssembly terisolasi dengan kemampuan hot-reloading tanpa downtime dan proteksi kegagalan berstandar militer."*

Selamat datang di **Modul 21 Dokumentasi Resmi Lumina Framework**! Pada modul ini, kita akan mempelajari arsitektur **Dynamic WebAssembly (WASM) Plugin Registry**, fitur enterprise Lumina yang memungkinkan penambahan logika kustom, filter template, dan lifecycle hooks pihak ketiga tanpa pernah mengorbankan kestabilan server inti (*zero-crash guarantee*).

---

## 🌟 1. Mengapa WebAssembly (WASM) di Backend?

Pada arsitektur monolitik konvensional, menambahkan kode pihak ketiga (misalnya kalkulator pajak daerah, parser dokumen khusus, atau modul anti-fraud) memiliki risiko fatal: jika kode tersebut mengalami *panic* (*out-of-bounds array*, *null dereference*), seluruh server akan jatuh (*crash*).

Lumina mengadopsi engine **WebAssembly runtime (`wasmi 2.0`)** dengan keunggulan mutlak:
1. **Linear Memory Isolation**: Setiap plugin berjalan dalam ruang memori 32-bit terpisah.
2. **Fault & Trap Isolation**: Jika plugin mengalami panic, hanya modul WASM tersebut yang ter-trap; server host tetap melayani jutaan pengguna dengan normal.
3. **Zero-Downtime Hot-Reloading**: Anda dapat memperbarui biner `.wasm` saat server sedang berjalan di lingkungan produksi tanpa perlu me-restart proses server.

---

## 🚀 2. Manajemen Plugin via CLI

Lumina CLI menyediakan toolchain lengkap untuk siklus hidup plugin:

### A. Membuat Plugin WASM Baru
```bash
lumina plugin:new sensor_kata
```
Perintah ini akan membuat sub-proyek Rust mandiri di folder `plugins/sensor_kata/` yang dikonfigurasi khusus untuk target `wasm32-unknown-unknown`.

### B. Mengompilasi Plugin ke Biner `.wasm`
```bash
# Kompilasi seluruh plugin di folder plugins/
lumina plugin:build

# Kompilasi plugin tertentu
lumina plugin:build sensor_kata
```
Biner output otomatis dioptimasi dan disimpan ke `plugins/sensor_kata.wasm`.

### C. Menampilkan Daftar Plugin
```bash
lumina plugin:list
```

### D. Memeriksa Dashboard Metrik & Kesehatan Plugin
```bash
lumina plugin:metrics
```
Output:
```text
📊 Lumina WASM Sandbox & Plugin Metrics Dashboard

Total WASM Modules Loaded : 2
Script Modules Cached     : 0

┌───────────────────────┬──────────────┬──────────────┬──────────────┬──────────────┐
│ PLUGIN NAME           │ CALLS COUNT  │ ERRORS COUNT │ AVG DURATION │ STATUS       │
├───────────────────────┼──────────────┼──────────────┼──────────────┼──────────────┤
│ sensor_kata           │ 1540         │ 0            │ 4.25µs       │ Healthy ✅   │
│ pajak_ppn             │ 320          │ 0            │ 2.80µs       │ Healthy ✅   │
└───────────────────────┴──────────────┴──────────────┴──────────────┴──────────────┘
```

### E. Memicu Reload Manual Tanpa Downtime
```bash
lumina plugin:reload
```
```text
🔄 Reloading Lumina WASM Plugins (Zero-Downtime Hot-Reload)...
✅ [Reload Success] Berhasil memuat ulang 2 plugin dalam 3.42ms tanpa downtime! ⚡
```

---

## 🔄 3. Zero-Downtime Hot-Reloading Otomatis

Di aplikasi Lumina, Anda dapat mengaktifkan pemantauan otomatis (*background file watcher*) yang mendeteksi pembaruan file `.wasm` secara asinkron:

```rust
use lumina::prelude::*;
use std::time::Duration;

#[tokio::main]
async fn main() {
    let app = Application::new();
    let plugin_manager = LuminaPluginManager::new();

    // Memuat seluruh plugin awal dari folder plugins/
    plugin_manager.load_directory("plugins").unwrap();

    // Aktifkan auto-watcher hot-reloading setiap 2 detik
    plugin_manager.start_hot_reload_watcher("plugins", Duration::from_secs(2));

    app.run().await;
}
```

Ketika Anda mengunggah berkas `.wasm` versi baru ke folder `plugins/`, pointer instance di memori diperbarui secara atomik (`Arc<RwLock<...>>`). Request yang sedang berjalan menyelesaikan prosesnya dengan instance lama, sedangkan request baru langsung menggunakan versi terbaru.

---

## 🪝 4. Lifecycle Hooks Pipeline

Plugin WASM dapat merespons berbagai event siklus hidup aplikasi dengan mengekspor fungsi standar:

| Hook Type | Nama Fungsi Export WASM | Waktu Eksekusi |
| :--- | :--- | :--- |
| `PluginHook::OnRequest` | `lumina_on_request` | Dijalankan saat HTTP Request masuk. |
| `PluginHook::BeforeSave` | `lumina_before_save` | Dijalankan sebelum model disimpan. |
| `PluginHook::AfterResponse` | `lumina_after_response` | Dijalankan sebelum HTTP Response dikirim. |

### Memanggil Hook dari Kode Host:
```rust
let results = plugin_manager.call_hook(PluginHook::OnRequest, &request_json);
for (plugin_name, result) in results {
    match result {
        Ok(transformed) => println!("Plugin {} memproses hook: {}", plugin_name, transformed),
        Err(e) => eprintln!("Plugin {} error: {}", plugin_name, e),
    }
}
```

---

## 🎨 5. Integrasi Filter Blade Template

Plugin WASM juga dapat digunakan langsung sebagai pipeline filter di template Blade `.blade.rs`:

```html
<!-- Menggunakan filter WASM sensor_kata -->
<p>{{ $comment | sensor_kata }}</p>

<!-- Menggunakan direktif @wasm -->
<div>@wasm('pajak_ppn', 'hitung', $subtotal)</div>
```

---

## 🎯 Kesimpulan
Dengan kehadiran **Lumina Tinker** dan **Dynamic WASM Plugin Registry**, Lumina Framework v1.0.0 resmi menetapkan standar baru bagi arsitektur framework backend modern: **produktif, seindah Laravel, dan secepat serta seaman Rust!**
