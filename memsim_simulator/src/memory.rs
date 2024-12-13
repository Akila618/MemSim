use std::collections::HashMap;

/* struct to represent the memery block data*/
#[derive(Debug, Clone)]
pub struct MemoryBlock {
    pub partition: String,
    pub location: String,
    pub size: i32,
    pub allocated: bool,
    pub fragmentation: Option<Fragment>,
}

/* struct to get the fragmentation info of the block */
#[derive(Debug, Clone)]
pub struct Fragment {
    pub _original: i32,
    pub _occupied: i32,
    pub free: i32,
}

/* finitialize a hashmap of memry size and block info */
pub fn initialize_memory() -> HashMap<String, MemoryBlock> {
    let mut memory = HashMap::new();

    memory.insert(
        "Block 01".to_string(),
        MemoryBlock {partition: "A".to_string(), location: "Block 1".to_string(), size: 64, allocated: true, fragmentation: None,},
    );
    memory.insert(
        "Block 02".to_string(),
        MemoryBlock {partition: "B".to_string(), location: "Block 2".to_string(), size: 130, allocated: false, fragmentation: None,},
    );
    memory.insert(
        "Block 03".to_string(),
        MemoryBlock {partition: "C".to_string(), location: "Block 3".to_string(), size: 90, allocated: true, fragmentation: None,},
    );
    memory.insert(
        "Block 04".to_string(),
        MemoryBlock {partition: "D".to_string(), location: "Block 4".to_string(), size: 70, allocated: false, fragmentation: None,},
    );
    memory.insert(
        "Block 05".to_string(),
        MemoryBlock {partition: "E".to_string(), location: "Block 5".to_string(), size: 160, allocated: true, fragmentation: None,},
    );
    memory.insert(
        "Block 06".to_string(),
        MemoryBlock {partition: "F".to_string(), location: "Block 6".to_string(), size: 120, allocated: false, fragmentation: None,},
    );
    memory.insert(
        "Block 07".to_string(),
        MemoryBlock {partition: "G".to_string(), location: "Block 7".to_string(), size: 75, allocated: true, fragmentation: None,},
    );
    memory.insert(
        "Block 08".to_string(),
        MemoryBlock {partition: "H".to_string(), location: "Block 8".to_string(), size: 45, allocated: false, fragmentation: None,},
    );
    memory.insert(
        "Block 09".to_string(),
        MemoryBlock {partition: "I".to_string(), location: "Block 9".to_string(), size: 25, allocated: true, fragmentation: None,},
    );
    memory.insert(
        "Block 10".to_string(),
        MemoryBlock {partition: "J".to_string(), location: "Block 10".to_string(), size: 100, allocated: false, fragmentation: None,},
    );
    memory.insert(
        "Block 11".to_string(),
        MemoryBlock {partition: "K".to_string(), location: "Block 11".to_string(), size: 35, allocated: true, fragmentation: None,},
    );
    memory.insert(
        "Block 12".to_string(),
        MemoryBlock {partition: "L".to_string(), location: "Block 12".to_string(), size: 110, allocated: false, fragmentation: None,},
    );
    
    memory
}
// pub fn initialize_memory() -> Vec<(String, MemoryBlock)> {
//     vec![
//         ("Block 01".to_string(), MemoryBlock { partition: "A".to_string(), location: "Block 1".to_string(), size: 64, allocated: true, fragmentation: None }),
//         ("Block 02".to_string(), MemoryBlock { partition: "B".to_string(), location: "Block 2".to_string(), size: 130, allocated: false, fragmentation: None }),
//         ("Block 03".to_string(), MemoryBlock { partition: "C".to_string(), location: "Block 3".to_string(), size: 90, allocated: true, fragmentation: None }),
//         ("Block 04".to_string(), MemoryBlock { partition: "D".to_string(), location: "Block 4".to_string(), size: 70, allocated: false, fragmentation: None }),
//         ("Block 05".to_string(), MemoryBlock { partition: "E".to_string(), location: "Block 5".to_string(), size: 160, allocated: true, fragmentation: None }),
//         ("Block 06".to_string(), MemoryBlock { partition: "F".to_string(), location: "Block 6".to_string(), size: 120, allocated: false, fragmentation: None }),
//         ("Block 07".to_string(), MemoryBlock { partition: "G".to_string(), location: "Block 7".to_string(), size: 75, allocated: true, fragmentation: None }),
//         ("Block 08".to_string(), MemoryBlock { partition: "H".to_string(), location: "Block 8".to_string(), size: 45, allocated: false, fragmentation: None }),
//         ("Block 09".to_string(), MemoryBlock { partition: "I".to_string(), location: "Block 9".to_string(), size: 25, allocated: true, fragmentation: None }),
//         ("Block 10".to_string(), MemoryBlock { partition: "J".to_string(), location: "Block 10".to_string(), size: 100, allocated: false, fragmentation: None }),
//         ("Block 11".to_string(), MemoryBlock { partition: "K".to_string(), location: "Block 11".to_string(), size: 35, allocated: true, fragmentation: None }),
//         ("Block 12".to_string(), MemoryBlock { partition: "L".to_string(), location: "Block 12".to_string(), size: 110, allocated: false, fragmentation: None }),
//     ]
// }


