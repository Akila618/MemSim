use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use rand::Rng;
use std::collections::HashMap;

// create the struct for the Memory block
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MemoryBlock {
    pub partition: String,
    pub location: String,
    pub size: i32,
    pub allocated: bool,
    pub fragmentation: Option<Fragment>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Fragment{
    pub original: i32,
    pub occupied: i32,
    pub free: i32
}
// create a pre defined number of memory blocks with definitions
pub fn initialize_memory() -> Vec<MemoryBlock> {
    let mut memory_blocks =  vec![
        MemoryBlock {partition: "partition_01".to_string(), location: "location 001".to_string(),  size: 100, allocated: false, fragmentation: None },
        MemoryBlock {partition: "partition_02".to_string(), location: "location 002".to_string(), size: 200, allocated: true, fragmentation: None },
        MemoryBlock {partition: "partition_03".to_string(), location: "location 003".to_string(),  size: 100, allocated: false, fragmentation: None },
        MemoryBlock {partition: "partition_04".to_string(), location: "location 004".to_string(), size: 200, allocated: true, fragmentation: None },
        MemoryBlock {partition: "partition_05".to_string(), location: "location 005".to_string(),  size: 100, allocated: false, fragmentation: None },
        MemoryBlock {partition: "partition_06".to_string(), location: "location 006".to_string(), size: 200, allocated: true, fragmentation: None },
        MemoryBlock {partition: "partition_07".to_string(), location: "location 007".to_string(),  size: 100, allocated: false, fragmentation: None },
        MemoryBlock {partition: "partition_08".to_string(), location: "location 008".to_string(), size: 200, allocated: true, fragmentation: None },
        MemoryBlock {partition: "partition_09".to_string(), location: "location 009".to_string(),  size: 100, allocated: false, fragmentation: None },
        MemoryBlock {partition: "partition_10".to_string(), location: "location 010".to_string(), size: 200, allocated: true, fragmentation: None },
    ];

    memory_blocks
}

// Function that allocates a random memery values for the memory locations
pub fn random_memory_allocator() -> Vec<MemoryBlock> {
    let init_memory = initialize_memory();
    let mut initial_vec: Vec<MemoryBlock> = Vec::new();
    let mut memory_capacity: i32 = 1024; // Total memory available
    let mut rng = rand::thread_rng(); // Random number generator

    for (i, item) in init_memory.into_iter().enumerate() {
        let mem_value: i32;

        if i < 9 {
            // Generate random memory values for the first 9 blocks
            let random_number: i32 = rng.gen_range(10..=30) * 5;
            println!("Random number for {}: {}", item.location, random_number);
            memory_capacity = memory_capacity.saturating_sub(random_number as i32); 
            mem_value = random_number;
        } else {
            // Assign remaining memory to the last block
            mem_value = memory_capacity;
        }

        // Create a new memory block with updated values
        let latest_mem_block = MemoryBlock {
            partition: item.partition,
            location: item.location,
            size: mem_value,
            allocated: item.allocated,
            fragmentation: None
        };

        initial_vec.push(latest_mem_block);
    }

    initial_vec
}

// Function to filter out the available memory locations and create a hashmap
fn create_free_location_map(memory: Vec<MemoryBlock>) -> HashMap<String, MemoryBlock> {
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
fn allocate_process_to_memory(process_size: i32, free_blocks: &mut HashMap<String, MemoryBlock>
) -> String {

    let mut freed_size : HashMap<String, i32> = HashMap::new();

    for (key, block) in free_blocks.iter(){
        if !block.allocated && block.size > process_size{
            println!("The process fits for the memory block. Allocating memory for process ....."); 
        }
        else if !block.allocated && block.size == process_size{
            println!("The process fits for the memory block. Allocating memory for process .....");
        }
        else if  !block.allocated && block.size < process_size{
            println!("The process fits for the memory block. Allocating memory for process .....");
        }
        else {
            println!("The process fits for the memory block. Allocating memory for process .....");
        }

    }
    "Hi".to_string()
}


/* Unit tests for functions */
#[cfg(test)]
mod unit_test_memory {
    use super::*;
    #[test]
    fn test_random_memory() {

        // test 1
        let ram = random_memory_allocator();
        assert!(!ram.is_empty());
        println!("random mem: {:#?}", ram);

        //test 2
        let free_mem = create_free_location_map(initialize_memory());
        println!("locations available: >>>>>>>> {:#?}", free_mem);
    }
}

