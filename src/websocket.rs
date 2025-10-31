use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::Response,
};
use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::broadcast;
use tracing::{error, info};

use crate::hot_reload::{ChangeType, FileChange};

/// WebSocket message types sent to clients
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ReloadMessage {
    #[serde(rename = "reload")]
    Reload { path: String, reason: String },
    #[serde(rename = "css_update")]
    CssUpdate { path: String },
    #[serde(rename = "ping")]
    Ping,
}

/// WebSocket handler state
#[derive(Clone)]
pub struct WsState {
    pub reload_tx: broadcast::Sender<FileChange>,
}

/// Handle WebSocket upgrade for hot reload
pub async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<WsState>>,
) -> Response {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

/// Handle individual WebSocket connection
async fn handle_socket(socket: WebSocket, state: Arc<WsState>) {
    info!("🔌 New WebSocket connection established");

    let (mut sender, mut receiver) = socket.split();

    // Subscribe to reload events
    let mut reload_rx = state.reload_tx.subscribe();

    // Spawn task to send reload notifications
    let mut send_task = tokio::spawn(async move {
        while let Ok(file_change) = reload_rx.recv().await {
            let message = match file_change.change_type {
                ChangeType::Template | ChangeType::Component => ReloadMessage::Reload {
                    path: file_change.path.to_string_lossy().to_string(),
                    reason: format!("{:?} changed", file_change.change_type),
                },
                ChangeType::SourceCode => {
                    // For source code changes, we can't hot reload
                    // User needs to restart the server
                    info!("⚠️  Source code changed - server restart required");
                    continue;
                }
            };

            let json = match serde_json::to_string(&message) {
                Ok(json) => json,
                Err(e) => {
                    error!("Failed to serialize message: {}", e);
                    continue;
                }
            };

            if sender.send(Message::Text(json)).await.is_err() {
                break;
            }
        }
    });

    // Spawn task to handle incoming messages (mainly for keep-alive)
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            match msg {
                Message::Text(text) => {
                    info!("Received text message: {}", text);
                }
                Message::Close(_) => {
                    info!("🔌 WebSocket closed by client");
                    break;
                }
                _ => {}
            }
        }
    });

    // Wait for either task to finish
    tokio::select! {
        _ = (&mut send_task) => {
            recv_task.abort();
        }
        _ = (&mut recv_task) => {
            send_task.abort();
        }
    }

    info!("🔌 WebSocket connection closed");
}
