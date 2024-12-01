use tokio::runtime::Runtime;
use std::sync::{Arc, Mutex};
use tokio::net::TcpListener;

mod memory;
mod sync;
mod ws_handler;

fn main() {
    // Initialize the runtime
    let runtime = Runtime::new().expect("Failed to create Tokio runtime");

    // Initialize memory state (shared across threads)
    let memory_state = Arc::new(Mutex::new(memory::initialize_memory()));

    // Run the async server inside the runtime
    runtime.block_on(async_main(memory_state));
}

async fn async_main(memory_state: Arc<Mutex<Vec<memory::MemoryBlock>>>) {
    // Start WebSocket listener
    let listener = TcpListener::bind("127.0.0.1:9090")
        .await
        .expect("Failed to bind");

    println!("WebSocket server listening on ws://127.0.0.1:9090");

    while let Ok((stream, _)) = listener.accept().await {
        let memory_state = Arc::clone(&memory_state); // Clone for each connection
        tokio::spawn(ws_handler::handle_connection(stream, memory_state)); // Spawn async task
    }
}
