# 🇮🇩 Indonesian Script (`.is`)

> **"Ngoding Web Kian Santai, Menyenangkan, dan Berbudaya."**  
> *Super lightweight reactive JavaScript compiler written in Rust with Indonesian & Javanese syntax helpers.*

[![License: MIT](https://img.shields.io/badge/License-MIT-emerald.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-2021%20edition-orange.svg)](https://www.rust-lang.org)
[![WebAssembly](https://img.shields.io/badge/WebAssembly-WASM-purple.svg)](https://webassembly.org)
[![Lumina Framework](https://img.shields.io/badge/ecosystem-Lumina%20Framework-blue.svg)](https://github.com/lumina-java/framework)

---

## 🌟 Tentang Indonesian Script

**Indonesian Script (`.is`)** adalah kompilator JavaScript reaktif yang sangat ringan, ditulis menggunakan **Rust**. Dirancang untuk menghadirkan sintaks pemrograman yang bersahabat (*human-readable*), berbasis kata kunci **Bahasa Indonesia** dan **Basa Jawa**, dengan reaktivitas state ala Svelte dan integrasi DOM dua arah (*two-way data binding*).

---

## ✨ Fitur Utama

- ⚡ **State Reaktif Otomatis**: Mendeklarasikan variabel reaktif semudah `reaktif hitungan = 0;`. Setiap kali nilai berubah, elemen HTML yang terikat otomatis diperbarui.
- 🗣️ **Dukungan Dwi-Bahasa (Indonesia & Jawa)**: Menggunakan kata kunci alami seperti `jika`/`yen`, `fungsi`/`pranata`, `kembalikan`/`balekno`, `cetak`/`dudohno`.
- 🌐 **Pengambilan Data Asinkron Instan**: `jupuk("api/data")` dan `kirim("api/simpan", payload)` langsung ditranspilasi menjadi `fetch()` modern.
- 🪶 **Zero External Runtime**: Runtime bawaan berbasis `Proxy` yang sangat kecil (< 1 KB) disematkan otomatis ke hasil kompilasi.
- 🦀 **Multi-Platform**: Dapat dijalankan sebagai **CLI compiler (`is-cli`)**, pustaka **Rust**, maupun dikompilasi ke **WebAssembly (WASM)** untuk browser.

---

## 📦 Instalasi & Penggunaan

### 1. Menggunakan CLI Compiler (`is-cli`)

Instal langsung via Cargo:
```bash
cargo install --git https://github.com/lumina-java/indonesia-script
```

Kompilasi berkas `.is` menjadi berkas JavaScript:
```bash
# Kompilasi dan cetak ke terminal
is-cli app.is

# Kompilasi dan simpan ke file output .js
is-cli app.is -o dist/app.js
```

---

### 2. Menggunakan sebagai Pustaka Rust

Tambahkan ke `Cargo.toml`:
```toml
[dependencies]
indonesian-script = { git = "https://github.com/lumina-java/indonesia-script" }
```

Gunakan di dalam kode Rust Anda:
```rust
use indonesian_script::compile_is;

fn main() {
    let source = r#"
        reaktif nama = "Budi";
        fungsi sapa() {
            cetak("Halo " + nama);
        }
    "#;

    let js = compile_is(source);
    println!("{}", js);
}
```

---

## 💻 Contoh Kode Indonesian Script (`counter.is`)

```javascript
/* Deklarasi State Reaktif */
reaktif angka = 0;
reaktif judul = "Aplikasi Hitungan";

/* Fungsi Penambah */
fungsi tambah() {
    angka = angka + 1;
    dudohno("Nilai sekarang: " + angka);
}

/* Fungsi Pengurang */
fungsi kurang() {
    yen (angka > 0) {
        angka = angka - 1;
    } liyane {
        cetak("Angka tidak boleh minus!");
    }
}
```

### Integrasi Template HTML:
```html
<div class="card">
    <h1 is-text="judul"></h1>
    <h2 is-text="angka">0</h2>

    <!-- Two-way binding ke input -->
    <input is-model="judul" type="text" />

    <!-- Pemicu aksi -->
    <button onclick="kurang()">Kurang (-)</button>
    <button onclick="tambah()">Tambah (+)</button>

    <!-- Ambil data API AJAX instan tanpa JavaScript manual -->
    <button is-jupuk="/api/pesan" is-target="#konten-pesan">Muat Pesan</button>
    <div id="konten-pesan"></div>
</div>

<!-- Muat file JS hasil kompilasi -->
<script src="counter.js"></script>
```

---

## 📑 Kamus Sintaks Kata Kunci

| Kata Kunci Indonesia | Basa Jawa | JavaScript Standar | Fungsi |
| :--- | :--- | :--- | :--- |
| `reaktif` / `state` | `reaktif` | `Proxy State ($state)` | Variabel reaktif (auto DOM sync) |
| `jika` / `kalau` | `yen` | `if` | Percabangan logika |
| `selain_itu jika` | `utawa yen` | `else if` | Percabangan alternatif |
| `selain_itu` | `liyane` | `else` | Percabangan terakhir |
| `fungsi` / `tugas` | `pranata` | `function` | Definisi fungsi |
| `kembalikan` | `balekno` | `return` | Mengembalikan nilai |
| `selama` | `suwene` | `while` | Perulangan kondisi |
| `untuk` | `kanggo` | `for` | Perulangan iterasi |
| `berhenti` | `mandek` | `break` | Menghentikan loop |
| `lanjut` | `terusno` | `continue` | Melanjutkan iterasi |
| `benar` | `bener` | `true` | Nilai boolean benar |
| `salah` | `kliru` | `false` | Nilai boolean salah |
| `kosong` | `suwung` | `null` | Nilai null |
| `cetak` / `tulis` | `dudohno` | `console.log()` | Mencetak log |
| `ambil` | `jupuk` | `await fetch().then()` | Fetch data GET (AJAX) |
| `kirimkan` | `kirim` | `await fetch(..., POST)` | Kirim data POST (AJAX) |

---

## 🤝 Berkontribusi (Contributing)

Kontribusi fitur, kosakata daerah nusantara lainnya, dan optimasi sangat terbuka!
1. Fork repositori ini.
2. Buat branch fitur (`git checkout -b feature/nama-fitur`).
3. Commit perubahan Anda (`git commit -m 'feat: tambah kata kunci baru'`).
4. Push ke branch Anda (`git push origin feature/nama-fitur`).
5. Buat **Pull Request**.

---

## 📜 Lisensi

Indonesian Script dirilis di bawah lisensi terbuka [MIT License](LICENSE).  
Dikelola dengan bangga oleh tim [Lumina Framework](https://github.com/lumina-java/framework).

