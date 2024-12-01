use tokio_tungstenite::tungstenite::protocol::Message;
use tokio_tungstenite::accept_async;
use tokio::net::TcpStream;
use std::sync::{Arc, Mutex};
use futures_util::{StreamExt, SinkExt};
use crate::memory;
use crate::sync;

pub async fn handle_connection(stream: TcpStream, memory_state: Arc<Mutex<Vec<memory::MemoryBlock>>>) {
    // Perform WebSocket handshake
    let ws_stream = match accept_async(stream).await {
        Ok(stream) => stream,
        Err(e) => {
            eprintln!("WebSocket handshake failed: {}", e);
            return;
        }
    };

    // Separate initialization of WebSocket sender and receiver
    let (split_tx, split_rx) = ws_stream.split();
    let mut tx = split_tx; // Sender
    let mut rx = split_rx; // Receiver

    // Get the initial memory state
    let memory_data = sync::get_memory_state(memory_state.clone());
    let memory_json = serde_json::to_string(&memory_data).unwrap();
    println!("Sending initial memory data: {:?}", memory_json);

    // Send the memory state to the client
    if let Err(e) = tx.send(Message::Text(memory_json)).await {
        eprintln!("Failed to send initial memory state: {}", e);
        return;
    }

    // Handle incoming messages from the client
    while let Some(Ok(msg)) = rx.next().await {
        if let Message::Text(text) = msg {
            println!("Received message: {}", text);

            // Process the client's request
            let response = memory::handle_request(&text, &memory_state);
            println!("Sending response: {}", response);

            if let Err(e) = tx.send(Message::Text(response)).await {
                eprintln!("Failed to send response: {}", e);
                break;
            }
        }
    }

    println!("Client disconnected.");
}
