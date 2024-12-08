mod memory;
mod gui;
mod bestfit;
use eframe::NativeOptions;

/* select algorithm for the project and show student info */
pub fn memory_algorithm_selector() {
    /* Student information */
    let s_name = "W.M.A.T. Wanninayake";
    let reg_no = 321428456;

    let algorithm = match reg_no % 6 {
        0 => "First Fit",
        1 => "Next Fit",
        2 => "Best Fit",
        3 => "Worst Fit",
        4 => "Buddy System",
        5 => "Quick Fit",
        _ => "",
    };

    println!("Student: {}", s_name);
    println!("Reg: {}", reg_no); 
    println!("Technique: {}", algorithm);

}

fn main() {

    memory_algorithm_selector();  
    let _options = NativeOptions::default();
    let _ = eframe::run_native(
        "Memory Allocation Simulator",
        eframe::NativeOptions::default(),
        Box::new(|_cc| {
            let app = gui::MemorySimulatorApp::new();
            Ok(Box::new(app))
        }),
    );
    

    /* Below commented code can be executed to get the results using the CMD */

    // let memory_blocks = memory::initialize_memory();
    // let mut locations = Vec::new();
    // println!("Initial memory blocks:");
    // for (name, block) in &memory_blocks {
    //     locations.push(block.clone());
    //     println!(
    //         "Partition: {}, Location: {}, Size: {} KB, Allocated: {}, Fragmentation: {:?}",
    //         block.partition, block.location, block.size, block.allocated, block.fragmentation
    //     );
    // }

    // let mut free_blocks = memory::create_free_location_map(locations);
    // println!("\nFree memory locations:");
    // for (key, block) in &free_blocks {
    //     println!(
    //         "Partition: {}, Location: {}, Size: {} KB",
    //         key, block.location, block.size
    //     );
    // }

    // println!("\nEnter the number of processes:");
    // let mut num_processes = String::new();
    // io::stdin()
    //     .read_line(&mut num_processes)
    //     .expect("Failed to read input");
    // let num_processes: usize = num_processes.trim().parse().expect("Enter a valid number");

    // let mut processes = Vec::new();
    // for i in 1..=num_processes {
    //     println!("Enter size for Process {} (in KB):", i);
    //     let mut process_size = String::new();
    //     io::stdin()
    //         .read_line(&mut process_size)
    //         .expect("Failed to read input");
    //     let process_size: i32 = process_size.trim().parse().expect("Enter a valid number");
    //     processes.push(process_size);
    // }

    // println!("\nAllocating processes...");
    // memory::handle_user_processes(processes, &mut free_blocks);

    // println!("\nFinal state of memory blocks:");
    // for (key, block) in &free_blocks {
    //     println!(
    //         "Partition: {}, Location: {}, Size: {} KB, Allocated: {}, Fragmentation: {:?}",
    //         key, block.location, block.size, block.allocated, block.fragmentation
    //     );
    // }

}
