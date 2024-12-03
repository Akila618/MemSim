use std::sync::{Arc,Mutex};
use std::collections::HashMap;
mod memory;
mod gui;

#[derive(Debug, Clone)]
pub struct ProjectInfo {
    pub student: String,
    pub reg: i32,
    pub technique: String,
}

// Selected algorithm for the project
pub fn memory_algorithm_selector() -> ProjectInfo {
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

    ProjectInfo {
        student: s_name.to_string(),
        reg: reg_no,
        technique: algorithm.to_string(),
    }
}



fn main() {
    // Student & project info
    println!("-----------------------------------------------------------------------------------");
    println!("Student_Name: {:?}", memory_algorithm_selector().student);
    println!("Reg_Number: {:?}", memory_algorithm_selector().reg);
    println!("Memory allocation algorithm: {:?}", memory_algorithm_selector().technique);
    println!("-----------------------------------------------------------------------------------");

    let mut free_blocks = memory::initialize_memory();
    let processes = vec![50, 30, 120];

    println!("Initial state of free blocks:");
    for (key, block) in &free_blocks {
        println!(
            "{}: Size = {}, Allocated = {}, Fragmentation = {:?}",
            key, block.size, block.allocated, block.fragmentation
        );
    }

    // handle_user_processes(processes, &mut free_blocks);

}

