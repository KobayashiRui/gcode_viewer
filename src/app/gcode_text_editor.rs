#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::{num::NonZeroU64, sync::Arc};

use eframe::{
    egui_wgpu::wgpu::util::DeviceExt,
    egui_wgpu::{self, wgpu},
};

use eframe::egui;
use egui::Color32;
use egui_extras::{Size, StripBuilder};

pub struct GcodeTextEditor {
    gcode_data: String
} 

fn count_lines(s: &String) -> usize {
    s.chars().filter(|&c| c == '\n').count() + 1
}

fn line_size_to_string(s: usize) -> String {

    let src: Vec<usize> = (1..(s+1)).collect();
    let dst: Vec<String> = src.iter().map(|x| x.to_string()).collect();
    dst.join("\n")


}

impl GcodeTextEditor {
    pub fn new() -> Self{

        Self{
            gcode_data: "G1 X100 \nG1 X200".to_string()
        }
    }

    pub fn update(&mut self, ui: &mut egui::Ui){
        let Self { gcode_data } = self;

        let dark_mode = ui.visuals().dark_mode;
        let faded_color = ui.visuals().window_fill();
        let faded_color = |color: Color32| -> Color32 {
            use egui::Rgba;
            let t = if dark_mode { 0.95 } else { 0.8 };
            egui::lerp(Rgba::from(color)..=Rgba::from(faded_color), t).into()
        };

        ui.spacing_mut().item_spacing = egui::vec2(0.0, 0.0);

        StripBuilder::new(ui)
            .size(Size::remainder())
            .size(Size::remainder())
            .vertical(|mut strip| {
                strip.strip(|builder| {
                    //builder.sizes(Size::remainder(), 2).horizontal(|mut strip| {
                    builder
                        .size(Size::exact(50.0))
                        .size(Size::exact(150.0))
                        .horizontal(|mut strip| {
                            strip.cell(|ui| {
                                ui.spacing_mut().item_spacing = egui::vec2(0.0, 0.0);  
                                ui.label(line_size_to_string(count_lines(gcode_data)));
                                //ui.add_sized(
                                //    ui.available_size(), 
                                //    egui::TextEdit::multiline(&mut line_size_to_string(count_lines(gcode_data))));
                            });
                            strip.cell(|ui| {
                                ui.spacing_mut().item_spacing = egui::vec2(1.0, 0.0);  
                                ui.add_sized(ui.available_size(), egui::TextEdit::multiline(gcode_data));
                            });
                    });
                });
                strip.strip(|builder| {
                    builder
                        .size(Size::remainder())
                        .size(Size::remainder())
                        .horizontal(|mut strip| {
                            strip.cell(|ui| {
                                ui.spacing_mut().item_spacing = egui::vec2(0.0, 0.0);  
                                ui.painter().rect_filled(
                                    ui.available_rect_before_wrap(),
                                    0.0,
                                    faded_color(Color32::BLUE),
                                );
                                ui.label("width: 100%\nheight: 50px");
                            });
                            strip.cell(|ui| {
                                ui.spacing_mut().item_spacing = egui::vec2(0.0, 0.0);  
                                ui.painter().rect_filled(
                                    ui.available_rect_before_wrap(),
                                    0.0,
                                    faded_color(Color32::BLUE),
                                );
                                ui.label("width: 100%\nheight: 50px");
                            });
                    });
                });
            });
        //egui::Grid::new("gcode_data")
        //    .spacing(egui::Vec2::new(0.0, 0.0))
        //    .striped(true)
        //    //.min_col_width(60.0)
        //    //.max_col_width(200.0)
        //    .show(ui, |ui| {
        //        ui.spacing_mut().item_spacing = egui::Vec2::ZERO; 
        //        ui.add(egui::TextEdit::multiline(&mut "1\n2\n".to_string())
        //            .desired_width(f32::INFINITY)
        //        );

        //        ui.add(egui::TextEdit::multiline(gcode_data)
        //            .desired_width(f32::INFINITY)
        //        );
        //        ui.end_row();
        //    });

    }
}
