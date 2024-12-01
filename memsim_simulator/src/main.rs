use tokio::runtime::Runtime;
use std::sync::{Arc, Mutex};
use tokio::net::TcpListener;

mod memory;
mod sync;
mod ws_handler;


/* Selected memory allocation technique */
pub struct project_info{
    pub student: String,
    pub reg: i32,
    pub technique: String,
}

// selected algorithm for the project
pub fn memory_algorithm_selector() -> project_info{
    /* Student information */
    let s_name = "W.M.A.T. Wanninayake";
    let reg_no = 321428456;
    let s_no = "S92068456";

    let mut algorithm = "";
    let result = reg_no % 6;
    match result {
        0 => algorithm = "First Fit ",
        1 => algorithm = "Next Fit",
        2 => algorithm = "Best Fit",
        3 => algorithm = "Worst Fit",
        4 => algorithm = "Buddy System",
        5 => algorithm = "Quick Fit",
        _ => algorithm = ""
    }

    project_info{
        student: s_name.to_string(),
        reg: reg_no,
        technique: algorithm.to_string(),
    }
}

fn main() {
    //student & project info
    println!("-----------------------------------------------------------------------------------");
    println!("Student_Name: {:#?}", memory_algorithm_selector().student);
    println!("Reg_Number: {:#?}", memory_algorithm_selector().reg);
    println!("Memory allocation algorithm: {:#?}", memory_algorithm_selector().technique);
    println!("-----------------------------------------------------------------------------------");



    // Initialize the runtime
    let runtime = Runtime::new().expect("Failed to create Tokio runtime");
    // Initialize memory state (shared across threads)
    let memory_state = Arc::new(Mutex::new(memory::initialize_memory()));
    // Run the async server inside the runtime
    runtime.block_on(establish_connection(memory_state));
}

async fn establish_connection(memory_state: Arc<Mutex<Vec<memory::MemoryBlock>>>) {
    // Start WebSocket listener
    let listener = TcpListener::bind("127.0.0.1:9090")
        .await
        .expect("Failed to bind");

    println!("WebSocket server listening on ws://127.0.0.1:9090");

    while let Ok((stream, _)) = listener.accept().await {
        let memory_state = Arc::clone(&memory_state);
        tokio::spawn(ws_handler::handle_connection(stream, memory_state));
    }
}
