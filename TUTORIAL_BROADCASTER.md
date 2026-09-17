# 📡 Panduan & Tutorial WebSocket Broadcaster (Lumina Echo)

Lumina Framework dilengkapi dengan sistem **WebSocket Event Broadcasting** bawaan (disebut **Lumina Echo**) yang terinspirasi dari Laravel Echo. Sistem ini memungkinkan Anda mengirimkan event dan data dari backend Rust ke frontend secara **real-time** dengan mudah.

---

## 🎯 Konsep Utama

1. **`EchoManager`**: Mengelola seluruh koneksi WebSocket aktif, pendaftaran channel, otorisasi, dan pengiriman pesan.
2. **`ShouldBroadcast`**: Trait yang diimplementasikan pada struct Event agar event tersebut dapat disiarkan ke client WebSocket.
3. **Channel**: Saluran pesan tempat event disiarkan:
   - **Public Channel** (misal: `news`, `notifications`): Bebas diakses oleh semua client tanpa otorisasi.
   - **Private Channel** (misal: `private-user-1`, `private-chat-room-5`): Memerlukan verifikasi otorisasi sebelum client diperbolehkan *subscribe*.
4. **WebSocket Endpoint**: Endpoint default `/lumina/echo` untuk koneksi WebSocket dan `/lumina/echo/auth` untuk otorisasi channel private.

---

## 🛠️ Langkah 1: Membuat Event Real-Time

Untuk membuat event yang dapat disiarkan secara real-time, buat struct Rust lalu implementasikan trait `Event` dan `ShouldBroadcast`.

### Contoh: `UserNotifiedEvent`

```rust
use lumina::core::echo::ShouldBroadcast;
use lumina::core::event::traits::Event;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserNotifiedEvent {
    pub user_id: u64,
    pub title: String,
    pub message: String,
}

// 1. Implementasikan ShouldBroadcast
impl ShouldBroadcast for UserNotifiedEvent {
    // Tentukan channel mana event ini akan dikirim
    fn broadcast_on(&self) -> Vec<String> {
        vec![
            format!("notifications"),                  // Public channel
            format!("private-user-{}", self.user_id), // Private channel
        ]
    }

    // Nama event yang akan diterima oleh client di frontend (default: nama struct)
    fn broadcast_as(&self) -> String {
        "UserNotified".to_string()
    }

    // Payload data yang dikirimkan ke frontend
    fn broadcast_with(&self) -> Value {
        json!({
            "user_id": self.user_id,
            "title": self.title,
            "message": self.message,
            "timestamp": chrono::Utc::now().to_rfc3339()
        })
    }
}

// 2. Implementasikan Event trait agar bisa dikirim via EventDispatcher
impl Event for UserNotifiedEvent {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn broadcaster(&self) -> Option<&dyn ShouldBroadcast> {
        Some(self)
    }
}
```

---

## 🚀 Langkah 2: Menyiarkan (Dispatch) Event dari Controller

Anda dapat menyiarkan event langsung dari handler controller menggunakan `req.events()`.

```rust
use lumina::core::request::Request;
use axum::response::IntoResponse;
use crate::events::UserNotifiedEvent;

pub async fn send_notification(req: Request) -> impl IntoResponse {
    let event = UserNotifiedEvent {
        user_id: 42,
        title: "Pesanan Dikirim".to_string(),
        message: "Paket Anda sedang dalam perjalanan oleh kurir.".to_string(),
    };

    // Opsi A: Dispatch via EventDispatcher (akan menjalankan Listener + Broadcast otomatis)
    req.events().dispatch(event, req.state_arc()).await;

    // Opsi B: Hanya broadcast langsung tanpa menjalankan Listener backend
    // req.events().dispatch_broadcast(&event, &req.state());

    req.json(serde_json::json!({
        "status": "success",
        "message": "Notifikasi berhasil disiarkan!"
    })).await
}
```

---

## 🔒 Langkah 3: Mengatur Otorisasi Channel Private

Secara default, channel biasa dapat di-subscribe oleh siapa saja. Namun untuk channel yang diawali `private-` atau `presence-`, Anda dapat mendaftarkan aturan otorisasi di file bootstrap / setup aplikasi Anda:

```rust
use lumina::core::application::AppState;

pub fn configure_broadcaster_auth(state: &AppState) {
    // Mengatur otorisasi untuk channel "private-user-{id}"
    state.echo.authorize_channel("private-user-*", |channel_name, socket_id| {
        // Logika verifikasi token / session / user id di sini
        // Return true jika diizinkan, false jika ditolak
        if let Some(_id) = socket_id {
            return true;
        }
        true
    });
}
```

---

## 🌐 Langkah 4: Menyiapkan Route WebSocket di Router

Pastikan route untuk WebSocket (`/lumina/echo`) dan Auth (`/lumina/echo/auth`) sudah terdaftar di router Lumina Anda:

