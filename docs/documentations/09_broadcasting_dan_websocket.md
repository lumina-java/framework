# 📘 Modul 09: Broadcasting & WebSocket (Lumina Echo)

**Lumina Echo** menyediakan server WebSocket real-time terintegrasi untuk menangani notifikasi langsung, obrolan (*live chat*), dasbor dinamis, dan pembaruan data instan tanpa polling HTTP.

---

## 1. Arsitektur Channel Lumina Echo

Lumina Echo membagi siaran ke dalam tiga kategori channel:

1. **Public Channel** (misal: `news`, `announcements`): Dapat didengarkan oleh siapa saja tanpa perlu otorisasi atau login.
2. **Private Channel** (awalan `private-`, misal: `private-user-42`): Memerlukan verifikasi token JWT dan hak akses sebelum client dapat mendengarkan (*subscribe*).
3. **Presence Channel** (awalan `presence-`, misal: `presence-room-10`): Private channel khusus yang melacak daftar user yang sedang online di dalam ruangan tersebut.

---

## 2. Mendefinisikan Broadcast Event (`ShouldBroadcast`)

Buat event yang mengimplementasikan trait `ShouldBroadcast`:

```rust
use serde::Serialize;
use lumina::core::echo::ShouldBroadcast;

#[derive(Serialize)]
pub struct OrderShippedEvent {
    pub order_id: i64,
    pub tracking_number: String,
    pub customer_id: i64,
}

impl ShouldBroadcast for OrderShippedEvent {
    // Tentukan ke channel mana pesan ini disiarkan
    fn broadcast_on(&self) -> Vec<String> {
        vec![
            format!("private-user-{}", self.customer_id),
            "orders-tracker".to_string(),
        ]
    }

    // Nama event di sisi client (frontend)
    fn broadcast_as(&self) -> String {
        "OrderShipped".to_string()
    }
}
```

---

## 3. Menyiarkan Event dari Controller

Cukup panggil method `state.echo.broadcast(&event)`:

```rust
use axum::{extract::State, Json};
use lumina::core::application::AppState;

pub async fn ship_order(
    State(state): State<AppState>,
    Json(payload): Json<ShipRequest>,
) -> Json<serde_json::Value> {
    let event = OrderShippedEvent {
        order_id: payload.order_id,
        tracking_number: "RESI-123456".to_string(),
        customer_id: payload.customer_id,
    };

    // Siarkan secara asinkron ke semua client yang terkoneksi
    state.echo.broadcast(&event).await;

    Json(serde_json::json!({ "status": "Shipped & Broadcasted" }))
}
```

---

## 4. Otorisasi Private Channel

Daftarkan aturan otorisasi channel di `EchoManager` menggunakan closure:

```rust
state.echo.authorize_channel("private-user-*", |channel_name: &str, token: Option<&str>| {
    let token = match token {
        Some(t) => t,
        None => return false,
    };

    // Validasi token JWT
    let auth_user = match lumina::http::auth::validate_token(token) {
        Ok(u) => u,
        Err(_) => return false,
    };

    // Pastikan user hanya bisa mendengarkan private channel miliknya sendiri
    if let Some(target_id) = channel_name.strip_prefix("private-user-") {
        return auth_user.sub == target_id;
    }

    false
});
```

---

## 5. Koneksi dari Frontend (JavaScript)

Client dapat terhubung ke endpoint `/ws` menggunakan WebSocket standar atau library klien WebSocket:

```javascript
const ws = new WebSocket("ws://localhost:8000/ws");

ws.onopen = () => {
    // Berlangganan ke channel privat dengan menyertakan token JWT
    ws.send(JSON.stringify({
        action: "subscribe",
        channel: "private-user-42",
        token: "jwt_token_user_di_sini"
    }));
};

ws.onmessage = (event) => {
    const data = JSON.parse(event.data);
    console.log("Notifikasi baru:", data);
};
```

---

### Navigasi Modul:
- **Sebelumnya**: [Modul 08: Autentikasi & Keamanan](./08_autentikasi_dan_keamanan.md) 
- **Berikutnya**: [Modul 10: Queue, Jobs & Scheduler](./10_queue_jobs_dan_scheduler.md) ➔
- **Kembali ke**: [Daftar Isi Dokumentasi](./README.md)
