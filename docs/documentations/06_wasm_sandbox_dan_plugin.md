# 📘 Modul 06: WASM Sandbox & Plugin

Lumina memperkenalkan arsitektur **WASM Native Hybrid** menggunakan engine WebAssembly `wasmi 2.0`. Fitur ini memungkinkan pengembang menambahkan plugin pihak ketiga atau filter kustom yang berjalan dalam lingkungan terisolasi (*sandboxed*), aman dari *memory-leak* atau *panic/crash* pada server utama.

---

## 1. Arsitektur Isolasi (Fault Isolation)

Setiap plugin WASM dieksekusi di dalam ruang memori virtual terpisah:
- **Zero Host Crash**: Jika plugin mengalami *panic* atau pembagian nol, engine menangkapnya sebagai `SandboxError::Trap` dan mengembalikan pesan error tanpa menghentikan aplikasi web Lumina.
- **Hot-Reloading**: File `.wasm` baru yang diletakkan di folder `plugins/` dapat langsung dideteksi tanpa perlu mengompilasi ulang aplikasi Rust utama.
- **Resource Guard**: Plugin tidak memiliki akses langsung ke filesystem atau socket jaringan host kecuali diizinkan secara eksplisit melalui Host State API.

---

## 2. Membuat Plugin Baru (`lumina plugin:new`)

Gunakan CLI Lumina untuk membuat starter crate plugin WebAssembly:

```bash
lumina plugin:new sensor_kata
```

Perintah ini akan membuat struktur direktori di `plugins/sensor_kata/`:
```text
plugins/sensor_kata/
├── Cargo.toml          # crate-type = ["cdylib"], target wasm32
├── src/
│   └── lib.rs          # Template implementasi ABI Lumina
└── README.md           # Panduan penggunaan
```

---

## 3. Implementasi Kode Plugin (`src/lib.rs`)

File `src/lib.rs` yang digenerate telah menyertakan fungsi manajemen memori `alloc`, `dealloc`, dan fungsi filter `transform`:

```rust
use std::mem;

/// Alokasikan buffer memori di heap guest untuk menerima input string dari host Lumina.
#[no_mangle]
pub extern "C" fn alloc(len: i32) -> i32 {
    let mut buf = Vec::with_capacity(len as usize);
    let ptr = buf.as_mut_ptr();
    mem::forget(buf);
    ptr as i32
}

/// Dealokasi buffer memori heap guest setelah selesai diproses.
#[no_mangle]
pub extern "C" fn dealloc(ptr: i32, len: i32) {
    unsafe {
        let _ = Vec::from_raw_parts(ptr as *mut u8, len as usize, len as usize);
    }
}

/// Fungsi transformasi teks (Filter).
/// Return: Mengembalikan 64-bit integer gabungan (out_ptr << 32) | out_len
#[no_mangle]
pub extern "C" fn transform(ptr: i32, len: i32) -> i64 {
    let input = unsafe {
        let slice = std::slice::from_raw_parts(ptr as *const u8, len as usize);
        std::str::from_utf8(slice).unwrap_or("")
    };

    // Logika sensor: mengganti kata terlarang
    let output = input.replace("kasar", "*****");

    let out_bytes = output.into_bytes();
    let out_len = out_bytes.len() as i64;
    let out_ptr = out_bytes.as_ptr() as i64;

    mem::forget(out_bytes);

    (out_ptr << 32) | (out_len & 0xFFFF_FFFF)
}
```

---

## 4. Kompilasi Plugin (`lumina plugin:build`)

Kompilasikan sub-crate plugin ke biner WebAssembly berekstensi `.wasm`:

```bash
# Kompilasi plugin tertentu
lumina plugin:build sensor_kata

# Atau kompilasi seluruh plugin di folder plugins/ sekaligus
lumina plugin:build
```

Alur kerja otomatis:
1. Menjalankan `cargo build --target wasm32-unknown-unknown --release`.
2. Menyalin biner output dari `target/wasm32-unknown-unknown/release/sensor_kata.wasm` langsung ke `plugins/sensor_kata.wasm`.
3. Menampilkan ukuran biner (misal: `✅ Berhasil! Binary tersimpan di: plugins/sensor_kata.wasm (14.20 KB)`).

---

## 5. Melihat Daftar Plugin Aktif (`lumina plugin:list`)

Periksa semua plugin dan skrip yang tersedia di proyek Anda:

```bash
lumina plugin:list
```
Output:
```text
NAMA PLUGIN / SKRIP              TIPE            UKURAN       STATUS / DETAIL                    
-------------------------------- --------------- ------------ -----------------------------------
sensor_kata.wasm                 WASM Binary     14.20 KB     Siap Dijalankan ✅                 
rupiah.wasm                      WASM Binary     18.42 KB     Siap Dijalankan ✅                 
markdown/ (Crate)                WASM Source     -            Terkonfigurasi & Terkompilasi      
app/scripts/promo.is             Skrip (.is)     1.20 KB      2 fungsi                           
-------------------------------- --------------- ------------ -----------------------------------
Total item terdeteksi: 4
```

---

## 6. Menggunakan Plugin di Blade Template & Controller

### Di Blade Template:
```html
<!-- Menggunakan pipa filter (otomatis fallback ke WASM jika bukan native) -->
<p>{{ $komentar | sensor_kata }}</p>

<!-- Menggunakan direktif eksplisit @wasm -->
<p>@wasm('sensor_kata', 'transform', $komentar)</p>
```

### Di Controller Rust:
```rust
let hasil = state.sandbox.call_filter("sensor_kata", "transform", "komentar kasar")?;
println!("Hasil sensor: {}", hasil); // "komentar *****"
```

---

### Navigasi Modul:
- **Sebelumnya**: [Modul 05: Blade Template Engine](./05_blade_template_engine.md) 
- **Berikutnya**: [Modul 07: Indonesian Script](./07_indonesian_script.md) ➔
- **Kembali ke**: [Daftar Isi Dokumentasi](./README.md)