```rust
use lumina::core::echo::{echo_auth_handler, echo_handler};
use axum::{routing::{get, post}, Router};

pub fn register_echo_routes(router: Router<AppState>) -> Router<AppState> {
    router
        .route("/lumina/echo", get(echo_handler))
        .route("/lumina/echo/auth", post(echo_auth_handler))
}
```

---

## 💻 Langkah 5: Contoh Penggunaan Real-time di Frontend

Berikut adalah contoh halaman web sederhana (`index.html`) yang menggunakan Vanilla JavaScript WebSocket Client untuk terhubung ke Lumina Echo, melakukan subscribe ke channel public maupun private, dan menerima notifikasi secara real-time.

```html
<!DOCTYPE html>
<html lang="id">
<head>
    <meta charset="UTF-8">
    <title>Lumina Echo - Realtime Demo</title>
    <style>
        body { font-family: sans-serif; margin: 2rem; background: #f4f6f9; }
        .card { background: white; padding: 1.5rem; border-radius: 8px; box-shadow: 0 2px 5px rgba(0,0,0,0.1); max-width: 600px; }
        .badge { background: #10b981; color: white; padding: 2px 8px; border-radius: 4px; font-size: 0.8rem; }
        #logs { background: #1e293b; color: #38bdf8; padding: 1rem; border-radius: 6px; font-family: monospace; height: 200px; overflow-y: auto; }
    </style>
</head>
<body>
    <div class="card">
        <h2>📡 Lumina Realtime Broadcaster</h2>
        <p>Status Koneksi: <span id="status" class="badge">Terhubung</span></p>

        <h3>Pesan Masuk:</h3>
        <div id="logs"></div>
    </div>

    <script>
        const logBox = document.getElementById('logs');
        function log(msg) {
            const p = document.createElement('div');
            p.textContent = `[${new Date().toLocaleTimeString()}] ${msg}`;
            logBox.appendChild(p);
            logBox.scrollTop = logBox.scrollHeight;
        }

        // 1. Buka koneksi WebSocket ke Lumina Echo
        const wsProtocol = location.protocol === 'https:' ? 'wss:' : 'ws:';
        const ws = new WebSocket(`${wsProtocol}//${location.host}/lumina/echo`);

        ws.onopen = () => {
            log("Terhubung ke server WebSocket Lumina!");

            // 2. Subscribe ke channel "notifications"
            ws.send(JSON.stringify({
                event: "subscribe",
                channel: "notifications"
            }));

            // 3. Subscribe ke channel private "private-user-42"
            ws.send(JSON.stringify({
                event: "subscribe",
                channel: "private-user-42"
            }));
        };

        // 4. Menerima event pesan dari backend
        ws.onmessage = (event) => {
            const data = JSON.parse(event.data);
            log(`Event received [${data.event}] on channel [${data.channel}]: ${JSON.stringify(data.data)}`);

            if (data.event === "UserNotified") {
                alert(`Notifikasi Baru: ${data.data.title}\n${data.data.message}`);
            }
        };

        ws.onclose = () => {
            log("Koneksi WebSocket terputus.");
            document.getElementById('status').textContent = "Terputus";
            document.getElementById('status').style.background = "#ef4444";
        };
    </script>
</body>
</html>
```

### Integrasi dengan Indonesian Script (`.is`)

Jika Anda menggunakan komponen Indonesian Script (`.is`) pada Blade template Lumina:

```svelte
<skrip>
    impor { padaMulai } dari 'lumina';

    variabel statusKoneksi = "Menghubungkan...";
    variabel daftarNotifikasi = [];

    padaMulai(() => {
        konstanta ws = baru WebSocket("ws://localhost:8000/lumina/echo");

        ws.onopen = () => {
            statusKoneksi = "Terhubung";
            ws.send(JSON.stringify({ event: "subscribe", channel: "notifications" }));
        };

        ws.onmessage = (e) => {
            konstanta res = JSON.parse(e.data);
            jika (res.event === "UserNotified") {
                daftarNotifikasi.push(res.data);
            }
        };
    });
</skrip>

<div kelas="p-4 border rounded">
    <h3 kelas="font-bold">Status: {statusKoneksi}</h3>
    <ul kelas="mt-2 space-y-1">
        {#sepanjang daftarNotifikasi sebagai item}
            <li kelas="bg-green-100 p-2 rounded shadow">
                <strong>{item.title}</strong>: {item.message}
            </li>
        {/sepanjang}
    </ul>
</div>
```

---

## ⚡ Ringkasan Format Pesan WebSocket Client

| Action | Payload JSON |
| :--- | :--- |
| **Subscribe Channel** | `{"event": "subscribe", "channel": "nama-channel"}` |
| **Unsubscribe Channel** | `{"event": "unsubscribe", "channel": "nama-channel"}` |
| **Ping / Heartbeat** | `{"event": "ping"}` |

---

Dengan langkah-langkah di atas, aplikasi Lumina Framework Anda siap mendukung fitur real-time seperti Notifikasi, Chatting, Live Dashboard, dan banyak lagi! 🎉
