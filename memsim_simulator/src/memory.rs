use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use rand::Rng;
use std::collections::HashMap;



#[derive(Debug, Clone)]
pub struct MemoryBlock {
    pub partition: String,
    pub location: String,
    pub size: i32,
    pub allocated: bool,
    pub fragmentation: Option<Fragment>,
}

#[derive(Debug, Clone)]
pub struct Fragment {
    pub original: i32,
    pub occupied: i32,
    pub free: i32,
}

pub fn initialize_memory() -> HashMap<String, MemoryBlock> {
    let mut memory = HashMap::new();
    memory.insert(
        "Block 1".to_string(),
        MemoryBlock {partition: "A".to_string(), location: "Block 1".to_string(), size: 60, allocated: true, fragmentation: None,},
    );
    memory.insert(
        "Block 2".to_string(),
        MemoryBlock {partition: "B".to_string(), location: "Block 2".to_string(), size: 130, allocated: false, fragmentation: None,},
    );
    memory.insert(
        "Block 3".to_string(),
        MemoryBlock {partition: "C".to_string(), location: "Block 3".to_string(), size: 90, allocated: true, fragmentation: None,},
    );
    memory.insert(
        "Block 4".to_string(),
        MemoryBlock {partition: "D".to_string(), location: "Block 4".to_string(), size: 50, allocated: false, fragmentation: None,},
    );
    memory.insert(
        "Block 5".to_string(),
        MemoryBlock {partition: "E".to_string(), location: "Block 5".to_string(), size: 160, allocated: true, fragmentation: None,},
    );
    memory.insert(
        "Block 6".to_string(),
        MemoryBlock {partition: "F".to_string(), location: "Block 6".to_string(), size: 120, allocated: false, fragmentation: None,},
    );
    memory.insert(
        "Block 7".to_string(),
        MemoryBlock {partition: "G".to_string(), location: "Block 7".to_string(), size: 75, allocated: true, fragmentation: None,},
    );
    memory.insert(
        "Block 8".to_string(),
        MemoryBlock {partition: "H".to_string(), location: "Block 8".to_string(), size: 40, allocated: false, fragmentation: None,},
    );
    memory.insert(
        "Block 9".to_string(),
        MemoryBlock {partition: "I".to_string(), location: "Block 9".to_string(), size: 25, allocated: true, fragmentation: None,},
    );
    memory.insert(
        "Block 10".to_string(),
        MemoryBlock {partition: "J".to_string(), location: "Block 10".to_string(), size: 100, allocated: false, fragmentation: None,},
    );
    
    memory
}


// Function that allocates a random memery values for the memory locations
// pub fn random_memory_allocator() -> Vec<MemoryBlock> {
//     let init_memory = initialize_memory();
//     let mut initial_vec: Vec<MemoryBlock> = Vec::new();
//     let mut memory_capacity: i32 = 1024; 
//     let mut rng = rand::thread_rng();

//     for (i, item) in init_memory.into_iter().enumerate() {
//         let mem_value: i32;

//         if i < 9 {
//             let random_number: i32 = rng.gen_range(10..=30) * 5;
//             println!("Random number for {}: {}", item.location, random_number);
//             memory_capacity = memory_capacity.saturating_sub(random_number as i32); 
//             mem_value = random_number;
//         } else {
//             mem_value = memory_capacity;
//         }

//         // let latest_mem_block = MemoryBlock {
//         //     partition: item.partition,
//         //     location: item.location,
//         //     size: mem_value,
//         //     allocated: item.allocated,
//         //     fragmentation: None
//         // };

//         initial_vec.push(latest_mem_block);
//     }

//     initial_vec
// }

// Function to filter out the available memory locations and create a hashmap
pub fn create_free_location_map(memory: Vec<MemoryBlock>) -> HashMap<String, MemoryBlock> {
    let mut free_locations: HashMap<String, MemoryBlock> = HashMap::new();

    for location in memory{
        let location_cloned = location.clone();
        if !location.allocated{
            free_locations.insert(location.partition, location_cloned);
        }
    }
    free_locations
}

// Function to allocate the process to the relevent memory
// pub fn allocate_process_to_memory(
//     process_size: i32,
//     free_blocks: &mut HashMap<String, MemoryBlock>,
// ) -> String {
//     let mut best_fit_block: Option<&mut MemoryBlock> = None;
//     let mut min_fragmentation = i32::MAX; // Start with a large value for comparison
//     let mut best_fit_key: Option<String> = None;

