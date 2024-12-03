use eframe::egui;
use std::collections::HashMap;
use crate::memory::{initialize_memory, create_free_location_map, allocate_process_to_memory, compact_memory, MemoryBlock};

pub struct MemorySimulatorApp {
    memory: HashMap<String, MemoryBlock>,
    processes: Vec<i32>,
    input_size: String,
    log: String,
}

impl MemorySimulatorApp {
    pub fn new() -> Self {
        let initial_memory = initialize_memory();
        let mut memory_map = HashMap::new();
    
        for (key, value) in initial_memory {
            memory_map.insert(key, value);
        }
    
        Self {
            memory: memory_map,
            processes: vec![],
            input_size: String::new(),
            log: String::new(),
        }
    }
    
    fn calculate_memory_summary(&self) -> (i32, i32, i32) {
        let total: i32 = self.memory.values().map(|block| block.size).sum(); // Sum of all block sizes
        let occupied: i32 = self
            .memory
            .values()
            .filter(|block| block.allocated) // Only allocated blocks
            .map(|block| block.size)
            .sum();
        let free = total - occupied;
        (total, occupied, free)
    }

    fn draw_memory_visualization(&self, ui: &mut egui::Ui) {
        for (key, block) in &self.memory {
            let color = if block.allocated {
                egui::Color32::RED
            } else {
                egui::Color32::GREEN 
            };

            ui.horizontal(|ui| {
                ui.label(format!("{}: ", key)); 

                let bar_text = format!("{} KB", block.size);
                ui.add(
                    egui::ProgressBar::new(block.size as f32 / 1024.0) // Normalize block size
                        .fill(color)
                        .text(egui::RichText::new(bar_text).color(egui::Color32::BLACK)), // Set text inside the bar
                );

                ui.label(format!(
                    "Allocated: {}",
                    if block.allocated { "Yes" } else { "No" }
                ));
            });
        }
    }

}

impl eframe::App for MemorySimulatorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Memory Allocation Simulator");

            // Memory summary
            let (total, occupied, free) = self.calculate_memory_summary();
            ui.label(format!("Total Memory: {} KB", total));
            ui.label(format!("Occupied Memory: {} KB", occupied));
            ui.label(format!("Free Memory: {} KB", free));

            ui.separator();

            // Graphical Memory
            ui.heading("Memory Visualization:");
            self.draw_memory_visualization(ui);

            ui.separator();

            // Add input
            ui.horizontal(|ui| {
                ui.label("Process Size (KB):");
                ui.text_edit_singleline(&mut self.input_size);
            });

            if ui.button("Add Process").clicked() {
                if let Ok(size) = self.input_size.trim().parse::<i32>() {
                    self.processes.push(size);
                    self.log = format!("Added process of size {} KB.", size);
                } else {
                    self.log = "Invalid process size.".to_string();
                }
                self.input_size.clear();
            }

            ui.separator();

            // Start allocation
            if ui.button("Allocate Processes").clicked() {
                for process in &self.processes {
                    let result = allocate_process_to_memory(*process, &mut self.memory);
                    self.log = format!("{}\n{}", self.log, result);
                }
                self.processes.clear();
            }

            // Compaction
            if ui.button("Compact Memory").clicked() {
                compact_memory(&mut self.memory);
                self.log = format!("{}\nMemory compacted.", self.log);
            }

            ui.separator();

            // Log output
            ui.label("Logs:");
            ui.label(&self.log);
        });
    }
}
