use warp::filters::ws::{Message, WebSocket};
use std::sync::{Arc, Mutex};
use tokio::sync::broadcast;
use futures_util::{StreamExt,SinkExt};
use crate::types::ChatMessage;
use serde::{Deserialize, Serialize};
use serde_json;

pub async fn handle_connection(ws: WebSocket, tx: Arc<Mutex<broadcast::Sender<String>>>) {
    println!("New WebSocket connection established!");
    let (mut ws_sender, mut ws_reciever) = ws.split();
    let mut reciever = tx.lock().unwrap().subscribe();
    tokio::spawn(async move {
        while let Ok(msg) = reciever.recv().await {
            println!("Broadcasting: {}", msg);
            // construct a chat message from the json
            match serde_json::from_str::<ChatMessage>(&msg) {
                Ok(chat_msg) => {
                    if ws_sender.send(Message::text(&chat_msg.format())).await.is_err(){
                        break;
                    }
                }
                Err(e) => {
                    println!("Error: Invalid Message Format");
                    println!("Send error back to client...?");
                }
            }
        }
    });
    while let Some(result) = ws_reciever.next().await {
        println!("Received something from client: {:?}", result);
        match result {
            Ok(message) => {
                if let Ok(text) = message.to_str() {
                    println!("Recieved Message: {}", text.to_string());
                    tx.lock().unwrap().send(text.to_string()).expect("Failed to broadcast message");
                }
            },
            Err(e) => {
                eprintln!("{e}");
                break
            },
        }
    }
    println!("WebSocket connection closed");
}
