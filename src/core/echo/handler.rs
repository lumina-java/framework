use axum::{
    extract::{ws::{Message, WebSocket, WebSocketUpgrade}, State},
    response::IntoResponse,
};
use futures::{sink::SinkExt, stream::StreamExt};
use uuid::Uuid;
use crate::core::application::AppState;
use crate::core::echo::manager::EchoMessage;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct ClientMessage {
    event: String,
    channel: Option<String>,
    data: Option<serde_json::Value>,
}

pub async fn echo_handler(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state))
}

async fn handle_socket(socket: WebSocket, state: AppState) {
    let (mut sender, mut receiver) = socket.split();
    let id = Uuid::new_v4();
    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<EchoMessage>();

    // Tambahkan koneksi ke manager
    state.echo.add_connection(id, tx);

    // Task untuk mengirim pesan ke client
    let mut send_task = tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            let json = serde_json::to_string(&msg).unwrap();
            if sender.send(Message::Text(json)).await.is_err() {
                break;
            }
        }
    });

    // Task untuk menerima pesan dari client (subscribe/unsubscribe)
    let echo_manager = state.echo.clone();
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            if let Message::Text(text) = msg {
                if let Ok(client_msg) = serde_json::from_str::<ClientMessage>(&text) {
                    match client_msg.event.as_str() {
                        "subscribe" => {
                            if let Some(channel) = client_msg.channel {
                                echo_manager.subscribe(id, &channel);
                            }
                        }
                        "unsubscribe" => {
                            if let Some(channel) = client_msg.channel {
                                echo_manager.unsubscribe(&id, &channel);
                            }
                        }
                        "ping" => {
                            // Heartbeat - client can ignore or pong
                        }
                        _ => {}
                    }
                }
            }
        }
    });

    // Tunggu salah satu task selesai (diskonek)
    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    };

    // Bersihkan koneksi
    state.echo.remove_connection(&id);
}
