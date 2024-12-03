use std::sync::{Arc,Mutex};
mod memory;
mod gui;


/* Selected memory allocation technique */
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

    // Initialize memory state (shared across threads)
    let memory_state = Arc::new(Mutex::new(memory::initialize_memory()));

}

