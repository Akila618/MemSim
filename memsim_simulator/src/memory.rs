use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use rand::{random, Rng};
use log::Log;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MemoryBlock {
    pub location: String,
    pub size: usize,
    pub allocated: bool,
}

pub fn initialize_memory() -> Vec<MemoryBlock> {
    let mut memory_blocks =  vec![
        MemoryBlock {location: "location 001".to_string(),  size: 100, allocated: false },
        MemoryBlock {location: "location 002".to_string(), size: 200, allocated: true },
        MemoryBlock {location: "location 003".to_string(),  size: 100, allocated: false },
        MemoryBlock {location: "location 004".to_string(), size: 200, allocated: true },
        MemoryBlock {location: "location 005".to_string(),  size: 100, allocated: false },
        MemoryBlock {location: "location 006".to_string(), size: 200, allocated: true },
        MemoryBlock {location: "location 007".to_string(),  size: 100, allocated: false },
        MemoryBlock {location: "location 008".to_string(), size: 200, allocated: true },
        MemoryBlock {location: "location 009".to_string(),  size: 100, allocated: false },
        MemoryBlock {location: "location 010".to_string(), size: 200, allocated: true },
    ];

    memory_blocks
}

// To assign random memory values to the locations
// pub fn random_memory_allocator() -> Vec<MemoryBlock>{
//     let init_memory = initialize_memory();
//     let mut initial_vec:Vec<MemoryBlock> = Vec::new();

//     let mut memory_capacity = 1000;
//     let mut counter = 50;
//     let mut rng = rand::thread_rng();
//     for item in init_memory{
//         let mut mem_value = 0;        
//         if counter <= 9{
//             let random_number: u32 = rng.gen_range(10..=20)*5 ;
//             println!("Random number: {:#?}", &random_number);
//             memory_capacity -= &random_number;
//             mem_value = random_number as usize;
//             counter += 1;
//         }
//         else {
//             mem_value = memory_capacity as usize;
//         }

//         let latest_mem_block = MemoryBlock{
//             location: item.location,
//             size: mem_value,
//             allocated: item.allocated,
//         };

//         initial_vec.push(latest_mem_block);
//     }

//     initial_vec
// }

pub fn random_memory_allocator() -> Vec<MemoryBlock> {
    let init_memory = initialize_memory();
    let mut initial_vec: Vec<MemoryBlock> = Vec::new();
    let mut memory_capacity: i32 = 1000; // Total memory available
    let mut rng = rand::thread_rng(); // Random number generator

    for (i, item) in init_memory.into_iter().enumerate() {
        let mem_value: usize;

        if i < 9 {
            // Generate random memory values for the first 9 blocks
            let random_number: u32 = rng.gen_range(10..=20) * 5;
            println!("Random number for {}: {}", item.location, random_number);
            memory_capacity = memory_capacity.saturating_sub(random_number as i32); 
            mem_value = random_number as usize;
        } else {
            // Assign remaining memory to the last block
            mem_value = memory_capacity as usize;
        }

        // Create a new memory block with updated values
        let latest_mem_block = MemoryBlock {
            location: item.location,
            size: mem_value,
            allocated: item.allocated,
        };

        initial_vec.push(latest_mem_block);
    }

    initial_vec
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


/* Unit tests for functions */
#[cfg(test)]
mod unit_test_memory {
    use super::*;
    #[test]
    fn test_random_memory() {
        let ram = random_memory_allocator();
        print!("random mem: {:#?}", ram);
    }
}

