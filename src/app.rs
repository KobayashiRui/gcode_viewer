#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::{num::NonZeroU64, sync::Arc};

use eframe::{
    egui_wgpu::wgpu::util::DeviceExt,
    egui_wgpu::{self, wgpu},
};

use eframe::egui;
use crate::utils;
//use gcode_viewer::utils;

mod gcode_path_3d;
mod gcode_text_editor_v2;
mod calculate_print_time;



pub struct MainApp {
    print_time: f32,
    gcode_path_3d: gcode_path_3d::GcodePath3d,
    gcode_text_editor: gcode_text_editor_v2::GcodeTextEditor,
    print_time_data: utils::TimeData,
    error_line: i32,
}

impl MainApp {
    pub fn new(cc: &eframe::CreationContext) -> Self {

        
        Self {
            print_time: 0.0,
            print_time_data: utils::TimeData::new(),
            gcode_path_3d: gcode_path_3d::GcodePath3d::new(cc),
            gcode_text_editor: gcode_text_editor_v2::GcodeTextEditor::new(),
            error_line: -1,
        }
    }
}

impl eframe::App for MainApp{
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        egui::SidePanel::left("left_panel")
            .show(ctx, |ui| {

                //ui.label("Gcode Viewer");
                //egui::ScrollArea::vertical().show(ui, |ui| {
                //    ui.label("Gcode Viewer");
                //    self.gcode_text_editor.update(ui)
                //});
                //Gcode text data

                ui.label("Gcode Viewer");

                ui.label(format!("Print time Sec:{}", self.print_time));
                ui.label(format!("Print time: {}d, {}h, {}m", self.print_time_data.day, self.print_time_data.hour, self.print_time_data.minute));
                if ui.add(egui::Button::new("Calculate Time")).clicked() {
                    self.error_line = -1;
                    let result = calculate_print_time::calculate_print_time(self.gcode_text_editor.get_gcode_data());
                    match result {
                        Ok(n) => {
                            self.print_time = n;
                            self.print_time_data =utils::sec_to_days_hours_minutes(self.print_time);
                        },
                        Err(e)=>{
                            self.error_line = e;
                        }
                    }
                }
                if self.error_line != -1 {
                    ui.label(format!("Error occurred on line {} !!", self.error_line));
                }
                
                self.gcode_text_editor.update(ui)
                //self.gcode_text_editor.update(ui)
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::both()
                .auto_shrink([false; 2])
                .show(ui, |ui| {
                    egui::Frame::canvas(ui.style()).show(ui, |ui| {
                        //self.custom_painting(ui);
                        self.gcode_path_3d.custom_painting(ui);
                    });
                    ui.label("Drag to rotate!");
            });
        });
    }
}
