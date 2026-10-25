# 📘 Modul 07: Indonesian Script (.is)

**Indonesian Script (`.is`)** adalah bahasa skrip terintegrasi yang dirancang khusus untuk ekosistem Lumina Framework. Menggunakan kata kunci bahasa Indonesia yang natural, skrip ini dapat dieksekusi secara instan di backend tanpa memerlukan proses kompilasi ulang Rust yang memakan waktu.

---

## 1. Keunggulan Indonesian Script di Lumina

- 🇮🇩 **Sintaks Bahasa Indonesia & Jawa Natural**: Ramah untuk pengembang lokal, pemula, dan tim non-teknis (misal tim promosi atau akuntansi yang membuat rumus bisnis).
- ⚡ **Zero-Compilation Overhead**: Skrip diinterpretasikan oleh Abstract Syntax Tree (AST) runner berbasis `IsScriptRunner` yang aman dan cepat.
- 🔄 **Hot-Swappable Business Logic**: Rumus promo diskon, perhitungan ongkir, atau aturan validasi dapat diubah sewaktu-waktu di server tanpa *downtime* server Rust.

---

## 2. Sintaks Dasar Bahasa

Simpan skrip Anda di folder `app/scripts/<nama>.is` atau `plugins/<nama>.is`.

### A. Deklarasi Fungsi & Variabel
```indonesian-script
fungsi hitung_total(harga, jumlah) {
    variabel subtotal = harga * jumlah;
    kembalikan subtotal;
}
```

### B. Percabangan Kondisional (`jika`, `selain_itu`)
```indonesian-script
fungsi hitung_promo(total, kode_kupon) {
    jika kode_kupon == "DISKON50" {
        kembalikan total * 0.50;
    } selain_itu jika kode_kupon == "HEMAT20" {
        kembalikan total * 0.20;
    } selain_itu {
        kembalikan 0;
    }
}
```

### C. Operator Logika Natural
Dapat menggunakan simbol standar atau padanan kata bahasa Indonesia:
- `dan` atau `&&`
- `atau` atau `||`
- `bukan` atau `!`
- `==`, `!=`, `<`, `<=`, `>`, `>=`

```indonesian-script
jika usia >= 17 dan status == "aktif" {
    kembalikan true;
}
```

### D. Perulangan (`selama`)
```indonesian-script
fungsi faktorial(n) {
    variabel hasil = 1;
    variabel i = 1;
    selama i <= n {
        hasil = hasil * i;
        i = i + 1;
    }
    kembalikan hasil;
}
```

---

## 3. Fungsi Bawaan (Built-in Helpers)

Lumina menyediakan sejumlah fungsi bawaan yang siap pakai di dalam skrip `.is`:

| Helper | Deskripsi | Contoh |
| :--- | :--- | :--- |
| `rupiah(angka)` | Format angka ke teks Rupiah | `rupiah(50000)` ➔ `"Rp 50.000"` |
| `panjang(teks)` | Menghitung jumlah karakter | `panjang("halo")` ➔ `4` |
| `huruf_besar(teks)` | Mengubah ke huruf besar | `huruf_besar("lumina")` ➔ `"LUMINA"` |
| `huruf_kecil(teks)` | Mengubah ke huruf kecil | `huruf_kecil("RUST")` ➔ `"rust"` |
| `potong(teks, batas)` | Memotong string panjang | `potong("artikel panjang", 7)` ➔ `"artikel..."` |
| `cetak(...)` | Menampilkan log debug ke console | `cetak("Nilai kalkulasi:", subtotal)` |

---

## 4. Eksekusi dari Controller Rust

Anda dapat memanggil fungsi di dalam skrip `.is` secara ergonomis menggunakan method `req.state.script("path.is")`:

```rust
use lumina::prelude::*;
use serde_json::json;

pub async fn hitung_diskon_handler(
    req: Request,
    ValidatedJson(payload): ValidatedJson<KuponRequest>,
) -> LuminaResult {
    // Memanggil fungsi 'hitung_promo' dari skrip 'app/scripts/promo.is'
    let diskon = req.state.script("promo.is")
        .call("hitung_promo", vec![json!(payload.total), json!(payload.kode)])
        .unwrap_or(json!(0));

    req.ok(json!({
        "total_belanja": payload.total,
        "potongan": diskon
    }))
}
```

---

## 5. Eksekusi Langsung dari Blade View (`@eval_is`)

Gunakan direktif `@eval_is` untuk merender kalkulasi skrip langsung di halaman HTML:

```html
<div class="summary-card">
    <h3>Rincian Pembayaran</h3>
    <p>Total: Rp {{ $total | rupiah }}</p>
    <p class="text-green-400">
        Hemat: Rp @eval_is('promo.is', 'hitung_promo', $total, $kode_kupon)
    </p>
</div>
```

---

### Navigasi Modul:
- **Sebelumnya**: [Modul 06: WASM Sandbox & Plugin](./06_wasm_sandbox_dan_plugin.md) 
- **Berikutnya**: [Modul 08: Autentikasi & Keamanan](./08_autentikasi_dan_keamanan.md) ➔
- **Kembali ke**: [Daftar Isi Dokumentasi](./README.md)