//     for (key, block) in free_blocks.iter_mut() {
//         if !block.allocated && block.size >= process_size {
//             let free_space = block.size - process_size;

//             // Check for best fit (minimum internal fragmentation)
//             if free_space < min_fragmentation {
//                 min_fragmentation = free_space;
//                 best_fit_block = Some(block);
//                 best_fit_key = Some(key.clone());
//             }
//         }
//     }

//     match best_fit_block {
//         Some(block) => {
//             // Update the block
//             block.allocated = true;
//             block.fragmentation = Some(Fragment {
//                 original: block.size,
//                 occupied: process_size,
//                 free: min_fragmentation,
//             });

//             // Remove the block from the free blocks list
//             if let Some(key) = best_fit_key {
//                 free_blocks.remove(&key);
//             }

//             format!(
//                 "Process allocated to {} with {} KB internal fragmentation.",
//                 block.location, min_fragmentation
//             )
//         }
//         None => "No suitable block found.".to_string(),
//     }
// }

fn allocate_process_to_memory(
    process_size: i32,
    free_blocks: &mut HashMap<String, MemoryBlock>,
) -> String {
    let mut best_fit_key: Option<String> = None;
    let mut min_fragmentation = i32::MAX; // Start with a large value for comparison

    for (key, block) in free_blocks.iter() {
        if !block.allocated && block.size >= process_size {
            let free_space = block.size - process_size;

            if free_space < min_fragmentation {
                min_fragmentation = free_space;
                best_fit_key = Some(key.clone());
            }
        }
    }

    match best_fit_key {
        Some(key) => {
            // Get a mutable reference to the block
            if let Some(block) = free_blocks.get_mut(&key) {
                // Update the block
                block.allocated = true;
                block.fragmentation = Some(Fragment {
                    original: block.size,
                    occupied: process_size,
                    free: min_fragmentation,
                });
            }

            free_blocks.remove(&key);

            format!(
                "Process allocated to {} with {} KB internal fragmentation.",
                key, min_fragmentation
            )
        }
        None => "No suitable block found.".to_string(),
    }
}



//function to get a array of user inputs and call allocate_process_to_memory() function
pub fn handle_user_processes(
    processes: Vec<i32>,
    free_blocks: &mut HashMap<String, MemoryBlock>,
) {
    for (i, process_size) in processes.iter().enumerate() {
        println!("\nAllocating Process {} with size {} KB:", i + 1, process_size);
        let result = allocate_process_to_memory(*process_size, free_blocks);
        println!("{}", result);
    }

    println!("\nDo you want to compact the memory? (yes/no):");
    let mut response = String::new();
    std::io::stdin()
        .read_line(&mut response)
        .expect("Failed to read input");

    if response.trim().eq_ignore_ascii_case("yes") {
        compact_memory(free_blocks);
    } else {
        println!("Compaction skipped.");
    }

    println!("\nFinal state of memory:");
    for (key, block) in free_blocks {
        println!(
            "{}: Size = {}, Allocated = {}, Fragmentation = {:?}",
            key, block.size, block.allocated, block.fragmentation
        );
    }
}

pub fn compact_memory(free_blocks: &mut HashMap<String, MemoryBlock>) {
    let mut total_free_space = 0;
    let mut compacted_blocks: HashMap<String, MemoryBlock> = HashMap::new();

    for (key, block) in free_blocks.iter_mut() {
        if block.allocated {
            // Add internal fragmentation to total free space
            if let Some(frag) = &block.fragmentation {
                total_free_space += frag.free;
            }
        } else {
            // Add unallocated block size to total free space
            total_free_space += block.size;
        }
    }

    let new_block = MemoryBlock {
        partition: "Compacted".to_string(),
        location: "New Free Block".to_string(),
        size: total_free_space,
        allocated: false,
        fragmentation: None,
    };

    compacted_blocks.insert(new_block.location.clone(), new_block);

    *free_blocks = compacted_blocks;

    println!(
        "Compaction completed. New free block created with {} KB.",
        total_free_space
    );
}



/* Unit tests for functions */
#[cfg(test)]
mod unit_test_memory {
    use super::*;
    #[test]
    fn test_random_memory() {

        // test 1
        // let ram = random_memory_allocator();
        // assert!(!ram.is_empty());
        // println!("random mem: {:#?}", ram);

        //test 2
        // let free_mem = create_free_location_map(initialize_memory());
        // println!("locations available: >>>>>>>> {:#?}", free_mem);
    }
}