/* filter out the available memory locations and create a hashmap */
pub fn _create_free_location_map(memory: Vec<MemoryBlock>) -> HashMap<String, MemoryBlock> {
    let mut free_locations: HashMap<String, MemoryBlock> = HashMap::new();

    for location in memory{
        let location_cloned = location.clone();
        if !location.allocated{
            free_locations.insert(location.partition, location_cloned);
        }
    }
    free_locations
}

/* perform best fit operation and aloocate process to block with less internal fragmentation */
pub fn allocate_process_to_memory(
    process_size: i32,
    memory: &mut HashMap<String, MemoryBlock>,
) -> String {
    let mut best_fit_key = None;
    let mut min_fragmentation = i32::MAX;

    // Find the best-fit block
    for (key, block) in memory.iter() {
        if !block.allocated && block.size >= process_size {
            let fragmentation = block.size - process_size;
            if fragmentation < min_fragmentation {
                min_fragmentation = fragmentation;
                best_fit_key = Some(key.clone());
            }
        }
    }

    match best_fit_key {
        Some(key) => {
            if let Some(block) = memory.get_mut(&key) {
                // update the block
                block.allocated = true;
                block.fragmentation = Some(Fragment {
                    _original: block.size,
                    _occupied: process_size,
                    free: min_fragmentation,
                });

                format!(
                    "Process allocated to {} with {} KB internal fragmentation.",
                    block.location, min_fragmentation
                )
            } else {
                "Block not found.".to_string()
            }
        }
        None => "No suitable block found.".to_string(),
    }
}



/* get a array of user inputs and call allocate_process_to_memory() function */
pub fn _handle_user_processes(
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

/* perform internal compaction */
pub fn compact_memory(free_blocks: &mut HashMap<String, MemoryBlock>) {
    let mut total_free_space = 0;
    let mut compacted_blocks: HashMap<String, MemoryBlock> = HashMap::new();

    for (_key, block) in free_blocks.iter_mut() {
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
    use crate::memory::initialize_memory;
    #[test]
    fn test_memory_size() {
        let memory = initialize_memory().clone();
        assert_eq!(memory.len(), 12);

    }

    #[test]
    fn test_block_allocated() {
        let memory = initialize_memory().clone();
        let block = memory.get("Block 01").unwrap();
        assert_eq!(block.size, 64);
        assert_eq!(block.allocated, true);
    }

    #[test]
    fn test_block_not_allocated(){
        let memory = initialize_memory().clone();
        let block = memory.get("Block 02").unwrap();   
        assert_eq!(block.size, 130);
        assert_eq!(block.allocated, false);
    }

    #[test]
    fn test_allocation(){
        let mut memory = initialize_memory();
        let result = super::allocate_process_to_memory(100, &mut memory);
        assert_eq!(result, "Process allocated to Block 10 with 0 KB internal fragmentation.");
    }

    #[test]
    fn test_size_exceed(){
        let mut memory = initialize_memory();
        let result = super::allocate_process_to_memory(200, &mut memory);
        assert_eq!(result, "No suitable block found.");
    }

    #[test]
    fn test_compaction(){
        let mut memory = initialize_memory();
        super::allocate_process_to_memory(100, &mut memory);
        super::compact_memory(&mut memory);
        let block = memory.get("New Free Block").unwrap();
        assert_eq!(block.size, 475);
    }
    
}

