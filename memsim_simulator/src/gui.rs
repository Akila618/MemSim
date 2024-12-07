use eframe::egui;
use std::collections::HashMap;
use crate::memory::{initialize_memory, allocate_process_to_memory, compact_memory, MemoryBlock};

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
        let mut total = 0;
        let mut occupied = 0;
    
        for block in self.memory.values() {
            total += block.size;
            if block.allocated {
                occupied += block.size;
            }
        }
    
        let free = total - occupied;
        (total, occupied, free)
    }
    

    fn draw_memory_visualization(&self, ui: &mut egui::Ui) {
        ui.heading("Memory Visualization");

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
                    egui::ProgressBar::new(block.size as f32 / 500.0)
                        .fill(color)
                        .text(egui::RichText::new(bar_text).color(egui::Color32::BLACK)),
                );
            });
        }
    }

    fn draw_memory_table(&self, ui: &mut egui::Ui) {
        ui.heading("Memory Blocks Status Table");
    
        egui::Grid::new("memory_table")
            .striped(true)
            .show(ui, |ui| {
                ui.label("EEX5563 - Computer Architecture and OS");
                ui.end_row();
                ui.label("W.M.T. Wanninayake");
                ui.end_row();
                ui.label("Reg: 32142845");
                ui.end_row();
                ui.separator();
                ui.end_row();

                ui.label("Block Name");
                ui.label("Size (KB)");
                ui.label("Allocated");
                ui.label("Internal Fragmentation (KB)");
                ui.end_row();
    
                for (key, block) in &self.memory {
                    ui.label(key); // Block Name
                    ui.label(format!("{}", block.size)); // Size
                    ui.label(if block.allocated { "Yes" } else { "No" }); // Allocated
                    let fragmentation = block
                        .fragmentation
                        .as_ref()
                        .map_or(0, |frag| frag.free); // Internal Fragmentation
                    ui.label(format!("{}", fragmentation));
                    ui.end_row();
                }
            });
    }
    

}

impl eframe::App for MemorySimulatorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("memory_table_panel") // Left panel for the table
            .resizable(true)
            .show(ctx, |ui| {
                self.draw_memory_table(ui); // Draw memory table
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("MEMSIM: Best-Fit Memory Allocation Simulator");
            ui.separator();

            let (total, occupied, free) = self.calculate_memory_summary();
            ui.label(format!("Total Memory: {} KB", total));
            ui.label(format!("Occupied Memory: {} KB", occupied));
            ui.label(format!("Free Memory: {} KB", free));

            ui.separator();
            self.draw_memory_visualization(ui);

            ui.separator();
            ui.horizontal(|ui| {
                ui.label("Process Size (KB):");
                ui.text_edit_singleline(&mut self.input_size);
            });

            if ui.add(
                egui::Button::new("Add Process")
                    .fill(egui::Color32::from_rgb(0, 128, 255))
                    .min_size(egui::vec2(120.0, 25.0)),
            )
            .clicked()
            {
                if let Ok(size) = self.input_size.trim().parse::<i32>() {
                    self.processes.push(size);
                    self.log = format!("Added process of size {} KB.", size);
                } else {
                    self.log = "Invalid process size.".to_string();
                }
                self.input_size.clear();
            }

            ui.separator();

            if ui.add(
                egui::Button::new("Allocate Processes")
                    .fill(egui::Color32::from_rgb(34, 177, 76))
                    .min_size(egui::vec2(120.0, 25.0)),
            )
            .clicked()
            {
                for process in &self.processes {
                    let result = allocate_process_to_memory(*process, &mut self.memory);
                    self.log = format!("{}\n{}", self.log, result);
                }
                self.processes.clear();
            }

            if ui.add(
                egui::Button::new("Compact Memory")
                    .fill(egui::Color32::from_rgb(255, 69, 0))
                    .min_size(egui::vec2(120.0, 25.0)),
            )
            .clicked()
            {
                compact_memory(&mut self.memory);
                self.log = format!("{}\nMemory compacted.", self.log);
            }

            ui.separator();

            ui.label("Logs:");
            ui.label(&self.log);

            ui.separator();
        });
    }
}