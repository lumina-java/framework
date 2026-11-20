# 🖨️ Modul 23: Lumina Hardware Bridge (Client WASM & POS Devices)

> *"Menghubungkan Server Web Linux di Cloud ke Perangkat Keras Lokal di Client (Printer Thermal, Barcode Scanner, Fingerprint, dan Camera) secara Langsung."*

---

## 🏛️ 1. Arsitektur Hardware Bridge

Aplikasi cloud (misal POS kasir, sistem rumah sakit, absensi) di server Linux dapat mengirimkan data cetak atau membaca sensor perangkat keras lokal client melalui jembatan **WebAssembly / WebUSB / WebSerial / WebHID**:

```text
Server Linux (Cloud) ──[ WebSocket / SSE ]──> Browser Client (WASM Bridge) ──> Perangkat Keras
                                                                               ├── Printer Thermal (ESC/POS)
                                                                               ├── Barcode Scanner (Serial)
                                                                               ├── Fingerprint Biometrik (HID)
                                                                               └── Kamera (WebRTC QR)
```

---

## 🧾 2. ESC/POS Thermal Receipt Printer Builder

Bangun format struk belanja kasir dengan builder lancar:

```rust
use lumina::prelude::*;

let mut receipt = EscPosBuilder::new();
receipt
    .align_center()
    .bold(true)
    .double_size(true)
    .line("TOKO ROTI LUMINA")
    .bold(false)
    .double_size(false)
    .line("Jl. Sudirman No. 12, Jakarta")
    .separator(32)
    .align_left()
    .two_columns("Roti Keju x2", "Rp 20.000", 32)
    .two_columns("Kopi Latte x1", "Rp 25.000", 32)
    .separator(32)
    .bold(true)
    .two_columns("TOTAL", "Rp 45.000", 32)
    .bold(false)
    .feed(3)
    .cut()
    .open_drawer();

// Konversi ke base64 untuk dikirim via WebSocket ke browser client:
let payload = receipt.to_base64();
```

---

## 📦 3. Barcode Scanner & EAN-13 Validation

Menangani data pemindaian barcode dengan validasi check digit:

```rust
use lumina::prelude::*;

let event = BarcodeScanEvent::new("9780201379624").with_format("EAN_13");
if event.is_valid_ean13() {
    println!("Barcode valid: {}", event.barcode);
}
```

---

## 🌐 4. Browser Client Bridge Script

Sertakan script client pada template Blade Anda:

```html
<script>
    // Koneksikan browser ke printer USB thermal lokal
    await LuminaDevice.printer.connectUsb();

    // Atau dengarkan barcode scanner serial (COM/USB port)
    LuminaDevice.scanner.listen(9600, (barcode) => {
        console.log("Barcode dipindai:", barcode);
    });

    // Hubungkan WebSocket ke server cloud untuk print otomatis
    LuminaDevice.connectServerBridge("wss://pos.tokoanda.com/ws/devices");
</script>
```
