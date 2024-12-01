use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

#[derive(Serialize, Deserialize, Clone)]
pub struct MemoryBlock {
    pub location: String,
    pub size: usize,
    pub allocated: bool,
}

pub fn initialize_memory() -> Vec<MemoryBlock> {
    vec![
        MemoryBlock {location: "location 001".to_string(),  size: 100, allocated: false },
        MemoryBlock {location: "location 002".to_string(), size: 200, allocated: false },
    ]
}

pub fn ranldom_memry_allocation() -> Vec<MemoryBlock>{
    let mut initial_vec = Vec::new();
    return initial_vec
}

pub fn handle_request(request: &str, memory_state: &Arc<Mutex<Vec<MemoryBlock>>>) -> String {
    // Lock the memory state (with error handling for lock poisoning)
    let mut memory = match memory_state.lock() {
        Ok(mem) => mem,
        Err(_) => return "Failed to access memory state".to_string(),
    };

    for block in memory.iter_mut() {
        if !block.allocated && block.size >= 50 {
            block.allocated = true;
            return "Memory allocated".to_string();
        }
    }

    return "No suitable block".to_string();
}

