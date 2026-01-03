use futures_util::{future, pin_mut, StreamExt};
use tokio::io::{AsyncReadExt, AsyncWriteExt, Stdin, stdin};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use futures_channel::mpsc::UnboundedReceiver;
use crate::types::ChatMessage;
use std::time::{SystemTime, UNIX_EPOCH};

mod types;


#[tokio::main]
async fn main() {
    // connect to websocket client
    let url = "ws://127.0.0.1:8080/ws";
    println!("Attempting to connect...");

    let name : String = read_name().await;

    let (stdin_tx, stdin_rx) = futures_channel::mpsc::unbounded();
    tokio::spawn(read_stdin(stdin_tx, name.clone())); // this process can outlive main, so it needs to be cloned

    let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");
    println!("WebSocket handshake has been successfully completed");


    let (sink, read) = ws_stream.split();

    let stdin_to_ws = stdin_rx.map(Ok).forward(sink);

    // Handles incoming messages
    let ws_to_stdout = {
        read.for_each(|message| async {
            let data_bytes = message.unwrap().into_data(); // convert from message into byte array
            match serde_json::from_slice::<ChatMessage>(&data_bytes) {
                Ok(msg) => println!("{}",&msg.format()),
                Error => println!("error~")
            }
        })
    };

    pin_mut!(stdin_to_ws, ws_to_stdout);
    future::select(stdin_to_ws, ws_to_stdout).await; // "pause the execution of this function until either one of these futures finish"
}

async fn read_name() -> String {
    let mut stdin = tokio::io::stdin();
    let mut buf = vec![0; 1024];
    let bytes_read = match stdin.read(&mut buf).await { // writes to buffer and returns the # of bytes read
        Ok(n) => n,
        Err(_) | Ok(0) => panic!("Unable to Aquire Name"),
    };
    buf.truncate(bytes_read);
    match String::from_utf8(buf) {
        Ok(name) => name,
        Err(err) => String::from("Anonymous"), // Fallback
    }
}

async fn read_stdin(tx: futures_channel::mpsc::UnboundedSender<Message>, name: String) {
    let mut stdin = tokio::io::stdin();
    let mut stdout = tokio::io::stdout();
    loop {
        stdout.write_all(b"> ").await.unwrap();
        stdout.flush().await.unwrap();

        let mut buf = vec![0; 1024];
        let n = match stdin.read(&mut buf).await {
            Err(_) | Ok(0) => break,
            Ok(n) => n,
        };
        buf.truncate(n); // so buffer only is as big as the message ("hi" wont take 1kb)
        if let Ok(text) = String::from_utf8(buf) {
            // build a chat message and then send that
            let msg =  ChatMessage::new(text, &name);
            tx.unbounded_send(Message::text(serde_json::to_string(&msg).unwrap())).unwrap();
        }
    }
}
