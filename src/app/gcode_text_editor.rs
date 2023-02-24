#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::{num::NonZeroU64, sync::Arc};

use eframe::{
    egui_wgpu::wgpu::util::DeviceExt,
    egui_wgpu::{self, wgpu},
};

use eframe::egui;

pub struct GcodeTextEditor {
    gcode_data: String
} 

impl GcodeTextEditor {
    pub fn new() -> Self{

        Self{
            gcode_data: "".to_string()
        }
    }

    pub fn update(&mut self, ui: &mut egui::Ui){
        egui::Grid::new("gcode_data")
            .show(ui, |ui| {
                ui.label("First row, first column");
                ui.label("First row, second column");
                ui.end_row();
        
                ui.label("Second row, first column");
                ui.label("Second row, second column");
                ui.label("Second row, third column");
                ui.end_row();
        
                ui.horizontal(|ui| { ui.label("Same"); ui.label("cell"); });
                ui.label("Third row, second column");
                ui.end_row();
            });

    }
}
