#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use core::num;
use std::{num::NonZeroU64, sync::Arc};
use std::fs;
use std::path::Path;

use eframe::{
    egui_wgpu::wgpu::util::DeviceExt,
    egui_wgpu::{self, wgpu},
    epaint,
};

use eframe::egui;
use egui::Color32;

use egui_extras::{Size, StripBuilder};

pub struct GcodeTextEditor{
    gcode_data: String,
    line_data:  String,
    selected_row:  usize,
    picked_path: Option<String>,
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
            gcode_data: "G1 X100 \nG1 X200".to_string(),
            line_data: "1\n2\n".to_string(),
            selected_row: 0,
            picked_path: None,
        }
    }


    pub fn get_gcode_data(&self) -> &String {
        &self.gcode_data
    }

    pub fn update(&mut self, ui: &mut egui::Ui){
        //let Self {mut gcode_data , mut selected_row, mut picked_path} = self;

        let dark_mode = ui.visuals().dark_mode;
        let faded_color = ui.visuals().window_fill();
        let faded_color = |color: Color32| -> Color32 {
            use egui::Rgba;
            let t = if dark_mode { 0.95 } else { 0.8 };
            egui::lerp(Rgba::from(color)..=Rgba::from(faded_color), t).into()
        };

        ui.label("Gcode File");
        if ui.button("Open file…").clicked() {
            if let Some(path) = rfd::FileDialog::new().pick_file() {
                self.picked_path = Some(path.display().to_string());

                println!("Update picked path: {:?}", self.picked_path);

                //load file
                if let Some(picked_path) = &self.picked_path {
                    let path_data = Path::new(&picked_path);
                    match fs::read_to_string(path_data) {
                        Ok(content) => {
                            //println!("{}", content);
                            self.line_data = line_size_to_string(count_lines(&content));
                            self.gcode_data = content;
                        }
                        Err(error) => {println!("Please Select Gcode File")},
                    }
                    
                }

                }
        }
        if let Some(picked_path) = &self.picked_path {
            ui.horizontal(|ui| {
                ui.label("Picked file:");
                ui.monospace(picked_path);

            });

        }



        ui.spacing_mut().item_spacing = egui::vec2(0.0, 0.0);

        egui::ScrollArea::vertical()
            .max_height(500.0)
            .show(ui,  |ui| {

        StripBuilder::new(ui)
            .size(Size::remainder())
            .size(Size::remainder())
            .vertical(|mut strip| {
                strip.strip(|builder| {
                    builder
                        .size(Size::exact(100.0))
                        .size(Size::exact(400.0))
                        .horizontal(|mut strip| {
                            strip.cell(|ui| {
                                ui.spacing_mut().item_spacing = egui::vec2(5.0, 0.0);  
                                egui::Frame::none()
                                    .fill(Color32::from_rgb(53, 140, 100))
                                    .show(ui, |ui| {

                                        //let mut line_data = &line_size_to_string(count_lines(&self.gcode_data)) as &str;
                                        let mut l = &self.line_data as &str;
                                        let text_edit_output = egui::TextEdit::multiline(&mut l)
                                                                            //.min_size(ui.available_size())
                                                                            //.layouter(&mut layouter)
                                                                            .show(ui);

                                        let rect_data = text_edit_output.response.rect;
                                        let row_num = self.selected_row as f32;
                                        ui.painter().rect_filled(
                                            egui::Rect::from_min_max(egui::Pos2::new(rect_data.left(), rect_data.top()+(row_num*14.0)+1.0 ), egui::Pos2::new(rect_data.right(), rect_data.top()+(row_num*14.0)+1.0+14.0)),
                                            0.0,
                                            Color32::from_rgba_premultiplied(100, 0, 0, 20),
                                            //faded_color(Color32::from_rgb(77, 163, 109)),
                                            //Color32::from_rgb(164, 192, 176),
                                        );

                                    });
                            });

                            strip.cell(|ui| {
                                ui.spacing_mut().item_spacing = egui::vec2(5.0, 0.0);  
                                ui.style_mut().visuals.extreme_bg_color = Color32::from_rgb(164, 192, 176);
                                ui.style_mut().visuals.override_text_color = Some(Color32::from_rgb(33, 33, 33));
                                let mut layouter = |ui: &egui::Ui, string: &str, wrap_width: f32| {
                                    let mut layout_job = egui::text::LayoutJob::default();
                                    layout_job.append(
                                        string,
                                        0.0,     
                                        egui::TextFormat {
                                            font_id: egui::FontId::new(12.0, egui::FontFamily::Monospace),
                                            color: Color32::BLACK,
                                            //background: Color32::RED,
                                            ..Default::default()
                                        },);
                                    
                                    
                                    ui.fonts(|f| f.layout_job(layout_job))
                                };

                                
                                let text_edit_output = egui::TextEdit::multiline(&mut self.gcode_data)
                                                                    //.min_size(ui.available_size())
                                                                    .desired_rows(10)
                                                                    .desired_width(f32::INFINITY)
                                                                    .layouter(&mut layouter)
                                                                    .show(ui);

                                let rect_data = text_edit_output.response.rect;
                                let row_num = self.selected_row as f32;
                                ui.painter().rect_filled(
                                    egui::Rect::from_min_max(egui::Pos2::new(rect_data.left(), rect_data.top()+(row_num*14.0)+1.0 ), egui::Pos2::new(rect_data.right(), rect_data.top()+(row_num*14.0)+1.0+14.0)),
                                    0.0,
                                    Color32::from_rgba_premultiplied(100, 0, 0, 20),
                                );


                                let cursor_pos = text_edit_output.cursor_range;


                                if let Some(cursor_pos) = cursor_pos {

                                    self.selected_row = cursor_pos.primary.rcursor.row;
                                }
                            });
                    });
                });
            });

        });


    }
}