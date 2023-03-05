#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::{num::NonZeroU64, sync::Arc};

use eframe::{
    egui_wgpu::wgpu::util::DeviceExt,
    egui_wgpu::{self, wgpu},
};

use eframe::egui;

mod gcode_path_3d;
mod gcode_text_editor_v2;

pub struct MainApp {
    gcode_path_3d: gcode_path_3d::GcodePath3d,
    gcode_text_editor: gcode_text_editor_v2::GcodeTextEditor,
}

impl MainApp {
    pub fn new(cc: &eframe::CreationContext) -> Self {
        
        Self {
            gcode_path_3d: gcode_path_3d::GcodePath3d::new(cc),
            gcode_text_editor: gcode_text_editor_v2::GcodeTextEditor::new(),
        }
    }
}

impl eframe::App for MainApp {
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
