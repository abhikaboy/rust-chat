use warp::filters::ws::{Message, WebSocket};
use std::sync::{Arc, Mutex};
use tokio::sync::broadcast;
use futures_util::{StreamExt,SinkExt};

pub async fn handle_connection(ws: WebSocket, tx: Arc<Mutex<broadcast::Sender<String>>>) {
    let (mut ws_sender, mut ws_reciever) = ws.split();
    let mut reciever = tx.lock().unwrap().subscribe();
    tokio::spawn(async move {
        while let Ok(msg) = reciever.recv().await {
            if ws_sender.send(Message::text(msg)).await.is_err(){
                break;
            }
        }
    });
    while let Some(result) = ws_reciever.next().await {
        match result {
            Ok(message) => {
                if let Ok(text) = message.to_str() {
                    tx.lock().unwrap().send(text.to_string()).expect("Failed to broadcast message");
                }
            },
            Err(e) => break,
        }
    }

}
