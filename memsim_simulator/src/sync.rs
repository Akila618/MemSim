use crate::memory::MemoryBlock;
use std::sync::{Arc, Mutex};

pub fn get_memory_state(memory_state: Arc<Mutex<Vec<MemoryBlock>>>) -> Vec<MemoryBlock> {
    let memory = memory_state.lock().unwrap();
    memory.clone()
}
